// Command gofacts reads the upstream Gateway API Go packages and writes the
// facts the xtask generates from, as one JSON file per release channel.
//
// Each channel is derived the way upstream's own CRD generator
// (tools/generator in sigs.k8s.io/gateway-api) derives it: a field whose doc
// has <gateway:experimental> is left out of the standard channel, and
// <gateway:CHANNEL:validation:Enum=...> replaces a field's enum values in that
// channel. Unlike upstream, an unknown <gateway:...> tag is an error.
package main

import (
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"os"
	"path/filepath"
	"reflect"
	"regexp"
	"sort"
	"strings"

	"golang.org/x/tools/go/packages"
)

const module = "sigs.k8s.io/gateway-api"

// The upstream API packages, relative to the module.
var apiPackages = []string{"apis/v1", "apis/v1beta1", "apis/v1alpha2", "apis/v1alpha3", "apisx/v1alpha1"}

// The Kubernetes package whose types the API packages use in fields.
const metav1 = "k8s.io/apimachinery/pkg/apis/meta/v1"

var channels = []string{"standard", "experimental"}

// Facts is one channel's view of the upstream Go packages.
type Facts struct {
	Module   string    `json:"module"`
	Version  string    `json:"version"`
	Channel  string    `json:"channel"`
	Packages []Package `json:"packages"`
}

type Package struct {
	// Relative to the module for upstream's packages, else the import path.
	Path string `json:"path"`
	Name string `json:"name"`
	// From the package's +groupName marker.
	Group  string  `json:"group,omitempty"`
	Kinds  []Kind  `json:"kinds,omitempty"`
	Types  []Type  `json:"types"`
	Consts []Const `json:"consts,omitempty"`
}

// Kind is a root type: one with +kubebuilder:object:root that isn't a list.
type Kind struct {
	Name   string   `json:"name"`
	Spec   *TypeRef `json:"spec,omitempty"`
	Status *TypeRef `json:"status,omitempty"`
}

type Type struct {
	Name string `json:"name"`
	File string `json:"file"`
	// alias (type X = Y), struct, scalar (a basic underlying type), or other.
	Kind string `json:"kind"`
	// For an alias, its right-hand side.
	Alias *TypeRef `json:"alias,omitempty"`
	// For a defined type whose right-hand side is a named type (type X v1.X).
	Defined    *TypeRef `json:"defined,omitempty"`
	Fields     []Field  `json:"fields,omitempty"`
	Basic      string   `json:"basic,omitempty"`
	Underlying *TypeRef `json:"underlying,omitempty"`
	// From a +kubebuilder:validation:Enum marker on the type.
	Enum    []string `json:"enum,omitempty"`
	Doc     string   `json:"doc,omitempty"`
	Markers []string `json:"markers,omitempty"`
}

type Field struct {
	Name string `json:"name"`
	// The JSON name; empty for an embedded field that's inlined.
	JSON     string   `json:"json,omitempty"`
	Inline   bool     `json:"inline,omitempty"`
	Type     *TypeRef `json:"type"`
	Optional bool     `json:"optional,omitempty"`
	// From a +kubebuilder:validation:Enum marker on the field, or the
	// channel's <gateway:CHANNEL:validation:Enum=...> override.
	Enum []string `json:"enum,omitempty"`
	// The raw value of a +kubebuilder:default marker.
	Default string   `json:"default,omitempty"`
	Doc     string   `json:"doc,omitempty"`
	Markers []string `json:"markers,omitempty"`
}

type Const struct {
	Name  string `json:"name"`
	Type  string `json:"type"`
	Value string `json:"value"`
	Doc   string `json:"doc,omitempty"`
}

// TypeRef is a type expression. Exactly one field is set.
type TypeRef struct {
	// A named type or alias, as <package path>.<name>.
	Named   string   `json:"named,omitempty"`
	Basic   string   `json:"basic,omitempty"`
	Pointer *TypeRef `json:"pointer,omitempty"`
	Slice   *TypeRef `json:"slice,omitempty"`
	Map     *MapRef  `json:"map,omitempty"`
}

type MapRef struct {
	Key   *TypeRef `json:"key"`
	Value *TypeRef `json:"value"`
}

func main() {
	out := flag.String("out", ".", "directory to write <channel>.json to")
	flag.Parse()
	if err := run(*out); err != nil {
		fmt.Fprintf(os.Stderr, "gofacts: %v\n", err)
		os.Exit(1)
	}
}

func run(out string) error {
	patterns := []string{metav1}
	for _, path := range apiPackages {
		patterns = append(patterns, module+"/"+path)
	}
	mode := packages.NeedName | packages.NeedFiles | packages.NeedSyntax | packages.NeedTypes |
		packages.NeedTypesInfo | packages.NeedModule | packages.NeedImports | packages.NeedDeps
	pkgs, err := packages.Load(&packages.Config{Mode: mode}, patterns...)
	if err != nil {
		return err
	}
	if packages.PrintErrors(pkgs) > 0 {
		return errors.New("failed to load packages")
	}
	byPath := map[string]*packages.Package{}
	for _, pkg := range pkgs {
		byPath[pkg.PkgPath] = pkg
	}

	r := &reader{fset: pkgs[0].Fset, docs: map[token.Pos]*docs{}, definedFrom: map[token.Pos]types.Type{}}
	for _, pkg := range pkgs {
		for _, file := range pkg.Syntax {
			r.index(file, pkg.TypesInfo)
		}
	}

	for _, channel := range channels {
		r.channel = channel
		facts := Facts{Module: module, Channel: channel}
		for _, path := range apiPackages {
			pkg := byPath[module+"/"+path]
			if pkg.Module == nil {
				return fmt.Errorf("%s: not loaded from a module", path)
			}
			facts.Version = pkg.Module.Version
			p, err := r.apiPackage(pkg)
			if err != nil {
				return fmt.Errorf("%s %s: %w", channel, path, err)
			}
			facts.Packages = append(facts.Packages, p)
		}
		external, err := r.externalPackage(byPath[metav1], facts.Packages)
		if err != nil {
			return fmt.Errorf("%s %s: %w", channel, metav1, err)
		}
		facts.Packages = append(facts.Packages, external)

		data, err := json.MarshalIndent(facts, "", "  ")
		if err != nil {
			return err
		}
		data = append(data, '\n')
		if err := os.WriteFile(filepath.Join(out, channel+".json"), data, 0o644); err != nil {
			return err
		}
	}
	return nil
}

// docs is the doc comment of one declaration, split into prose and markers.
type docs struct {
	text    string
	markers []string
}

type reader struct {
	fset    *token.FileSet
	channel string
	// By the position of the declared name: types, consts and struct fields.
	docs map[token.Pos]*docs
	// By the position of the declared name: the named type a defined type is
	// declared from, as in type TLSRouteRule v1.TLSRouteRule. go/types
	// doesn't keep it.
	definedFrom map[token.Pos]types.Type
}

// index records the docs of every type, const and field declared in a file. A
// type also takes the markers of a comment group just above its doc,
// separated by one blank line, as controller-gen does.
func (r *reader) index(file *ast.File, info *types.Info) {
	ast.Inspect(file, func(node ast.Node) bool {
		switch node := node.(type) {
		case *ast.GenDecl:
			for _, spec := range node.Specs {
				switch spec := spec.(type) {
				case *ast.TypeSpec:
					doc := spec.Doc
					if doc == nil && node.Lparen == token.NoPos {
						doc = node.Doc
					}
					d := parseDocs(doc)
					start := node.Pos()
					if doc != nil {
						start = doc.Pos()
					}
					if above := r.groupAbove(file, start); above != nil {
						d.markers = append(parseDocs(above).markers, d.markers...)
					}
					r.docs[spec.Name.Pos()] = d
					if !spec.Assign.IsValid() {
						switch rhs := info.TypeOf(spec.Type).(type) {
						case *types.Named, *types.Alias:
							r.definedFrom[spec.Name.Pos()] = rhs
						}
					}
				case *ast.ValueSpec:
					doc := spec.Doc
					if doc == nil && node.Lparen == token.NoPos {
						doc = node.Doc
					}
					for _, name := range spec.Names {
						r.docs[name.Pos()] = parseDocs(doc)
					}
				}
			}
		case *ast.StructType:
			for _, field := range node.Fields.List {
				d := parseDocs(field.Doc)
				if len(field.Names) == 0 {
					r.docs[embeddedPos(field.Type)] = d
				}
				for _, name := range field.Names {
					r.docs[name.Pos()] = d
				}
			}
		}
		return true
	})
}

// groupAbove returns the comment group that ends one blank line above line
// start, if any.
func (r *reader) groupAbove(file *ast.File, start token.Pos) *ast.CommentGroup {
	line := r.fset.Position(start).Line
	for _, group := range file.Comments {
		if r.fset.Position(group.End()).Line == line-2 {
			return group
		}
	}
	return nil
}

// embeddedPos is where go/types puts an embedded field's object.
func embeddedPos(expr ast.Expr) token.Pos {
	switch expr := expr.(type) {
	case *ast.StarExpr:
		return embeddedPos(expr.X)
	case *ast.SelectorExpr:
		return expr.Sel.Pos()
	default:
		return expr.Pos()
	}
}

func parseDocs(group *ast.CommentGroup) *docs {
	d := &docs{}
	if group == nil {
		return d
	}
	var lines []string
	for _, comment := range group.List {
		line := strings.TrimPrefix(comment.Text, "//")
		line = strings.TrimPrefix(line, " ")
		if marker, ok := strings.CutPrefix(strings.TrimSpace(line), "+"); ok {
			d.markers = append(d.markers, marker)
			continue
		}
		lines = append(lines, line)
	}
	d.text = strings.Join(lines, "\n")
	return d
}

func (r *reader) docsOf(pos token.Pos) *docs {
	if d, ok := r.docs[pos]; ok {
		return d
	}
	return &docs{}
}

var (
	tagRe          = regexp.MustCompile(`<gateway:([A-Za-z:]+)`)
	anyTagRe       = regexp.MustCompile(`<gateway:.*>`)
	blankLinesRe   = regexp.MustCompile(`\n{3,}`)
	knownTagPrefix = []string{"standard:validation:", "experimental:validation:"}
	knownTags      = map[string]bool{
		"experimental":             true,
		"experimental:description": true,
		"util:excludeFromCRD":      true,
		"validateIPAddress":        true,
	}
	knownValidations = map[string]bool{"Enum": true, "Pattern": true, "XValidation:message": true}
)

// checkTags fails on a <gateway:...> tag that upstream's generator doesn't
// handle, which it would silently strip.
func (r *reader) checkTags(text string, pos token.Pos) error {
	for _, match := range tagRe.FindAllStringSubmatch(text, -1) {
		tag := match[1]
		if knownTags[tag] {
			continue
		}
		known := false
		for _, prefix := range knownTagPrefix {
			if validation, ok := strings.CutPrefix(tag, prefix); ok && knownValidations[validation] {
				known = true
			}
		}
		if !known {
			return fmt.Errorf("%s: unknown tag <gateway:%s", r.fset.Position(pos), tag)
		}
	}
	return nil
}

func experimental(d *docs) bool {
	return strings.Contains(d.text, "<gateway:experimental>")
}

// cleanDoc removes upstream's tags from a doc, as upstream's formatDescription
// does for CRD descriptions: experimental-only paragraphs go in standard, and
// text excluded from the CRD goes everywhere.
func (r *reader) cleanDoc(text string) string {
	text = cutBlock(text, "experimental:description", r.channel == "standard")
	text = cutBlock(text, "util:excludeFromCRD", true)
	text = anyTagRe.ReplaceAllLiteralString(text, "")
	text = blankLinesRe.ReplaceAllString(text, "\n\n")
	return strings.Trim(text, "\n ")
}

// cutBlock removes <gateway:TAG>...</gateway:TAG> blocks, or only the tags
// when cut is false.
func cutBlock(text, tag string, cut bool) string {
	start, end := "<gateway:"+tag+">", "</gateway:"+tag+">"
	if !cut {
		return strings.ReplaceAll(strings.ReplaceAll(text, start, ""), end, "")
	}
	re := regexp.MustCompile(`\n*` + regexp.QuoteMeta(start) + `(?s:.*?)` + regexp.QuoteMeta(end) + `\n*`)
	return re.ReplaceAllString(text, "\n\n")
}

func markerValue(markers []string, name string) (string, bool) {
	for _, marker := range markers {
		if value, ok := strings.CutPrefix(marker, name+"="); ok {
			return value, true
		}
	}
	return "", false
}

func hasMarker(markers []string, name string) bool {
	for _, marker := range markers {
		if marker == name || strings.HasPrefix(marker, name+"=") || strings.HasPrefix(marker, name+":") {
			return true
		}
	}
	return false
}

func enumValues(markers []string) []string {
	value, ok := markerValue(markers, "kubebuilder:validation:Enum")
	if !ok {
		return nil
	}
	var values []string
	for _, v := range strings.Split(value, ";") {
		values = append(values, strings.Trim(v, `"`))
	}
	return values
}

func (r *reader) apiPackage(pkg *packages.Package) (Package, error) {
	p := Package{Path: pkgKey(pkg.PkgPath), Name: pkg.Name}
	for _, file := range pkg.Syntax {
		if group, ok := markerValue(parseDocs(file.Doc).markers, "groupName"); ok {
			p.Group = group
		}
	}

	scope := pkg.Types.Scope()
	for _, name := range r.sorted(scope) {
		obj := scope.Lookup(name)
		if !obj.Exported() {
			continue
		}
		switch obj := obj.(type) {
		case *types.TypeName:
			t, err := r.typeFacts(obj)
			if err != nil {
				return p, err
			}
			p.Types = append(p.Types, t)
			if kind, ok := kindFacts(t); ok {
				p.Kinds = append(p.Kinds, kind)
			}
		case *types.Const:
			named, ok := obj.Type().(*types.Named)
			if !ok || !strings.HasPrefix(named.Obj().Pkg().Path(), module+"/") {
				continue
			}
			d := r.docsOf(obj.Pos())
			if err := r.checkTags(d.text, obj.Pos()); err != nil {
				return p, err
			}
			if r.channel == "standard" && experimental(d) {
				continue
			}
			value := obj.Val().ExactString()
			if obj.Val().Kind() == constant.String {
				value = constant.StringVal(obj.Val())
			}
			p.Consts = append(p.Consts, Const{
				Name:  obj.Name(),
				Type:  objKey(named.Obj()),
				Value: value,
				Doc:   r.cleanDoc(d.text),
			})
		}
	}
	return p, nil
}

// externalPackage holds the types of pkg that upstream's non-root types
// use, and the types those use in turn.
func (r *reader) externalPackage(pkg *packages.Package, api []Package) (Package, error) {
	p := Package{Path: pkg.PkgPath, Name: pkg.Name}
	roots := map[string]bool{}
	for _, apiPkg := range api {
		for _, t := range apiPkg.Types {
			if hasMarker(t.Markers, "kubebuilder:object:root") {
				roots[t.Name] = true
			}
		}
	}

	var queue []string
	for _, apiPkg := range api {
		for _, t := range apiPkg.Types {
			if roots[t.Name] {
				continue
			}
			for _, f := range t.Fields {
				queue = append(queue, namedRefs(f.Type)...)
			}
		}
	}
	seen := map[string]bool{}
	scope := pkg.Types.Scope()
	for len(queue) > 0 {
		key := queue[0]
		queue = queue[1:]
		name, ok := strings.CutPrefix(key, pkg.PkgPath+".")
		if !ok || seen[name] {
			continue
		}
		seen[name] = true
		obj, ok := scope.Lookup(name).(*types.TypeName)
		if !ok {
			return p, fmt.Errorf("no type %s", key)
		}
		t, err := r.typeFacts(obj)
		if err != nil {
			return p, err
		}
		for _, f := range t.Fields {
			queue = append(queue, namedRefs(f.Type)...)
		}
		p.Types = append(p.Types, t)
	}
	sort.Slice(p.Types, func(i, j int) bool { return p.Types[i].Name < p.Types[j].Name })
	return p, nil
}

func namedRefs(ref *TypeRef) []string {
	switch {
	case ref == nil:
		return nil
	case ref.Named != "":
		return []string{ref.Named}
	case ref.Pointer != nil:
		return namedRefs(ref.Pointer)
	case ref.Slice != nil:
		return namedRefs(ref.Slice)
	case ref.Map != nil:
		return append(namedRefs(ref.Map.Key), namedRefs(ref.Map.Value)...)
	}
	return nil
}

// sorted returns a scope's names in source order.
func (r *reader) sorted(scope *types.Scope) []string {
	names := scope.Names()
	sort.SliceStable(names, func(i, j int) bool {
		a, b := r.fset.Position(scope.Lookup(names[i]).Pos()), r.fset.Position(scope.Lookup(names[j]).Pos())
		if a.Filename != b.Filename {
			return a.Filename < b.Filename
		}
		return a.Offset < b.Offset
	})
	return names
}

func kindFacts(t Type) (Kind, bool) {
	if !hasMarker(t.Markers, "kubebuilder:object:root") || strings.HasSuffix(t.Name, "List") {
		return Kind{}, false
	}
	kind := Kind{Name: t.Name}
	for _, f := range t.Fields {
		switch f.JSON {
		case "spec":
			kind.Spec = f.Type
		case "status":
			kind.Status = f.Type
		}
	}
	return kind, true
}

func (r *reader) typeFacts(obj *types.TypeName) (Type, error) {
	d := r.docsOf(obj.Pos())
	if err := r.checkTags(d.text, obj.Pos()); err != nil {
		return Type{}, err
	}
	t := Type{
		Name:    obj.Name(),
		File:    filepath.Base(r.fset.Position(obj.Pos()).Filename),
		Doc:     r.cleanDoc(d.text),
		Markers: d.markers,
		Enum:    enumValues(d.markers),
	}

	if alias, ok := obj.Type().(*types.Alias); ok {
		t.Kind = "alias"
		t.Alias = typeRef(alias.Rhs())
		return t, nil
	}
	named := obj.Type().(*types.Named)
	if rhs, ok := r.definedFrom[obj.Pos()]; ok {
		t.Defined = typeRef(rhs)
	}
	switch underlying := named.Underlying().(type) {
	case *types.Struct:
		t.Kind = "struct"
		for i := range underlying.NumFields() {
			f, err := r.fieldFacts(underlying.Field(i), underlying.Tag(i))
			if err != nil {
				return t, fmt.Errorf("%s: %w", obj.Name(), err)
			}
			if f != nil {
				t.Fields = append(t.Fields, *f)
			}
		}
	case *types.Basic:
		t.Kind = "scalar"
		t.Basic = underlying.Name()
	default:
		t.Kind = "other"
		t.Underlying = typeRef(underlying)
	}
	return t, nil
}

func (r *reader) fieldFacts(v *types.Var, tag string) (*Field, error) {
	jsonTag := reflect.StructTag(tag).Get("json")
	name, options, _ := strings.Cut(jsonTag, ",")
	if name == "-" || !v.Exported() && !v.Embedded() {
		return nil, nil
	}
	d := r.docsOf(v.Pos())
	if err := r.checkTags(d.text, v.Pos()); err != nil {
		return nil, err
	}
	if r.channel == "standard" && experimental(d) {
		return nil, nil
	}

	f := &Field{
		Name:    v.Name(),
		JSON:    name,
		Type:    typeRef(v.Type()),
		Doc:     r.cleanDoc(d.text),
		Markers: d.markers,
		Enum:    enumValues(d.markers),
	}
	omitEmpty := false
	for _, option := range strings.Split(options, ",") {
		switch option {
		case "inline":
			f.Inline = true
		case "omitempty", "omitzero":
			omitEmpty = true
		}
	}
	if v.Embedded() && name == "" {
		f.Inline = true
	}
	// controller-gen's rule, for packages that don't set a default.
	switch {
	case hasMarker(d.markers, "kubebuilder:validation:Optional"), hasMarker(d.markers, "optional"):
		f.Optional = true
	case hasMarker(d.markers, "kubebuilder:validation:Required"), hasMarker(d.markers, "required"):
		f.Optional = false
	default:
		f.Optional = omitEmpty
	}
	if value, ok := markerValue(d.markers, "kubebuilder:default"); ok {
		f.Default = value
	}
	override := regexp.MustCompile(`<gateway:` + r.channel + `:validation:Enum=([A-Za-z;]*)>`)
	if match := override.FindStringSubmatch(d.text); match != nil {
		f.Enum = strings.Split(match[1], ";")
	}
	return f, nil
}

func typeRef(t types.Type) *TypeRef {
	switch t := t.(type) {
	case *types.Alias:
		return &TypeRef{Named: objKey(t.Obj())}
	case *types.Named:
		return &TypeRef{Named: objKey(t.Obj())}
	case *types.Basic:
		return &TypeRef{Basic: t.Name()}
	case *types.Pointer:
		return &TypeRef{Pointer: typeRef(t.Elem())}
	case *types.Slice:
		return &TypeRef{Slice: typeRef(t.Elem())}
	case *types.Map:
		return &TypeRef{Map: &MapRef{Key: typeRef(t.Key()), Value: typeRef(t.Elem())}}
	default:
		return &TypeRef{Basic: "unsupported:" + t.String()}
	}
}

func objKey(obj types.Object) string {
	return pkgKey(obj.Pkg().Path()) + "." + obj.Name()
}

func pkgKey(path string) string {
	if rel, ok := strings.CutPrefix(path, module+"/"); ok {
		return rel
	}
	return path
}

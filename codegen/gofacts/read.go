package gofacts

import (
	"fmt"
	"go/ast"
	"go/constant"
	"go/token"
	"go/types"
	"path/filepath"
	"reflect"
	"sort"
	"strings"

	"golang.org/x/tools/go/packages"
)

// reader reads the API packages' declarations into the IR, for one profile
// at a time.
type reader struct {
	module string
	fset   *token.FileSet
	tags   *Tags
	// The profile being read.
	profile string
	// By the position of the declared name: types, consts and struct fields.
	docs map[token.Pos]doc
	// By the position of the declared name: the named type a defined type is
	// declared from, as in type TLSRouteRule v1.TLSRouteRule. go/types
	// doesn't keep it.
	definedFrom map[token.Pos]types.Type
}

func newReader(module string, tags *Tags, pkgs []*packages.Package) *reader {
	r := &reader{
		module:      module,
		fset:        pkgs[0].Fset,
		tags:        tags,
		docs:        map[token.Pos]doc{},
		definedFrom: map[token.Pos]types.Type{},
	}
	for _, pkg := range pkgs {
		for _, file := range pkg.Syntax {
			r.index(file, pkg.TypesInfo)
		}
	}
	return r
}

// index records the docs of every type, const and field declared in a file.
// A type also takes the markers of a comment group just above its doc,
// separated by one blank line, as controller-gen does.
func (r *reader) index(file *ast.File, info *types.Info) {
	ast.Inspect(file, func(node ast.Node) bool {
		switch node := node.(type) {
		case *ast.GenDecl:
			for _, spec := range node.Specs {
				switch spec := spec.(type) {
				case *ast.TypeSpec:
					group := spec.Doc
					if group == nil && node.Lparen == token.NoPos {
						group = node.Doc
					}
					d := parseDoc(group)
					start := node.Pos()
					if group != nil {
						start = group.Pos()
					}
					if above := r.groupAbove(file, start); above != nil {
						d.markers = append(parseDoc(above).markers, d.markers...)
					}
					r.docs[spec.Name.Pos()] = d
					if !spec.Assign.IsValid() {
						switch rhs := info.TypeOf(spec.Type).(type) {
						case *types.Named, *types.Alias:
							r.definedFrom[spec.Name.Pos()] = rhs
						}
					}
				case *ast.ValueSpec:
					group := spec.Doc
					if group == nil && node.Lparen == token.NoPos {
						group = node.Doc
					}
					for _, name := range spec.Names {
						r.docs[name.Pos()] = parseDoc(group)
					}
				}
			}
		case *ast.StructType:
			for _, field := range node.Fields.List {
				d := parseDoc(field.Doc)
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

// parseDoc splits a comment group into prose and markers, the lines that
// start with +.
func parseDoc(group *ast.CommentGroup) doc {
	var d doc
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

// docOf returns the doc of the declaration whose name is at pos, with the
// tag rules applied, and whether the rules drop it.
func (r *reader) docOf(pos token.Pos) (doc, bool, error) {
	d, dropped, err := r.tags.apply(r.profile, r.docs[pos])
	if err != nil {
		return d, false, fmt.Errorf("%s: %w", r.fset.Position(pos), err)
	}
	return d, dropped, nil
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

// apiPackage reads one of the config's packages.
func (r *reader) apiPackage(pkg *packages.Package) (Package, error) {
	p := Package{Path: r.key(pkg.PkgPath), Name: pkg.Name}
	for _, file := range pkg.Syntax {
		if group, ok := markerValue(parseDoc(file.Doc).markers, "groupName"); ok {
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
			t, err := r.typeDecl(obj)
			if err != nil {
				return p, err
			}
			p.Types = append(p.Types, t)
		case *types.Const:
			c, ok, err := r.constDecl(obj)
			if err != nil {
				return p, err
			}
			if ok {
				p.Consts = append(p.Consts, c)
			}
		}
	}
	return p, nil
}

// constDecl reads a const whose type is one of the module's named types,
// unless the tag rules drop it.
func (r *reader) constDecl(obj *types.Const) (Const, bool, error) {
	named, ok := obj.Type().(*types.Named)
	if !ok || !strings.HasPrefix(named.Obj().Pkg().Path(), r.module+"/") {
		return Const{}, false, nil
	}
	d, dropped, err := r.docOf(obj.Pos())
	if err != nil || dropped {
		return Const{}, false, err
	}
	value := obj.Val().ExactString()
	if obj.Val().Kind() == constant.String {
		value = constant.StringVal(obj.Val())
	}
	return Const{Name: obj.Name(), Type: r.objKey(named.Obj()), Value: value, Doc: d.text}, true, nil
}

// sorted returns a scope's names in source order: by file, then position.
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

func (r *reader) typeDecl(obj *types.TypeName) (Type, error) {
	d, _, err := r.docOf(obj.Pos())
	if err != nil {
		return Type{}, err
	}
	t := Type{
		Name:        obj.Name(),
		File:        filepath.Base(r.fset.Position(obj.Pos()).Filename),
		Doc:         d.text,
		Markers:     d.markers,
		Enum:        enumValues(d.markers),
		Root:        hasMarker(d.markers, "kubebuilder:object:root"),
		SkipVersion: hasMarker(d.markers, "kubebuilder:skipversion"),
	}

	if alias, ok := obj.Type().(*types.Alias); ok {
		t.Kind = "alias"
		t.Alias = r.typeRef(alias.Rhs())
		return t, nil
	}
	named := obj.Type().(*types.Named)
	if rhs, ok := r.definedFrom[obj.Pos()]; ok {
		t.Defined = r.typeRef(rhs)
	}
	switch underlying := named.Underlying().(type) {
	case *types.Struct:
		t.Kind = "struct"
		for i := range underlying.NumFields() {
			f, dropped, err := r.field(underlying.Field(i), underlying.Tag(i))
			if err != nil {
				return t, fmt.Errorf("%s: %w", obj.Name(), err)
			}
			switch {
			case f == nil:
			case dropped:
				t.Dropped = append(t.Dropped, DroppedField{Name: f.Name, Type: f.Type})
			default:
				t.Fields = append(t.Fields, *f)
			}
		}
	case *types.Basic:
		t.Kind = "scalar"
		t.Basic = underlying.Name()
	default:
		t.Kind = "other"
		t.Underlying = r.typeRef(underlying)
	}
	return t, nil
}

// field reads a struct field, or returns nil for one JSON leaves out.
func (r *reader) field(v *types.Var, tag string) (*Field, bool, error) {
	jsonTag := reflect.StructTag(tag).Get("json")
	name, options, _ := strings.Cut(jsonTag, ",")
	if name == "-" || !v.Exported() && !v.Embedded() {
		return nil, false, nil
	}
	d, dropped, err := r.docOf(v.Pos())
	if err != nil {
		return nil, false, err
	}

	f := &Field{
		Name:    v.Name(),
		JSON:    name,
		Type:    r.typeRef(v.Type()),
		Doc:     d.text,
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
	return f, dropped, nil
}

func (r *reader) typeRef(t types.Type) *TypeRef {
	switch t := t.(type) {
	case *types.Alias:
		return &TypeRef{Named: r.objKey(t.Obj())}
	case *types.Named:
		return &TypeRef{Named: r.objKey(t.Obj())}
	case *types.Basic:
		return &TypeRef{Basic: t.Name()}
	case *types.Pointer:
		return &TypeRef{Pointer: r.typeRef(t.Elem())}
	case *types.Slice:
		return &TypeRef{Slice: r.typeRef(t.Elem())}
	case *types.Map:
		return &TypeRef{Map: &MapRef{Key: r.typeRef(t.Key()), Value: r.typeRef(t.Elem())}}
	default:
		// The emitter fails on it, naming the Go type.
		return &TypeRef{Basic: "unsupported:" + t.String()}
	}
}

func (r *reader) objKey(obj types.Object) string {
	return r.key(obj.Pkg().Path()) + "." + obj.Name()
}

// key is a package's path relative to the module, or its import path for
// another module's package.
func (r *reader) key(path string) string {
	if rel, ok := strings.CutPrefix(path, r.module+"/"); ok {
		return rel
	}
	return path
}

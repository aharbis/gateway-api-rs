package gofacts

// IRVersion is the version of the IR format. It changes whenever the format
// does, and the Rust side rejects any other.
const IRVersion = 1

// IR is one profile's view of a Go module: its API packages with the
// config's tag rules applied, and the envelopes of the profile's CRDs. It's
// the contract between gofacts and the Rust emitter, documented in
// codegen/README.md.
type IR struct {
	IRVersion int    `json:"irVersion"`
	Module    string `json:"module"`
	// The module version go.mod pins.
	Version  string    `json:"version"`
	Profile  string    `json:"profile"`
	Packages []Package `json:"packages"`
	CRDs     []CRD     `json:"crds"`
}

// Package is one of the config's Go packages.
type Package struct {
	// Relative to the module.
	Path string `json:"path"`
	Name string `json:"name"`
	// From the package's +groupName marker.
	Group  string  `json:"group,omitempty"`
	Types  []Type  `json:"types"`
	Consts []Const `json:"consts,omitempty"`
}

// Type is an exported type declaration.
type Type struct {
	Name string `json:"name"`
	// The file it's declared in, without its directory.
	File string `json:"file"`
	// alias (type X = Y), struct, scalar (a basic underlying type), or
	// other.
	Kind string `json:"kind"`
	// For an alias, its right-hand side.
	Alias *TypeRef `json:"alias,omitempty"`
	// For a defined type declared from a named type, as in
	// type TLSRouteRule v1.TLSRouteRule, that type.
	Defined *TypeRef `json:"defined,omitempty"`
	// For a struct, its fields, without those the tag rules drop.
	Fields []Field `json:"fields,omitempty"`
	// For a struct, the fields the tag rules drop: their names and types.
	Dropped []DroppedField `json:"dropped,omitempty"`
	// For a scalar, its basic type, e.g. string.
	Basic string `json:"basic,omitempty"`
	// For any other type, its underlying type.
	Underlying *TypeRef `json:"underlying,omitempty"`
	// From the type's +kubebuilder:validation:Enum marker.
	Enum []string `json:"enum,omitempty"`
	// Whether it has +kubebuilder:object:root.
	Root bool `json:"root,omitempty"`
	// Whether it has +kubebuilder:skipversion.
	SkipVersion bool     `json:"skipVersion,omitempty"`
	Doc         string   `json:"doc,omitempty"`
	Markers     []string `json:"markers,omitempty"`
}

// Field is a struct field, or an embedded struct.
type Field struct {
	Name string `json:"name"`
	// The JSON name; empty for an inlined embedded field.
	JSON   string   `json:"json,omitempty"`
	Inline bool     `json:"inline,omitempty"`
	Type   *TypeRef `json:"type"`
	// By controller-gen's rule: +optional or +kubebuilder:validation:Optional,
	// else omitempty or omitzero, unless +required or
	// +kubebuilder:validation:Required.
	Optional bool `json:"optional,omitempty"`
	// From the field's own +kubebuilder:validation:Enum marker, after
	// overrides.
	Enum []string `json:"enum,omitempty"`
	// The raw value of the field's +kubebuilder:default marker.
	Default string   `json:"default,omitempty"`
	Doc     string   `json:"doc,omitempty"`
	Markers []string `json:"markers,omitempty"`
}

// DroppedField is a field the tag rules drop in the profile.
type DroppedField struct {
	Name string   `json:"name"`
	Type *TypeRef `json:"type"`
}

// Const is a const whose type is a named type of one of the packages.
type Const struct {
	Name string `json:"name"`
	// Its type, as a TypeRef's named key.
	Type string `json:"type"`
	// A string's value, or the exact value of any other kind.
	Value string `json:"value"`
	Doc   string `json:"doc,omitempty"`
}

// TypeRef is a type expression. Exactly one field is set.
type TypeRef struct {
	// A named type or alias, as <package>.<name>, where <package> is
	// relative to the module for its packages, else an import path.
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

// CRD is a CustomResourceDefinition's envelope: what it says about the
// resource, apart from the schema.
type CRD struct {
	// The file it's read from, in the profile's CRD directory.
	File       string   `json:"file"`
	Group      string   `json:"group"`
	Kind       string   `json:"kind"`
	ListKind   string   `json:"listKind,omitempty"`
	Plural     string   `json:"plural"`
	Singular   string   `json:"singular,omitempty"`
	Scope      string   `json:"scope"`
	ShortNames []string `json:"shortNames,omitempty"`
	Categories []string `json:"categories,omitempty"`
	// In the CRD's order.
	Versions []CRDVersion `json:"versions"`
}

type CRDVersion struct {
	Name string `json:"name"`
	// The package the version's Go kind is in.
	Package            string          `json:"package"`
	Served             bool            `json:"served"`
	Storage            bool            `json:"storage"`
	Deprecated         bool            `json:"deprecated,omitempty"`
	DeprecationWarning string          `json:"deprecationWarning,omitempty"`
	Status             bool            `json:"status,omitempty"`
	PrinterColumns     []PrinterColumn `json:"printerColumns,omitempty"`
}

type PrinterColumn struct {
	Name        string `json:"name" yaml:"name"`
	Type        string `json:"type" yaml:"type"`
	JSONPath    string `json:"jsonPath" yaml:"jsonPath"`
	Description string `json:"description,omitempty" yaml:"description"`
	Format      string `json:"format,omitempty" yaml:"format"`
	Priority    int32  `json:"priority,omitempty" yaml:"priority"`
}

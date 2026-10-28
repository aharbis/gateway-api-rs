package gofacts

import (
	"reflect"
	"strings"
	"testing"

	"go.yaml.in/yaml/v2"
)

const testTags = `
prefix: gw
rules:
  - tag: experimental
    drop: item
    in: [standard]
  - tag: experimental:description
    drop: block
    in: [standard]
  - tag: internal
    drop: block
    in: all
  - tag: "{profile}:validation:Enum"
    override: kubebuilder:validation:Enum
  - tag: "{profile}:validation:Pattern"
`

func loadTestTags(t *testing.T) *Tags {
	t.Helper()
	var tags Tags
	if err := yaml.UnmarshalStrict([]byte(testTags), &tags); err != nil {
		t.Fatal(err)
	}
	if err := tags.check(map[string]bool{"standard": true, "experimental": true}); err != nil {
		t.Fatal(err)
	}
	return &tags
}

func TestApply(t *testing.T) {
	tags := loadTestTags(t)
	text := strings.Join([]string{
		"Type is the type.",
		"",
		"<gw:experimental:description>",
		"Only in experimental.",
		"</gw:experimental:description>",
		"",
		"<gw:internal>",
		"Notes for implementors.",
		"</gw:internal>",
		"",
		"Support: Core <gw:standard:validation:Pattern=`^[a-z>]+$`>",
		"<gw:experimental:validation:Enum=A;B;C>",
	}, "\n")
	markers := []string{"kubebuilder:validation:Enum=A;B", "optional"}

	tests := []struct {
		profile string
		want    doc
	}{
		{"standard", doc{
			text:    "Type is the type.\n\nSupport: Core",
			markers: []string{"kubebuilder:validation:Enum=A;B", "optional"},
		}},
		{"experimental", doc{
			text:    "Type is the type.\n\nOnly in experimental.\n\nSupport: Core",
			markers: []string{"kubebuilder:validation:Enum=A;B;C", "optional"},
		}},
	}
	for _, test := range tests {
		got, dropped, err := tags.apply(test.profile, doc{text: text, markers: markers})
		if err != nil {
			t.Fatal(err)
		}
		if dropped || !reflect.DeepEqual(got, test.want) {
			t.Errorf("%s: got %#v (dropped %v), want %#v", test.profile, got, dropped, test.want)
		}
	}
}

func TestApplyDropsItems(t *testing.T) {
	tags := loadTestTags(t)
	for profile, want := range map[string]bool{"standard": true, "experimental": false} {
		_, dropped, err := tags.apply(profile, doc{text: "A field.\n<gw:experimental>"})
		if err != nil || dropped != want {
			t.Errorf("%s: dropped %v (%v), want %v", profile, dropped, err, want)
		}
	}
}

func TestApplyFails(t *testing.T) {
	tags := loadTestTags(t)
	for text, want := range map[string]string{
		"<gw:experimentl>":                 "unknown tag <gw:experimentl>",
		"<gw:other:validation:Enum=A>":     "unknown tag <gw:other:validation:Enum>",
		"<gw:internal>\nnever closed":      "has no closing tag",
		"</gw:nothing>":                    "unknown tag </gw:nothing>",
		"<gw:standard:validation:Pattern=": "malformed tag",
		"<gw:standard:validation:Pattern":  "unterminated",
	} {
		_, _, err := tags.apply("standard", doc{text: text})
		if err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%q: got %v, want an error containing %q", text, err, want)
		}
	}
}

func TestCheck(t *testing.T) {
	profiles := map[string]bool{"standard": true}
	for rules, want := range map[string]string{
		"[{tag: a, drop: item}]":                                   "drop without in",
		"[{tag: a, in: all}]":                                      "in without drop",
		"[{tag: a, drop: item, in: [nightly]}]":                    "isn't a profile",
		"[{tag: a, override: m}]":                                  "must contain {profile}",
		"[{tag: a, drop: some, in: all}]":                          "not item or block",
		"[{tag: a}, {tag: a}]":                                     "more than one rule",
		"[{tag: '{profile}:a', drop: item, in: all, override: m}]": "both drop and override",
	} {
		var tags Tags
		if err := yaml.UnmarshalStrict([]byte("prefix: gw\nrules: "+rules), &tags); err != nil {
			t.Fatal(err)
		}
		err := tags.check(profiles)
		if err == nil || !strings.Contains(err.Error(), want) {
			t.Errorf("%s: got %v, want an error containing %q", rules, err, want)
		}
	}
}

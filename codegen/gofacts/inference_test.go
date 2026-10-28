package gofacts

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// TestInferenceExtension runs gofacts on the Inference Extension, a second
// Go-based API group: plain controller-gen, one CRD directory, no profiles
// and no tags. testdata/inference-extension pins it.
func TestInferenceExtension(t *testing.T) {
	out := t.TempDir()
	if err := Run(Options{Config: "testdata/inference-extension/config.yaml", Out: out}); err != nil {
		t.Fatal(err)
	}

	entries, err := os.ReadDir(out)
	if err != nil {
		t.Fatal(err)
	}
	if len(entries) != 1 || entries[0].Name() != DefaultProfile+".json" {
		t.Fatalf("got IR files %v, want one, %s.json", entries, DefaultProfile)
	}
	data, err := os.ReadFile(filepath.Join(out, entries[0].Name()))
	if err != nil {
		t.Fatal(err)
	}
	var ir IR
	if err := json.Unmarshal(data, &ir); err != nil {
		t.Fatal(err)
	}

	if ir.IRVersion != IRVersion || ir.Profile != DefaultProfile || ir.Version != "v1.6.2" {
		t.Errorf("got IR version %d, profile %s, module version %s", ir.IRVersion, ir.Profile, ir.Version)
	}
	groups := map[string]string{}
	for _, p := range ir.Packages {
		groups[p.Path] = p.Group
	}
	wantGroups := map[string]string{
		"api/v1":        "inference.networking.k8s.io",
		"apix/v1alpha1": "inference.networking.x-k8s.io",
	}
	for path, group := range wantGroups {
		if groups[path] != group {
			t.Errorf("package %s: got group %q, want %q", path, groups[path], group)
		}
	}
	kinds := map[string]string{}
	for _, crd := range ir.CRDs {
		for _, v := range crd.Versions {
			kinds[crd.Kind+" "+v.Name] = v.Package
		}
	}
	wantKinds := map[string]string{
		"InferencePool v1":             "api/v1",
		"InferencePoolImport v1alpha1": "apix/v1alpha1",
	}
	for kind, pkg := range wantKinds {
		if kinds[kind] != pkg {
			t.Errorf("CRD version %s: got package %q, want %q", kind, kinds[kind], pkg)
		}
	}
}

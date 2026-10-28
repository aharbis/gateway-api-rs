package gofacts

import (
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"go.yaml.in/yaml/v2"
)

// crdFile is a CRD document and the file it's read from.
type crdFile struct {
	name string
	data []byte
	crd  crdDoc
}

// crdDoc is the part of a CustomResourceDefinition the envelope needs.
type crdDoc struct {
	APIVersion string `yaml:"apiVersion"`
	Kind       string `yaml:"kind"`
	Metadata   struct {
		Annotations map[string]string `yaml:"annotations"`
	} `yaml:"metadata"`
	Spec struct {
		Group string `yaml:"group"`
		Names struct {
			Kind       string   `yaml:"kind"`
			ListKind   string   `yaml:"listKind"`
			Plural     string   `yaml:"plural"`
			Singular   string   `yaml:"singular"`
			ShortNames []string `yaml:"shortNames"`
			Categories []string `yaml:"categories"`
		} `yaml:"names"`
		Scope    string `yaml:"scope"`
		Versions []struct {
			Name               string  `yaml:"name"`
			Served             bool    `yaml:"served"`
			Storage            bool    `yaml:"storage"`
			Deprecated         bool    `yaml:"deprecated"`
			DeprecationWarning *string `yaml:"deprecationWarning"`
			Subresources       *struct {
				Status *struct{} `yaml:"status"`
				Scale  *struct{} `yaml:"scale"`
			} `yaml:"subresources"`
			AdditionalPrinterColumns []PrinterColumn `yaml:"additionalPrinterColumns"`
			SelectableFields         []any           `yaml:"selectableFields"`
		} `yaml:"versions"`
	} `yaml:"spec"`
}

// readCRDs reads the CustomResourceDefinitions in dir, sorted by file name,
// skipping files that hold anything else. Each is copied as a fixture, so a
// file that holds a CRD must hold only that.
func readCRDs(dir string) ([]crdFile, error) {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return nil, err
	}
	var crds []crdFile
	for _, entry := range entries {
		if entry.IsDir() || !strings.HasSuffix(entry.Name(), ".yaml") {
			continue
		}
		file := filepath.Join(dir, entry.Name())
		data, err := os.ReadFile(file)
		if err != nil {
			return nil, err
		}
		docs := strings.Split(string(data), "\n---\n")
		var found []crdDoc
		for _, d := range docs {
			var crd crdDoc
			if err := yaml.Unmarshal([]byte(d), &crd); err != nil {
				return nil, fmt.Errorf("%s: %w", file, err)
			}
			if crd.APIVersion == "apiextensions.k8s.io/v1" && crd.Kind == "CustomResourceDefinition" {
				found = append(found, crd)
			}
		}
		switch {
		case len(found) == 0:
			continue
		case len(docs) > 1:
			return nil, fmt.Errorf("%s: a CRD file holds more than one YAML document", file)
		}
		crds = append(crds, crdFile{name: entry.Name(), data: data, crd: found[0]})
	}
	sort.Slice(crds, func(i, j int) bool { return crds[i].name < crds[j].name })
	return crds, nil
}

// envelope checks a CRD against the Go packages and returns its envelope.
// It fails when the CRD's bundle version isn't the module's, or when a CRD
// version has no Go kind, or one marked +kubebuilder:skipversion.
func envelope(f crdFile, version string, packages []Package) (CRD, error) {
	c := f.crd
	for key, value := range c.Metadata.Annotations {
		if strings.HasSuffix(key, "/bundle-version") && value != version {
			return CRD{}, fmt.Errorf("%s: %s is %s, but go.mod pins %s", f.name, key, value, version)
		}
	}
	names := c.Spec.Names
	crd := CRD{
		File:       f.name,
		Group:      c.Spec.Group,
		Kind:       names.Kind,
		ListKind:   names.ListKind,
		Plural:     names.Plural,
		Singular:   names.Singular,
		Scope:      c.Spec.Scope,
		ShortNames: names.ShortNames,
		Categories: names.Categories,
	}
	for _, v := range c.Spec.Versions {
		pkg, err := kindPackage(packages, c.Spec.Group, v.Name, names.Kind)
		if err != nil {
			return CRD{}, fmt.Errorf("%s %s: %w", f.name, v.Name, err)
		}
		cv := CRDVersion{
			Name:           v.Name,
			Package:        pkg,
			Served:         v.Served,
			Storage:        v.Storage,
			Deprecated:     v.Deprecated,
			PrinterColumns: v.AdditionalPrinterColumns,
		}
		if v.DeprecationWarning != nil {
			cv.DeprecationWarning = *v.DeprecationWarning
		}
		if s := v.Subresources; s != nil {
			if s.Scale != nil {
				return CRD{}, fmt.Errorf("%s %s: the scale subresource isn't supported", f.name, v.Name)
			}
			cv.Status = s.Status != nil
		}
		if len(v.SelectableFields) > 0 {
			return CRD{}, fmt.Errorf("%s %s: selectable fields aren't supported", f.name, v.Name)
		}
		crd.Versions = append(crd.Versions, cv)
	}
	return crd, nil
}

// kindPackage finds the package that declares a CRD version's Go kind: the
// one whose group is the CRD's and whose name is the version.
func kindPackage(packages []Package, group, version, kind string) (string, error) {
	for _, p := range packages {
		if p.Group != group || p.Name != version {
			continue
		}
		for _, t := range p.Types {
			if t.Name != kind || !t.Root {
				continue
			}
			if t.SkipVersion {
				return "", fmt.Errorf("Go kind %s.%s is marked +kubebuilder:skipversion", p.Path, kind)
			}
			return p.Path, nil
		}
		return "", fmt.Errorf("no Go kind %s in %s", kind, p.Path)
	}
	return "", fmt.Errorf("no Go package for %s/%s", group, version)
}

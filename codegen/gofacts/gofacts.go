// Package gofacts reads a Go module's API packages and CRDs and writes the IR
// a crate is generated from: one JSON file per profile in the config.
//
// A crate's codegen directory holds a config.yaml, a go.mod that pins the
// module, and a main package that calls Main. See codegen/README.md.
package gofacts

import (
	"encoding/json"
	"errors"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"golang.org/x/tools/go/packages"
)

// Options are Run's inputs.
type Options struct {
	// The config file. Packages load from its directory, under the go.mod
	// there, and the IR is written to ir/ next to it.
	Config string
	// The repository root, for the config's crate and fixtures paths.
	// Empty means the config is at <root>/<crate>/codegen/config.yaml.
	Root string
}

// Main runs gofacts with the options in the command-line flags and exits on
// failure.
func Main() {
	var opts Options
	flag.StringVar(&opts.Config, "config", "config.yaml", "the codegen config")
	flag.StringVar(&opts.Root, "root", "", "the repository root (default: from the config's location)")
	flag.Parse()
	if err := Run(opts); err != nil {
		fmt.Fprintf(os.Stderr, "gofacts: %v\n", err)
		os.Exit(1)
	}
}

// Run writes the IR of each profile in the config, and copies each profile's
// CRDs to its fixtures directory if the config has one.
func Run(opts Options) error {
	config, err := LoadConfig(opts.Config)
	if err != nil {
		return err
	}
	configFile, err := filepath.Abs(opts.Config)
	if err != nil {
		return err
	}
	dir := filepath.Dir(configFile)

	pkgs, err := load(dir, config)
	if err != nil {
		return err
	}
	module := pkgs[0].Module
	r := newReader(config.Module, config.Tags, pkgs)

	irDir := filepath.Join(dir, "ir")
	if err := os.MkdirAll(irDir, 0o755); err != nil {
		return err
	}
	written := map[string]bool{}
	for _, profile := range config.Profiles {
		ir, crds, err := readProfile(r, pkgs, module.Dir, module.Version, profile)
		if err != nil {
			return fmt.Errorf("profile %s: %w", profile.Name, err)
		}
		data, err := json.MarshalIndent(ir, "", "  ")
		if err != nil {
			return err
		}
		name := profile.Name + ".json"
		if err := os.WriteFile(filepath.Join(irDir, name), append(data, '\n'), 0o644); err != nil {
			return err
		}
		written[name] = true

		if config.Fixtures != "" {
			root, err := repoRoot(opts.Root, dir, config.Crate)
			if err != nil {
				return err
			}
			fixtures := filepath.Join(root, config.Crate, config.Fixtures, profile.Name)
			source := config.Module + "@" + module.Version + "/" + profile.CRDs
			if err := writeFixtures(fixtures, source, crds); err != nil {
				return err
			}
		}
	}
	return removeStale(irDir, ".json", written)
}

// load loads the config's packages from the module go.mod in dir pins.
func load(dir string, config *Config) ([]*packages.Package, error) {
	var patterns []string
	for path := range config.Packages {
		patterns = append(patterns, config.Module+"/"+path)
	}
	sort.Strings(patterns)
	mode := packages.NeedName | packages.NeedFiles | packages.NeedSyntax | packages.NeedTypes |
		packages.NeedTypesInfo | packages.NeedModule | packages.NeedImports | packages.NeedDeps
	pkgs, err := packages.Load(&packages.Config{Mode: mode, Dir: dir}, patterns...)
	if err != nil {
		return nil, err
	}
	if packages.PrintErrors(pkgs) > 0 {
		return nil, errors.New("failed to load packages")
	}
	sort.Slice(pkgs, func(i, j int) bool { return pkgs[i].PkgPath < pkgs[j].PkgPath })
	for _, pkg := range pkgs {
		if pkg.Module == nil || pkg.Module.Path != config.Module {
			return nil, fmt.Errorf("%s isn't loaded from module %s: check the go.mod in %s", pkg.PkgPath, config.Module, dir)
		}
		if pkg.Module.Version != pkgs[0].Module.Version {
			return nil, fmt.Errorf("%s is loaded at two versions", config.Module)
		}
	}
	return pkgs, nil
}

// readProfile reads one profile's IR, and its CRD files.
func readProfile(r *reader, pkgs []*packages.Package, moduleDir, version string, profile Profile) (*IR, []crdFile, error) {
	r.profile = profile.Name
	ir := &IR{IRVersion: IRVersion, Module: r.module, Version: version, Profile: profile.Name}
	for _, pkg := range pkgs {
		p, err := r.apiPackage(pkg)
		if err != nil {
			return nil, nil, err
		}
		ir.Packages = append(ir.Packages, p)
	}
	crds, err := readCRDs(filepath.Join(moduleDir, profile.CRDs))
	if err != nil {
		return nil, nil, err
	}
	if len(crds) == 0 {
		return nil, nil, fmt.Errorf("no CRDs in %s", profile.CRDs)
	}
	for _, f := range crds {
		crd, err := envelope(f, version, ir.Packages)
		if err != nil {
			return nil, nil, err
		}
		ir.CRDs = append(ir.CRDs, crd)
	}
	return ir, crds, nil
}

// repoRoot is root, or the directory the config's directory is crate/codegen
// in.
func repoRoot(root, configDir, crate string) (string, error) {
	if root != "" {
		return root, nil
	}
	crateDir := filepath.Dir(configDir)
	suffix := string(filepath.Separator) + filepath.FromSlash(filepath.Clean(crate))
	if filepath.Base(configDir) != "codegen" || !strings.HasSuffix(crateDir, suffix) {
		return "", fmt.Errorf("%s isn't %s/codegen: pass -root", configDir, crate)
	}
	return strings.TrimSuffix(crateDir, suffix), nil
}

const fixtureHeader = `# WARNING: generated file — do not edit
# Copied unchanged below this header by gofacts from
# %s/%s
`

// writeFixtures copies CRD files to dir, with a header naming their source,
// and removes other YAML files there.
func writeFixtures(dir, source string, crds []crdFile) error {
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return err
	}
	written := map[string]bool{}
	for _, f := range crds {
		data := append([]byte(fmt.Sprintf(fixtureHeader, source, f.name)), f.data...)
		if err := os.WriteFile(filepath.Join(dir, f.name), data, 0o644); err != nil {
			return err
		}
		written[f.name] = true
	}
	return removeStale(dir, ".yaml", written)
}

// removeStale removes the files in dir with the extension that weren't just
// written.
func removeStale(dir, ext string, written map[string]bool) error {
	entries, err := os.ReadDir(dir)
	if err != nil {
		return err
	}
	for _, entry := range entries {
		if !entry.IsDir() && strings.HasSuffix(entry.Name(), ext) && !written[entry.Name()] {
			if err := os.Remove(filepath.Join(dir, entry.Name())); err != nil {
				return err
			}
		}
	}
	return nil
}

package gofacts

import (
	"errors"
	"fmt"
	"os"
	"path"
	"strings"

	"go.yaml.in/yaml/v2"
)

// Config is one generated crate's codegen config, config.yaml. The Rust
// emitter reads the same file; gofacts only uses the keys that concern the Go
// side, but rejects unknown ones like the emitter does.
type Config struct {
	// The Go module the types come from, pinned by the go.mod next to the
	// config.
	Module string `yaml:"module"`
	// The crate's directory, relative to the repository root.
	Crate string `yaml:"crate"`
	// The Go packages to read, relative to the module, and the Rust module
	// path each becomes, e.g. apisx/v1alpha1: apisx::v1alpha1.
	Packages map[string]string `yaml:"packages"`
	// The profiles: one generated module tree each. Without them, CRDs and
	// Out make one profile named "default".
	Profiles []Profile `yaml:"profiles,omitempty"`
	CRDs     string    `yaml:"crds,omitempty"`
	Out      string    `yaml:"out,omitempty"`
	Tags     *Tags     `yaml:"tags,omitempty"`
	// Files whose types are generated even where no CRD uses them.
	LibraryFiles []string `yaml:"library_files,omitempty"`
	// Rust paths for Go types from other modules, by package and type name.
	External map[string]map[string]string `yaml:"external,omitempty"`
	// Where CRD fixtures are copied to, relative to the crate, one
	// directory per profile.
	Fixtures string `yaml:"fixtures,omitempty"`
}

// Profile is one generated module tree, from one CRD directory.
type Profile struct {
	Name string `yaml:"name"`
	// The CRD directory, relative to the module.
	CRDs string `yaml:"crds"`
	// The module tree, relative to the crate's src/.
	Out string `yaml:"out"`
	// The Cargo feature that gates the tree, if any.
	Feature string `yaml:"feature,omitempty"`
}

// DefaultProfile is the name of the one profile a config without profiles has.
const DefaultProfile = "default"

// LoadConfig reads and checks a config file.
func LoadConfig(file string) (*Config, error) {
	data, err := os.ReadFile(file)
	if err != nil {
		return nil, err
	}
	var c Config
	if err := yaml.UnmarshalStrict(data, &c); err != nil {
		return nil, fmt.Errorf("%s: %w", file, err)
	}
	if err := c.check(); err != nil {
		return nil, fmt.Errorf("%s: %w", file, err)
	}
	return &c, nil
}

func (c *Config) check() error {
	if c.Module == "" || c.Crate == "" || len(c.Packages) == 0 {
		return errors.New("module, crate and packages are required")
	}
	switch {
	case len(c.Profiles) == 0 && (c.CRDs == "" || c.Out == ""):
		return errors.New("without profiles, crds and out are required")
	case len(c.Profiles) == 0:
		c.Profiles = []Profile{{Name: DefaultProfile, CRDs: c.CRDs, Out: c.Out}}
	case c.CRDs != "" || c.Out != "":
		return errors.New("crds and out go in each profile when there are profiles")
	}
	names := map[string]bool{}
	for _, p := range c.Profiles {
		if p.Name == "" || p.CRDs == "" || p.Out == "" {
			return errors.New("each profile needs a name, crds and out")
		}
		if names[p.Name] {
			return fmt.Errorf("profile %s is defined twice", p.Name)
		}
		names[p.Name] = true
		if path.IsAbs(p.Out) || path.Clean(p.Out) != p.Out || strings.HasPrefix(p.Out, "..") {
			return fmt.Errorf("profile %s: out %q isn't a clean relative path", p.Name, p.Out)
		}
	}
	if c.Tags != nil {
		if err := c.Tags.check(names); err != nil {
			return fmt.Errorf("tags: %w", err)
		}
	}
	return nil
}

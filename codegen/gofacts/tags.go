package gofacts

import (
	"errors"
	"fmt"
	"regexp"
	"sort"
	"strings"
)

// Tags configures the tags in Go doc comments, such as upstream Gateway API's
// <gateway:experimental>. A tag is <PREFIX:NAME> or <PREFIX:NAME=VALUE>; a
// block tag also has a closing </PREFIX:NAME>.
type Tags struct {
	Prefix string `yaml:"prefix"`
	Rules  []Rule `yaml:"rules"`

	// The configured profile names, for {profile} in rule tags.
	profiles []string
}

// Rule says what one tag does. Without drop or override, it only declares
// the tag known.
type Rule struct {
	// The tag's name. {profile} matches any profile's name.
	Tag string `yaml:"tag"`
	// item drops every field or const whose doc has the tag; block drops
	// the doc text between the opening and closing tag.
	Drop string `yaml:"drop,omitempty"`
	// The profiles drop applies in.
	In *Profiles `yaml:"in,omitempty"`
	// A marker, such as kubebuilder:validation:Enum, whose value the tag's
	// value replaces in the profile {profile} matched.
	Override string `yaml:"override,omitempty"`
}

// Profiles is a list of profile names, or all of them.
type Profiles struct {
	All   bool
	Names []string
}

func (p *Profiles) UnmarshalYAML(unmarshal func(any) error) error {
	var all string
	if unmarshal(&all) == nil {
		if all != "all" {
			return fmt.Errorf("in: %q is neither a list of profiles nor all", all)
		}
		p.All = true
		return nil
	}
	return unmarshal(&p.Names)
}

func (p *Profiles) has(profile string) bool {
	if p.All {
		return true
	}
	for _, name := range p.Names {
		if name == profile {
			return true
		}
	}
	return false
}

const profilePlaceholder = "{profile}"

func (t *Tags) check(profiles map[string]bool) error {
	if t.Prefix == "" {
		return errors.New("prefix is required")
	}
	for name := range profiles {
		t.profiles = append(t.profiles, name)
	}
	sort.Strings(t.profiles)
	seen := map[string]bool{}
	for _, r := range t.Rules {
		if r.Tag == "" {
			return errors.New("a rule has no tag")
		}
		if seen[r.Tag] {
			return fmt.Errorf("%s: more than one rule", r.Tag)
		}
		seen[r.Tag] = true
		switch r.Drop {
		case "":
			if r.In != nil {
				return fmt.Errorf("%s: in without drop", r.Tag)
			}
		case "item", "block":
			if r.In == nil {
				return fmt.Errorf("%s: drop without in", r.Tag)
			}
			for _, name := range r.In.Names {
				if !profiles[name] {
					return fmt.Errorf("%s: in names %s, which isn't a profile", r.Tag, name)
				}
			}
		default:
			return fmt.Errorf("%s: drop is %q, not item or block", r.Tag, r.Drop)
		}
		if r.Drop != "" && r.Override != "" {
			return fmt.Errorf("%s: both drop and override", r.Tag)
		}
		if r.Override != "" && !strings.Contains(r.Tag, profilePlaceholder) {
			return fmt.Errorf("%s: an override's tag must contain %s", r.Tag, profilePlaceholder)
		}
	}
	return nil
}

// tag is one tag found in a doc.
type tag struct {
	name    string
	value   string
	closing bool
	// Its byte span in the doc.
	start, end int
	rule       *Rule
	// The profile {profile} matched, if the rule's tag has it.
	profile string
}

// match finds the rule for a tag name.
func (t *Tags) match(name string) (*Rule, string) {
	for i := range t.Rules {
		r := &t.Rules[i]
		if !strings.Contains(r.Tag, profilePlaceholder) {
			if r.Tag == name {
				return r, ""
			}
			continue
		}
		for _, profile := range t.profiles {
			if strings.ReplaceAll(r.Tag, profilePlaceholder, profile) == name {
				return r, profile
			}
		}
	}
	return nil, ""
}

// find returns the tags in a doc. A tag's value runs to the last > on its
// line, since values such as CEL rules can hold >.
func (t *Tags) find(text string) ([]tag, error) {
	var tags []tag
	open := "<" + t.Prefix + ":"
	for i := 0; i < len(text); {
		j := strings.Index(text[i:], "<")
		if j < 0 {
			break
		}
		start := i + j
		rest := text[start:]
		closing := strings.HasPrefix(rest, "</"+t.Prefix+":")
		if !closing && !strings.HasPrefix(rest, open) {
			i = start + 1
			continue
		}
		nameStart := start + len(open)
		if closing {
			nameStart++
		}
		lineEnd := strings.IndexByte(text[start:], '\n')
		if lineEnd < 0 {
			lineEnd = len(text)
		} else {
			lineEnd += start
		}
		line := text[nameStart:lineEnd]
		k := strings.IndexAny(line, "=>")
		if k < 0 {
			return nil, fmt.Errorf("unterminated tag %q", text[start:lineEnd])
		}
		tg := tag{name: line[:k], closing: closing, start: start}
		if line[k] == '=' {
			last := strings.LastIndexByte(line, '>')
			if closing || last < k {
				return nil, fmt.Errorf("malformed tag %q", text[start:lineEnd])
			}
			tg.value = line[k+1 : last]
			tg.end = nameStart + last + 1
		} else {
			tg.end = nameStart + k + 1
		}
		tg.rule, tg.profile = t.match(tg.name)
		if tg.rule == nil {
			slash := ""
			if closing {
				slash = "/"
			}
			return nil, fmt.Errorf("unknown tag <%s%s:%s>: add a rule for it to the config", slash, t.Prefix, tg.name)
		}
		tags = append(tags, tg)
		i = tg.end
	}
	return tags, nil
}

// doc is a declaration's doc comment, split into prose and markers.
type doc struct {
	text    string
	markers []string
}

// apply applies the tag rules in profile to a doc: it reports whether the
// item is dropped, applies overrides to the markers, drops blocks and strips
// every tag left.
func (t *Tags) apply(profile string, d doc) (doc, bool, error) {
	if t == nil {
		return doc{text: tidy(d.text), markers: d.markers}, false, nil
	}
	tags, err := t.find(d.text)
	if err != nil {
		return d, false, err
	}
	dropped := false
	markers := append([]string(nil), d.markers...)
	type span struct{ start, end int }
	var cut []span
	for i := 0; i < len(tags); i++ {
		tg := tags[i]
		r := tg.rule
		applies := r.In != nil && r.In.has(profile)
		switch {
		case tg.closing:
		case r.Drop == "item" && applies:
			dropped = true
		case r.Override != "" && tg.profile == profile:
			markers = override(markers, r.Override, tg.value)
		case r.Drop == "block" && applies:
			end := -1
			for j := i + 1; j < len(tags); j++ {
				if tags[j].closing && tags[j].name == tg.name {
					end = j
					break
				}
			}
			if end < 0 {
				return d, false, fmt.Errorf("<%s:%s> has no closing tag", t.Prefix, tg.name)
			}
			cut = append(cut, span{tg.start, tags[end].end})
			i = end
			continue
		}
		cut = append(cut, span{tg.start, tg.end})
	}

	var b strings.Builder
	last := 0
	for _, s := range cut {
		b.WriteString(d.text[last:s.start])
		last = s.end
	}
	b.WriteString(d.text[last:])
	return doc{text: tidy(b.String()), markers: markers}, dropped, nil
}

// override sets a marker's value, replacing the marker if it's there.
func override(markers []string, name, value string) []string {
	for i, m := range markers {
		if m == name || strings.HasPrefix(m, name+"=") {
			markers[i] = name + "=" + value
			return markers
		}
	}
	return append(markers, name+"="+value)
}

var blankLines = regexp.MustCompile(`\n{3,}`)

// tidy trims trailing space from each line, collapses runs of blank lines
// and trims blank lines at either end.
func tidy(text string) string {
	lines := strings.Split(text, "\n")
	for i, line := range lines {
		lines[i] = strings.TrimRight(line, " \t")
	}
	text = blankLines.ReplaceAllString(strings.Join(lines, "\n"), "\n\n")
	return strings.Trim(text, "\n")
}

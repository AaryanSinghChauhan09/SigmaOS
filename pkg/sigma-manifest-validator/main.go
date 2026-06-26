// SPDX-License-Identifier: GPL-2.0-or-later
// sigma-pkg manifest validator — validates a sigma-manifest.toml against the
// SigmaOS package manifest schema (Alpine/NixOS inspired).
//
// Usage:
//   sigma-validate ./path/to/sigma-manifest.toml
//   sigma-validate --sbom ./pkg/sigma-manifest.toml   # also emit SPDX SBOM
//
// Exit codes:
//   0  — manifest is valid
//   1  — validation errors found (details printed to stderr)
//   2  — file not found / parse error

package main

import (
	"encoding/hex"
	"flag"
	"fmt"
	"os"
	"regexp"
	"strings"

	"github.com/BurntSushi/toml"
)

// ---- Schema types ----------------------------------------------------------

type Manifest struct {
	Package      PackageSection      `toml:"package"`
	Dependencies DependenciesSection `toml:"dependencies"`
	Files        []FileEntry         `toml:"files"`
	Signing      SigningSection      `toml:"signing"`
}

type PackageSection struct {
	Name        string `toml:"name"`
	Version     string `toml:"version"`
	Arch        string `toml:"arch"`
	License     string `toml:"license"`
	Description string `toml:"description"`
	Maintainer  string `toml:"maintainer"`
	URL         string `toml:"url"`
}

type DependenciesSection struct {
	Runtime []string `toml:"runtime"`
	Build   []string `toml:"build"`
	Shlibs  []string `toml:"shlibs"`
}

type FileEntry struct {
	Path   string `toml:"path"`
	SHA256 string `toml:"sha256"`
	Mode   string `toml:"mode"`
	Size   int64  `toml:"size"`
}

type SigningSection struct {
	Algorithm string `toml:"algorithm"`
	Pubkey    string `toml:"pubkey"`
	Signature string `toml:"signature"`
}

// ---- Validation ------------------------------------------------------------

type ValidationError struct {
	Field   string
	Message string
}

func (e ValidationError) Error() string {
	return fmt.Sprintf("  [%s] %s", e.Field, e.Message)
}

var (
	validArches  = map[string]bool{"x86_64": true, "aarch64": true, "riscv64": true, "any": true}
	validAlgos   = map[string]bool{"ed25519": true, "dilithium3": true}
	semverRe     = regexp.MustCompile(`^\d+\.\d+\.\d+$`)
	nameRe       = regexp.MustCompile(`^[a-z][a-z0-9-]*$`)
	modeRe       = regexp.MustCompile(`^0[0-7]{3}$`)
	sha256Re     = regexp.MustCompile(`^[0-9a-f]{64}$`)
	maintainerRe = regexp.MustCompile(`^.+@.+$`)
)

func validate(m *Manifest) []ValidationError {
	var errs []ValidationError

	add := func(field, msg string) {
		errs = append(errs, ValidationError{field, msg})
	}

	// [package]
	if m.Package.Name == "" {
		add("package.name", "required field missing")
	} else if !nameRe.MatchString(m.Package.Name) {
		add("package.name", "must match [a-z][a-z0-9-]* (e.g. 'zenith-browser')")
	}

	if !semverRe.MatchString(m.Package.Version) {
		add("package.version", fmt.Sprintf("must be semver X.Y.Z, got %q", m.Package.Version))
	}

	if !validArches[m.Package.Arch] {
		add("package.arch", fmt.Sprintf("must be one of: x86_64, aarch64, riscv64, any — got %q", m.Package.Arch))
	}

	if m.Package.License == "" {
		add("package.license", "required — use an SPDX identifier, e.g. GPL-2.0-or-later")
	}

	if m.Package.Maintainer == "" {
		add("package.maintainer", "required — unmaintained packages must use orphan@sigma.os")
	} else if !maintainerRe.MatchString(m.Package.Maintainer) {
		add("package.maintainer", fmt.Sprintf("must be an email address, got %q", m.Package.Maintainer))
	}

	// [signing]
	if !validAlgos[m.Signing.Algorithm] {
		add("signing.algorithm",
			fmt.Sprintf("must be 'ed25519' or 'dilithium3' — got %q. "+
				"NOTE: Kyber-1024 is a KEM, not a signature scheme; do not use it here.", m.Signing.Algorithm))
	}
	if m.Signing.Pubkey == "" {
		add("signing.pubkey", "required")
	}
	if m.Signing.Signature == "" {
		add("signing.signature", "required")
	}

	// [[files]]
	if len(m.Files) == 0 {
		add("files", "at least one [[files]] entry is required")
	}
	for i, f := range m.Files {
		prefix := fmt.Sprintf("files[%d]", i)
		if !strings.HasPrefix(f.Path, "/") {
			add(prefix+".path", fmt.Sprintf("must be absolute (start with /), got %q", f.Path))
		}
		rawSHA := strings.ToLower(strings.TrimSpace(f.SHA256))
		if !sha256Re.MatchString(rawSHA) {
			add(prefix+".sha256", fmt.Sprintf("must be 64 lowercase hex chars, got %q", f.SHA256))
		} else {
			// Verify it decodes cleanly (no invalid hex chars slipped past the regex)
			if _, err := hex.DecodeString(rawSHA); err != nil {
				add(prefix+".sha256", "invalid hex encoding")
			}
		}
		if !modeRe.MatchString(f.Mode) {
			add(prefix+".mode", fmt.Sprintf("must be octal like '0755', got %q", f.Mode))
		}
		if f.Size <= 0 {
			add(prefix+".size", "must be a positive integer (bytes)")
		}
	}

	// [dependencies] shlibs format: "libfoo.so.1  package:version"
	for i, s := range m.Dependencies.Shlibs {
		parts := strings.Fields(s)
		if len(parts) != 2 || !strings.Contains(parts[1], ":") {
			add(fmt.Sprintf("dependencies.shlibs[%d]", i),
				fmt.Sprintf("must be 'libfoo.so.N  package:minversion', got %q", s))
		}
	}

	return errs
}

// ---- SBOM generation (minimal SPDX 2.3 text) ------------------------------

func emitSBOM(m *Manifest) {
	fmt.Printf("SPDXVersion: SPDX-2.3\n")
	fmt.Printf("DataLicense: CC0-1.0\n")
	fmt.Printf("SPDXID: SPDXRef-DOCUMENT\n")
	fmt.Printf("DocumentName: %s-%s\n", m.Package.Name, m.Package.Version)
	fmt.Printf("DocumentNamespace: https://sigma.os/sbom/%s-%s\n\n", m.Package.Name, m.Package.Version)
	fmt.Printf("PackageName: %s\n", m.Package.Name)
	fmt.Printf("SPDXID: SPDXRef-Package-%s-%s\n", m.Package.Name, m.Package.Version)
	fmt.Printf("PackageVersion: %s\n", m.Package.Version)
	fmt.Printf("PackageLicense: %s\n", m.Package.License)
	if m.Package.URL != "" {
		fmt.Printf("PackageHomePage: %s\n", m.Package.URL)
	}
	fmt.Printf("FilesAnalyzed: true\n\n")
	for _, f := range m.Files {
		fmt.Printf("FileName: %s\n", f.Path)
		fmt.Printf("FileChecksum: SHA256: %s\n\n", strings.ToLower(f.SHA256))
	}
}

// ---- Main ------------------------------------------------------------------

func main() {
	sbomFlag := flag.Bool("sbom", false, "also emit an SPDX 2.3 SBOM to stdout")
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Usage: sigma-validate [--sbom] <path/to/sigma-manifest.toml>\n")
		flag.PrintDefaults()
	}
	flag.Parse()

	args := flag.Args()
	if len(args) != 1 {
		flag.Usage()
		os.Exit(2)
	}

	data, err := os.ReadFile(args[0])
	if err != nil {
		fmt.Fprintf(os.Stderr, "error: cannot read %s: %v\n", args[0], err)
		os.Exit(2)
	}

	var m Manifest
	if _, err := toml.Decode(string(data), &m); err != nil {
		fmt.Fprintf(os.Stderr, "error: TOML parse error in %s:\n  %v\n", args[0], err)
		os.Exit(2)
	}

	errs := validate(&m)
	if len(errs) > 0 {
		fmt.Fprintf(os.Stderr, "✗ %s has %d error(s):\n", args[0], len(errs))
		for _, e := range errs {
			fmt.Fprintln(os.Stderr, e)
		}
		os.Exit(1)
	}

	fmt.Printf("✓ %s is valid\n", args[0])
	fmt.Printf("  name: %s\n  version: %s\n  arch: %s\n  files: %d\n",
		m.Package.Name, m.Package.Version, m.Package.Arch, len(m.Files))

	if *sbomFlag {
		fmt.Println()
		emitSBOM(&m)
	}
}

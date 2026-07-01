// SPDX-License-Identifier: GPL-2.0-or-later
// sigmad/pkg/generations/generations.go — Guix-style generation management
//
// Packages are never modified in place. Install creates a new generation
// (symlink tree → store paths). Rollback is an atomic symlink swap.
// No package removal needed — just switch which generation is active.
//
// CLI:
//   sigma-pkg generations list
//   sigma-pkg generations rollback          ← implements sigma_rollback.cpp (was 404)
//   sigma-pkg generations switch 2
//   sigma-pkg generations delete 1

package generations

import (
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"time"
)

const (
	GenerationsDir = "/sigma/var/sigma-pkg/generations"
	CurrentProfile = "/sigma/profile"        // symlink → active generation
	StoreDir       = "/sigma/store"           // /sigma/store/<hash>-name-version/
)

// ── Types ─────────────────────────────────────────────────────────────────────

type PackageRef struct {
	Name      string `json:"name"`
	Version   string `json:"version"`
	StorePath string `json:"store_path"` // /sigma/store/<drv-hash>-name-version/
}

type Generation struct {
	Number      int          `json:"number"`
	Timestamp   time.Time    `json:"timestamp"`
	Packages    []PackageRef `json:"packages"`
	ProfilePath string       `json:"profile_path"` // /sigma/store/<hash>-profile-N/
	Active      bool         `json:"active"`
	Description string       `json:"description,omitempty"`
}

// ── Storage ───────────────────────────────────────────────────────────────────

func genDir(n int) string { return filepath.Join(GenerationsDir, fmt.Sprintf("%04d", n)) }

func saveGeneration(g *Generation) error {
	dir := genDir(g.Number)
	os.MkdirAll(dir, 0o700)
	b, err := json.MarshalIndent(g, "", "  ")
	if err != nil { return err }
	return os.WriteFile(filepath.Join(dir, "manifest.json"), b, 0o600)
}

func ListGenerations() ([]Generation, error) {
	entries, err := os.ReadDir(GenerationsDir)
	if err != nil {
		if os.IsNotExist(err) { return nil, nil }
		return nil, err
	}
	current, _ := os.Readlink(CurrentProfile)
	var gens []Generation
	for _, e := range entries {
		if !e.IsDir() { continue }
		b, err := os.ReadFile(filepath.Join(GenerationsDir, e.Name(), "manifest.json"))
		if err != nil { continue }
		var g Generation
		if json.Unmarshal(b, &g) == nil {
			g.Active = (g.ProfilePath == current)
			gens = append(gens, g)
		}
	}
	sort.Slice(gens, func(i, j int) bool { return gens[i].Number < gens[j].Number })
	return gens, nil
}

// ── Profile symlink tree ──────────────────────────────────────────────────────

// buildProfileSymtree creates /sigma/store/<hash>-profile-N/ with symlinks to
// each installed package's store path. This is the Guix "profile" concept.
func buildProfileSymtree(pkgs []PackageRef, genNum int) (string, error) {
	// Compute a deterministic hash of the package list
	h := sha256.New()
	for _, p := range pkgs {
		h.Write([]byte(p.StorePath))
	}
	hash := hex.EncodeToString(h.Sum(nil))[:16]
	profilePath := filepath.Join(StoreDir, fmt.Sprintf("%s-profile-%d", hash, genNum))

	os.MkdirAll(filepath.Join(profilePath, "bin"), 0o755)
	os.MkdirAll(filepath.Join(profilePath, "lib"), 0o755)
	os.MkdirAll(filepath.Join(profilePath, "share"), 0o755)

	// Symlink each package's bin/ lib/ share/ into the profile
	for _, p := range pkgs {
		for _, sub := range []string{"bin", "lib", "share"} {
			src := filepath.Join(p.StorePath, sub)
			if _, err := os.Stat(src); err != nil { continue }
			entries, _ := os.ReadDir(src)
			for _, e := range entries {
				link := filepath.Join(profilePath, sub, e.Name())
				target := filepath.Join(src, e.Name())
				os.Remove(link) // remove stale link
				os.Symlink(target, link)
			}
		}
	}
	return profilePath, nil
}

// ── CRUD ──────────────────────────────────────────────────────────────────────

// mergePackages adds new packages and removes named ones from the current set
func mergePackages(current []PackageRef, add []PackageRef, remove []string) []PackageRef {
	removeSet := make(map[string]bool)
	for _, r := range remove { removeSet[r] = true }
	var result []PackageRef
	for _, p := range current {
		if !removeSet[p.Name] { result = append(result, p) }
	}
	result = append(result, add...)
	return result
}

// CreateGeneration installs/removes packages by creating a new immutable generation.
func CreateGeneration(add []PackageRef, remove []string, desc string) (*Generation, error) {
	gens, _ := ListGenerations()
	var current []PackageRef
	if len(gens) > 0 { current = gens[len(gens)-1].Packages }

	next := Generation{
		Number:      len(gens) + 1,
		Timestamp:   time.Now().UTC(),
		Packages:    mergePackages(current, add, remove),
		Description: desc,
	}

	profilePath, err := buildProfileSymtree(next.Packages, next.Number)
	if err != nil { return nil, fmt.Errorf("build profile: %w", err) }
	next.ProfilePath = profilePath

	if err := saveGeneration(&next); err != nil { return nil, err }
	if err := Activate(next.Number); err != nil { return nil, err }
	return &next, nil
}

// Activate atomically switches /sigma/profile to the given generation.
func Activate(n int) error {
	gens, _ := ListGenerations()
	var target *Generation
	for i := range gens {
		if gens[i].Number == n { target = &gens[i]; break }
	}
	if target == nil { return fmt.Errorf("generation %d not found", n) }

	// Atomic symlink swap (POSIX rename is atomic)
	tmp := CurrentProfile + ".new"
	os.Remove(tmp)
	if err := os.Symlink(target.ProfilePath, tmp); err != nil { return err }
	if err := os.Rename(tmp, CurrentProfile); err != nil { return err }

	fmt.Printf("[sigma-pkg] Generation %d active (%s)\n",
		target.Number, target.Timestamp.Format(time.RFC3339))
	return nil
}

// Rollback atomically switches to the previous generation.
// This FINALLY implements sigma_rollback.cpp (previously a 404 file).
func Rollback() error {
	gens, _ := ListGenerations()
	var active, prev int
	for _, g := range gens {
		if g.Active { active = g.Number }
	}
	// Find the generation just before the active one
	for _, g := range gens {
		if g.Number < active { prev = g.Number }
	}
	if prev == 0 { return fmt.Errorf("no previous generation to roll back to") }
	fmt.Printf("[sigma-pkg] Rolling back: generation %d → %d\n", active, prev)
	return Activate(prev)
}

// DeleteGeneration removes a non-active generation's manifest.
// Store objects are retained (GC handles orphan collection separately).
func DeleteGeneration(n int) error {
	gens, _ := ListGenerations()
	for _, g := range gens {
		if g.Number == n && g.Active {
			return fmt.Errorf("cannot delete active generation %d", n)
		}
	}
	return os.RemoveAll(genDir(n))
}

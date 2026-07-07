# Package Manager — sigpkg

> SigmaOS v15.0 "Zenith" — Package Manager Reference

## Overview

`sigpkg` is the native SigmaOS package manager — a unified hybrid combining the best of `apt`, `dnf`, `pacman`, and `nix`. It features:

- **SAT-solver** dependency resolution (DPLL-based)
- **Dilithium-5** post-quantum package signing
- **Generation-based rollback** (atomic transactions)
- **AI-assisted recommendations**
- **GUI front-end** for desktop environments

---

## Quick Reference

```bash
# Install a package
sigpkg install firefox

# Remove a package
sigpkg remove firefox

# Update all packages
sigpkg upgrade

# Search for packages
sigpkg search "text editor"

# AI-powered recommendations
sigpkg recommend "I want to browse the web"

# Rollback to previous generation
sigpkg rollback

# List installed generations
sigpkg list-generations
```

---

## Architecture

### Core (`sigma-pkg/sigma_pkg_core.rs`)

The resolver uses a **DPLL-based SAT solver** to handle complex dependency graphs:

1. Build a constraint set from all package dependencies
2. Apply unit propagation to simplify clauses
3. Select branching variables (heuristic: most constrained first)
4. Backtrack on conflicts
5. Return a satisfying assignment = installation plan

**Transaction model:**
- All changes are applied atomically to a new **generation**
- Generations are symlinked: `/usr/sigma-gen/current → gen-42`
- Rollback = `sigpkg rollback` switches the symlink

---

### Repository Manager (`sigma-pkg/sigma_pkg_repo.rs`)

| Feature | Details |
|---|---|
| Index format | `index.json.zst` (compressed JSON) |
| Signature | Dilithium-5 over SHA-3-512 of index |
| Mirrors | Priority-ordered, auto-failover |
| Cache | `~/.cache/sigma-pkg/` (SHA-3 content-addressed) |
| Transport | HTTPS only; TLS 1.3 minimum |

**Repository configuration** (`/etc/sigma/repos.toml`):
```toml
[[repo]]
name = "sigma-main"
url = "https://packages.sigmaos.dev/main"
priority = 100
enabled = true
public_key = "/etc/sigma/keys/sigma-main.pub"

[[repo]]
name = "sigma-community"
url = "https://community.sigmaos.dev"
priority = 50
enabled = true
```

---

### GUI (`sigma-pkg/sigma_pkg_gui.rs`)

The graphical front-end connects to the `sigpkgd` daemon via Unix socket and provides:

- Category browser
- Search with AI recommendations
- Install/Remove with progress bar
- Update manager with changelog preview
- Snapshot and rollback management

---

### AI Recommender (`sigma-pkg/sigma_pkg_ai_recommender.rs`)

Uses the local AI agent to parse natural-language queries:

```
Input:  "I need something to edit photos"
Output: [gimp, darktable, rawtherapee]
```

Ranked by: popularity + dependency footprint + compatibility score.

---

## Package Format (`.sigma`)

| Field | Description |
|---|---|
| Metadata | TOML header (name, version, deps, license) |
| Content | Zstandard-compressed tar archive |
| Signature | Dilithium-5 detached signature file |
| Checksum | SHA-3-512 of compressed archive |

---

## Security Model

1. All packages **must** be signed by a trusted key in `/etc/sigma/keys/`
2. New keys require manual user approval (`sigpkg trust-key <fingerprint>`)
3. Packages cannot execute post-install scripts by default (opt-in via `--allow-scripts`)
4. All files are installed with minimal permissions and verified against the manifest

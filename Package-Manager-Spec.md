# SigmaOS Package Manager Specification

The Sovereign Package Manager (`sigma-pkg`) provides reproducible, cryptographically verified, atomic package management.

---

## Package Format: `.spkg`

A `.spkg` archive contains:

```
myapp-1.0.0.spkg
├── sigma.manifest.json   ← package metadata + dependencies
├── content/              ← application files
├── BLAKE3.sum            ← content hashes for every file
├── signature.dilithium5  ← Dilithium-5 signature over BLAKE3.sum
└── build.proof           ← reproducible build attestation
```

### sigma.manifest.json

```json
{
  "name": "myapp",
  "version": "1.0.0",
  "arch": ["x86_64", "arm64"],
  "description": "My SigmaOS application",
  "license": "MIT",
  "depends": ["libsigma>=15.0", "sigma-runtime>=1.0"],
  "sigma_bus": ["com.example.myapp"],
  "capabilities": ["stdio", "inet", "rpath"],
  "unveil": [
    { "path": "/home/$USER/.config/myapp", "perm": "rw" }
  ],
  "build_hash": "blake3:abc123...",
  "signed_by": "developer@example.com"
}
```

---

## CLI Reference

```bash
# Install
sigma-pkg install firefox
sigma-pkg install ./myapp-1.0.0.spkg    # local file
sigma-pkg install firefox@120.0.0       # specific version

# Remove
sigma-pkg remove firefox
sigma-pkg remove firefox --purge        # also removes config

# Update
sigma-pkg update                        # all packages
sigma-pkg update firefox               # single package
sigma-pkg update --delta               # incremental delta only

# Search
sigma-pkg search "text editor"
sigma-pkg search --tag dev-tools

# Info
sigma-pkg info firefox
sigma-pkg info --files firefox          # list installed files

# List
sigma-pkg list                          # installed packages
sigma-pkg list --upgradeable           # packages with updates

# Verify
sigma-pkg verify firefox               # check signature + hashes
sigma-pkg verify --all                 # verify all installed packages

# Build
sigma-pkg build ./myapp/               # build from source
sigma-pkg sign myapp-1.0.0.spkg        # sign a package

# Repo management
sigma-pkg repo add https://repo.sigmaos.dev/main
sigma-pkg repo list
sigma-pkg repo remove old-repo
sigma-pkg repo sync                    # refresh package lists
```

---

## Reproducible Builds

Every `.spkg` in the official registry is reproducibly built:
1. Build environment is a hermetic sigma-pod container
2. All inputs (source, deps, build tools) are content-addressed
3. Build hash recorded in `build.proof`
4. Any developer can verify: `sigma-pkg verify --repro firefox`

---

## Delta Updates

`sigma-pkg update --delta` uses binary diff (bsdiff-compatible) to transmit only changed bytes between versions, reducing bandwidth by 60-90% for large packages.

---

## Registry

Official registry: `https://repo.sigmaos.dev/`

| Channel | Description |
|---------|-------------|
| `main` | Stable releases |
| `updates` | Security + bug fix updates |
| `backports` | Newer versions backported to stable |
| `testing` | Pre-release, may be unstable |
| `community` | Community-maintained packages |

---

## Atomic Installs

Package installs are atomic — either the full install completes or nothing changes:

```
sigma-pkg install firefox
  1. Download + verify signature
  2. Extract to /sigma/pkg/staging/firefox-120/
  3. Run pre-install hooks
  4. Atomic rename: staging/ → /sigma/pkg/installed/firefox/
  5. Register in /sigma/pkg/db/
  6. Run post-install hooks
  → On failure at any step: staging/ is removed, system unchanged
```

---

## Source Files

| File | Purpose |
|------|---------|
| `pkg/sigma_pkg_core.cpp` | Core package engine |
| `pkg/sigma_pkg_cli.cpp` | CLI frontend |
| `pkg/SovereignPkgManager.cpp` | High-level manager |
| `sigma_pkg_registry/sigma_pkg_recipe.c` | Package recipe format |
| `userland/pkg/sigma_registry.cpp` | Registry client |
| `tools/sigma_pkg.cpp` | Tool integration |

---

*See also: [App-Manifest](App-Manifest) · [Sovereign-Packaging-Specification](Sovereign-Packaging-Specification) · [System-Daemons](System-Daemons)*

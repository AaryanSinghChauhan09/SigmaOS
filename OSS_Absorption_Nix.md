# OSS Absorption: Nix Package Manager & Reproducible Builds

> **Status**: 🔄 Active | **Source Project**: Nix 2.18 / Nix Flakes | **Target Shard**: `SigmaOS Package Integrity / Reproducible Build Layer`

---

## 1. Executive Summary

The Nix package manager is the world's most sophisticated approach to reproducible builds and declarative system configuration. Its core innovation is the **Nix store**: a content-addressed, immutable directory tree (`/nix/store/`) where every package is identified by a cryptographic hash of all its inputs (source, dependencies, build flags, compiler version).

SigmaOS absorbs three key Nix innovations:
1. **Content-addressed package store** — `sigma-store` (`/sigma/store/`)
2. **Reproducible builds** — every SigmaOS package can be rebuilt bit-for-bit from its derivation
3. **Nix Flakes**-style declarative system specs — `sigma.toml` (the NixOS-equivalent config)

---

## 2. Architecture

```
┌────────────────────────────────────────────────────────────────┐
│             SIGMA CONTENT-ADDRESSED PACKAGE STORE              │
│                                                                │
│  Package Request: "firefox 120.0 with wayland+av1"            │
│                         │                                      │
│                         ▼                                      │
│  Hash = SHA256(name + version + deps_hash + features + src)   │
│  = "abc123def456..."                                          │
│                         │                                      │
│         ┌───────────────┴──────────────────┐                  │
│         │ Cache hit?                        │                  │
│     YES ▼                              NO  ▼                  │
│  /sigma/store/abc123-firefox     Build from derivation        │
│        (symlink current)          → Store at /sigma/store/abc  │
│                         │                                      │
│  All paths IMMUTABLE, no package modifies another             │
│  Multiple versions co-exist without conflict                  │
└────────────────────────────────────────────────────────────────┘
```

---

## 3. Key Features

### 3.1 Content-Addressed Store (`sigma-store`)

```rust
// userland/package_manager/store.rs
// SPDX-License-Identifier: MIT

pub struct SigmaStore {
    root: PathBuf,    // /sigma/store/
    db:   StoreDb,    // SQLite index of all known derivations
}

pub struct Derivation {
    pub name:     String,
    pub version:  Version,
    pub src_hash: Sha256,
    pub deps:     Vec<StoreHash>,   // Hashes of all dependency derivations
    pub builder:  BuildScript,      // How to build this package
    pub env:      HashMap<String, String>,
}

impl SigmaStore {
    pub fn derive_hash(drv: &Derivation) -> StoreHash {
        // Deterministic: same inputs → always same hash
        let mut hasher = Sha256::new();
        hasher.update(drv.name.as_bytes());
        hasher.update(drv.version.to_string().as_bytes());
        for dep in &drv.deps {
            hasher.update(dep.as_bytes());
        }
        hasher.update(&drv.builder.hash());
        StoreHash::from(hasher.finalize())
    }

    pub fn build(&mut self, drv: Derivation) -> Result<StorePath> {
        let hash = Self::derive_hash(&drv);
        let path = self.root.join(format!("{hash}-{}", drv.name));

        if path.exists() {
            println!("Σ [STORE] Cache hit: {path:?}");
            return Ok(path);
        }

        // Build in isolated sandbox (no network, no /etc access)
        let sandbox = BuildSandbox::new(&drv)?;
        sandbox.run()?;
        self.make_immutable(&path)?;  // chmod -R -w to prevent mutation

        Ok(path)
    }

    pub fn gc(&mut self) -> Result<usize> {
        // Remove store paths not reachable from any current profile/root
        let roots = self.db.get_gc_roots()?;
        let live = self.reachable_paths(&roots);
        let dead = self.db.all_paths()?.difference(&live).cloned().collect();
        self.remove_paths(dead)
    }
}
```

### 3.2 Declarative System Configuration (`sigma.toml` — NixOS-style)

```toml
# /etc/sigma/sigma.toml — Declarative system spec
[system]
hostname  = "my-workstation"
timezone  = "Asia/Kolkata"
locale    = "en_IN.UTF-8"

[users.alice]
groups   = ["wheel", "sigma-audio", "sigma-video"]
shell    = "/usr/bin/sigma-zsh"
home_dir = "/home/alice"

[packages.system]
include = [
    "firefox@120",
    "code@1.85",
    "rust@stable",
    "python@3.12",
    "sigma-dev-tools",
]

[services]
sigma-networking   = { enable = true }
sigma-ssh          = { enable = true, port = 22, allow_users = ["alice"] }
sigma-printing     = { enable = false }
sigma-bluetooth    = { enable = true }

[kernel]
params = ["quiet", "splash", "mitigations=auto"]

[security]
firewall         = { enable = true, allow_ports = [22, 80, 443] }
mac_policy       = "sigma-default"
auto_update      = true

[desktop]
environment = "zenith"
wayland_only = true
```

Apply the configuration atomically:

```bash
$ sigma system apply /etc/sigma/sigma.toml
Σ [APPLY] Comparing current state vs desired state...
  To install : code@1.85, sigma-dev-tools
  To remove  : vim (not in packages.system)
  Config changes: /etc/sigma/ssh.toml (port change)
  Services: start sigma-printing=false → stop

Apply changes? [y/N] y
Σ [APPLY] Applying changes atomically (snapshot created first)...
Σ [SUCCESS] System is now in desired state. Reboot required for kernel changes.

# Roll back to previous state
$ sigma system rollback
```

### 3.3 Flake-Style Reproducible Environments

```bash
# Enter a reproducible dev environment (like nix develop)
$ sigma env --with "rust@stable python@3.12 nodejs@20" bash
Σ [ENV] Entering reproducible environment...
  rust    1.75.0  (/sigma/store/a1b2c3-rust-1.75.0/bin)
  python  3.12.1  (/sigma/store/d4e5f6-python-3.12.1/bin)
  node    20.11.0 (/sigma/store/g7h8i9-node-20.11.0/bin)
  (environment exits when you `exit` or close the shell)

$ sigma env --lock             # Pin environment to a sigma.lock file
$ sigma env --from sigma.lock  # Reproduce environment exactly from lock
```

### 3.4 Binary Cache

```bash
# Push a locally built package to a team binary cache
$ sigma store push --cache https://cache.mycompany.com my-package
Σ [STORE] Uploading /sigma/store/abc123-my-package...
  Signed with Dilithium5 key: mycompany-build-key
  Uploaded: 14.2MB in 3.2s

# Everyone on the team fetches the pre-built binary:
$ sigma pkg add my-package
Σ [PKG] Binary cache hit! (company cache)
  Downloading: 14.2MB... done (2.1s)
```

---

## 4. References & Standards

- Nix Package Manager — `nixos.org/nix` (MIT)
- NixOS — `nixos.org` (MIT)
- Nix Flakes — RFC 0049
- Reproducible Builds initiative — `reproducible-builds.org`
- Content-addressed storage design — `eelcovisser.org/post/nix-thesis`

# SigmaOS Package Management (sigpkg)

## Overview

SigmaOS implements a declarative, reproducible package management ecosystem (`sigpkg`) inspired by NixOS and Guix. Every package installation is deterministic and transaction-safe, using a Content-Addressed Store. System upgrades utilize Fedora Silverblue-style immutable image staging, enabling instant rollback to previous system states on boot failure.

### Key Features

- **Declarative Configuration**: Define system state in configuration files
- **Reproducible Builds**: Same source + same environment = identical binary
- **Content-Addressed Storage**: Packages stored by cryptographic hash
- **Atomic Updates**: Transaction-safe package operations
- **Immutable Root**: Fedora Silverblue-style immutable base system
- **Instant Rollback**: Revert to previous system states
- **Signed Repositories**: GPG-signed packages and repositories
- **SBOM Generation**: Software Bill of Materials for all packages

## Architecture

### Package Management Flow

```
 [declarative.sigma configuration]
                 │
                 ▼
     [Generate Dependency Graph]
                 │
                 ▼
 [Fetch Content-Addressed Packages] ──► [Verify GPG Repository Signature]
                 │
                 ▼
     [Link files into /run/current-system/sw/bin]
                 │
                 ▼
         [Activate New Generation]
                 │
                 ▼
         [Update Boot Configuration]
```

### Content-Addressed Store

```
/var/lib/sigpkg/store/
├── nix/
│   └── store/
│       ├── 00bgj0z5v1d...-bash-5.1/
│       ├── 1a2b3c4d5e6f...-zenith-desktop-1.0/
│       └── 2b3c4d5e6f7a...-sigma-ai-runtime-0.1/
└── profiles/
    ├── default-1-link -> /nix/store/...
    └── default-2-link -> /nix/store/...
```

## Configuration

### System Profile

**File**: `/etc/sigpkg/system.sigma`

```toml
[profile]
name = "sovereign-desktop"
immutable_root = true
version = "1"

[packages]
include = [
    "core-utils",
    "zenith-desktop",
    "sigma-ai-runtime",
    "scilab",
    "octave",
    "geogebra"
]

[repository]
url = "https://pkg.sigmaos.org/stable"
gpg_key = "/etc/keys/sigma-repo-pub.gpg"
priority = 10

[repositories.testing]
url = "https://pkg.sigmaos.org/testing"
gpg_key = "/etc/keys/sigma-repo-pub.gpg"
priority = 5

[updates]
auto_update = false
security_only = true
schedule = "weekly"
rollback_on_failure = true

[build]
reproducible = true
sbom_generation = true
sign_packages = true
```

### Package Definition

**File**: `sigpkg/packages/core-utils.sigma`

```toml
[package]
name = "core-utils"
version = "9.4"
description = "Core GNU utilities"
license = "GPL-3.0"
homepage = "https://www.gnu.org/software/coreutils/"

[source]
url = "https://ftp.gnu.org/gnu/coreutils/coreutils-9.4.tar.xz"
hash = "sha256:..."
patches = ["patches/fix-build.patch"]

[build]
dependencies = ["glibc", "gcc"]
build_dependencies = ["autoconf", "automake"]
configure = "./configure --prefix=$out"
make = "make -j$(nproc)"
install = "make install"

[outputs]
bin = ["bin/ls", "bin/cp", "bin/mv"]
man = ["share/man/man1/ls.1"]
```

## Technical Implementation

### Package Verification

```rust
// userland/sigpkg/sigpkg_core.rs
use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::Read;

pub fn verify_package_integrity(package_path: &Path, expected_hash: &str) -> Result<bool, PackageError> {
    let mut file = File::open(package_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    
    let result = hasher.finalize();
    let actual_hash = format!("{:x}", result);
    
    Ok(actual_hash == expected_hash)
}

pub fn verify_gpg_signature(package_path: &Path, signature_path: &Path) -> Result<bool, PackageError> {
    // Use GPG to verify signature
    let output = Command::new("gpg")
        .arg("--verify")
        .arg(signature_path)
        .arg(package_path)
        .output()?;
    
    Ok(output.status.success())
}
```

### Dependency Resolution

```rust
// userland/sigpkg/dependency_resolver.rs
use petgraph::Graph;
use petgraph::algo::toposort;

pub struct DependencyResolver {
    graph: Graph<String, ()>,
}

impl DependencyResolver {
    pub fn new() -> Self {
        DependencyResolver {
            graph: Graph::new(),
        }
    }
    
    pub fn add_package(&mut self, name: &str, dependencies: &[String]) {
        let package_idx = self.graph.add_node(name.to_string());
        
        for dep in dependencies {
            let dep_idx = self.graph.add_node(dep.clone());
            self.graph.add_edge(package_idx, dep_idx, ());
        }
    }
    
    pub fn resolve_order(&self) -> Result<Vec<String>, ResolveError> {
        let mut order = Vec::new();
        toposort(&self.graph, Some(&mut order))?;
        
        Ok(order.into_iter().map(|idx| {
            self.graph[idx].clone()
        }).collect())
    }
}
```

### Content-Addressed Storage

```rust
// userland/sigpkg/store.rs
use std::path::{Path, PathBuf};

pub struct ContentAddressedStore {
    base_path: PathBuf,
}

impl ContentAddressedStore {
    pub fn new(base_path: PathBuf) -> Self {
        ContentAddressedStore { base_path }
    }
    
    pub fn add_path(&self, path: &Path) -> Result<String, StoreError> {
        let hash = self.compute_hash(path)?;
        let store_path = self.store_path(&hash);
        
        // Copy to store
        std::fs::create_dir_all(&store_path)?;
        std::fs::copy(path, store_path.join("content"))?;
        
        Ok(hash)
    }
    
    pub fn get_path(&self, hash: &str) -> PathBuf {
        self.store_path(hash).join("content")
    }
    
    fn compute_hash(&self, path: &Path) -> Result<String, StoreError> {
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        std::io::copy(&mut file, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }
    
    fn store_path(&self, hash: &str) -> PathBuf {
        self.base_path.join(&hash[..2]).join(&hash[2..])
    }
}
```

## Immutable Root System

### A/B Partition Layout

```
/dev/sda1  EFI System Partition
/dev/sda2  Boot Partition
/dev/sda3  System A (Current)
/dev/sda4  System B (Previous)
/dev/sda5  Data Partition
/dev/sda6  Swap
```

### Update Process

```rust
// userland/sigpkg/immutable_update.rs
pub struct ImmutableUpdater {
    boot_config: BootConfig,
}

impl ImmutableUpdater {
    pub fn update_system(&mut self, new_profile: &Profile) -> Result<(), UpdateError> {
        // Determine inactive partition
        let target_partition = self.boot_config.inactive_partition();
        
        // Mount target partition
        let mount_point = self.mount_partition(target_partition)?;
        
        // Install new system
        self.install_profile(&mount_point, new_profile)?;
        
        // Update boot configuration
        self.boot_config.set_next_boot(target_partition)?;
        
        // Unmount
        self.unmount_partition(mount_point)?;
        
        Ok(())
    }
    
    pub fn rollback(&mut self) -> Result<(), UpdateError> {
        // Switch to previous partition
        let previous_partition = self.boot_config.inactive_partition();
        self.boot_config.set_next_boot(previous_partition)?;
        
        Ok(())
    }
}
```

## SBOM Generation

### SPDX Format

```json
{
  "SPDXID": "SPDXRef-DOCUMENT",
  "spdxVersion": "SPDX-2.3",
  "name": "core-utils-9.4",
  "documentNamespace": "https://sigmaos.org/sbom/core-utils-9.4",
  "creationInfo": {
    "created": "2024-01-15T10:30:00Z",
    "creators": ["Tool: sigpkg-sbom-generator-1.0"]
  },
  "packages": [
    {
      "SPDXID": "SPDXRef-Package-core-utils",
      "name": "core-utils",
      "versionInfo": "9.4",
      "downloadLocation": "https://ftp.gnu.org/gnu/coreutils/coreutils-9.4.tar.xz",
      "filesAnalyzed": false,
      "licenseConcluded": "GPL-3.0-only",
      "externalRefs": [
        {
          "referenceCategory": "PACKAGE-MANAGER",
          "referenceLocator": "pkg:sigmaos/core-utils@9.4",
          "referenceType": "purl"
        }
      ]
    }
  ]
}
```

## Repository Management

### Repository Structure

```
https://pkg.sigmaos.org/stable/
├── repo-index.json
├── repo-index.json.sig
├── packages/
│   ├── core-utils-9.4.sigpkg
│   ├── core-utils-9.4.sigpkg.sig
│   ├── zenith-desktop-1.0.sigpkg
│   └── zenith-desktop-1.0.sigpkg.sig
└── metadata/
    └── repo-metadata.json
```

### Repository Index

```json
{
  "version": "1.0",
  "name": "stable",
  "timestamp": "2024-01-15T10:30:00Z",
  "packages": [
    {
      "name": "core-utils",
      "version": "9.4",
      "architecture": "x86_64",
      "hash": "sha256:...",
      "location": "packages/core-utils-9.4.sigpkg",
      "signature": "packages/core-utils-9.4.sigpkg.sig",
      "dependencies": ["glibc >= 2.35"],
      "sbom": "sbom/core-utils-9.4.json"
    }
  ]
}
```

## CLI Commands

### Package Operations

```bash
# Install package
sigpkg install core-utils

# Remove package
sigpkg remove core-utils

# Update package
sigpkg update core-utils

# Update all packages
sigpkg upgrade

# Search packages
sigpkg search python

# Package information
sigpkg info core-utils

# List installed packages
sigpkg list
```

### System Operations

```bash
# Apply system profile
sigpkg apply /etc/sigpkg/system.sigma

# Rollback to previous generation
sigpkg rollback

# List generations
sigpkg generations

# Show diff between generations
sigpkg diff 1 2

# Garbage collect old generations
sigpkg gc
```

### Repository Operations

```bash
# Add repository
sigpkg repo add stable https://pkg.sigmaos.org/stable

# Remove repository
sigpkg repo remove stable

# Update repository
sigpkg repo update stable

# List repositories
sigpkg repo list

# Verify repository
sigpkg repo verify stable
```

## Best Practices

### Package Development

1. **Reproducible Builds**: Use fixed toolchain versions
2. **Clear Dependencies**: Specify all dependencies explicitly
3. **Proper Licensing**: Include license information
4. **Documentation**: Provide comprehensive documentation
5. **Testing**: Include test cases

### System Configuration

1. **Version Control**: Keep system configuration in version control
2. **Minimal Profiles**: Start with minimal profiles
3. **Incremental Changes**: Make incremental changes
4. **Testing**: Test changes in VM before applying
5. **Backups**: Keep backups of important data

### Security

1. **Verify Signatures**: Always verify GPG signatures
2. **Use HTTPS**: Use HTTPS for repository URLs
3. **Key Management**: Manage GPG keys securely
4. **Regular Updates**: Keep packages updated
5. **Audit Logs**: Review audit logs regularly

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- Declarative profile parser
- Local Content-Addressed symlinking engine
- Basic package operations
- CLI implementation

### Phase 2 (Months 3-6)
- Immutable root file system mounting
- A/B boot partition switching
- Repository management
- GPG signature verification

### Phase 3 (Months 6-9)
- Signed repository manifest validation
- SBOM generation
- Dependency resolution
- Transaction rollback

### Phase 4 (Months 9-12)
- Automated rollbacks on health-check failures
- Build farm integration
- Automated testing
- Performance optimization

## References

- [NixOS Package Management](https://nixos.org/manual/nix/stable/)
- [Guix Package Manager](https://guix.gnu.org/manual/)
- [Fedora Silverblue](https://docs.fedoraproject.org/en-US/fedora-silverblue/)
- [SPDX Specification](https://spdx.github.io/spdx-spec/)
- [Content-Addressed Storage](https://en.wikipedia.org/wiki/Content-addressable_storage)

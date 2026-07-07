# Package Management Technical Specifications

## Overview

This document provides detailed technical specifications for the Sigma Package Manager (SPM), inspired by Pacman, DNF5, and Nix package managers.

## Architecture

### Core Components

```rust
pub struct SigmaPackageManager {
    pub backend: PackageBackend,
    pub resolver: DependencyResolver,
    pub repository: Repository,
    pub cache: PackageCache,
    pub config: PackageManagerConfig,
}

pub enum PackageBackend {
    Native,      // Custom SigmaOS format (.sigma)
    Ostree,      // For atomic updates
    Container,   // OCI-compatible containers
}

pub enum DependencyResolver {
    Topological,    // Fast, simple (like Pacman)
    SatSolver,      // Advanced (like DNF/Zypper)
    Functional,     // Reproducible (like Nix)
}
```

## Package Format

### Native Format (.sigma.tar.zst)

**Structure:**
```
.sigma.tar.zst
├── .SIGMAINFO          # Package metadata
├── .BUILDINFO         # Build information
├── .INSTALL           # Install script
├── .REMOVE            # Remove script
├── usr/               # Package files
├── etc/               # Configuration files
└── var/               # Variable data
```

**SIGMAINFO Format:**
```ini
pkgname = sigma-editor
pkgver = 1.0.0
pkgrel = 1
pkgdesc = Advanced text editor for SigmaOS
url = https://sigmaos.org/packages/sigma-editor
arch = x86_64
license = MIT
depends = sigma-gtk >= 2.0.0
makedepends = rust cargo
provides = editor
conflicts = nano-editor
```

## Repository Structure

### Repository Configuration

```yaml
# /etc/sigma/repositories.yaml
repositories:
  - name: core
    url: https://packages.sigmaos.org/core
    priority: 1
    gpg_key: /etc/sigma/keys/core.gpg
    auto_sync: true
    
  - name: community
    url: https://packages.sigmaos.org/community
    priority: 2
    gpg_key: /etc/sigma/keys/community.gpg
    auto_sync: true
    
  - name: extra
    url: https://packages.sigmaos.org/extra
    priority: 3
    gpg_key: /etc/sigma/keys/extra.gpg
    auto_sync: false
```

### Repository Metadata

**Repository Index Format:**
```json
{
  "version": "1.0",
  "packages": [
    {
      "name": "sigma-editor",
      "version": "1.0.0",
      "architecture": "x86_64",
      "filename": "sigma-editor-1.0.0-1-x86_64.sigma.tar.zst",
      "checksum": "sha256:abc123...",
      "size": 5242880,
      "dependencies": ["sigma-gtk>=2.0.0"],
      "provides": ["editor"],
      "conflicts": ["nano-editor"]
    }
  ]
}
```

## Dependency Resolution

### SAT Solver Integration

**Implementation:**
```rust
pub struct SatSolver {
    pub constraints: Vec<Constraint>,
    pub variables: Vec<Variable>,
}

pub struct Constraint {
    pub literals: Vec<Literal>,
    pub weight: i32,
}

pub struct Literal {
    pub variable: Variable,
    pub negated: bool,
}
```

**Resolution Process:**
1. Parse package dependencies
2. Build constraint satisfaction problem
3. Apply SAT solver to find solution
4. Handle conflicts with user guidance
5. Generate transaction plan

## Transaction Management

### Transaction Structure

```rust
pub struct PackageTransaction {
    pub id: TransactionId,
    pub operations: Vec<PackageOperation>,
    pub state: TransactionState,
    pub rollback_data: RollbackData,
    pub timestamp: DateTime,
}

pub enum PackageOperation {
    Install { package: Package },
    Remove { package: Package },
    Upgrade { from: Package, to: Package },
    Downgrade { from: Package, to: Package },
    Reinstall { package: Package },
}

pub enum TransactionState {
    Prepared,
    InProgress,
    Completed,
    Failed,
    RolledBack,
}
```

### Rollback Mechanism

**Implementation:**
```rust
pub struct RollbackData {
    pub previous_state: SystemState,
    pub backup_files: Vec<BackupFile>,
    pub transaction_log: TransactionLog,
}

pub struct SystemState {
    pub installed_packages: HashSet<Package>,
    pub config_files: HashMap<PathBuf, String>,
    pub database_state: DatabaseState,
}
```

## Delta Updates

### Binary Delta Implementation

**Implementation:**
```rust
pub struct DeltaUpdate {
    pub old_version: PackageVersion,
    pub new_version: PackageVersion,
    pub delta_file: DeltaFile,
    pub compression: CompressionAlgorithm,
}

pub enum CompressionAlgorithm {
    Zstd,
    Xz,
    Gzip,
    Brotli,
}
```

**Delta Generation:**
1. Compare old and new package files
2. Generate binary diff using bsdiff or similar
3. Compress delta with ZSTD
4. Sign delta with repository key
5. Upload to repository

## Content-Addressed Storage

### Store Implementation

```rust
pub struct ContentStore {
    pub base_path: PathBuf,
    pub compression: CompressionAlgorithm,
}

pub struct StorePath {
    pub hash: String,      // SHA256 of content
    pub path: PathBuf,
    pub references: Vec<StorePath>,
}
```

**Benefits:**
- Automatic deduplication
- Immutable storage
- Easy garbage collection
- Reproducible builds

## Build System Integration

### PKGBUILD-Style Build Scripts

**Example:**
```bash
# PKGBUILD for sigma-editor
pkgname=sigma-editor
pkgver=1.0.0
pkgrel=1
pkgdesc="Advanced text editor for SigmaOS"
arch=('x86_64')
url="https://sigmaos.org/packages/sigma-editor"
license=('MIT')
depends=('sigma-gtk>=2.0.0')
makedepends=('rust' 'cargo')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$pkgname-$pkgver"
    cargo install --path . --root "$pkgdir"
}
```

### Compilation Optimization

**Per-Package Configuration:**
```ini
# /etc/sigma/package.env
sigma-editor CFLAGS="-O3 -march=native" MAKEOPTS="-j4"
chromium CFLAGS="-O2" MAKEOPTS="-j2" PORTAGE_TMPDIR=/var/tmp/chromium
```

## Security Features

### Package Signing

**Implementation:**
```rust
pub struct PackageSignature {
    pub key_id: String,
    pub signature: Vec<u8>,
    pub algorithm: SignatureAlgorithm,
}

pub enum SignatureAlgorithm {
    Ed25519,
    RSA4096,
    ECDSA,
}
```

### Signature Verification

**Process:**
1. Download package signature
2. Verify signature with repository key
3. Check key revocation status
4. Verify package checksum
5. Only install if all checks pass

## CLI Interface

### Command Structure

```bash
# Install package
sigma-pkg install sigma-editor

# Remove package
sigma-pkg remove sigma-editor

# Update system
sigma-pkg update

# Search packages
sigma-pkg search editor

# Show package info
sigma-pkg info sigma-editor

# Upgrade package
sigma-pkg upgrade sigma-editor

# Rollback transaction
sigma-pkg rollback <transaction-id>

# Clean cache
sigma-pkg clean
```

## Configuration

### Main Configuration File

```yaml
# /etc/sigma/sigma-pkg.yaml
general:
  color_output: true
  verbose_pkglists: true
  download_timeout: 300

repositories:
  auto_sync: true
  sync_interval: 86400  # 24 hours

cache:
  enabled: true
  max_size: 10737418240  # 10GB
  min_free_space: 536870912  # 512MB

transactions:
  auto_rollback: true
  max_transactions: 100
  keep_history: true

build:
  parallel_jobs: 4
  use_ccache: true
  build_in_ram: false
```

## Performance Optimizations

### Parallel Downloads

**Implementation:**
```rust
pub struct DownloadManager {
    pub max_connections: u32,
    pub chunk_size: usize,
    pub resume_support: bool,
}
```

### Compression

**Default Compression:**
- ZSTD level 3 for packages
- ZSTD level 15 for repository metadata
- Delta updates use ZSTD level 5

## Implementation Priority

1. **Phase 1 (Weeks 9-12):** Basic package manager infrastructure
2. **Phase 2 (Weeks 33-36):** Advanced package management (SAT solver)
3. **Phase 3 (Weeks 45-48):** Delta updates and optimization
4. **Phase 4 (Weeks 61-64):** Package building tools

## Testing

### Test Suite

- Package installation/removal tests
- Dependency resolution tests
- Transaction rollback tests
- Delta update tests
- Signature verification tests
- Performance benchmarks

## References

- Arch Linux Pacman Documentation
- DNF5 Architecture
- Nix Package Manager Design
- Libsolv SAT Solver Documentation

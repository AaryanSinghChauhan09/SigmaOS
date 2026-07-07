# Atomic Updates and Rollback Technical Specifications

## Overview

This document provides detailed technical specifications for atomic update and rollback mechanisms in SigmaOS, inspired by OSTree, Fedora CoreOS, and openSUSE transactional-update.

## Architecture

### Core Components

```rust
pub struct AtomicUpdateSystem {
    pub ostree_repo: OstreeRepo,
    pub deployments: Vec<Deployment>,
    pub bootloader: BootloaderManager,
    pub health_checker: HealthChecker,
}

pub struct Deployment {
    pub id: String,
    pub checksum: String,
    pub timestamp: DateTime,
    pub kernel: KernelVersion,
    pub status: DeploymentStatus,
    pub bootable: bool,
}

pub enum DeploymentStatus {
    Booted,
    Pending,
    RolledBack,
    Failed,
}
```

## OSTree Integration

### Repository Structure

**OSTree Repository Layout:**
```
/ostree/repo/
├── config
├── objects/
│   ├── unpacked/
│   └── packed/
├── refs/
│   ├── heads/
│   └── remotes/
└── state
```

### Deployment Model

**A/B Update Pattern:**
```rust
pub struct ABDeployment {
    pub deployment_a: Deployment,
    pub deployment_b: Deployment,
    pub active_deployment: DeploymentId,
}
```

**Update Process:**
1. Pull new commit from repository
2. Create new deployment in staging
3. Verify deployment integrity
4. Update bootloader configuration
5. Set new deployment as default
6. Reboot to activate

### Boot Configuration

**Swapped Directory Pattern:**
```rust
pub struct BootManager {
    pub boot_version: u32,  // 0 or 1
    pub boot_symlink: PathBuf,
}

pub fn swap_boot() -> Result<()> {
    let new_version = if boot_version == 0 { 1 } else { 0 };
    let new_boot = format!("/ostree/boot.{}", new_version);
    symlink(&new_boot, "/ostree/boot")?;
    boot_version = new_version;
    Ok(())
}
```

## Snapshot-Based Rollback

### Btrfs Integration

**Snapshot Manager:**
```rust
pub struct SnapshotManager {
    pub filesystem: BtrfsFilesystem,
    pub snapshots: Vec<Snapshot>,
    pub retention_policy: RetentionPolicy,
}

pub struct Snapshot {
    pub id: String,
    pub timestamp: DateTime,
    pub description: String,
    pub bootable: bool,
    pub subvolume_id: u64,
}
```

**Automatic Snapshots:**
```yaml
# /etc/sigma/snapshot.yaml
snapshots:
  auto_create:
    - before_update: true
    - before_package_install: false
    - schedule: daily
  
  retention:
    daily: 7
    weekly: 4
    monthly: 6
    
  bootable:
    keep_last: 5
    verify_before_promote: true
```

### ZFS Integration

**ZFS Snapshot Manager:**
```rust
pub struct ZfsSnapshotManager {
    pub pool: String,
    pub dataset: String,
    pub snapshots: Vec<ZfsSnapshot>,
}

pub struct ZfsSnapshot {
    pub name: String,
    pub creation: DateTime,
    pub used: u64,
    pub referenced: u64,
}
```

## Health Checking

### Health Check Framework

**Implementation:**
```rust
pub struct HealthChecker {
    pub checks: Vec<Box<dyn HealthCheck>>,
    pub timeout: Duration,
    pub rollback_on_failure: bool,
}

pub trait HealthCheck {
    fn check(&self) -> Result<(), HealthError>;
    fn name(&self) -> &str;
    fn critical(&self) -> bool;
}
```

**Built-in Checks:**
```rust
pub struct BootCheck;
pub struct ServiceCheck { pub service: String }
pub struct NetworkCheck;
pub struct FileSystemCheck;
pub struct KernelModuleCheck { pub module: String }
```

### Health Check Configuration

```yaml
# /etc/sigma/health-check.yaml
checks:
  - name: boot_check
    type: boot
    critical: true
    timeout: 30
    
  - name: essential_services
    type: service
    services:
      - sigma-network
      - sigma-security
    critical: true
    
  - name: network_connectivity
    type: network
    critical: false
    
  - name: filesystem_integrity
    type: filesystem
    paths:
      - /
      - /boot
    critical: true
```

## Rollback Mechanism

### Instant Rollback

**Implementation:**
```rust
pub struct RollbackManager {
    pub deployments: Vec<Deployment>,
    pub snapshots: Vec<Snapshot>,
}

pub fn rollback_to_deployment(id: &str) -> Result<()> {
    let deployment = find_deployment(id)?;
    verify_deployment(&deployment)?;
    set_default_deployment(&deployment)?;
    reboot()?;
    Ok(())
}

pub fn rollback_to_snapshot(id: &str) -> Result<()> {
    let snapshot = find_snapshot(id)?;
    verify_snapshot(&snapshot)?;
    swap_subvolumes(&snapshot)?;
    update_bootloader()?;
    reboot()?;
    Ok(())
}
```

### Rollback Safety

**Verification Steps:**
1. Check deployment/snapshot exists
2. Verify deployment/snapshot integrity
3. Ensure boot configuration is valid
4. Test bootloader entry
5. Confirm rollback with user
6. Perform atomic swap
7. Reboot system

## Delta Updates

### Binary Delta Implementation

**Delta Generation:**
```rust
pub struct DeltaGenerator {
    pub algorithm: DeltaAlgorithm,
    pub compression: CompressionLevel,
}

pub enum DeltaAlgorithm {
    Bsdiff,
    Xdelta,
    Custom,
}
```

**Delta Application:**
```rust
pub fn apply_delta(old_package: &Package, delta: &Delta) -> Result<Package> {
    let old_content = read_package(old_package)?;
    let new_content = delta.apply(&old_content)?;
    let new_package = Package::from_content(new_content)?;
    verify_package(&new_package)?;
    Ok(new_package)
}
```

## Bootloader Integration

### UEFI Boot

**Configuration:**
```rust
pub struct UefiBootManager {
    pub entries: Vec<BootEntry>,
    pub default_entry: BootEntryId,
}

pub struct BootEntry {
    pub id: BootEntryId,
    pub label: String,
    pub kernel_path: PathBuf,
    pub initrd_path: PathBuf,
    pub kernel_params: Vec<String>,
    pub ostree_arg: String,
}
```

### Secure Boot

**Implementation:**
```rust
pub struct SecureBoot {
    pub keys: SecureBootKeys,
    pub enforcement: bool,
}

pub struct SecureBootKeys {
    pub db: Vec<PublicKey>,     // Allowed keys
    pub dbx: Vec<PublicKey>,    // Forbidden keys
    pub kek: Vec<PublicKey>,    // Key Exchange Keys
    pub pk: PublicKey,           // Platform Key
}
```

## Configuration

### Main Configuration

```yaml
# /etc/sigma/atomic-updates.yaml
updates:
  auto_update: true
  update_schedule: "weekly"
  auto_reboot: false
  
ostree:
  repo_path: /ostree/repo
  remote_url: https://updates.sigmaos.org
  gpg_verify: true
  
rollback:
  automatic_on_failure: true
  keep_deployments: 5
  keep_snapshots: 10
  
bootloader:
  timeout: 5
  default_entry: latest
```

## CLI Interface

### Command Structure

```bash
# Check for updates
sigma-update check

# Apply updates
sigma-update apply

# Rollback to previous deployment
sigma-update rollback

# List deployments
sigma-update list-deployments

# Create manual snapshot
sigma-update snapshot create "Pre-upgrade"

# List snapshots
sigma-update snapshot list

# Rollback to snapshot
sigma-update snapshot rollback <id>

# Verify system health
sigma-update health-check
```

## Performance Optimizations

### Deduplication

**Content-Addressed Storage:**
- Automatic deduplication at block level
- Shared references between deployments
- Efficient storage utilization

### Compression

**Compression Levels:**
- ZSTD level 3 for package data
- ZSTD level 15 for metadata
- LZ4 for boot-time critical data

## Security Features

### Signature Verification

**Implementation:**
```rust
pub struct SignatureVerifier {
    pub trusted_keys: Vec<PublicKey>,
    pub revocation_list: Vec<KeyId>,
}

pub fn verify_commit(commit: &OstreeCommit) -> Result<()> {
    let signature = commit.signature()?;
    verifier.verify(&signature, commit.content())?;
    verifier.check_revocation(signature.key_id())?;
    Ok(())
}
```

### Immutable Root

**Implementation:**
```rust
pub struct ImmutableRoot {
    pub enabled: bool,
    pub exceptions: Vec<PathBuf>,
}

pub fn enforce_immutability() -> Result<()> {
    mount_root_readonly()?;
    setup_overlayfs()?;
    Ok(())
}
```

## Implementation Priority

1. **Phase 1 (Weeks 17-20):** Atomic update system (OSTree integration)
2. **Phase 2 (Weeks 37-40):** Snapshot-based rollback
3. **Phase 3 (Weeks 45-48):** Health checking system
4. **Phase 4 (Weeks 49-52):** Delta updates and optimization

## Testing

### Test Suite

- Atomic update tests
- Rollback functionality tests
- Health check tests
- Bootloader integration tests
- Signature verification tests
- Performance benchmarks

### Validation Criteria

- 100% rollback success rate
- < 5 second rollback time
- Zero data loss during rollback
- Boot verification before promotion
- Secure boot compatibility

## References

- OSTree Documentation
- Fedora CoreOS Architecture
- openSUSE transactional-update
- Btrfs Documentation
- ZFS Documentation

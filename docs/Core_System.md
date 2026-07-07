# SigmaOS Core System Design

## Overview

The Core System is the foundation of SigmaOS, providing kernel-level functionality, hardware abstraction, and system initialization. Taking inspiration from Arch Linux and Fedora, SigmaOS combines a Rolling/Stable hybrid release model with Fedora's strong hardware driver integration and Arch's minimal base philosophy.

### Hybrid Release Model

```
          [SigmaOS Release Strategy]
                      │
            ┌─────────┴─────────┐
            ▼                   ▼
    [Arch Rolling Stream]  [Fedora Stable Stream]
    (Developers/Cutting)    (Enterprise/Stable)
            │                   │
            └─────────┬─────────┘
                      ▼
            [Unified Base System]
```

## System Properties & Models

### 1. Rolling & Stable Hybrid Model

**Developer Stream (Rolling)**:
- Immediate package upgrades from rolling repository
- Latest kernel versions and drivers
- Cutting-edge features and improvements
- Target audience: Developers, enthusiasts, early adopters

**Enterprise Stream (Stable)**:
- Frozen, hardened packages on 6-month cycles
- Extended support and security patches
- Predictable update schedule
- Target audience: Enterprises, production systems, stability-focused users

**Configuration**:
```toml
[system]
stream = "stable" # stable or rolling
version = "2026.07"
minimal_base = true
```

### 2. Minimal Base System

Following Arch's minimal core footprint philosophy:

**Default Base Components**:
- Microkernel (SigmaOS kernel)
- `sigmad` init system
- `sigpkg` package manager
- Basic terminal utilities
- Hardware abstraction layer

**Everything as Packages**:
- Desktop environments
- Office suites
- Development tools
- Security tools
- All user-facing applications

**Benefits**:
- Smaller attack surface
- Faster boot times
- Easier maintenance
- Customizable installations

### 3. Hardware Compatibility Matrix

**Upstream Driver Integration**:
- Direct integration with Linux kernel upstream
- Fedora infrastructure driver integration patterns
- Automated HCL generation from user telemetry
- Hardware-specific optimizations for Indian market

**Hardware Compatibility List (HCL)**:
- Published automatically from telemetry
- Categorized by device type
- Tested and verified hardware
- Community-contributed compatibility reports

**Target Hardware**:
- Intel/AMD laptops (common in India)
- Broadcom Wi-Fi chipsets
- NVIDIA/AMD GPUs
- Indian-specific peripherals

### 4. Fedora Infrastructure Integration

**Build System (Koji-inspired)**:
- Automated package compilation
- Multi-architecture builds
- Build artifact management
- Build farm orchestration

**Update Gating (Bodhi-inspired)**:
- Hardware testing feedback loops
- Automated testing before stable release
- Update approval workflow
- Rollback capabilities

**Mirror Management (MirrorManager-inspired)**:
- High-speed local driver package delivery
- Geographic mirror selection
- CDN integration
- Bandwidth optimization

## System Configuration

### Core Configuration

**File**: `/etc/sigma/core.conf`

```toml
[system]
stream = "stable" # stable or rolling
version = "2026.07"
minimal_base = true
profile = "default"

[hcl]
telemetry_enabled = true
publish_hcl_status = true
hardware_id = "auto"

[repositories]
rolling = "https://repo.sigmaos.org/rolling"
stable = "https://repo.sigmaos.org/stable"
testing = "https://repo.sigmaos.org/testing"

[updates]
auto_update = false
security_only = true
schedule = "weekly"
```

### Profile Configuration

**File**: `/etc/sigma/profiles/default.conf`

```toml
[profile]
name = "default"
description = "Standard SigmaOS installation"

[packages]
base = ["kernel", "sigmad", "sigpkg", "bash", "coreutils"]
desktop = ["zenith-desktop", "zenith-compositor"]
development = ["sigma-sdk", "rust", "gcc", "make"]
security = ["sigmasec-suite", "firewall", "antivirus"]

[services]
enabled = ["sigmad", "networkd", "timed", "logd"]
disabled = []
```

## Technical Implementation

### System Bootstrap

```rust
// kernel/init/sigma_init.rs
pub fn determine_system_profile() -> SystemProfile {
    let config = load_system_config("/etc/sigma/core.conf");
    match config.get("system", "stream") {
        Some("rolling") => SystemProfile::Rolling,
        _ => SystemProfile::Stable,
    }
}

pub fn initialize_hardware() -> Result<(), InitError> {
    // Detect hardware
    let hardware = detect_hardware();
    
    // Load appropriate drivers
    load_drivers(&hardware)?;
    
    // Publish to HCL if enabled
    if config.hcl.telemetry_enabled {
        publish_hcl_status(hardware);
    }
    
    Ok(())
}
```

### Repository Management

```rust
// userland/system_api/sigpkg/src/repository.rs
pub struct Repository {
    name: String,
    url: String,
    stream: ReleaseStream,
    gpg_key: Option<String>,
}

impl Repository {
    pub fn sync(&self) -> Result<(), RepoError> {
        // Fetch repository index
        let index = self.fetch_index()?;
        
        // Verify GPG signature
        self.verify_signature(&index)?;
        
        // Update local cache
        self.update_cache(index)?;
        
        Ok(())
    }
    
    pub fn get_package(&self, name: &str) -> Result<Package, RepoError> {
        // Search local cache
        if let Some(pkg) = self.cache.get(name) {
            return Ok(pkg.clone());
        }
        
        // Fetch from remote
        self.fetch_package(name)
    }
}
```

## Hardware Abstraction Layer

### HAL Architecture

```
┌─────────────────────────────────────┐
│         User Space Programs         │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│      System Call Interface          │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│   Hardware Abstraction Layer (HAL)  │
│  ┌──────────┬──────────┬──────────┐ │
│  │  Driver  │  Driver  │  Driver  │ │
│  │  Manager │  Loader  │  Registry│ │
│  └──────────┴──────────┴──────────┘ │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│           Kernel Space               │
│  ┌──────────┬──────────┬──────────┐ │
│  │  Memory  │ Scheduler│  I/O     │ │
│  │ Manager  │          │ Subsystem│ │
│  └──────────┴──────────┴──────────┘ │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│           Hardware                  │
└─────────────────────────────────────┘
```

### Driver Management

```rust
// kernel/core/hal/driver_manager.rs
pub struct DriverManager {
    loaded_drivers: HashMap<String, Driver>,
    driver_registry: DriverRegistry,
}

impl DriverManager {
    pub fn load_driver(&mut self, name: &str) -> Result<(), DriverError> {
        // Check if already loaded
        if self.loaded_drivers.contains_key(name) {
            return Ok(());
        }
        
        // Load driver from registry
        let driver = self.driver_registry.get(name)?;
        
        // Initialize driver
        driver.initialize()?;
        
        // Register driver
        self.loaded_drivers.insert(name.to_string(), driver);
        
        Ok(())
    }
    
    pub fn unload_driver(&mut self, name: &str) -> Result<(), DriverError> {
        // Check if loaded
        let driver = self.loaded_drivers.remove(name)
            .ok_or(DriverError::NotLoaded)?;
        
        // Cleanup driver
        driver.cleanup()?;
        
        Ok(())
    }
}
```

## Security Model

### Secure Boot

**Chain of Trust**:
1. UEFI firmware verifies bootloader
2. Bootloader verifies kernel
3. Kernel verifies initramfs
4. Initramfs verifies drivers
5. Drivers verify modules

**Implementation**:
```rust
// kernel/security/secure_boot.rs
pub fn verify_chain_of_trust() -> Result<(), SecurityError> {
    // Verify bootloader signature
    verify_bootloader()?;
    
    // Verify kernel signature
    verify_kernel()?;
    
    // Verify initramfs signature
    verify_initramfs()?;
    
    // Verify driver signatures
    verify_drivers()?;
    
    Ok(())
}
```

### Mandatory Access Control

**SELinux Integration**:
- Permissive mode for development
- Enforcing mode for production
- Custom policies for SigmaOS
- Policy management tools

## Performance Optimizations

### Boot Performance

**Optimizations**:
- Parallel service startup
- Lazy loading of drivers
- Cached hardware detection
- Optimized init scripts

**Metrics**:
- Target boot time: < 10 seconds
- Service startup: < 2 seconds
- Driver loading: < 1 second

### Runtime Performance

**Optimizations**:
- Zero-copy I/O
- Memory-mapped files
- JIT compilation for hot paths
- Profile-guided optimization

## Roadmap & Milestones

### Phase 1 (Months 0-3)
- Base bootloader optimization
- Rolling release repository hosting
- Core HAL implementation
- Basic driver management

### Phase 2 (Months 3-6)
- Hardware detection suite
- Automated HCL publication
- Fedora infrastructure integration
- Stable release repository

### Phase 3 (Months 6-9)
- Containerized testing farm
- Stable release validation
- Driver reproducibility
- Build farm automation

### Phase 4 (Months 9-12)
- Upstream driver submission
- Automated compatibility warnings
- Advanced security features
- Performance optimization

## Best Practices

### Development

1. **Modular Design**: Keep components independent and reusable
2. **Clear Interfaces**: Define clear APIs between components
3. **Documentation**: Document all public interfaces
4. **Testing**: Comprehensive unit and integration tests

### Security

1. **Principle of Least Privilege**: Minimize privileges
2. **Defense in Depth**: Multiple layers of security
3. **Secure by Default**: Enable security features by default
4. **Regular Audits**: Regular security audits

### Performance

1. **Measure First**: Profile before optimizing
2. **Optimize Hot Paths**: Focus on frequently used code
3. **Avoid Premature Optimization**: Optimize based on measurements
4. **Continuous Monitoring**: Monitor performance metrics

## References

- [Arch Linux Philosophy](https://wiki.archlinux.org/title/Arch_Linux)
- [Fedora Documentation](https://docs.fedoraproject.org/)
- [Linux Kernel Documentation](https://www.kernel.org/doc/html/latest/)
- [Driver Strategy](Driver_Strategy.md)
- [Hardware Compatibility List](HCL.md)

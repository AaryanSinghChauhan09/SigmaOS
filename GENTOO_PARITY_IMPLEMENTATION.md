# Gentoo Parity Implementation Guide

## Overview

This document provides the implementation guide for Gentoo parity features in SigmaOS, focusing on practical integration of Gentoo's focus on performance, flexibility, and source-based package management.

## Implementation Status

| Feature | Status | Notes |
|---------|--------|-------|
| Portage Package Manager | ✅ Complete | Source-based package management implemented |
| USE Flags System | ✅ Complete | Build configuration management ready |
| Ebuild System | ✅ Complete | Package build scripts implemented |
| Gentoo Repository | ✅ Complete | Package repository management |
| Profile System | ✅ Complete | System configuration profiles |
| OpenRC Init System | ✅ Complete | Service management parity |
| Kernel Configuration | ✅ Complete | Kernel building and configuration |
| GCC Toolchain | ✅ Complete | Compiler and toolchain management |

## Core Components

### 1. SigmaPortage Package Manager

The Portage-like package manager provides source-based package management:

```rust
pub struct SigmaPortage {
    pub database: PortageDatabase,
    pub repository: GentooRepository,
    pub use_flags: UseFlagManager,
    pub profiles: ProfileManager,
    pub overlay_manager: OverlayManager,
}

pub struct Ebuild {
    pub name: String,
    pub version: String,
    pub slot: String,
    pub depends: Vec<Dependency>,
    pub rdepends: Vec<Dependency>,
    pub pdepends: Vec<Dependency>,
    pub use_flags: Vec<UseFlag>,
    pub src_uri: Vec<String>,
    pub homepage: String,
    pub description: String,
}

impl SigmaPortage {
    pub fn emerge(&mut self, package: &str) -> Result<(), PortageError> {
        // Get ebuild
        let ebuild = self.repository.get_ebuild(package)?;
        
        // Apply USE flags
        let use_flags = self.use_flags.resolve_flags(&ebuild)?;
        
        // Resolve dependencies
        let dependencies = self.resolve_dependencies(&ebuild, &use_flags)?;
        
        // Build dependency graph
        let graph = self.build_dependency_graph(dependencies)?;
        
        // Fetch sources
        self.fetch_sources(&ebuild)?;
        
        // Build packages in dependency order
        for pkg in graph.topological_order() {
            self.build_package(&pkg, &use_flags)?;
        }
        
        // Install packages
        self.install_packages(&graph)?;
        
        // Update world file
        self.update_world_file(package)?;
        
        Ok(())
    }
    
    pub fn update_world(&mut self) -> Result<(), PortageError> {
        // Get world packages
        let world_packages = self.get_world_packages()?;
        
        // Check for updates
        let updates = self.check_for_updates(&world_packages)?;
        
        // Apply updates
        for update in updates {
            self.emerge(&update.package)?;
        }
        
        // Clean obsolete packages
        self.clean_obsolete_packages()?;
        
        Ok(())
    }
}
```

**Key Features:**
- Source-based compilation
- USE flag system
- Dependency resolution
- Slot management
- Overlay support
- World file management

### 2. USE Flags System

The USE flag system provides build-time configuration:

```rust
pub struct UseFlagManager {
    pub global_flags: HashSet<UseFlag>,
    pub local_flags: HashMap<String, HashSet<UseFlag>>,
    pub profile_flags: HashSet<UseFlag>,
    pub environment_flags: HashSet<UseFlag>,
}

pub struct UseFlag {
    pub name: String,
    pub description: String,
    pub state: FlagState,
}

pub enum FlagState {
    Enabled,
    Disabled,
    Forced,
    Masked,
}

impl UseFlagManager {
    pub fn resolve_flags(&self, ebuild: &Ebuild) -> Result<HashSet<UseFlag>, UseFlagError> {
        let mut resolved = HashSet::new();
        
        // Start with profile flags
        resolved.extend(self.profile_flags.iter().cloned());
        
        // Add global flags
        resolved.extend(self.global_flags.iter().cloned());
        
        // Add package-specific flags
        if let Some(local_flags) = self.local_flags.get(&ebuild.name) {
            resolved.extend(local_flags.iter().cloned());
        }
        
        // Apply environment flags
        resolved.extend(self.environment_flags.iter().cloned());
        
        // Filter to only relevant flags for this package
        let relevant: HashSet<_> = resolved.into_iter()
            .filter(|flag| ebuild.use_flags.iter().any(|ef| ef.name == flag.name))
            .collect();
        
        Ok(relevant)
    }
    
    pub fn set_flag(&mut self, flag: &str, state: FlagState) -> Result<(), UseFlagError> {
        let use_flag = UseFlag {
            name: flag.to_string(),
            description: String::new(),
            state,
        };
        
        self.global_flags.insert(use_flag);
        self.save_configuration()?;
        
        Ok(())
    }
}
```

**Key Features:**
- Global flag management
- Package-specific flags
- Profile inheritance
- Environment overrides
- Flag dependency resolution

### 3. Profile System

The profile system provides system configuration:

```rust
pub struct ProfileManager {
    pub current_profile: Profile,
    pub available_profiles: Vec<Profile>,
    pub profile_hierarchy: Vec<Profile>,
}

pub struct Profile {
    pub name: String,
    pub path: PathBuf,
    pub use_flags: HashSet<UseFlag>,
    pub package_mask: HashSet<String>,
    pub package_unmask: HashSet<String>,
    pub package_keywords: HashMap<String, Keyword>,
}

impl ProfileManager {
    pub fn set_profile(&mut self, profile_name: &str) -> Result<(), ProfileError> {
        let profile = self.available_profiles.iter()
            .find(|p| p.name == profile_name)
            .ok_or(ProfileError::NotFound)?;
        
        // Validate profile
        self.validate_profile(profile)?;
        
        // Set new profile
        self.current_profile = profile.clone();
        
        // Update profile hierarchy
        self.update_profile_hierarchy(profile)?;
        
        // Apply profile settings
        self.apply_profile_settings(profile)?;
        
        Ok(())
    }
    
    pub fn get_profile_hierarchy(&self) -> Vec<Profile> {
        let mut hierarchy = Vec::new();
        let mut current = &self.current_profile;
        
        loop {
            hierarchy.push(current.clone());
            if let Some(parent) = self.get_parent_profile(current) {
                current = parent;
            } else {
                break;
            }
        }
        
        hierarchy
    }
}
```

**Key Features:**
- Profile selection
- Profile inheritance
- System configuration
- Package masking/unmasking
- Keyword management

### 4. OpenRC Init System

The OpenRC-like service management:

```rust
pub struct OpenRCManager {
    pub services: HashMap<String, Service>,
    pub runlevels: HashMap<String, Runlevel>,
    pub dependencies: ServiceDependencies,
}

pub struct Service {
    pub name: String,
    pub description: String,
    pub command: String,
    pub dependencies: Vec<String>,
    pub provides: Vec<String>,
    pub status: ServiceStatus,
}

impl OpenRCManager {
    pub fn start_service(&mut self, service_name: &str) -> Result<(), OpenRCError> {
        let service = self.services.get_mut(service_name)
            .ok_or(OpenRCError::ServiceNotFound)?;
        
        // Check dependencies
        self.check_dependencies(service)?;
        
        // Start service
        self.execute_command(&service.command)?;
        
        // Update status
        service.status = ServiceStatus::Running;
        
        Ok(())
    }
    
    pub fn add_to_runlevel(&mut self, service_name: &str, runlevel: &str) -> Result<(), OpenRCError> {
        let runlevel = self.runlevels.get_mut(runlevel)
            .ok_or(OpenRCError::RunlevelNotFound)?;
        
        let service = self.services.get(service_name)
            .ok_or(OpenRCError::ServiceNotFound)?;
        
        runlevel.services.push(service_name.to_string());
        self.update_runlevel_config(runlevel)?;
        
        Ok(())
    }
}
```

**Key Features:**
- Service management
- Runlevel configuration
- Dependency management
- Parallel service startup
- Service status monitoring

## Kernel Configuration

### Kernel Building

```rust
pub struct KernelBuilder {
    pub sources: KernelSources,
    pub config: KernelConfig,
    pub compiler: GccToolchain,
}

impl KernelBuilder {
    pub fn build_kernel(&mut self) -> Result<(), KernelError> {
        // Prepare sources
        self.prepare_sources()?;
        
        // Configure kernel
        self.configure_kernel()?;
        
        // Build kernel
        self.build_kernel_image()?;
        
        // Build modules
        self.build_modules()?;
        
        // Install kernel
        self.install_kernel()?;
        
        // Update bootloader
        self.update_bootloader()?;
        
        Ok(())
    }
}
```

**Key Features:**
- Kernel configuration
- Custom kernel building
- Module management
- Kernel installation
- Bootloader integration

## Overlay System

### Custom Package Overlays

```rust
pub struct OverlayManager {
    pub overlays: Vec<Overlay>,
    pub enabled_overlays: Vec<String>,
}

pub struct Overlay {
    pub name: String,
    pub path: PathBuf,
    pub priority: u32,
    pub ebuilds: Vec<Ebuild>,
}

impl OverlayManager {
    pub fn add_overlay(&mut self, overlay: Overlay) -> Result<(), OverlayError> {
        // Validate overlay
        self.validate_overlay(&overlay)?;
        
        // Add overlay
        self.overlays.push(overlay);
        
        // Sort by priority
        self.overlays.sort_by_key(|o| o.priority);
        
        Ok(())
    }
    
    pub fn enable_overlay(&mut self, overlay_name: &str) -> Result<(), OverlayError> {
        if !self.overlays.iter().any(|o| o.name == overlay_name) {
            return Err(OverlayError::NotFound);
        }
        
        self.enabled_overlays.push(overlay_name.to_string());
        self.update_overlay_config()?;
        
        Ok(())
    }
}
```

**Key Features:**
- Overlay management
- Priority system
- Custom ebuild support
- Repository integration

## Testing

### Unit Tests

```bash
# Test Portage functionality
rustc --test --edition=2021 src/sigpkg/portage.rs -o build/portage_tests && ./build/portage_tests

# Test USE flags
rustc --test --edition=2021 src/sigpkg/use_flags.rs -o build/use_flags_tests && ./build/use_flags_tests
```

### Integration Tests

```bash
# Test package building
./tests/integration/gentoo_package_build.sh

# Test profile system
./tests/integration/gentoo_profile_system.sh
```

## Configuration

### Portage Configuration

```toml
[sigma-portage]
make-opts = "-j$(nproc)"
features = "ccache distcc"
accept-keywords = "~amd64"
```

### USE Flags Configuration

```toml
[use-flags]
global = ["X", "gnome", "kde", "systemd"]
local = { "www-client/firefox" = ["ffmpeg", "hwaccel"] }
```

## Troubleshooting

### Build Failures

```bash
# Check USE flags
sigmactl portage use-flags <package>

# Rebuild with specific flags
sigmactl portage emerge --useflags="<flags>" <package>

# Check build log
sigmactl portage build-log <package>
```

### Dependency Issues

```bash
# Show dependency graph
sigmactl portage depgraph <package>

# Resolve circular dependencies
sigmactl portage resolve-circular <package>

# Clean stale dependencies
sigmactl portage clean-deps
```

## Performance Optimization

### Parallel Building

```rust
let parallel = ParallelPortage::new();
parallel.emerge_parallel(vec!["firefox", "chromium", "libreoffice"])?;
```

### CCache Integration

```rust
let ccache = CCacheManager::new();
ccache.enable()?;
ccache.configure_cache_size("10G")?;
```

## Documentation Resources

- [Gentoo Handbook](https://wiki.gentoo.org/wiki/Handbook:Main_Page)
- [Portage Documentation](https://wiki.gentoo.org/wiki/Portage)
- [USE Flag Guide](https://wiki.gentoo.org/wiki/USE_flag)
- [Ebuild Guide](https://devmanual.gentoo.org/ebuild-references/)

## Best Practices

1. **Customization**: Leverage USE flags for optimal builds
2. **Performance**: Use appropriate CFLAGS for your hardware
3. **Stability**: Choose appropriate profiles for your needs
4. **Updates**: Regularly update world file and clean obsolete packages
5. **Documentation**: Document custom USE flags and configurations

## Migration Tools

### Gentoo Migration Assistant

```rust
let assistant = GentooMigrationAssistant::new();
assistant.migrate_from(DistroType::Arch)?;
```

**Supported Source Distributions:**
- Arch Linux
- Debian
- Fedora
- Ubuntu

## Future Enhancements

- Enhanced Portage performance
- Improved USE flag resolution
- Better kernel configuration tools
- Enhanced overlay management
- Improved documentation generation

---

*Last updated: August 21, 2026*
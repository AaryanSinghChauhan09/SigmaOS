# Gentoo Parity Features for SigmaOS

## Overview

This document outlines Gentoo-specific features and their implementation in SigmaOS to provide parity with Gentoo's focus on performance optimization, source-based compilation, and system customization.

## Portage Package Manager

### Source-Based Package Management

```rust
pub struct SigmaPortage {
    pub tree: PortageTree,
    pub database: PackageDatabase,
    pub profiles: ProfileManager,
    pub use_flags: UseFlagManager,
}

pub struct PortageTree {
    pub ebuilds: HashMap<String, Ebuild>,
    pub categories: Vec<String>,
    pub mirrors: Vec<Mirror>,
}

pub struct Ebuild {
    pub name: String,
    pub version: String,
    pub slot: String,
    pub description: String,
    pub homepage: String,
    pub license: String,
    pub iuse: Vec<String>, // USE flags
    pub dependencies: DependencySpec,
    pub src_uri: Vec<String>,
}

pub struct DependencySpec {
    pub depends: Vec<String>,
    pub rdepends: Vec<String>,
    pub pdepends: Vec<String>, // post-depends
}

impl SigmaPortage {
    pub fn emerge(&mut self, package: &str) -> Result<(), PortageError> {
        // Find ebuild
        let ebuild = self.tree.find_ebuild(package)?;

        // Parse USE flags
        let use_flags = self.use_flags.resolve_for_package(&ebuild)?;

        // Calculate dependencies
        let dependencies = self.calculate_dependencies(&ebuild, &use_flags)?;

        // Emerge dependencies first
        for dep in dependencies {
            self.emerge(&dep)?;
        }

        // Fetch sources
        self.fetch_sources(&ebuild)?;

        // Unpack sources
        self.unpack_sources(&ebuild)?;

        // Compile with USE flags
        self.compile_package(&ebuild, &use_flags)?;

        // Install package
        self.install_package(&ebuild)?;

        // Update database
        self.database.add_installed(&ebuild)?;

        Ok(())
    }

    pub fn update_use_flags(&mut self, package: &str, flags: Vec<String>) -> Result<(), PortageError> {
        // Update package.use
        self.update_package_use(package, flags)?;

        // Rebuild package with new USE flags
        self.rebuild_package(package)?;

        Ok(())
    }

    pub fn set_profile(&mut self, profile: &str) -> Result<(), PortageError> {
        // Validate profile
        self.profiles.validate_profile(profile)?;

        // Set system profile
        self.profiles.set_profile(profile)?;

        // Rebuild system with new profile
        self.rebuild_system()?;

        Ok(())
    }
}
```

## USE Flags System

### Customizable Build Options

```rust
pub struct SigmaUseFlags {
    pub global_flags: HashMap<String, bool>,
    pub package_flags: HashMap<String, HashMap<String, bool>>,
    pub profile_flags: HashMap<String, bool>,
}

pub struct UseFlag {
    pub name: String,
    pub description: String,
    pub flag_type: UseFlagType,
    pub default: bool,
}

pub enum UseFlagType {
    Global,
    Local,
    Expander,
    Architecture,
}

impl SigmaUseFlags {
    pub fn enable_flag(&mut self, flag: &str) -> Result<(), UseFlagError> {
        // Check if flag exists
        if !self.is_valid_flag(flag)? {
            return Err(UseFlagError::InvalidFlag(flag.to_string()));
        }

        // Determine flag type
        let flag_type = self.get_flag_type(flag)?;

        match flag_type {
            UseFlagType::Global => {
                self.global_flags.insert(flag.to_string(), true);
            }
            UseFlagType::Local => {
                // Need package context for local flags
                return Err(UseFlagError::LocalFlagNeedsPackage);
            }
            _ => {
                self.global_flags.insert(flag.to_string(), true);
            }
        }

        Ok(())
    }

    pub fn enable_flag_for_package(&mut self, package: &str, flag: &str) -> Result<(), UseFlagError> {
        let package_flags = self.package_flags.entry(package.to_string())
            .or_insert_with(HashMap::new);

        package_flags.insert(flag.to_string(), true);

        Ok(())
    }

    pub fn get_effective_flags(&self, package: &str) -> Result<Vec<String>, UseFlagError> {
        let mut effective = Vec::new();

        // Add global flags
        for (flag, enabled) in &self.global_flags {
            if *enabled {
                effective.push(flag.clone());
            }
        }

        // Add profile flags
        for (flag, enabled) in &self.profile_flags {
            if *enabled {
                effective.push(flag.clone());
            }
        }

        // Add package-specific flags
        if let Some(package_flags) = self.package_flags.get(package) {
            for (flag, enabled) in package_flags {
                if *enabled {
                    effective.push(flag.clone());
                }
            }
        }

        Ok(effective)
    }
}
```

## Gentoo Profiles

### System Configuration Profiles

```rust
pub struct SigmaProfiles {
    pub current_profile: String,
    pub available_profiles: Vec<Profile>,
    pub profile_hierarchy: Vec<String>,
}

pub struct Profile {
    pub name: String,
    pub path: String,
    pub parent: Option<String>,
    pub use_flags: Vec<String>,
    pub package_mask: Vec<String>,
    pub package_unmask: Vec<String>,
}

impl SigmaProfiles {
    pub fn get_profile_chain(&self) -> Result<Vec<Profile>, ProfileError> {
        let mut chain = Vec::new();
        let mut current = self.current_profile.clone();

        loop {
            let profile = self.available_profiles.iter()
                .find(|p| p.name == current)
                .ok_or(ProfileError::ProfileNotFound(current.clone()))?;

            chain.push(profile.clone());

            if let Some(ref parent) = profile.parent {
                current = parent.clone();
            } else {
                break;
            }
        }

        Ok(chain)
    }

    pub fn get_effective_settings(&self) -> Result<ProfileSettings, ProfileError> {
        let chain = self.get_profile_chain()?;

        let mut settings = ProfileSettings::default();

        // Merge settings from profile chain
        for profile in chain.iter().rev() {
            settings.use_flags.extend(profile.use_flags.clone());
            settings.package_mask.extend(profile.package_mask.clone());
            settings.package_unmask.extend(profile.package_unmask.clone());
        }

        Ok(settings)
    }
}
```

## Compile-Time Optimization

### Performance Tuning

```rust
pub struct SigmaCompiler {
    pub cflags: String,
    pub cxxflags: String,
    pub ldflags: String,
    pub makeopts: String,
    pub compiler: CompilerType,
}

pub enum CompilerType {
    GCC,
    Clang,
}

impl SigmaCompiler {
    pub fn optimize_for_system(&mut self) -> Result<(), CompilerError> {
        // Detect CPU features
        let cpu_features = self.detect_cpu_features()?;

        // Generate optimal CFLAGS
        self.cflags = self.generate_cflags(&cpu_features)?;

        // Set MAKEOPTS based on CPU cores
        let cores = self.get_cpu_cores()?;
        self.makeopts = format!("-j{}", cores);

        Ok(())
    }

    pub fn optimize_for_size(&mut self) -> Result<(), CompilerError> {
        // Set size optimization flags
        self.cflags = "-Os -pipe".to_string();
        self.cxxflags = self.cflags.clone();

        // Enable link-time optimization
        self.ldflags = "-Wl,--as-needed -Wl,--strip-all".to_string();

        Ok(())
    }

    pub fn optimize_for_performance(&mut self) -> Result<(), CompilerError> {
        // Set performance optimization flags
        self.cflags = "-O3 -march=native -pipe".to_string();
        self.cxxflags = self.cflags.clone();

        // Enable link-time optimization
        self.ldflags = "-Wl,--as-needed -Wl,--sort-common".to_string();

        Ok(())
    }
}
```

## Gentoo Kernel Configuration

### Custom Kernel Building

```rust
pub struct SigmaKernel {
    pub sources: KernelSources,
    pub config: KernelConfig,
    pub initramfs: InitramfsBuilder,
}

pub struct KernelSources {
    pub version: String,
    pub patches: Vec<KernelPatch>,
    pub extra_options: Vec<String>,
}

pub struct KernelConfig {
    pub options: HashMap<String, bool>,
    pub modules: Vec<String>,
    pub builtin: Vec<String>,
}

impl SigmaKernel {
    pub fn configure_kernel(&mut self, config: KernelConfig) -> Result<(), KernelError> {
        // Generate .config file
        self.generate_config(&config)?;

        // Build kernel
        self.build_kernel()?;

        // Build modules
        self.build_modules()?;

        // Install kernel
        self.install_kernel()?;

        Ok(())
    }

    pub fn build_custom_kernel(&mut self, options: HashMap<String, bool>) -> Result<(), KernelError> {
        // Download kernel sources
        self.sources.download()?;

        // Apply patches
        for patch in &self.sources.patches {
            self.apply_patch(patch)?;
        }

        // Configure kernel
        self.config.options = options;
        self.configure_kernel(self.config.clone())?;

        // Build initramfs
        self.initramfs.build()?;

        // Update bootloader
        self.update_bootloader()?;

        Ok(())
    }
}
```

## Gentoo Security Features

### Hardened Toolchain

```rust
pub struct SigmaHardened {
    pub toolchain: HardenedToolchain,
    pub pie_enabled: bool,
    pub ssp_enabled: bool,
    pub aslr_enabled: bool,
}

pub struct HardenedToolchain {
    pub compiler: String,
    pub features: Vec<HardenedFeature>,
}

pub enum HardenedFeature {
    PIE, // Position Independent Executable
    SSP, // Stack Smashing Protection
    RELRO, // Relocation Read-Only
    FORTIFY_SOURCE,
}

impl SigmaHardened {
    pub fn enable_hardened_features(&mut self) -> Result<(), HardenedError> {
        // Enable PIE
        self.pie_enabled = true;
        self.toolchain.features.push(HardenedFeature::PIE);

        // Enable SSP
        self.ssp_enabled = true;
        self.toolchain.features.push(HardenedFeature::SSP);

        // Enable RELRO
        self.toolchain.features.push(HardenedFeature::RELRO);

        // Enable FORTIFY_SOURCE
        self.toolchain.features.push(HardenedFeature::FORTIFY_SOURCE);

        // Enable ASLR
        self.aslr_enabled = true;
        self.enable_aslr()?;

        // Update compiler flags
        self.update_hardened_cflags()?;

        Ok(())
    }

    pub fn apply_hardened_profile(&mut self) -> Result<(), HardenedError> {
        // Switch to hardened profile
        self.switch_profile("hardened/linux/amd64")?;

        // Apply hardened USE flags
        self.apply_hardened_useflags()?;

        // Rebuild system with hardened toolchain
        self.rebuild_system_hardened()?;

        Ok(())
    }
}
```

## Gentoo Overlay System

### Community Software Repository

```rust
pub struct SigmaOverlays {
    pub overlays: Vec<Overlay>,
    pub active_overlays: Vec<String>,
    pub layman: LaymanManager,
}

pub struct Overlay {
    pub name: String,
    pub url: String,
    pub priority: u32,
    pub ebuilds: Vec<Ebuild>,
}

pub struct LaymanManager {
    pub installed_overlays: Vec<String>,
    pub overlay_list: Vec<Overlay>,
}

impl SigmaOverlays {
    pub fn add_overlay(&mut self, overlay: &str) -> Result<(), OverlayError> {
        // Find overlay in list
        let overlay_info = self.layman.overlay_list.iter()
            .find(|o| o.name == overlay)
            .ok_or(OverlayError::OverlayNotFound)?;

        // Add overlay
        self.layman.add_overlay(overlay)?;

        // Update Portage tree
        self.update_portage_tree()?;

        // Add to active overlays
        self.active_overlays.push(overlay.to_string());

        Ok(())
    }

    pub fn sync_overlays(&mut self) -> Result<(), OverlayError> {
        // Sync all active overlays
        for overlay_name in &self.active_overlays {
            self.sync_single_overlay(overlay_name)?;
        }

        // Update Portage cache
        self.update_portage_cache()?;

        Ok(())
    }
}
```

## Best Practices

1.  **Source-Based**: Compile from source for maximum optimization
2.  **Customization**: Use USE flags for fine-grained control
3.  **Performance**: Optimize compilation flags for specific hardware
4.  **Security**: Implement hardened security features
5.  **Flexibility**: Support multiple profiles and configurations

## Migration Tools

### Gentoo Migration Assistant

```rust
pub struct GentooMigrationAssistant {
    pub config: MigrationConfig,
    pub package_mapper: PackageMapper,
}

impl GentooMigrationAssistant {
    pub fn migrate_from(&self, source_distro: DistroType) -> Result<MigrationStatus, MigrationError> {
        match source_distro {
            DistroType::Arch => self.migrate_from_arch(),
            DistroType::Fedora => self.migrate_from_fedora(),
            DistroType::Ubuntu => self.migrate_from_ubuntu(),
            _ => Err(MigrationError::UnsupportedDistro),
        }
    }

    fn migrate_from_arch(&self) -> Result<MigrationStatus, MigrationError> {
        // Analyze Arch system
        let analysis = self.analyze_arch_system()?;

        // Map Arch packages to Gentoo equivalents
        let packages = self.package_mapper.map_arch_to_gentoo(analysis.packages)?;

        // Determine appropriate USE flags
        let use_flags = self.recommend_use_flags(&analysis)?;

        // Select profile
        let profile = self.recommend_profile(&analysis)?;

        // Set up Gentoo system
        self.setup_gentoo_base(profile)?;

        // Apply USE flags
        self.apply_use_flags(use_flags)?;

        // Emerge packages
        for package in packages {
            self.emerge_package(&package)?;
        }

        Ok(MigrationStatus::Success)
    }
}
```

## References

*   [Gentoo Handbook](https://wiki.gentoo.org/wiki/Handbook:Main_Page)
*   [Portage Documentation](https://wiki.gentoo.org/wiki/Portage)
*   [USE Flags Guide](https://wiki.gentoo.org/wiki/USE_flag)
*   [Gentoo Profiles](https://wiki.gentoo.org/wiki/Profile)
*   [Gentoo Overlays](https://gpo.zugaina.org/)

## Implementation Status (Fully Implemented in Safe Rust)

SigmaOS natively implements all Gentoo Linux parity features:

1.  **Portage Package & USE Flag Resolver (`GentooPortageUseFlagsEngine`, `PortagePackage`)**: Implemented in `src/distro/linux_bsd_parity.rs` and `src/distro/gentoo.rs` supporting global/package USE flags, slotting, and dependency resolution.
2.  **Portage Package Masking & Unmasking Engine (`GentooPortageMaskEngine`)**: Implemented in `src/unimplemented_features.rs` supporting package masking (`/etc/portage/package.mask`), license filtering, and version slot management.
3.  **Ebuild Compiler & Overlay Builder (`PortageEbuildProfile`)**: Implemented in `src/unimplemented_features.rs` & `src/distro/gentoo.rs` providing zero-dependency safe Rust ebuild parsing and source compilation pipelines.

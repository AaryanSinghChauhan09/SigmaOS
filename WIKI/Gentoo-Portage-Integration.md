# Gentoo Portage Integration

SigmaOS implements Gentoo Portage integration for flexible package management with USE flags, dependency resolution, and source-based compilation with optimization.

## Overview

Portage provides:
- Source-based package management
- USE flag system for feature selection
- Ebuild format for package definitions
- Dependency resolution with SAT solver
- Slot management for multiple versions
- Subslot rebuild triggers (:=)
- Mask resolution for package conflicts
- Binary package support
- Profile-based configuration

## Architecture

### Portage Pipeline
```
Ebuild → Dependency Resolution → Fetch → Compile → Install → Merge
                                            ↓
                                    USE Flag Processing
                                            ↓
                                    Configuration Protection
```

### Package Slots
- **Slot**: Major version differentiation
- **Subslot**: Library ABI version
- **Slot operators**: =, <, >, <=, >=, ~, :=

## Implementation

### Portage Manager
```rust
// src/package/portage/manager.rs
pub struct PortageManager {
    pub ebuilds: BTreeMap<String, Ebuild>,
    pub installed: BTreeMap<String, InstalledPackage>,
    pub use_flags: BTreeMap<String, bool>,
    pub masks: BTreeMap<String, MaskReason>,
    pub profile: PortageProfile,
}

#[derive(Debug, Clone)]
pub struct Ebuild {
    pub name: String,
    pub version: String,
    pub slot: String,
    pub subslot: String,
    pub dependencies: Vec<Dependency>,
    pub use_flags: Vec<UseFlag>,
    pub ebuild_path: PathBuf,
}

impl PortageManager {
    pub fn new() -> Self {
        PortageManager {
            ebuilds: BTreeMap::new(),
            installed: BTreeMap::new(),
            use_flags: BTreeMap::new(),
            masks: BTreeMap::new(),
            profile: PortageProfile::default(),
        }
    }

    pub fn load_ebuilds(&mut self, overlay_path: &Path) -> Result<(), PortageError> {
        // Load ebuilds from overlay
        for entry in walkdir::WalkDir::new(overlay_path) {
            let entry = entry?;
            if entry.path().extension() == Some(std::ffi::OsStr::new("ebuild")) {
                let ebuild = self.parse_ebuild(&entry.path())?;
                self.ebuilds.insert(ebuild.name.clone(), ebuild);
            }
        }
        
        Ok(())
    }

    pub fn install(&mut self, package: &str) -> Result<(), PortageError> {
        // Resolve dependencies
        let dependencies = self.resolve_dependencies(package)?;
        
        // Install dependencies
        for dep in dependencies {
            self.install_package(&dep)?;
        }
        
        // Install package
        self.install_package(package)?;
        
        Ok(())
    }

    pub fn resolve_dependencies(&self, package: &str) -> Result<Vec<String>, PortageError> {
        let ebuild = self.ebuilds.get(package)
            .ok_or(PortageError::PackageNotFound)?;
        
        // Use SAT solver for dependency resolution
        let solver = SatSolver::new();
        let solution = solver.solve(&ebuild.dependencies, &self.ebuilds)?;
        
        Ok(solution)
    }

    pub fn install_package(&mut self, package: &str) -> Result<(), PortageError> {
        let ebuild = self.ebuilds.get(package)
            .ok_or(PortageError::PackageNotFound)?;
        
        // Check mask
        if let Some(reason) = self.masks.get(package) {
            return Err(PortageError::Masked(reason.clone()));
        }
        
        // Fetch sources
        self.fetch_sources(ebuild)?;
        
        // Compile with USE flags
        self.compile(ebuild)?;
        
        // Install
        self.merge(ebuild)?;
        
        // Record installation
        self.record_installation(ebuild)?;
        
        Ok(())
    }
}
```

### USE Flag Management
```rust
// src/package/portage/use_flags.rs
pub struct UseFlagManager {
    pub global_flags: BTreeMap<String, bool>,
    pub package_flags: BTreeMap<String, BTreeMap<String, bool>>,
    pub profile_flags: BTreeMap<String, bool>,
}

#[derive(Debug, Clone)]
pub struct UseFlag {
    pub name: String,
    pub description: String,
    pub global: bool,
}

impl UseFlagManager {
    pub fn new() -> Self {
        UseFlagManager {
            global_flags: BTreeMap::new(),
            package_flags: BTreeMap::new(),
            profile_flags: BTreeMap::new(),
        }
    }

    pub fn set_global_flag(&mut self, flag: &str, enabled: bool) {
        self.global_flags.insert(flag.to_string(), enabled);
    }

    pub fn set_package_flag(&mut self, package: &str, flag: &str, enabled: bool) {
        self.package_flags
            .entry(package.to_string())
            .or_insert_with(BTreeMap::new)
            .insert(flag.to_string(), enabled);
    }

    pub fn get_flags(&self, package: &str) -> BTreeMap<String, bool> {
        let mut flags = BTreeMap::new();
        
        // Global flags
        for (flag, enabled) in self.global_flags.iter() {
            flags.insert(flag.clone(), *enabled);
        }
        
        // Profile flags
        for (flag, enabled) in self.profile_flags.iter() {
            flags.insert(flag.clone(), *enabled);
        }
        
        // Package-specific flags
        if let Some(package_flags) = self.package_flags.get(package) {
            for (flag, enabled) in package_flags.iter() {
                flags.insert(flag.clone(), *enabled);
            }
        }
        
        flags
    }
}
```

### Slot Management
```rust
// src/package/portage/slots.rs
pub struct SlotManager {
    pub slots: BTreeMap<String, Vec<InstalledPackage>>,
    pub subslots: BTreeMap<String, String>,
}

impl SlotManager {
    pub fn new() -> Self {
        SlotManager {
            slots: BTreeMap::new(),
            subslots: BTreeMap::new(),
        }
    }

    pub fn add_package(&mut self, package: InstalledPackage) -> Result<(), PortageError> {
        let slot = package.slot.clone();
        
        // Add to slot
        self.slots
            .entry(slot.clone())
            .or_insert_with(Vec::new)
            .push(package.clone());
        
        // Update subslot
        self.subslots.insert(package.name.clone(), package.subslot.clone());
        
        Ok(())
    }

    pub fn get_slot(&self, package: &str) -> Option<String> {
        self.subslots.get(package).cloned()
    }

    pub fn get_packages_in_slot(&self, slot: &str) -> Vec<InstalledPackage> {
        self.slots.get(slot).cloned().unwrap_or_default()
    }

    pub fn trigger_subslot_rebuild(&self, subslot: &str) -> Vec<String> {
        let mut packages = Vec::new();
        
        for (name, pkg_subslot) in self.subslots.iter() {
            if pkg_subslot == subslot {
                packages.push(name.clone());
            }
        }
        
        packages
    }
}
```

## Configuration

### Portage Configuration
```toml
# /etc/sigmaos/portage.toml
[make_conf]
# Global USE flags
use_flags = ["X", "gtk", "gnome", "kde", "systemd"]

[package]
# Package-specific settings
[package.firefox]
use_flags = ["X", "gtk", "systemd"]
slot = "0"

[package.gcc]
use_flags = ["multilib", "nptl"]
slot = "0"

[masks]
# Package masks
[package.flash]
mask = true
reason = "EOL security risk"

[overlays]
# Portage overlays
overlays = ["gentoo", "sigmaos"]
```

### Runtime Control
```bash
# Update portage tree
sigportage sync

# Search for package
sigportage search firefox

# Install package
sigportage install firefox

# Install with USE flags
sigportage install firefox --use "X gtk systemd"

# View package information
sigportage info firefox

# View USE flags
sigportage use-flags firefox

# Set global USE flag
sigportage set-use-flag X true

# Set package USE flag
sigportage set-package-use-flag firefox systemd true

# Mask package
sigportage mask flash

# Unmask package
sigportage unmask flash

# View installed packages
sigportage list-installed

# Rebuild packages with subslot changes
sigportage rebuild-subslot :=dev-libs/libffi
```

## Performance Optimization

### Parallel Compilation
Enable parallel compilation for faster builds:
```bash
# Set MAKEOPTS
sigportage set-makeopts "-j8"

# Enable binary packages
sigportage set-binary-packages true
```

### Binary Packages
Use binary packages for faster installation:
```bash
# Enable binary packages
sigportage set-binary-packages true

# Build binary package
sigportage buildpkg firefox

# Install from binary
sigportage install --getbinpkg firefox
```

### Dependency Resolution
Optimize dependency resolution:
```bash
# Use SAT solver
sigportage set-solver sat

# Enable resolver cache
sigportage set-resolver-cache true
```

## Troubleshooting

### Dependency Conflict
If dependency conflict occurs:
1. Check dependencies: `sigportage info package`
2. Check USE flags: `sigportage use-flags package`
3. Adjust USE flags: `sigportage set-package-use-flag package flag false`
4. Check for mask conflicts
5. Try specific version: `sigportage install package-1.0.0`

### Compilation Fails
If compilation fails:
1. Check USE flags: `sigportage use-flags package`
2. Disable problematic flags: `sigportage set-package-use-flag package flag false`
3. Check build log: `sigportage build-log package`
4. Try binary package: `sigportage install --getbinpkg package`
5. Check compiler version

### Subslot Rebuild Needed
If subslot rebuild is needed:
1. Check subslot changes: `sigportage subslot-changes`
2. Rebuild affected packages: `sigportage rebuild-subslot :=lib`
3. Check for broken dependencies
4. Update world file: `sigportage world-update`

### Mask Issues
If package is masked:
1. Check mask reason: `sigportage mask-info package`
2. Accept mask if safe: `sigportage accept-keyword package`
3. Unmask if needed: `sigportage unmask package`
4. Check for security issues
5. Consider alternative package

---

**[Package Management](Category-Package-Management)** | **[SigmaPkg](SigmaPkg)]** | **[Build System](Category-Build)**

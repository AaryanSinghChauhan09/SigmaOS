# Debian Parity Features for SigmaOS

## Overview

This document outlines Debian-specific features and their implementation in SigmaOS to provide parity with Debian's focus on stability, security, and free software principles.

## APT Package Management

### Advanced Package Tool Integration

```rust
pub struct SigmaAPT {
    pub database: PackageDatabase,
    pub sources: Vec<Source>,
    pub dpkg_status: DpkgStatus,
    pub configuration: APTConfig,
}

pub struct Source {
    pub uri: String,
    pub distribution: String,
    pub components: Vec<String>,
    pub enabled: bool,
    pub trusted: bool,
}

pub struct APTConfig {
    pub install_recommends: bool,
    pub install_suggests: bool,
    pub allow_unauthenticated: bool,
    pub timeout: Duration,
}

impl SigmaAPT {
    pub fn update(&mut self) -> Result<(), AptError> {
        // Update package lists from all sources
        for source in &mut self.sources {
            if source.enabled {
                self.update_source(source)?;
            }
        }
        
        // Update internal database
        self.rebuild_database()?;
        
        Ok(())
    }
    
    pub fn install(&mut self, packages: Vec<String>) -> Result<(), AptError> {
        for package in packages {
            // Check if package is already installed
            if self.dpkg_status.is_installed(&package) {
                continue;
            }
            
            // Get package information
            let pkg_info = self.database.get_package(&package)?;
            
            // Resolve dependencies
            let dependencies = self.resolve_dependencies(&pkg_info)?;
            
            // Install dependencies first
            for dep in dependencies {
                self.install(vec![dep])?;
            }
            
            // Download package
            let deb_file = self.download_package(&pkg_info)?;
            
            // Verify package signature
            self.verify_package(&deb_file)?;
            
            // Install package using dpkg
            self.dpkg_install(&deb_file)?;
            
            // Update status
            self.dpkg_status.mark_installed(&package);
        }
        
        Ok(())
    }
    
    pub fn remove(&mut self, packages: Vec<String>, purge: bool) -> Result<(), AptError> {
        for package in packages {
            // Check for reverse dependencies
            let dependents = self.find_reverse_dependencies(&package);
            if !dependents.is_empty() {
                return Err(AptError::HasDependents(dependents));
            }
            
            // Remove package
            self.dpkg_remove(&package)?;
            
            // Purge configuration files if requested
            if purge {
                self.dpkg_purge(&package)?;
            }
            
            // Update status
            self.dpkg_status.mark_removed(&package);
        }
        
        Ok(())
    }
}
```

## dpkg Backend

### Low-Level Package Management

```rust
pub struct SigmaDpkg {
    pub database: DpkgDatabase,
    pub config_files: HashMap<String, PathBuf>,
    pub diversions: Vec<Divergence>,
}

pub struct DpkgDatabase {
    pub status: DpkgStatus,
    pub available: HashMap<String, PackageInfo>,
    pub installed: HashMap<String, InstalledPackage>,
}

pub struct Divergence {
    pub original: PathBuf,
    pub diverted: PathBuf,
    pub package: String,
}

impl SigmaDpkg {
    pub fn configure(&mut self, package: &str) -> Result<(), DpkgError> {
        let installed = self.database.installed.get(package)
            .ok_or(DpkgError::PackageNotInstalled)?;
        
        // Run pre-configure script
        self.run_script(&installed.scripts.pre_config)?;
        
        // Configure package
        self.configure_package(package)?;
        
        // Run post-configure script
        self.run_script(&installed.scripts.post_config)?;
        
        Ok(())
    }
    
    pub fn trigger(&mut self, trigger_name: &str) -> Result<(), DpkgError> {
        // Find packages with pending triggers
        let packages = self.find_packages_with_trigger(trigger_name)?;
        
        for package in packages {
            // Run trigger script
            self.run_trigger_script(&package, trigger_name)?;
        }
        
        Ok(())
    }
}
```

## Debian Policy Compliance

### Filesystem Hierarchy Standard

```rust
pub struct SigmaFHS {
    pub directories: Vec<FHSDirectory>,
    pub permissions: HashMap<PathBuf, FileMode>,
    pub owners: HashMap<PathBuf, Owner>,
}

pub struct FHSDirectory {
    pub path: PathBuf,
    pub purpose: String,
    pub minimal: bool,
    pub required: bool,
}

impl SigmaFHS {
    pub fn create_fhs_structure(&self) -> Result<(), FhsError> {
        // Create essential directories
        for dir in &self.directories {
            if dir.required {
                self.create_directory(&dir.path)?;
                
                // Set permissions
                if let Some(&mode) = self.permissions.get(&dir.path) {
                    self.set_permissions(&dir.path, mode)?;
                }
                
                // Set ownership
                if let Some(&owner) = self.owners.get(&dir.path) {
                    self.set_ownership(&dir.path, owner)?;
                }
            }
        }
        
        Ok(())
    }
    
    pub fn validate_fhs_compliance(&self) -> Result<ComplianceReport, FhsError> {
        let mut report = ComplianceReport::new();
        
        // Check directory existence
        for dir in &self.directories {
            if dir.required && !self.directory_exists(&dir.path) {
                report.add_violation(dir.path.clone(), "Required directory missing");
            }
        }
        
        // Check permissions
        for (path, expected_mode) in &self.permissions {
            let actual_mode = self.get_permissions(path)?;
            if actual_mode != *expected_mode {
                report.add_violation(path.clone(), "Incorrect permissions");
            }
        }
        
        Ok(report)
    }
}
```

## Debian Security Integration

### Security Updates Management

```rust
pub struct SigmaSecurityManager {
    pub security_repo: SecurityRepository,
    pub vulnerability_database: VulnerabilityDatabase,
    pub auto_updates: AutoUpdateConfig,
}

pub struct SecurityRepository {
    pub packages: Vec<SecurityPackage>,
    pub signature_key: String,
}

pub struct SecurityPackage {
    pub name: String,
    pub cve_ids: Vec<String>,
    pub severity: SecuritySeverity,
    pub fix_version: String,
}

pub enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
}

impl SigmaSecurityManager {
    pub fn check_vulnerabilities(&self) -> Result<Vec<Vulnerability>, SecurityError> {
        let mut vulnerabilities = Vec::new();
        
        // Get installed packages
        let installed = self.get_installed_packages()?;
        
        // Check against vulnerability database
        for package in installed {
            if let Some(cves) = self.vulnerability_database.get_cves(&package.name, &package.version) {
                for cve in cves {
                    vulnerabilities.push(cve);
                }
            }
        }
        
        Ok(vulnerabilities)
    }
    
    pub fn apply_security_update(&mut self, package: &str) -> Result<(), SecurityError> {
        // Get security update
        let security_pkg = self.security_repo.get_security_package(package)?;
        
        // Check if update is available
        if security_pkg.is_none() {
            return Err(SecurityError::NoSecurityUpdate);
        }
        
        let security_pkg = security_pkg.unwrap();
        
        // Apply update
        self.install_security_package(&security_pkg)?;
        
        // Mark as fixed
        self.vulnerability_database.mark_fixed(package, &security_pkg.fix_version);
        
        Ok(())
    }
}
```

## Debian Alternatives System

### Alternative Programs Management

```rust
pub struct SigmaAlternatives {
    pub groups: HashMap<String, AlternativeGroup>,
    pub selections: HashMap<String, String>,
}

pub struct AlternativeGroup {
    pub name: String,
    pub path: PathBuf,
    pub alternatives: Vec<Alternative>,
    pub current: Option<String>,
}

pub struct Alternative {
    pub name: String,
    pub path: PathBuf,
    pub priority: i32,
    pub slaves: Vec<SlaveLink>,
}

pub struct SlaveLink {
    pub name: String,
    pub path: PathBuf,
}

impl SigmaAlternatives {
    pub fn register_alternative(&mut self, group: AlternativeGroup) -> Result<(), AlternativesError> {
        // Check if group exists
        if self.groups.contains_key(&group.name) {
            return Err(AlternativesError::GroupExists);
        }
        
        // Create group
        self.groups.insert(group.name.clone(), group);
        
        // Select highest priority alternative
        self.select_best_alternative(&group.name)?;
        
        Ok(())
    }
    
    pub fn set_alternative(&mut self, group_name: &str, alternative_name: &str) -> Result<(), AlternativesError> {
        let group = self.groups.get_mut(group_name)
            .ok_or(AlternativesError::GroupNotFound)?;
        
        // Check if alternative exists
        if !group.alternatives.iter().any(|a| a.name == alternative_name) {
            return Err(AlternativesError::AlternativeNotFound);
        }
        
        // Set current selection
        group.current = Some(alternative_name.to_string());
        self.selections.insert(group_name.to_string(), alternative_name.to_string());
        
        // Update symlinks
        self.update_symlinks(group)?;
        
        Ok(())
    }
}
```

## Init System Integration

### SysVinit Compatibility

```rust
pub struct SigmaSysVInit {
    pub scripts: HashMap<String, InitScript>,
    pub runlevels: HashMap<u8, Runlevel>,
    pub services: HashMap<String, Service>,
}

pub struct InitScript {
    pub name: String,
    pub description: String,
    pub provides: Vec<String>,
    pub required_start: Vec<String>,
    pub required_stop: Vec<String>,
    pub default_start: Vec<u8>,
    pub default_stop: Vec<u8>,
}

pub struct Runlevel {
    pub level: u8,
    pub services: Vec<String>,
    pub scripts: Vec<PathBuf>,
}

impl SigmaSysVInit {
    pub fn enable_service(&mut self, service: &str, runlevel: u8) -> Result<(), InitError> {
        let script = self.scripts.get(service)
            .ok_or(InitError::ScriptNotFound)?;
        
        // Check if runlevel is valid
        if !script.default_start.contains(&runlevel) {
            return Err(InitError::InvalidRunlevel);
        }
        
        // Create symlink in runlevel directory
        let runlevel_dir = self.get_runlevel_dir(runlevel);
        let script_path = self.get_script_path(service);
        let link_path = runlevel_dir.join(format!("S{:02}{}", self.get_start_priority(), service));
        
        self.create_symlink(&script_path, &link_path)?;
        
        // Update runlevel configuration
        self.runlevels.get_mut(&runlevel)
            .ok_or(InitError::RunlevelNotFound)?
            .services.push(service.to_string());
        
        Ok(())
    }
    
    pub fn start_service(&mut self, service: &str) -> Result<(), InitError> {
        let script = self.scripts.get(service)
            .ok_or(InitError::ScriptNotFound)?;
        
        // Check dependencies
        self.check_dependencies(&script.required_start)?;
        
        // Execute start script
        self.execute_script(service, "start")?;
        
        // Update service status
        self.services.get_mut(service)
            .ok_or(InitError::ServiceNotFound)?
            .status = ServiceStatus::Running;
        
        Ok(())
    }
}
```

## Debian Installer Compatibility

### Installation Framework

```rust
pub struct SigmaInstaller {
    pub bootloader: BootloaderInstaller,
    pub partitioner: Partitioner,
    pub network_configurator: NetworkConfigurator,
    pub user_manager: UserManager,
    pub software_selector: SoftwareSelector,
}

pub struct SoftwareSelector {
    pub tasks: Vec<Task>,
    pub packages: Vec<Package>,
    pub selections: Vec<String>,
}

pub struct Task {
    pub name: String,
    pub description: String,
    pub packages: Vec<String>,
    pub essential: bool,
}

impl SigmaInstaller {
    pub fn install_system(&mut self, config: InstallConfig) -> Result<(), InstallError> {
        // Partition disk
        self.partitioner.partition_disk(&config.disk_config)?;
        
        // Install base system
        self.install_base_system()?;
        
        // Configure bootloader
        self.bootloader.install(&config.bootloader_config)?;
        
        // Configure network
        self.network_configurator.configure(&config.network_config)?;
        
        // Setup users
        self.user_manager.setup_users(&config.user_config)?;
        
        // Install selected software
        self.install_selected_software(&config.software_selection)?;
        
        // Finalize installation
        self.finalize_installation()?;
        
        Ok(())
    }
    
    fn install_selected_software(&mut self, selections: &Vec<String>) -> Result<(), InstallError> {
        for selection in selections {
            // Check if it's a task
            if let Some(task) = self.software_selector.tasks.iter().find(|t| &t.name == selection) {
                for package in &task.packages {
                    self.install_package(package)?;
                }
            }
            // Otherwise it's a package
            else {
                self.install_package(selection)?;
            }
        }
        
        Ok(())
    }
}
```

## Debian Live System

### Live Image Creation

```rust
pub struct SigmaLiveBuilder {
    pub base_system: BaseSystem,
    pub live_config: LiveConfig,
    pub packages: Vec<Package>,
    pub hooks: Vec<LiveHook>,
}

pub struct LiveConfig {
    pub distribution: String,
    pub architecture: String,
    pub bootloader: BootloaderType,
    pub hostname: String,
    pub username: String,
}

pub struct LiveHook {
    pub name: String,
    pub stage: HookStage,
    pub script: PathBuf,
}

pub enum HookStage {
    Bootstrap,
    Chroot,
    Binary,
    Source,
}

impl SigmaLiveBuilder {
    pub fn build_live_image(&mut self) -> Result<PathBuf, LiveError> {
        // Bootstrap base system
        self.bootstrap_system()?;
        
        // Install packages
        self.install_packages()?;
        
        // Configure system
        self.configure_system()?;
        
        // Run chroot hooks
        self.run_hooks(HookStage::Chroot)?;
        
        // Build binary image
        let image = self.build_binary()?;
        
        // Run binary hooks
        self.run_hooks(HookStage::Binary)?;
        
        Ok(image)
    }
    
    fn bootstrap_system(&mut self) -> Result<(), LiveError> {
        // Create bootstrap directory
        let bootstrap_dir = self.create_bootstrap_dir()?;
        
        // Bootstrap system using debootstrap
        self.run_debootstrap(&bootstrap_dir, &self.live_config.distribution)?;
        
        // Set up bootstrap
        self.setup_bootstrap(&bootstrap_dir)?;
        
        Ok(())
    }
}
```

## Best Practices

1. **Stability First**: Prioritize stability and reliability over new features
2. **Free Software**: Maintain commitment to free software principles
3. **Security**: Implement robust security measures and regular updates
4. **Policy Compliance**: Follow Debian Policy Guidelines strictly
5. **Minimalism**: Keep base system minimal and extensible

## Migration Tools

### Debian Migration Assistant

```rust
pub struct DebianMigrationAssistant {
    pub config: MigrationConfig,
    pub package_mapper: PackageMapper,
}

impl DebianMigrationAssistant {
    pub fn migrate_from(&self, source_distro: DistroType) -> Result<MigrationStatus, MigrationError> {
        match source_distro {
            DistroType::Ubuntu => self.migrate_from_ubuntu(),
            DistroType::Mint => self.migrate_from_mint(),
            DistroType::Fedora => self.migrate_from_fedora(),
            _ => Err(MigrationError::UnsupportedDistro),
        }
    }
    
    fn migrate_from_ubuntu(&self) -> Result<MigrationStatus, MigrationError> {
        // Map Ubuntu packages to Debian equivalents
        let packages = self.package_mapper.map_ubuntu_to_debian();
        
        // Install mapped packages
        for pkg in packages {
            self.install_package(&pkg)?;
        }
        
        // Configure APT sources
        self.configure_apt_sources()?;
        
        // Migrate user data
        self.migrate_user_data()?;
        
        Ok(MigrationStatus::Success)
    }
}
```

## References

- [Debian Policy Manual](https://www.debian.org/doc/debian-policy/)
- [APT Documentation](https://manpages.debian.org/bookworm/apt/apt.8.en.html)
- [dpkg Documentation](https://manpages.debian.org/bookworm/dpkg/dpkg.1.en.html)
- [Debian Security Team](https://www.debian.org/security/)
- [Filesystem Hierarchy Standard](https://refspecs.linuxfoundation.org/FHS_3.0/fhs-3.0.pdf)

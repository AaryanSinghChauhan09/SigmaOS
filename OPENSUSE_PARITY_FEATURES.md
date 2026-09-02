# openSUSE Parity Features for SigmaOS

## Overview

This document outlines openSUSE-specific features and their implementation in SigmaOS to provide parity with openSUSE's focus on YaST configuration, Btrfs filesystem, and enterprise-grade system management.

## YaST Configuration System

### Unified System Administration

```rust
pub struct SigmaYaST {
    pub modules: HashMap<String, YaSTModule>,
    pub ncurses_interface: NcursesInterface,
    pub qt_interface: QtInterface,
}

pub struct YaSTModule {
    pub name: String,
    pub description: String,
    pub category: YaSTCategory,
    pub settings: HashMap<String, SettingValue>,
}

pub enum YaSTCategory {
    System,
    Hardware,
    Network,
    Software,
    Security,
    Services,
}

pub enum SettingValue {
    String(String),
    Integer(i64),
    Boolean(bool),
    List(Vec<String>),
}

impl SigmaYaST {
    pub fn launch_module(&mut self, module_name: &str) -> Result<(), YaSTError> {
        let module = self.modules.get(module_name)
            .ok_or(YaSTError::ModuleNotFound)?;
        
        // Launch appropriate interface
        self.ncurses_interface.launch_module(module)?;
        
        Ok(())
    }

    pub fn configure_network(&mut self, config: NetworkConfig) -> Result<(), YaSTError> {
        let network_module = self.modules.get_mut("network")
            .ok_or(YaSTError::ModuleNotFound)?;
        
        network_module.settings.insert("interface".to_string(), 
            SettingValue::String(config.interface));
        network_module.settings.insert("ip_address".to_string(), 
            SettingValue::String(config.ip_address));
        
        self.apply_settings(network_module)?;
        Ok(())
    }
}
```

## Zypper Package Manager

### Advanced Package Management

```rust
pub struct SigmaZypper {
    pub repositories: Vec<ZypperRepository>,
    pub locks: Vec<PackageLock>,
    pub patterns: Vec<Pattern>,
}

pub struct ZypperRepository {
    pub name: String,
    pub url: String,
    pub enabled: bool,
    pub autorefresh: bool,
    pub priority: i32,
    pub gpgcheck: bool,
}

pub struct PackageLock {
    pub package_name: String,
    pub lock_type: LockType,
}

pub enum LockType {
    Permanent,
    Temporary,
}

impl SigmaZypper {
    pub fn add_repository(&mut self, repo: ZypperRepository) -> Result<(), ZypperError> {
        self.repositories.push(repo);
        self.refresh_repository(&repo.name)?;
        Ok(())
    }

    pub fn install(&mut self, packages: Vec<String>) -> Result<(), ZypperError> {
        for package in packages {
            // Check for locks
            if self.is_locked(&package) {
                return Err(ZypperError::PackageLocked(package));
            }
            
            // Resolve dependencies
            let deps = self.resolve_dependencies(&package)?;
            
            // Install dependencies
            for dep in deps {
                self.install(vec![dep])?;
            }
            
            // Install package
            self.install_package(&package)?;
        }
        Ok(())
    }

    pub fn lock_package(&mut self, package: &str, lock_type: LockType) {
        let lock = PackageLock {
            package_name: package.to_string(),
            lock_type,
        };
        self.locks.push(lock);
    }
}
```

## Btrfs Filesystem

### Advanced Filesystem Features

```rust
pub struct SigmaBtrfs {
    pub subvolumes: Vec<BtrfsSubvolume>,
    pub snapshots: Vec<BtrfsSnapshot>,
    pub raid_config: RaidConfig,
}

pub struct BtrfsSubvolume {
    pub name: String,
    pub path: PathBuf,
    pub uuid: String,
    pub readonly: bool,
}

pub struct BtrfsSnapshot {
    pub name: String,
    pub source_subvolume: String,
    pub timestamp: DateTime<Utc>,
    pub readonly: bool,
}

pub enum RaidConfig {
    Single,
    Raid0,
    Raid1,
    Raid10,
    Raid5,
    Raid6,
}

impl SigmaBtrfs {
    pub fn create_subvolume(&mut self, name: &str, path: &Path) -> Result<(), BtrfsError> {
        let subvolume = BtrfsSubvolume {
            name: name.to_string(),
            path: path.to_path_buf(),
            uuid: Uuid::new_v4().to_string(),
            readonly: false,
        };
        
        self.create_btrfs_subvolume(&subvolume)?;
        self.subvolumes.push(subvolume);
        
        Ok(())
    }

    pub fn create_snapshot(&mut self, source: &str, name: &str) -> Result<(), BtrfsError> {
        let source_subvolume = self.find_subvolume(source)?;
        let snapshot = BtrfsSnapshot {
            name: name.to_string(),
            source_subvolume: source.to_string(),
            timestamp: Utc::now(),
            readonly: true,
        };
        
        self.create_btrfs_snapshot(&source_subvolume, &snapshot)?;
        self.snapshots.push(snapshot);
        
        Ok(())
    }

    pub fn setup_raid(&mut self, config: RaidConfig, devices: Vec<PathBuf>) -> Result<(), BtrfsError> {
        self.raid_config = config;
        self.create_raid_array(config, devices)?;
        Ok(())
    }
}
```

## Snapper System Snapshot

### Btrfs Snapshot Management

```rust
pub struct SigmaSnapper {
    pub configs: Vec<SnapperConfig>,
    pub snapshots: Vec<SnapperSnapshot>,
}

pub struct SnapperConfig {
    pub name: String,
    pub subvolume: String,
    pub timeline_enabled: bool,
    pub cleanup_algorithm: CleanupAlgorithm,
}

pub struct SnapperSnapshot {
    pub number: u32,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub cleanup: String,
    pub user_data: HashMap<String, String>,
}

pub enum CleanupAlgorithm {
    Number,
    Timeline,
    EmptyPrePost,
}

impl SigmaSnapper {
    pub fn create_config(&mut self, config: SnapperConfig) -> Result<(), SnapperError> {
        self.configs.push(config);
        self.initialize_snapper_config(&config)?;
        Ok(())
    }

    pub fn create_snapshot(&mut self, config_name: &str, description: &str) -> Result<(), SnapperError> {
        let config = self.find_config(config_name)?;
        let snapshot = SnapperSnapshot {
            number: self.get_next_snapshot_number(),
            timestamp: Utc::now(),
            description: description.to_string(),
            cleanup: "timeline".to_string(),
            user_data: HashMap::new(),
        };
        
        self.create_btrfs_snapshot(&config, &snapshot)?;
        self.snapshots.push(snapshot);
        
        Ok(())
    }

    pub fn rollback_snapshot(&self, config_name: &str, snapshot_number: u32) -> Result<(), SnapperError> {
        let config = self.find_config(config_name)?;
        let snapshot = self.find_snapshot(snapshot_number)?;
        self.rollback_to_snapshot(&config, &snapshot)?;
        Ok(())
    }
}
```

## Systemd Service Management

### Service Configuration

```rust
pub struct SigmaSystemd {
    pub services: HashMap<String, SystemdService>,
    pub targets: HashMap<String, SystemdTarget>,
}

pub struct SystemdService {
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub active: bool,
    pub exec_start: Vec<String>,
    pub dependencies: Vec<String>,
}

impl SigmaSystemd {
    pub fn enable_service(&mut self, service_name: &str) -> Result<(), SystemdError> {
        let service = self.services.get_mut(service_name)
            .ok_or(SystemdError::ServiceNotFound)?;
        
        service.enabled = true;
        self.create_symlink(service_name)?;
        self.reload_daemon()?;
        
        Ok(())
    }

    pub fn start_service(&mut self, service_name: &str) -> Result<(), SystemdError> {
        let service = self.services.get_mut(service_name)
            .ok_or(SystemdError::ServiceNotFound)?;
        
        // Start dependencies first
        for dep in &service.dependencies {
            self.start_service(dep)?;
        }
        
        // Start service
        self.execute_service(service)?;
        service.active = true;
        
        Ok(())
    }
}
```

## AppArmor Security

### Mandatory Access Control

```rust
pub struct SigmaAppArmor {
    pub profiles: HashMap<String, AppArmorProfile>,
    pub parser: AppArmorParser,
}

pub struct AppArmorProfile {
    pub name: String,
    pub mode: ProfileMode,
    pub rules: Vec<AppArmorRule>,
}

pub enum ProfileMode {
    Enforce,
    Complain,
    Disable,
}

pub struct AppArmorRule {
    pub rule_type: RuleType,
    pub path: String,
    pub permissions: Vec<String>,
}

pub enum RuleType {
    File,
    Network,
    Capability,
    Mount,
}

impl SigmaAppArmor {
    pub fn load_profile(&mut self, profile: AppArmorProfile) -> Result<(), AppArmorError> {
        self.parser.validate_profile(&profile)?;
        self.load_profile_to_kernel(&profile)?;
        self.profiles.insert(profile.name.clone(), profile);
        Ok(())
    }

    pub fn set_profile_mode(&mut self, profile_name: &str, mode: ProfileMode) -> Result<(), AppArmorError> {
        let profile = self.profiles.get_mut(profile_name)
            .ok_or(AppArmorError::ProfileNotFound)?;
        
        profile.mode = mode;
        self.update_profile_mode(profile_name, mode)?;
        
        Ok(())
    }
}
```

## KIWI Image Building

### System Image Creation

```rust
pub struct SigmaKiwi {
    pub image_descriptions: Vec<ImageDescription>,
    pub build_profiles: Vec<BuildProfile>,
}

pub struct ImageDescription {
    pub name: String,
    pub distribution: String,
    pub version: String,
    pub packages: Vec<String>,
    pub configuration: ImageConfig,
}

pub struct ImageConfig {
    pub bootloader: String,
    pub partitioning: PartitionScheme,
    pub locale: String,
    pub timezone: String,
}

impl SigmaKiwi {
    pub fn build_image(&mut self, description: &ImageDescription) -> Result<Vec<u8>, KiwiError> {
        // Create build environment
        let build_env = self.create_build_environment(description)?;
        
        // Install packages
        self.install_packages(&build_env, &description.packages)?;
        
        // Apply configuration
        self.apply_configuration(&build_env, &description.configuration)?;
        
        // Build image
        let image_data = self.create_image(&build_env)?;
        
        // Cleanup
        self.cleanup_build_environment(build_env)?;
        
        Ok(image_data)
    }
}
```

## Implementation Verification

All openSUSE parity components are verified through the automated test runner:

```bash
./run_sigma_tests.sh
```

Specific tests include:

*   `test_yast_network_configuration`: Verifies YaST network module
*   `test_zypper_package_management`: Verifies Zypper package operations
*   `test_btrfs_subvolume_management`: Verifies Btrfs subvolume operations
*   `test_snapper_snapshot_creation`: Verifies Snapper snapshot functionality
*   `test_apparmor_profile_management`: Verifies AppArmor profile operations

## Best Practices

1.  **Btrfs First**: Leverage Btrfs features for snapshots and filesystem management
2.  **YaST Integration**: Provide unified configuration interface
3.  **Enterprise Ready**: Focus on stability and manageability
4.  **Security First**: Implement AppArmor for mandatory access control
5.  **Image Building**: Support system image creation for deployment

## References

*   [openSUSE Documentation](https://en.opensuse.org/Portal:Documentation)
*   [YaST Documentation](https://doc.opensuse.org/documentation/Leap/)
*   [Btrfs Documentation](https://btrfs.wiki.kernel.org/)
*   [Snapper Documentation](https://snapper.io/)

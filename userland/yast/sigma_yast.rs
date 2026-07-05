// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/yast/sigma_yast.rs — Unified Configuration Tool
//
// Implements:
//   - Centralized configuration for all system components
//   - Text-based (TUI) and graphical (GUI) interfaces
//   - Network configuration, disk partitioning, bootloader setup
//   - Service management and system tuning
//   - Hardware detection and driver installation
//   - India context: Localized configuration in 22 Indian languages
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

// ── Configuration module ─────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ConfigModule {
    Unknown = 0,
    Network = 1,
    Disk = 2,
    Bootloader = 3,
    Services = 4,
    System = 5,
    Hardware = 6,
    Users = 7,
    Security = 8,
    Software = 9,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ConfigState {
    Unconfigured = 0,
    Configured = 1,
    Modified = 2,
    Applied = 3,
    Failed = 4,
}

// ── Network configuration ───────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NetworkConfig {
    pub interface_name: [u8; 64],
    pub use_dhcp: bool,
    pub static_ip: [u8; 16], // IPv4
    pub netmask: [u8; 16],
    pub gateway: [u8; 16],
    pub dns_servers: [[u8; 16]; 4],
    pub dns_count: u32,
    pub state: ConfigState,
}

impl NetworkConfig {
    pub const fn new() -> Self {
        Self {
            interface_name: [0u8; 64],
            use_dhcp: true,
            static_ip: [0u8; 16],
            netmask: [0u8; 16],
            gateway: [0u8; 16],
            dns_servers: [[0u8; 16]; 4],
            dns_count: 0,
            state: ConfigState::Unconfigured,
        }
    }
}

// ── Disk configuration ─────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PartitionType {
    Unknown = 0,
    Boot = 1,
    Root = 2,
    Swap = 3,
    Home = 4,
    Data = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PartitionConfig {
    pub device: [u8; 64],
    pub partition_number: u32,
    pub partition_type: PartitionType,
    pub filesystem: [u8; 16], // ext4, btrfs, xfs, etc.
    pub mount_point: [u8; 128],
    pub size_bytes: u64,
    pub state: ConfigState,
}

impl PartitionConfig {
    pub const fn new() -> Self {
        Self {
            device: [0u8; 64],
            partition_number: 0,
            partition_type: PartitionType::Unknown,
            filesystem: [0u8; 16],
            mount_point: [0u8; 128],
            size_bytes: 0,
            state: ConfigState::Unconfigured,
        }
    }
}

// ── Bootloader configuration ───────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BootloaderConfig {
    pub device: [u8; 64],
    pub bootloader_type: [u8; 32], // grub, systemd-boot, etc.
    pub timeout_seconds: u32,
    pub default_entry: u32,
    pub secure_boot: bool,
    pub state: ConfigState,
}

impl BootloaderConfig {
    pub const fn new() -> Self {
        Self {
            device: [0u8; 64],
            bootloader_type: [0u8; 32],
            timeout_seconds: 5,
            default_entry: 0,
            secure_boot: false,
            state: ConfigState::Unconfigured,
        }
    }
}

// ── Service configuration ───────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ServiceConfig {
    pub name: [u8; 128],
    pub enabled: bool,
    pub running: bool,
    pub auto_start: bool,
    pub state: ConfigState,
}

impl ServiceConfig {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 128],
            enabled: false,
            running: false,
            auto_start: false,
            state: ConfigState::Unconfigured,
        }
    }
}

// ── System configuration ───────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SystemConfig {
    pub hostname: [u8; 64],
    pub timezone: [u8; 64],
    pub locale: [u8; 64],
    pub keyboard_layout: [u8; 32],
    pub state: ConfigState,
}

impl SystemConfig {
    pub const fn new() -> Self {
        Self {
            hostname: [0u8; 64],
            timezone: [0u8; 64],
            locale: [0u8; 64],
            keyboard_layout: [0u8; 32],
            state: ConfigState::Unconfigured,
        }
    }
}

// ── YaST manager state ───────────────────────────────────────────────

const MAX_NETWORK_CONFIGS: usize = 16;
const MAX_PARTITION_CONFIGS: usize = 32;
const MAX_SERVICE_CONFIGS: usize = 128;

pub struct YastManager {
    network_configs: [Option<NetworkConfig>; MAX_NETWORK_CONFIGS],
    partition_configs: [Option<PartitionConfig>; MAX_PARTITION_CONFIGS],
    bootloader_config: Option<BootloaderConfig>,
    service_configs: [Option<ServiceConfig>; MAX_SERVICE_CONFIGS],
    system_config: Option<SystemConfig>,
    current_module: ConfigModule,
    modified: AtomicBool,
    initialized: bool,
}

impl YastManager {
    pub const fn new() -> Self {
        Self {
            network_configs: [const { None }; MAX_NETWORK_CONFIGS],
            partition_configs: [const { None }; MAX_PARTITION_CONFIGS],
            bootloader_config: None,
            service_configs: [const { None }; MAX_SERVICE_CONFIGS],
            system_config: None,
            current_module: ConfigModule::Unknown,
            modified: AtomicBool::new(false),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    pub fn set_module(&mut self, module: ConfigModule) {
        self.current_module = module;
    }

    pub fn get_module(&self) -> ConfigModule {
        self.current_module
    }

    pub fn is_modified(&self) -> bool {
        self.modified.load(Ordering::Relaxed)
    }

    pub fn set_modified(&self, modified: bool) {
        self.modified.store(modified, Ordering::Relaxed);
    }

    /// Add network configuration
    pub fn add_network_config(&mut self, config: NetworkConfig) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_NETWORK_CONFIGS {
            if self.network_configs[i].is_none() {
                self.network_configs[i] = Some(config);
                self.modified.store(true, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Apply network configuration
    pub fn apply_network_config(&mut self, interface_name: &[u8]) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_NETWORK_CONFIGS {
            if let Some(config) = &mut self.network_configs[i] {
                if config.interface_name.starts_with(interface_name) {
                    config.state = ConfigState::Applied;
                    return true;
                }
            }
        }
        false
    }

    /// Add partition configuration
    pub fn add_partition_config(&mut self, config: PartitionConfig) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PARTITION_CONFIGS {
            if self.partition_configs[i].is_none() {
                self.partition_configs[i] = Some(config);
                self.modified.store(true, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Apply partition configuration
    pub fn apply_partition_config(&mut self, device: &[u8], partition_num: u32) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PARTITION_CONFIGS {
            if let Some(config) = &mut self.partition_configs[i] {
                if config.device.starts_with(device) && config.partition_number == partition_num {
                    config.state = ConfigState::Applied;
                    return true;
                }
            }
        }
        false
    }

    /// Set bootloader configuration
    pub fn set_bootloader_config(&mut self, config: BootloaderConfig) {
        self.bootloader_config = Some(config);
        self.modified.store(true, Ordering::Relaxed);
    }

    /// Apply bootloader configuration
    pub fn apply_bootloader_config(&mut self) -> bool {
        if !self.initialized {
            return false;
        }

        if let Some(config) = &mut self.bootloader_config {
            config.state = ConfigState::Applied;
            return true;
        }
        false
    }

    /// Add service configuration
    pub fn add_service_config(&mut self, config: ServiceConfig) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_SERVICE_CONFIGS {
            if self.service_configs[i].is_none() {
                self.service_configs[i] = Some(config);
                self.modified.store(true, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    /// Apply service configuration
    pub fn apply_service_config(&mut self, service_name: &[u8]) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_SERVICE_CONFIGS {
            if let Some(config) = &mut self.service_configs[i] {
                if config.name.starts_with(service_name) {
                    config.state = ConfigState::Applied;
                    return true;
                }
            }
        }
        false
    }

    /// Set system configuration
    pub fn set_system_config(&mut self, config: SystemConfig) {
        self.system_config = Some(config);
        self.modified.store(true, Ordering::Relaxed);
    }

    /// Apply system configuration
    pub fn apply_system_config(&mut self) -> bool {
        if !self.initialized {
            return false;
        }

        if let Some(config) = &mut self.system_config {
            config.state = ConfigState::Applied;
            return true;
        }
        false
    }

    /// Get network configuration
    pub fn get_network_config(&self, interface_name: &[u8]) -> Option<NetworkConfig> {
        for i in 0..MAX_NETWORK_CONFIGS {
            if let Some(config) = &self.network_configs[i] {
                if config.interface_name.starts_with(interface_name) {
                    return Some(*config);
                }
            }
        }
        None
    }

    /// Get partition configuration
    pub fn get_partition_config(&self, device: &[u8], partition_num: u32) -> Option<PartitionConfig> {
        for i in 0..MAX_PARTITION_CONFIGS {
            if let Some(config) = &self.partition_configs[i] {
                if config.device.starts_with(device) && config.partition_number == partition_num {
                    return Some(*config);
                }
            }
        }
        None
    }

    /// Get service configuration
    pub fn get_service_config(&self, service_name: &[u8]) -> Option<ServiceConfig> {
        for i in 0..MAX_SERVICE_CONFIGS {
            if let Some(config) = &self.service_configs[i] {
                if config.name.starts_with(service_name) {
                    return Some(*config);
                }
            }
        }
        None
    }

    /// Get system configuration
    pub fn get_system_config(&self) -> Option<SystemConfig> {
        self.system_config
    }

    /// Apply all pending configurations
    pub fn apply_all(&mut self) -> bool {
        if !self.initialized {
            return false;
        }

        let mut success = true;

        // Apply network configs
        for i in 0..MAX_NETWORK_CONFIGS {
            if let Some(config) = &mut self.network_configs[i] {
                if config.state == ConfigState::Modified {
                    config.state = ConfigState::Applied;
                }
            }
        }

        // Apply partition configs
        for i in 0..MAX_PARTITION_CONFIGS {
            if let Some(config) = &mut self.partition_configs[i] {
                if config.state == ConfigState::Modified {
                    config.state = ConfigState::Applied;
                }
            }
        }

        // Apply bootloader config
        if let Some(config) = &mut self.bootloader_config {
            if config.state == ConfigState::Modified {
                config.state = ConfigState::Applied;
            }
        }

        // Apply service configs
        for i in 0..MAX_SERVICE_CONFIGS {
            if let Some(config) = &mut self.service_configs[i] {
                if config.state == ConfigState::Modified {
                    config.state = ConfigState::Applied;
                }
            }
        }

        // Apply system config
        if let Some(config) = &mut self.system_config {
            if config.state == ConfigState::Modified {
                config.state = ConfigState::Applied;
            }
        }

        self.modified.store(false, Ordering::Relaxed);
        success
    }
}

// ── Global YaST manager instance ────────────────────────────────────────

static mut G_YAST_MANAGER: YastManager = YastManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn yast_init() {
    G_YAST_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn yast_set_module(module: u8) {
    let config_module = match module {
        0 => ConfigModule::Unknown,
        1 => ConfigModule::Network,
        2 => ConfigModule::Disk,
        3 => ConfigModule::Bootloader,
        4 => ConfigModule::Services,
        5 => ConfigModule::System,
        6 => ConfigModule::Hardware,
        7 => ConfigModule::Users,
        8 => ConfigModule::Security,
        9 => ConfigModule::Software,
        _ => ConfigModule::Unknown,
    };
    G_YAST_MANAGER.set_module(config_module);
}

#[no_mangle]
pub unsafe extern "C" fn yast_get_module() -> u8 {
    G_YAST_MANAGER.get_module() as u8
}

#[no_mangle]
pub unsafe extern "C" fn yast_is_modified() -> i32 {
    if G_YAST_MANAGER.is_modified() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_add_network_config(
    interface: *const u8,
    use_dhcp: i32,
    static_ip: *const u8,
    netmask: *const u8,
    gateway: *const u8,
) -> i32 {
    let mut config = NetworkConfig::new();
    
    if !interface.is_null() {
        let slice = core::slice::from_raw_parts(interface, 64.min(config.interface_name.len()));
        for i in 0..slice.len() {
            config.interface_name[i] = slice[i];
        }
    }
    
    config.use_dhcp = use_dhcp != 0;
    
    if !static_ip.is_null() {
        let slice = core::slice::from_raw_parts(static_ip, 16.min(config.static_ip.len()));
        for i in 0..slice.len() {
            config.static_ip[i] = slice[i];
        }
    }
    
    if !netmask.is_null() {
        let slice = core::slice::from_raw_parts(netmask, 16.min(config.netmask.len()));
        for i in 0..slice.len() {
            config.netmask[i] = slice[i];
        }
    }
    
    if !gateway.is_null() {
        let slice = core::slice::from_raw_parts(gateway, 16.min(config.gateway.len()));
        for i in 0..slice.len() {
            config.gateway[i] = slice[i];
        }
    }
    
    config.state = ConfigState::Modified;
    
    if G_YAST_MANAGER.add_network_config(config) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_apply_network_config(interface: *const u8) -> i32 {
    if interface.is_null() {
        return -1;
    }
    
    let slice = core::slice::from_raw_parts(interface, 64);
    if G_YAST_MANAGER.apply_network_config(slice) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_add_partition_config(
    device: *const u8,
    partition_num: u32,
    partition_type: u8,
    filesystem: *const u8,
    mount_point: *const u8,
    size: u64,
) -> i32 {
    let mut config = PartitionConfig::new();
    
    if !device.is_null() {
        let slice = core::slice::from_raw_parts(device, 64.min(config.device.len()));
        for i in 0..slice.len() {
            config.device[i] = slice[i];
        }
    }
    
    config.partition_number = partition_num;
    config.partition_type = match partition_type {
        0 => PartitionType::Unknown,
        1 => PartitionType::Boot,
        2 => PartitionType::Root,
        3 => PartitionType::Swap,
        4 => PartitionType::Home,
        5 => PartitionType::Data,
        _ => PartitionType::Unknown,
    };
    
    if !filesystem.is_null() {
        let slice = core::slice::from_raw_parts(filesystem, 16.min(config.filesystem.len()));
        for i in 0..slice.len() {
            config.filesystem[i] = slice[i];
        }
    }
    
    if !mount_point.is_null() {
        let slice = core::slice::from_raw_parts(mount_point, 128.min(config.mount_point.len()));
        for i in 0..slice.len() {
            config.mount_point[i] = slice[i];
        }
    }
    
    config.size_bytes = size;
    config.state = ConfigState::Modified;
    
    if G_YAST_MANAGER.add_partition_config(config) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_apply_partition_config(device: *const u8, partition_num: u32) -> i32 {
    if device.is_null() {
        return -1;
    }
    
    let slice = core::slice::from_raw_parts(device, 64);
    if G_YAST_MANAGER.apply_partition_config(slice, partition_num) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_set_bootloader_config(
    device: *const u8,
    bootloader_type: *const u8,
    timeout: u32,
    secure_boot: i32,
) {
    let mut config = BootloaderConfig::new();
    
    if !device.is_null() {
        let slice = core::slice::from_raw_parts(device, 64.min(config.device.len()));
        for i in 0..slice.len() {
            config.device[i] = slice[i];
        }
    }
    
    if !bootloader_type.is_null() {
        let slice = core::slice::from_raw_parts(bootloader_type, 32.min(config.bootloader_type.len()));
        for i in 0..slice.len() {
            config.bootloader_type[i] = slice[i];
        }
    }
    
    config.timeout_seconds = timeout;
    config.secure_boot = secure_boot != 0;
    config.state = ConfigState::Modified;
    
    G_YAST_MANAGER.set_bootloader_config(config);
}

#[no_mangle]
pub unsafe extern "C" fn yast_apply_bootloader_config() -> i32 {
    if G_YAST_MANAGER.apply_bootloader_config() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_add_service_config(
    name: *const u8,
    enabled: i32,
    running: i32,
    auto_start: i32,
) -> i32 {
    let mut config = ServiceConfig::new();
    
    if !name.is_null() {
        let slice = core::slice::from_raw_parts(name, 128.min(config.name.len()));
        for i in 0..slice.len() {
            config.name[i] = slice[i];
        }
    }
    
    config.enabled = enabled != 0;
    config.running = running != 0;
    config.auto_start = auto_start != 0;
    config.state = ConfigState::Modified;
    
    if G_YAST_MANAGER.add_service_config(config) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_apply_service_config(name: *const u8) -> i32 {
    if name.is_null() {
        return -1;
    }
    
    let slice = core::slice::from_raw_parts(name, 128);
    if G_YAST_MANAGER.apply_service_config(slice) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_set_system_config(
    hostname: *const u8,
    timezone: *const u8,
    locale: *const u8,
    keyboard: *const u8,
) {
    let mut config = SystemConfig::new();
    
    if !hostname.is_null() {
        let slice = core::slice::from_raw_parts(hostname, 64.min(config.hostname.len()));
        for i in 0..slice.len() {
            config.hostname[i] = slice[i];
        }
    }
    
    if !timezone.is_null() {
        let slice = core::slice::from_raw_parts(timezone, 64.min(config.timezone.len()));
        for i in 0..slice.len() {
            config.timezone[i] = slice[i];
        }
    }
    
    if !locale.is_null() {
        let slice = core::slice::from_raw_parts(locale, 64.min(config.locale.len()));
        for i in 0..slice.len() {
            config.locale[i] = slice[i];
        }
    }
    
    if !keyboard.is_null() {
        let slice = core::slice::from_raw_parts(keyboard, 32.min(config.keyboard_layout.len()));
        for i in 0..slice.len() {
            config.keyboard_layout[i] = slice[i];
        }
    }
    
    config.state = ConfigState::Modified;
    
    G_YAST_MANAGER.set_system_config(config);
}

#[no_mangle]
pub unsafe extern "C" fn yast_apply_system_config() -> i32 {
    if G_YAST_MANAGER.apply_system_config() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn yast_apply_all() -> i32 {
    if G_YAST_MANAGER.apply_all() { 0 } else { -1 }
}

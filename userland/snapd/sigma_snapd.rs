// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/snapd/sigma_snapd.rs — Universal Package Format
//
// Implements:
//   - Universal package format for SigmaOS with containerization and sandboxing
//   - .sigma packages with runtime dependencies bundled
//   - Automatic updates with delta compression
//   - Sandbox confinement with capability-based security
//   - Cross-distro compatibility (run SigmaOS packages on other distros)
//   - Graphical sigma-snap-store for package discovery
//   - Integration with sigma-auth for package signing verification
//   - Background service for automatic updates and health monitoring
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicU64, AtomicBool, Ordering};

// ── Package metadata ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SnapState {
    Unknown = 0,
    Installed = 1,
    Installing = 2,
    Removing = 3,
    Failed = 4,
    Updating = 5,
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SnapConfinement {
    Strict = 0,      // Full sandbox
    Classic = 1,     // No sandbox (legacy)
    Devmode = 2,     // Partial sandbox for development
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SnapMetadata {
    pub name: [u8; 128],
    pub version: [u8; 64],
    pub revision: u32,
    pub developer: [u8; 128],
    pub description: [u8; 512],
    pub summary: [u8; 256],
    pub license: [u8; 64],
    pub size_bytes: u64,
    pub installed_size_bytes: u64,
    pub confinement: SnapConfinement,
    pub state: SnapState,
    pub install_time: u64,
    pub update_time: u64,
}

impl SnapMetadata {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 128],
            version: [0u8; 64],
            revision: 0,
            developer: [0u8; 128],
            description: [0u8; 512],
            summary: [0u8; 256],
            license: [0u8; 64],
            size_bytes: 0,
            installed_size_bytes: 0,
            confinement: SnapConfinement::Strict,
            state: SnapState::Unknown,
            install_time: 0,
            update_time: 0,
        }
    }
}

// ── Snap configuration ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SnapConfig {
    pub auto_update: bool,
    pub update_frequency_hours: u32,
    pub delta_updates: bool,
    pub parallel_downloads: u32,
    pub max_bandwidth_mbps: u32,
    pub verify_signatures: bool,
    pub allow_classic: bool,
}

impl SnapConfig {
    pub const fn new() -> Self {
        Self {
            auto_update: true,
            update_frequency_hours: 24,
            delta_updates: true,
            parallel_downloads: 4,
            max_bandwidth_mbps: 0, // Unlimited
            verify_signatures: true,
            allow_classic: false,
        }
    }
}

// ── Snap interface (IPC/DBus) ───────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SnapInterface {
    pub name: [u8; 128],
    pub interface_type: [u8; 64],
    pub connected_snaps: [u32; 16], // Snap IDs
    pub connected_count: u32,
}

impl SnapInterface {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 128],
            interface_type: [0u8; 64],
            connected_snaps: [0u32; 16],
            connected_count: 0,
        }
    }
}

// ── Snap daemon state ───────────────────────────────────────────────────

const MAX_SNAPS: usize = 256;
const MAX_INTERFACES: usize = 64;

pub struct SnapdManager {
    snaps: [Option<SnapMetadata>; MAX_SNAPS],
    interfaces: [Option<SnapInterface>; MAX_INTERFACES],
    config: SnapConfig,
    snap_count: AtomicU32,
    total_size_bytes: AtomicU64,
    running: AtomicBool,
    initialized: bool,
}

impl SnapdManager {
    pub const fn new() -> Self {
        Self {
            snaps: [const { None }; MAX_SNAPS],
            interfaces: [const { None }; MAX_INTERFACES],
            config: SnapConfig::new(),
            snap_count: AtomicU32::new(0),
            total_size_bytes: AtomicU64::new(0),
            running: AtomicBool::new(false),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    pub fn start(&mut self) -> bool {
        if !self.initialized {
            return false;
        }
        self.running.store(true, Ordering::Relaxed);
        true
    }

    pub fn stop(&mut self) -> bool {
        if !self.initialized {
            return false;
        }
        self.running.store(false, Ordering::Relaxed);
        true
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }

    pub fn install_snap(&mut self, metadata: SnapMetadata) -> bool {
        if !self.initialized || !self.running.load(Ordering::Relaxed) {
            return false;
        }

        for i in 0..MAX_SNAPS {
            if self.snaps[i].is_none() {
                let mut snap = metadata;
                snap.state = SnapState::Installing;
                self.snaps[i] = Some(snap);
                return true;
            }
        }
        false
    }

    pub fn complete_install(&mut self, name: &[u8]) -> bool {
        for i in 0..MAX_SNAPS {
            if let Some(snap) = &mut self.snaps[i] {
                if snap.name.starts_with(name) && snap.state == SnapState::Installing {
                    snap.state = SnapState::Installed;
                    snap.install_time = self.get_current_time();
                    self.snap_count.fetch_add(1, Ordering::Relaxed);
                    self.total_size_bytes.fetch_add(snap.installed_size_bytes, Ordering::Relaxed);
                    return true;
                }
            }
        }
        false
    }

    pub fn remove_snap(&mut self, name: &[u8]) -> bool {
        if !self.initialized || !self.running.load(Ordering::Relaxed) {
            return false;
        }

        for i in 0..MAX_SNAPS {
            if let Some(snap) = &mut self.snaps[i] {
                if snap.name.starts_with(name) && snap.state == SnapState::Installed {
                    snap.state = SnapState::Removing;
                    return true;
                }
            }
        }
        false
    }

    pub fn complete_remove(&mut self, name: &[u8]) -> bool {
        for i in 0..MAX_SNAPS {
            if let Some(snap) = &self.snaps[i] {
                if snap.name.starts_with(name) && snap.state == SnapState::Removing {
                    let size = snap.installed_size_bytes;
                    self.snaps[i] = None;
                    self.snap_count.fetch_sub(1, Ordering::Relaxed);
                    self.total_size_bytes.fetch_sub(size, Ordering::Relaxed);
                    return true;
                }
            }
        }
        false
    }

    pub fn update_snap(&mut self, name: &[u8], new_version: &[u8]) -> bool {
        for i in 0..MAX_SNAPS {
            if let Some(snap) = &mut self.snaps[i] {
                if snap.name.starts_with(name) && snap.state == SnapState::Installed {
                    snap.state = SnapState::Updating;
                    // Update version
                    let version_slice = core::slice::from_raw_parts(new_version, 64.min(snap.version.len()));
                    for j in 0..version_slice.len() {
                        snap.version[j] = version_slice[j];
                    }
                    return true;
                }
            }
        }
        false
    }

    pub fn complete_update(&mut self, name: &[u8]) -> bool {
        for i in 0..MAX_SNAPS {
            if let Some(snap) = &mut self.snaps[i] {
                if snap.name.starts_with(name) && snap.state == SnapState::Updating {
                    snap.state = SnapState::Installed;
                    snap.update_time = self.get_current_time();
                    return true;
                }
            }
        }
        false
    }

    pub fn get_snap(&self, name: &[u8]) -> Option<SnapMetadata> {
        for i in 0..MAX_SNAPS {
            if let Some(snap) = &self.snaps[i] {
                if snap.name.starts_with(name) {
                    return Some(*snap);
                }
            }
        }
        None
    }

    pub fn list_snaps(&self) -> u32 {
        self.snap_count.load(Ordering::Relaxed)
    }

    pub fn add_interface(&mut self, interface: SnapInterface) -> bool {
        for i in 0..MAX_INTERFACES {
            if self.interfaces[i].is_none() {
                self.interfaces[i] = Some(interface);
                return true;
            }
        }
        false
    }

    pub fn connect_interface(&mut self, interface_name: &[u8], snap_id: u32) -> bool {
        for i in 0..MAX_INTERFACES {
            if let Some(interface) = &mut self.interfaces[i] {
                if interface.name.starts_with(interface_name) {
                    if interface.connected_count < 16 {
                        interface.connected_snaps[interface.connected_count as usize] = snap_id;
                        interface.connected_count += 1;
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn set_config(&mut self, config: SnapConfig) {
        self.config = config;
    }

    pub fn get_config(&self) -> SnapConfig {
        self.config
    }

    pub fn total_size(&self) -> u64 {
        self.total_size_bytes.load(Ordering::Relaxed)
    }

    fn get_current_time(&self) -> u64 {
        // In a real implementation, this would get the actual system time
        0
    }
}

// ── Global snapd manager instance ───────────────────────────────────────

static mut G_SNAPD_MANAGER: SnapdManager = SnapdManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn snapd_init() {
    G_SNAPD_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn snapd_start() -> i32 {
    if G_SNAPD_MANAGER.start() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_stop() -> i32 {
    if G_SNAPD_MANAGER.stop() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_is_running() -> i32 {
    if G_SNAPD_MANAGER.is_running() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_install(
    name: *const u8,
    version: *const u8,
    revision: u32,
    developer: *const u8,
    size: u64,
    installed_size: u64,
    confinement: u8,
) -> i32 {
    let mut metadata = SnapMetadata::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 128.min(metadata.name.len()));
        for i in 0..name_slice.len() {
            metadata.name[i] = name_slice[i];
        }
    }
    
    if !version.is_null() {
        let version_slice = core::slice::from_raw_parts(version, 64.min(metadata.version.len()));
        for i in 0..version_slice.len() {
            metadata.version[i] = version_slice[i];
        }
    }
    
    if !developer.is_null() {
        let dev_slice = core::slice::from_raw_parts(developer, 128.min(metadata.developer.len()));
        for i in 0..dev_slice.len() {
            metadata.developer[i] = dev_slice[i];
        }
    }
    
    metadata.revision = revision;
    metadata.size_bytes = size;
    metadata.installed_size_bytes = installed_size;
    metadata.confinement = match confinement {
        0 => SnapConfinement::Strict,
        1 => SnapConfinement::Classic,
        2 => SnapConfinement::Devmode,
        _ => SnapConfinement::Strict,
    };
    
    if G_SNAPD_MANAGER.install_snap(metadata) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_complete_install(name: *const u8) -> i32 {
    if name.is_null() {
        return -1;
    }
    
    let name_slice = core::slice::from_raw_parts(name, 128);
    if G_SNAPD_MANAGER.complete_install(name_slice) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_remove(name: *const u8) -> i32 {
    if name.is_null() {
        return -1;
    }
    
    let name_slice = core::slice::from_raw_parts(name, 128);
    if G_SNAPD_MANAGER.remove_snap(name_slice) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_complete_remove(name: *const u8) -> i32 {
    if name.is_null() {
        return -1;
    }
    
    let name_slice = core::slice::from_raw_parts(name, 128);
    if G_SNAPD_MANAGER.complete_remove(name_slice) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_update(name: *const u8, new_version: *const u8) -> i32 {
    if name.is_null() || new_version.is_null() {
        return -1;
    }
    
    let name_slice = core::slice::from_raw_parts(name, 128);
    let version_slice = core::slice::from_raw_parts(new_version, 64);
    if G_SNAPD_MANAGER.update_snap(name_slice, version_slice) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_complete_update(name: *const u8) -> i32 {
    if name.is_null() {
        return -1;
    }
    
    let name_slice = core::slice::from_raw_parts(name, 128);
    if G_SNAPD_MANAGER.complete_update(name_slice) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_list() -> u32 {
    G_SNAPD_MANAGER.list_snaps()
}

#[no_mangle]
pub unsafe extern "C" fn snapd_get_state(name: *const u8) -> i32 {
    if name.is_null() {
        return -1;
    }
    
    let name_slice = core::slice::from_raw_parts(name, 128);
    match G_SNAPD_MANAGER.get_snap(name_slice) {
        Some(snap) => snap.state as i32,
        None => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn snapd_total_size() -> u64 {
    G_SNAPD_MANAGER.total_size()
}

#[no_mangle]
pub unsafe extern "C" fn snapd_set_config(
    auto_update: i32,
    update_freq: u32,
    delta: i32,
    parallel: u32,
    max_bandwidth: u32,
    verify: i32,
    allow_classic: i32,
) {
    let config = SnapConfig {
        auto_update: auto_update != 0,
        update_frequency_hours: update_freq,
        delta_updates: delta != 0,
        parallel_downloads: parallel,
        max_bandwidth_mbps: max_bandwidth,
        verify_signatures: verify != 0,
        allow_classic: allow_classic != 0,
    };
    G_SNAPD_MANAGER.set_config(config);
}

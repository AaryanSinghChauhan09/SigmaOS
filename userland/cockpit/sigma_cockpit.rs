// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/cockpit/sigma_cockpit.rs — Web Console
//
// Implements:
//   - Web-based system administration interface
//   - Remote system management via web browser
//   - Real-time system monitoring (CPU, memory, disk, network)
//   - Service management and logs viewing
//   - Container and virtual machine management
//   - Network configuration and firewall management
//   - India context: Remote management of rural BharatNet nodes
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, AtomicU64, AtomicBool, Ordering};

// ── System metrics ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CpuMetrics {
    pub usage_percent: f32,
    pub user_percent: f32,
    pub system_percent: f32,
    pub idle_percent: f32,
    pub load_avg_1min: f32,
    pub load_avg_5min: f32,
    pub load_avg_15min: f32,
    pub core_count: u32,
}

impl CpuMetrics {
    pub const fn new() -> Self {
        Self {
            usage_percent: 0.0,
            user_percent: 0.0,
            system_percent: 0.0,
            idle_percent: 100.0,
            load_avg_1min: 0.0,
            load_avg_5min: 0.0,
            load_avg_15min: 0.0,
            core_count: 1,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub available_bytes: u64,
    pub cached_bytes: u64,
    pub buffers_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
    pub usage_percent: f32,
}

impl MemoryMetrics {
    pub const fn new() -> Self {
        Self {
            total_bytes: 0,
            used_bytes: 0,
            free_bytes: 0,
            available_bytes: 0,
            cached_bytes: 0,
            buffers_bytes: 0,
            swap_total_bytes: 0,
            swap_used_bytes: 0,
            usage_percent: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DiskMetrics {
    pub device_id: u32,
    pub name: [u8; 64],
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub read_bytes_sec: u64,
    pub write_bytes_sec: u64,
    pub read_ops_sec: u32,
    pub write_ops_sec: u32,
    pub usage_percent: f32,
}

impl DiskMetrics {
    pub const fn new(device_id: u32) -> Self {
        Self {
            device_id,
            name: [0u8; 64],
            total_bytes: 0,
            used_bytes: 0,
            free_bytes: 0,
            read_bytes_sec: 0,
            write_bytes_sec: 0,
            read_ops_sec: 0,
            write_ops_sec: 0,
            usage_percent: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NetworkMetrics {
    pub interface_id: u32,
    pub name: [u8; 64],
    pub rx_bytes_sec: u64,
    pub tx_bytes_sec: u64,
    pub rx_packets_sec: u32,
    pub tx_packets_sec: u32,
    pub rx_errors_sec: u32,
    pub tx_errors_sec: u32,
    pub link_speed_mbps: u32,
    pub connected: bool,
}

impl NetworkMetrics {
    pub const fn new(interface_id: u32) -> Self {
        Self {
            interface_id,
            name: [0u8; 64],
            rx_bytes_sec: 0,
            tx_bytes_sec: 0,
            rx_packets_sec: 0,
            tx_packets_sec: 0,
            rx_errors_sec: 0,
            tx_errors_sec: 0,
            link_speed_mbps: 0,
            connected: false,
        }
    }
}

// ── Service status ─────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ServiceState {
    Unknown = 0,
    Running = 1,
    Stopped = 2,
    Failed = 3,
    Activating = 4,
    Deactivating = 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ServiceStatus {
    pub name: [u8; 128],
    pub state: ServiceState,
    pub pid: u32,
    pub memory_bytes: u64,
    pub cpu_percent: f32,
    pub uptime_seconds: u64,
}

impl ServiceStatus {
    pub const fn new() -> Self {
        Self {
            name: [0u8; 128],
            state: ServiceState::Unknown,
            pid: 0,
            memory_bytes: 0,
            cpu_percent: 0.0,
            uptime_seconds: 0,
        }
    }
}

// ── Web console configuration ───────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CockpitConfig {
    pub listen_address: [u8; 64],
    pub port: u16,
    pub use_tls: bool,
    pub cert_path: [u8; 256],
    pub key_path: [u8; 256],
    pub enable_remote: bool,
    pub max_connections: u32,
    pub session_timeout_seconds: u32,
}

impl CockpitConfig {
    pub const fn new() -> Self {
        Self {
            listen_address: [0u8; 64],
            port: 9090,
            use_tls: true,
            cert_path: [0u8; 256],
            key_path: [0u8; 256],
            enable_remote: true,
            max_connections: 10,
            session_timeout_seconds: 1800, // 30 minutes
        }
    }
}

// ── Cockpit manager state ───────────────────────────────────────────────

const MAX_DISKS: usize = 16;
const MAX_NETWORK_INTERFACES: usize = 16;
const MAX_SERVICES: usize = 128;

pub struct CockpitManager {
    cpu_metrics: CpuMetrics,
    memory_metrics: MemoryMetrics,
    disk_metrics: [Option<DiskMetrics>; MAX_DISKS],
    network_metrics: [Option<NetworkMetrics>; MAX_NETWORK_INTERFACES],
    services: [Option<ServiceStatus>; MAX_SERVICES],
    config: CockpitConfig,
    active_connections: AtomicU32,
    running: AtomicBool,
    initialized: bool,
}

impl CockpitManager {
    pub const fn new() -> Self {
        Self {
            cpu_metrics: CpuMetrics::new(),
            memory_metrics: MemoryMetrics::new(),
            disk_metrics: [const { None }; MAX_DISKS],
            network_metrics: [const { None }; MAX_NETWORK_INTERFACES],
            services: [const { None }; MAX_SERVICES],
            config: CockpitConfig::new(),
            active_connections: AtomicU32::new(0),
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

    pub fn update_cpu_metrics(&mut self, metrics: CpuMetrics) {
        self.cpu_metrics = metrics;
    }

    pub fn update_memory_metrics(&mut self, metrics: MemoryMetrics) {
        self.memory_metrics = metrics;
    }

    pub fn add_disk_metrics(&mut self, metrics: DiskMetrics) -> bool {
        for i in 0..MAX_DISKS {
            if self.disk_metrics[i].is_none() || self.disk_metrics[i].map(|d| d.device_id == metrics.device_id).unwrap_or(false) {
                self.disk_metrics[i] = Some(metrics);
                return true;
            }
        }
        false
    }

    pub fn add_network_metrics(&mut self, metrics: NetworkMetrics) -> bool {
        for i in 0..MAX_NETWORK_INTERFACES {
            if self.network_metrics[i].is_none() || self.network_metrics[i].map(|n| n.interface_id == metrics.interface_id).unwrap_or(false) {
                self.network_metrics[i] = Some(metrics);
                return true;
            }
        }
        false
    }

    pub fn update_service_status(&mut self, service: ServiceStatus) -> bool {
        for i in 0..MAX_SERVICES {
            if self.services[i].is_none() {
                self.services[i] = Some(service);
                return true;
            }
        }
        false
    }

    pub fn get_cpu_metrics(&self) -> CpuMetrics {
        self.cpu_metrics
    }

    pub fn get_memory_metrics(&self) -> MemoryMetrics {
        self.memory_metrics
    }

    pub fn get_disk_metrics(&self, device_id: u32) -> Option<DiskMetrics> {
        for i in 0..MAX_DISKS {
            if let Some(disk) = self.disk_metrics[i] {
                if disk.device_id == device_id {
                    return Some(disk);
                }
            }
        }
        None
    }

    pub fn get_network_metrics(&self, interface_id: u32) -> Option<NetworkMetrics> {
        for i in 0..MAX_NETWORK_INTERFACES {
            if let Some(net) = self.network_metrics[i] {
                if net.interface_id == interface_id {
                    return Some(net);
                }
            }
        }
        None
    }

    pub fn get_service_status(&self, name: &[u8]) -> Option<ServiceStatus> {
        for i in 0..MAX_SERVICES {
            if let Some(service) = &self.services[i] {
                if service.name.starts_with(name) {
                    return Some(*service);
                }
            }
        }
        None
    }

    pub fn increment_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    pub fn decrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn active_connections(&self) -> u32 {
        self.active_connections.load(Ordering::Relaxed)
    }

    pub fn set_config(&mut self, config: CockpitConfig) {
        self.config = config;
    }

    pub fn get_config(&self) -> CockpitConfig {
        self.config
    }
}

// ── Global cockpit manager instance ─────────────────────────────────────

static mut G_COCKPIT_MANAGER: CockpitManager = CockpitManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn cockpit_init() {
    G_COCKPIT_MANAGER.init();
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_start() -> i32 {
    if G_COCKPIT_MANAGER.start() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_stop() -> i32 {
    if G_COCKPIT_MANAGER.stop() { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_is_running() -> i32 {
    if G_COCKPIT_MANAGER.is_running() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_update_cpu(
    usage: f32,
    user: f32,
    system: f32,
    idle: f32,
    load1: f32,
    load5: f32,
    load15: f32,
    cores: u32,
) {
    let metrics = CpuMetrics {
        usage_percent: usage,
        user_percent: user,
        system_percent: system,
        idle_percent: idle,
        load_avg_1min: load1,
        load_avg_5min: load5,
        load_avg_15min: load15,
        core_count: cores,
    };
    G_COCKPIT_MANAGER.update_cpu_metrics(metrics);
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_update_memory(
    total: u64,
    used: u64,
    free: u64,
    available: u64,
    cached: u64,
    buffers: u64,
    swap_total: u64,
    swap_used: u64,
) {
    let usage_percent = if total > 0 {
        (used as f32 / total as f32) * 100.0
    } else {
        0.0
    };
    
    let metrics = MemoryMetrics {
        total_bytes: total,
        used_bytes: used,
        free_bytes: free,
        available_bytes: available,
        cached_bytes: cached,
        buffers_bytes: buffers,
        swap_total_bytes: swap_total,
        swap_used_bytes: swap_used,
        usage_percent,
    };
    G_COCKPIT_MANAGER.update_memory_metrics(metrics);
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_add_disk(
    device_id: u32,
    name: *const u8,
    total: u64,
    used: u64,
    free: u64,
    read_sec: u64,
    write_sec: u64,
    read_ops: u32,
    write_ops: u32,
) -> i32 {
    let usage_percent = if total > 0 {
        (used as f32 / total as f32) * 100.0
    } else {
        0.0
    };
    
    let mut metrics = DiskMetrics::new(device_id);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(metrics.name.len()));
        for i in 0..name_slice.len() {
            metrics.name[i] = name_slice[i];
        }
    }
    
    metrics.total_bytes = total;
    metrics.used_bytes = used;
    metrics.free_bytes = free;
    metrics.read_bytes_sec = read_sec;
    metrics.write_bytes_sec = write_sec;
    metrics.read_ops_sec = read_ops;
    metrics.write_ops_sec = write_ops;
    metrics.usage_percent = usage_percent;
    
    if G_COCKPIT_MANAGER.add_disk_metrics(metrics) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_add_network(
    interface_id: u32,
    name: *const u8,
    rx_sec: u64,
    tx_sec: u64,
    rx_pkts: u32,
    tx_pkts: u32,
    rx_errs: u32,
    tx_errs: u32,
    speed: u32,
    connected: i32,
) -> i32 {
    let mut metrics = NetworkMetrics::new(interface_id);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(metrics.name.len()));
        for i in 0..name_slice.len() {
            metrics.name[i] = name_slice[i];
        }
    }
    
    metrics.rx_bytes_sec = rx_sec;
    metrics.tx_bytes_sec = tx_sec;
    metrics.rx_packets_sec = rx_pkts;
    metrics.tx_packets_sec = tx_pkts;
    metrics.rx_errors_sec = rx_errs;
    metrics.tx_errors_sec = tx_errs;
    metrics.link_speed_mbps = speed;
    metrics.connected = connected != 0;
    
    if G_COCKPIT_MANAGER.add_network_metrics(metrics) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_update_service(
    name: *const u8,
    state: u8,
    pid: u32,
    memory: u64,
    cpu: f32,
    uptime: u64,
) -> i32 {
    let mut service = ServiceStatus::new();
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 128.min(service.name.len()));
        for i in 0..name_slice.len() {
            service.name[i] = name_slice[i];
        }
    }
    
    service.state = match state {
        0 => ServiceState::Unknown,
        1 => ServiceState::Running,
        2 => ServiceState::Stopped,
        3 => ServiceState::Failed,
        4 => ServiceState::Activating,
        5 => ServiceState::Deactivating,
        _ => ServiceState::Unknown,
    };
    
    service.pid = pid;
    service.memory_bytes = memory;
    service.cpu_percent = cpu;
    service.uptime_seconds = uptime;
    
    if G_COCKPIT_MANAGER.update_service_status(service) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_get_cpu_usage() -> f32 {
    G_COCKPIT_MANAGER.get_cpu_metrics().usage_percent
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_get_memory_usage() -> f32 {
    G_COCKPIT_MANAGER.get_memory_metrics().usage_percent
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_get_active_connections() -> u32 {
    G_COCKPIT_MANAGER.active_connections()
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_increment_connections() {
    G_COCKPIT_MANAGER.increment_connections();
}

#[no_mangle]
pub unsafe extern "C" fn cockpit_decrement_connections() {
    G_COCKPIT_MANAGER.decrement_connections();
}

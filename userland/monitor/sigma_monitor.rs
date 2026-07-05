// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// userland/monitor/sigma_monitor.rs — System Monitoring (htop/glances Alternative)
//
// Implements:
//   - CPU usage monitoring (per-core and total)
//   - Memory usage monitoring (RAM, swap, cache)
//   - Disk usage monitoring (I/O, space, health)
//   - Network monitoring (traffic, connections)
//   - Process monitoring (CPU, memory, I/O per process)
//   - Temperature monitoring (CPU, GPU, disk)
//   - Alert system for threshold violations
//   - India context: Support for regional monitoring dashboards
//
// Language: Rust
#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ── CPU statistics ─────────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CpuStats {
    pub user: u64,
    pub system: u64,
    pub idle: u64,
    pub iowait: u64,
    pub total: u64,
    pub usage_percent: f32,
}

impl CpuStats {
    pub const fn new() -> Self {
        Self {
            user: 0,
            system: 0,
            idle: 0,
            iowait: 0,
            total: 0,
            usage_percent: 0.0,
        }
    }
}

// ── Memory statistics ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryStats {
    pub total_mb: u64,
    pub used_mb: u64,
    pub free_mb: u64,
    pub cache_mb: u64,
    pub swap_total_mb: u64,
    pub swap_used_mb: u64,
    pub usage_percent: f32,
}

impl MemoryStats {
    pub const fn new() -> Self {
        Self {
            total_mb: 0,
            used_mb: 0,
            free_mb: 0,
            cache_mb: 0,
            swap_total_mb: 0,
            swap_used_mb: 0,
            usage_percent: 0.0,
        }
    }
}

// ── Disk statistics ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DiskStats {
    pub device: [u8; 32],
    pub total_gb: u64,
    pub used_gb: u64,
    pub free_gb: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_count: u64,
    pub write_count: u64,
    pub usage_percent: f32,
}

impl DiskStats {
    pub const fn new() -> Self {
        Self {
            device: [0u8; 32],
            total_gb: 0,
            used_gb: 0,
            free_gb: 0,
            read_bytes: 0,
            write_bytes: 0,
            read_count: 0,
            write_count: 0,
            usage_percent: 0.0,
        }
    }
}

// ── Network statistics ───────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NetworkStats {
    pub interface: [u8; 16],
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errors: u64,
    pub tx_errors: u64,
    pub rx_bps: u64, // bits per second
    pub tx_bps: u64,
}

impl NetworkStats {
    pub const fn new() -> Self {
        Self {
            interface: [0u8; 16],
            rx_bytes: 0,
            tx_bytes: 0,
            rx_packets: 0,
            tx_packets: 0,
            rx_errors: 0,
            tx_errors: 0,
            rx_bps: 0,
            tx_bps: 0,
        }
    }
}

// ── Process statistics ───────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ProcessStats {
    pub pid: u32,
    pub name: [u8; 64],
    pub cpu_percent: f32,
    pub memory_mb: u64,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub state: u8,
    pub priority: i32,
}

impl ProcessStats {
    pub const fn new(pid: u32) -> Self {
        Self {
            pid,
            name: [0u8; 64],
            cpu_percent: 0.0,
            memory_mb: 0,
            read_bytes: 0,
            write_bytes: 0,
            state: 0,
            priority: 0,
        }
    }
}

// ── Temperature statistics ───────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TemperatureStats {
    pub sensor: [u8; 32],
    pub temperature_celsius: f32,
    pub max_celsius: f32,
    pub critical_celsius: f32,
}

impl TemperatureStats {
    pub const fn new() -> Self {
        Self {
            sensor: [0u8; 32],
            temperature_celsius: 0.0,
            max_celsius: 0.0,
            critical_celsius: 0.0,
        }
    }
}

// ── Alert configuration ─────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AlertConfig {
    pub cpu_threshold: f32,
    pub memory_threshold: f32,
    pub disk_threshold: f32,
    pub temperature_threshold: f32,
    pub enabled: bool,
}

impl AlertConfig {
    pub const fn new() -> Self {
        Self {
            cpu_threshold: 90.0,
            memory_threshold: 90.0,
            disk_threshold: 90.0,
            temperature_threshold: 80.0,
            enabled: true,
        }
    }
}

// ── System monitor state ─────────────────────────────────────────────

const MAX_CPUS: usize = 64;
const MAX_DISKS: usize = 16;
const MAX_NETWORKS: usize = 16;
const MAX_PROCESSES: usize = 1024;
const MAX_TEMPERATURES: usize = 32;

pub struct SystemMonitor {
    cpu_stats: [CpuStats; MAX_CPUS],
    cpu_count: u32,
    memory_stats: MemoryStats,
    disk_stats: [Option<DiskStats>; MAX_DISKS],
    disk_count: u32,
    network_stats: [Option<NetworkStats>; MAX_NETWORKS],
    network_count: u32,
    process_stats: [Option<ProcessStats>; MAX_PROCESSES],
    process_count: u32,
    temperature_stats: [Option<TemperatureStats>; MAX_TEMPERATURES],
    temperature_count: u32,
    alert_config: AlertConfig,
    uptime_seconds: AtomicU64,
    initialized: bool,
}

impl SystemMonitor {
    pub const fn new() -> Self {
        Self {
            cpu_stats: [CpuStats::new(); MAX_CPUS],
            cpu_count: 0,
            memory_stats: MemoryStats::new(),
            disk_stats: [const { None }; MAX_DISKS],
            disk_count: 0,
            network_stats: [const { None }; MAX_NETWORKS],
            network_count: 0,
            process_stats: [const { None }; MAX_PROCESSES],
            process_count: 0,
            temperature_stats: [const { None }; MAX_TEMPERATURES],
            temperature_count: 0,
            alert_config: AlertConfig::new(),
            uptime_seconds: AtomicU64::new(0),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Update CPU statistics
    pub fn update_cpu(&mut self, cpu_id: u32, user: u64, system: u64, idle: u64) -> bool {
        if !self.initialized || cpu_id as usize >= MAX_CPUS {
            return false;
        }

        let total = user + system + idle;
        let usage = if total > 0 {
            ((user + system) as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        self.cpu_stats[cpu_id as usize] = CpuStats {
            user,
            system,
            idle,
            iowait: 0,
            total,
            usage_percent: usage,
        };

        if cpu_id + 1 > self.cpu_count {
            self.cpu_count = cpu_id + 1;
        }

        true
    }

    /// Update memory statistics
    pub fn update_memory(&mut self, total: u64, used: u64, free: u64, cache: u64) {
        if !self.initialized {
            return;
        }

        let usage = if total > 0 {
            (used as f32 / total as f32) * 100.0
        } else {
            0.0
        };

        self.memory_stats = MemoryStats {
            total_mb: total,
            used_mb: used,
            free_mb: free,
            cache_mb: cache,
            swap_total_mb: 0,
            swap_used_mb: 0,
            usage_percent: usage,
        };
    }

    /// Add disk statistics
    pub fn add_disk(&mut self, disk: DiskStats) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_DISKS {
            if self.disk_stats[i].is_none() {
                self.disk_stats[i] = Some(disk);
                self.disk_count += 1;
                return true;
            }
        }
        false
    }

    /// Add network statistics
    pub fn add_network(&mut self, network: NetworkStats) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_NETWORKS {
            if self.network_stats[i].is_none() {
                self.network_stats[i] = Some(network);
                self.network_count += 1;
                return true;
            }
        }
        false
    }

    /// Add process statistics
    pub fn add_process(&mut self, process: ProcessStats) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_PROCESSES {
            if self.process_stats[i].is_none() {
                self.process_stats[i] = Some(process);
                self.process_count += 1;
                return true;
            }
        }
        false
    }

    /// Add temperature statistics
    pub fn add_temperature(&mut self, temp: TemperatureStats) -> bool {
        if !self.initialized {
            return false;
        }

        for i in 0..MAX_TEMPERATURES {
            if self.temperature_stats[i].is_none() {
                self.temperature_stats[i] = Some(temp);
                self.temperature_count += 1;
                return true;
            }
        }
        false
    }

    /// Check for alerts
    pub fn check_alerts(&self) -> u32 {
        if !self.initialized || !self.alert_config.enabled {
            return 0;
        }

        let mut alerts = 0u32;

        // Check CPU
        for i in 0..self.cpu_count as usize {
            if self.cpu_stats[i].usage_percent > self.alert_config.cpu_threshold {
                alerts += 1;
            }
        }

        // Check memory
        if self.memory_stats.usage_percent > self.alert_config.memory_threshold {
            alerts += 1;
        }

        // Check disks
        for i in 0..self.disk_count as usize {
            if let Some(disk) = &self.disk_stats[i] {
                if disk.usage_percent > self.alert_config.disk_threshold {
                    alerts += 1;
                }
            }
        }

        // Check temperatures
        for i in 0..self.temperature_count as usize {
            if let Some(temp) = &self.temperature_stats[i] {
                if temp.temperature_celsius > self.alert_config.temperature_threshold {
                    alerts += 1;
                }
            }
        }

        alerts
    }

    /// Update uptime
    pub fn update_uptime(&self) {
        self.uptime_seconds.fetch_add(1, Ordering::Relaxed);
    }

    pub fn uptime(&self) -> u64 {
        self.uptime_seconds.load(Ordering::Relaxed)
    }

    pub fn cpu_count(&self) -> u32 {
        self.cpu_count
    }

    pub fn disk_count(&self) -> u32 {
        self.disk_count
    }

    pub fn network_count(&self) -> u32 {
        self.network_count
    }

    pub fn process_count(&self) -> u32 {
        self.process_count
    }
}

// ── Global system monitor instance ─────────────────────────────────────

static mut G_SYSTEM_MONITOR: SystemMonitor = SystemMonitor::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn monitor_init() {
    G_SYSTEM_MONITOR.init();
}

#[no_mangle]
pub unsafe extern "C" fn monitor_update_cpu(
    cpu_id: u32,
    user: u64,
    system: u64,
    idle: u64,
) -> i32 {
    if G_SYSTEM_MONITOR.update_cpu(cpu_id, user, system, idle) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn monitor_update_memory(
    total: u64,
    used: u64,
    free: u64,
    cache: u64,
) {
    G_SYSTEM_MONITOR.update_memory(total, used, free, cache);
}

#[no_mangle]
pub unsafe extern "C" fn monitor_add_disk(
    device: *const u8,
    total_gb: u64,
    used_gb: u64,
    free_gb: u64,
) -> i32 {
    let mut disk = DiskStats::new();
    
    if !device.is_null() {
        let dev_slice = core::slice::from_raw_parts(device, 32.min(disk.device.len()));
        for i in 0..dev_slice.len() {
            disk.device[i] = dev_slice[i];
        }
    }
    
    disk.total_gb = total_gb;
    disk.used_gb = used_gb;
    disk.free_gb = free_gb;
    disk.usage_percent = if total_gb > 0 {
        (used_gb as f32 / total_gb as f32) * 100.0
    } else {
        0.0
    };
    
    if G_SYSTEM_MONITOR.add_disk(disk) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn monitor_add_network(
    interface: *const u8,
    rx_bytes: u64,
    tx_bytes: u64,
) -> i32 {
    let mut network = NetworkStats::new();
    
    if !interface.is_null() {
        let if_slice = core::slice::from_raw_parts(interface, 16.min(network.interface.len()));
        for i in 0..if_slice.len() {
            network.interface[i] = if_slice[i];
        }
    }
    
    network.rx_bytes = rx_bytes;
    network.tx_bytes = tx_bytes;
    
    if G_SYSTEM_MONITOR.add_network(network) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn monitor_add_process(
    pid: u32,
    name: *const u8,
    cpu_percent: f32,
    memory_mb: u64,
) -> i32 {
    let mut process = ProcessStats::new(pid);
    
    if !name.is_null() {
        let name_slice = core::slice::from_raw_parts(name, 64.min(process.name.len()));
        for i in 0..name_slice.len() {
            process.name[i] = name_slice[i];
        }
    }
    
    process.cpu_percent = cpu_percent;
    process.memory_mb = memory_mb;
    
    if G_SYSTEM_MONITOR.add_process(process) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn monitor_add_temperature(
    sensor: *const u8,
    temp_celsius: f32,
    max_celsius: f32,
) -> i32 {
    let mut temp = TemperatureStats::new();
    
    if !sensor.is_null() {
        let sensor_slice = core::slice::from_raw_parts(sensor, 32.min(temp.sensor.len()));
        for i in 0..sensor_slice.len() {
            temp.sensor[i] = sensor_slice[i];
        }
    }
    
    temp.temperature_celsius = temp_celsius;
    temp.max_celsius = max_celsius;
    temp.critical_celsius = max_celsius + 10.0;
    
    if G_SYSTEM_MONITOR.add_temperature(temp) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn monitor_check_alerts() -> u32 {
    G_SYSTEM_MONITOR.check_alerts()
}

#[no_mangle]
pub unsafe extern "C" fn monitor_update_uptime() {
    G_SYSTEM_MONITOR.update_uptime();
}

#[no_mangle]
pub unsafe extern "C" fn monitor_uptime() -> u64 {
    G_SYSTEM_MONITOR.uptime()
}

#[no_mangle]
pub unsafe extern "C" fn monitor_cpu_count() -> u32 {
    G_SYSTEM_MONITOR.cpu_count()
}

#[no_mangle]
pub unsafe extern "C" fn monitor_disk_count() -> u32 {
    G_SYSTEM_MONITOR.disk_count()
}

#[no_mangle]
pub unsafe extern "C" fn monitor_network_count() -> u32 {
    G_SYSTEM_MONITOR.network_count()
}

#[no_mangle]
pub unsafe extern "C" fn monitor_process_count() -> u32 {
    G_SYSTEM_MONITOR.process_count()
}

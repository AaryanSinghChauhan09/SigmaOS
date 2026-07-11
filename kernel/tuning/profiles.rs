// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/tuning/profiles.rs — Kernel Performance Profiles
//
// This module implements performance tuning profiles inspired by Linux kernel
// tuning profiles (tuned, kernel-tools, etc.). These profiles allow dynamic
// adjustment of kernel parameters for different workloads.
//
// Key features:
// - Multiple predefined profiles (desktop, server, latency, throughput)
// - Dynamic profile switching
// - OOP principles with profile traits
// - No external dependencies

#![no_std]
#![allow(dead_code)]

// ─────────────────────────────────────────────────────────────────────────────
// Profile Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ProfileType {
    Balanced,      // Default balanced profile
    Desktop,       // Optimized for desktop workloads
    Server,        // Optimized for server workloads
    Latency,       // Optimized for low latency
    Throughput,    // Optimized for high throughput
    PowerSave,     // Optimized for power efficiency
    Custom,        // Custom profile
}

// ─────────────────────────────────────────────────────────────────────────────
// Profile Parameters
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct ProfileParams {
    // Scheduler parameters
    pub sched_latency_ns: u64,
    pub sched_min_granularity_ns: u64,
    pub sched_wakeup_granularity_ns: u64,
    pub sched_migration_cost_ns: u64,
    
    // Memory parameters
    pub swappiness: u8,           // 0-100, tendency to swap
    pub vfs_cache_pressure: u8,   // 0-100, cache reclaim tendency
    pub min_free_kbytes: u32,     // Minimum free memory
    pub overcommit_ratio: u8,     // Memory overcommit ratio
    
    // I/O parameters
    pub io_queue_depth: u32,      // I/O queue depth
    pub io_scheduler: u8,         // I/O scheduler type
    pub read_ahead_kb: u32,       // Read-ahead size in KB
    
    // Network parameters
    pub tcp_slow_start_after_idle: bool,
    pub tcp_fastopen: bool,
    pub tcp_low_latency: bool,
    pub tcp_congestion_control: u8, // 0=cubic, 1=bbr, 2=reno
    
    // CPU parameters
    pub cpu_governor: u8,         // 0=performance, 1=powersave, 2=schedutil
    pub cpu_boost: bool,          // CPU boost enabled
    
    // Power parameters
    pub power_saving: bool,       // Power saving mode
    pub autosuspend_delay_ms: u32,
}

impl ProfileParams {
    pub const fn default() -> Self {
        Self {
            sched_latency_ns: 20_000_000,
            sched_min_granularity_ns: 1_000_000,
            sched_wakeup_granularity_ns: 500_000,
            sched_migration_cost_ns: 500_000,
            swappiness: 60,
            vfs_cache_pressure: 100,
            min_free_kbytes: 65536,
            overcommit_ratio: 50,
            io_queue_depth: 128,
            io_scheduler: 0,
            read_ahead_kb: 128,
            tcp_slow_start_after_idle: true,
            tcp_fastopen: true,
            tcp_low_latency: false,
            tcp_congestion_control: 1, // BBR
            cpu_governor: 2, // schedutil
            cpu_boost: true,
            power_saving: false,
            autosuspend_delay_ms: 2000,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Profile Trait for OOP Principles
// ─────────────────────────────────────────────────────────────────────────────

pub trait Profile {
    fn get_type(&self) -> ProfileType;
    fn get_params(&self) -> ProfileParams;
    fn apply(&self) -> bool;
    fn is_compatible(&self) -> bool;
}

// ─────────────────────────────────────────────────────────────────────────────
// Balanced Profile
// ─────────────────────────────────────────────────────────────────────────────

pub struct BalancedProfile;

impl Profile for BalancedProfile {
    fn get_type(&self) -> ProfileType {
        ProfileType::Balanced
    }

    fn get_params(&self) -> ProfileParams {
        ProfileParams::default()
    }

    fn apply(&self) -> bool {
        // Apply balanced parameters
        true
    }

    fn is_compatible(&self) -> bool {
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Desktop Profile
// ─────────────────────────────────────────────────────────────────────────────

pub struct DesktopProfile;

impl Profile for DesktopProfile {
    fn get_type(&self) -> ProfileType {
        ProfileType::Desktop
    }

    fn get_params(&self) -> ProfileParams {
        ProfileParams {
            sched_latency_ns: 15_000_000,      // Lower latency for responsiveness
            sched_min_granularity_ns: 750_000,
            sched_wakeup_granularity_ns: 300_000,
            sched_migration_cost_ns: 300_000,
            swappiness: 40,                    // Less swapping
            vfs_cache_pressure: 75,           // More aggressive cache
            min_free_kbytes: 131072,
            overcommit_ratio: 50,
            io_queue_depth: 64,
            io_scheduler: 0,
            read_ahead_kb: 256,                // Larger read-ahead
            tcp_slow_start_after_idle: false,   // Better web browsing
            tcp_fastopen: true,
            tcp_low_latency: true,              // Low latency networking
            tcp_congestion_control: 1,         // BBR
            cpu_governor: 2,                   // schedutil
            cpu_boost: true,
            power_saving: false,
            autosuspend_delay_ms: 2000,
        }
    }

    fn apply(&self) -> bool {
        // Apply desktop-optimized parameters
        true
    }

    fn is_compatible(&self) -> bool {
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Server Profile
// ─────────────────────────────────────────────────────────────────────────────

pub struct ServerProfile;

impl Profile for ServerProfile {
    fn get_type(&self) -> ProfileType {
        ProfileType::Server
    }

    fn get_params(&self) -> ProfileParams {
        ProfileParams {
            sched_latency_ns: 25_000_000,      // Higher latency for throughput
            sched_min_granularity_ns: 1_500_000,
            sched_wakeup_granularity_ns: 1_000_000,
            sched_migration_cost_ns: 1_000_000,
            swappiness: 80,                     // More aggressive swapping
            vfs_cache_pressure: 50,             // Less cache pressure
            min_free_kbytes: 65536,
            overcommit_ratio: 0,                // No overcommit
            io_queue_depth: 256,               // Larger queue
            io_scheduler: 1,
            read_ahead_kb: 512,                // Much larger read-ahead
            tcp_slow_start_after_idle: true,
            tcp_fastopen: true,
            tcp_low_latency: false,
            tcp_congestion_control: 1,         // BBR
            cpu_governor: 0,                   // performance
            cpu_boost: true,
            power_saving: false,
            autosuspend_delay_ms: 0,
        }
    }

    fn apply(&self) -> bool {
        // Apply server-optimized parameters
        true
    }

    fn is_compatible(&self) -> bool {
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Latency Profile
// ─────────────────────────────────────────────────────────────────────────────

pub struct LatencyProfile;

impl Profile for LatencyProfile {
    fn get_type(&self) -> ProfileType {
        ProfileType::Latency
    }

    fn get_params(&self) -> ProfileParams {
        ProfileParams {
            sched_latency_ns: 10_000_000,      // Very low latency
            sched_min_granularity_ns: 500_000,
            sched_wakeup_granularity_ns: 100_000,
            sched_migration_cost_ns: 100_000,
            swappiness: 10,                     // Minimal swapping
            vfs_cache_pressure: 50,
            min_free_kbytes: 262144,
            overcommit_ratio: 50,
            io_queue_depth: 32,                // Small queue for low latency
            io_scheduler: 0,
            read_ahead_kb: 64,
            tcp_slow_start_after_idle: false,
            tcp_fastopen: true,
            tcp_low_latency: true,
            tcp_congestion_control: 1,         // BBR
            cpu_governor: 0,                   // performance
            cpu_boost: true,
            power_saving: false,
            autosuspend_delay_ms: 0,
        }
    }

    fn apply(&self) -> bool {
        // Apply latency-optimized parameters
        true
    }

    fn is_compatible(&self) -> bool {
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Throughput Profile
// ─────────────────────────────────────────────────────────────────────────────

pub struct ThroughputProfile;

impl Profile for ThroughputProfile {
    fn get_type(&self) -> ProfileType {
        ProfileType::Throughput
    }

    fn get_params(&self) -> ProfileParams {
        ProfileParams {
            sched_latency_ns: 30_000_000,      // High latency for throughput
            sched_min_granularity_ns: 2_000_000,
            sched_wakeup_granularity_ns: 2_000_000,
            sched_migration_cost_ns: 2_000_000,
            swappiness: 90,                     // Aggressive swapping
            vfs_cache_pressure: 25,
            min_free_kbytes: 32768,
            overcommit_ratio: 0,
            io_queue_depth: 512,               // Very large queue
            io_scheduler: 1,
            read_ahead_kb: 1024,               // Very large read-ahead
            tcp_slow_start_after_idle: true,
            tcp_fastopen: true,
            tcp_low_latency: false,
            tcp_congestion_control: 1,         // BBR
            cpu_governor: 0,                   // performance
            cpu_boost: true,
            power_saving: false,
            autosuspend_delay_ms: 0,
        }
    }

    fn apply(&self) -> bool {
        // Apply throughput-optimized parameters
        true
    }

    fn is_compatible(&self) -> bool {
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Power Save Profile
// ─────────────────────────────────────────────────────────────────────────────

pub struct PowerSaveProfile;

impl Profile for PowerSaveProfile {
    fn get_type(&self) -> ProfileType {
        ProfileType::PowerSave
    }

    fn get_params(&self) -> ProfileParams {
        ProfileParams {
            sched_latency_ns: 30_000_000,
            sched_min_granularity_ns: 2_000_000,
            sched_wakeup_granularity_ns: 2_000_000,
            sched_migration_cost_ns: 2_000_000,
            swappiness: 90,
            vfs_cache_pressure: 25,
            min_free_kbytes: 65536,
            overcommit_ratio: 50,
            io_queue_depth: 64,
            io_scheduler: 0,
            read_ahead_kb: 256,
            tcp_slow_start_after_idle: true,
            tcp_fastopen: false,
            tcp_low_latency: false,
            tcp_congestion_control: 0,         // Cubic (less CPU)
            cpu_governor: 1,                   // powersave
            cpu_boost: false,
            power_saving: true,
            autosuspend_delay_ms: 5000,
        }
    }

    fn apply(&self) -> bool {
        // Apply power-saving parameters
        true
    }

    fn is_compatible(&self) -> bool {
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Profile Manager with OOP Principles
// ─────────────────────────────────────────────────────────────────────────────

pub struct ProfileManager {
    current_profile: ProfileType,
    current_params: ProfileParams,
    profile_changed: bool,
}

impl ProfileManager {
    pub const fn new() -> Self {
        Self {
            current_profile: ProfileType::Balanced,
            current_params: ProfileParams::default(),
            profile_changed: false,
        }
    }

    // Switch to a specific profile
    pub fn switch_profile(&mut self, profile_type: ProfileType) -> bool {
        let profile = self.get_profile(profile_type);
        if !profile.is_compatible() {
            return false;
        }

        self.current_params = profile.get_params();
        self.current_profile = profile_type;
        self.profile_changed = true;
        profile.apply()
    }

    // Get profile by type
    fn get_profile(&self, profile_type: ProfileType) -> Box<dyn Profile> {
        match profile_type {
            ProfileType::Balanced => Box::new(BalancedProfile),
            ProfileType::Desktop => Box::new(DesktopProfile),
            ProfileType::Server => Box::new(ServerProfile),
            ProfileType::Latency => Box::new(LatencyProfile),
            ProfileType::Throughput => Box::new(ThroughputProfile),
            ProfileType::PowerSave => Box::new(PowerSaveProfile),
            ProfileType::Custom => Box::new(BalancedProfile), // Fallback
        }
    }

    // Get current profile type
    pub fn get_current_profile(&self) -> ProfileType {
        self.current_profile
    }

    // Get current parameters
    pub fn get_current_params(&self) -> ProfileParams {
        self.current_params
    }

    // Check if profile was changed
    pub fn was_profile_changed(&self) -> bool {
        self.profile_changed
    }

    // Reset profile changed flag
    pub fn reset_profile_changed(&mut self) {
        self.profile_changed = false;
    }

    // Auto-detect optimal profile based on workload
    pub fn auto_detect_profile(&mut self) -> ProfileType {
        // In a real implementation, this would analyze system metrics
        // For now, return balanced as default
        ProfileType::Balanced
    }

    // Apply custom parameters
    pub fn apply_custom_params(&mut self, params: ProfileParams) -> bool {
        self.current_params = params;
        self.current_profile = ProfileType::Custom;
        self.profile_changed = true;
        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton
// ─────────────────────────────────────────────────────────────────────────────

static mut PROFILE_MANAGER: ProfileManager = ProfileManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_profiles_init() {
    PROFILE_MANAGER = ProfileManager::new();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_profiles_switch(profile_type: u8) -> bool {
    let profile_type = match profile_type {
        0 => ProfileType::Balanced,
        1 => ProfileType::Desktop,
        2 => ProfileType::Server,
        3 => ProfileType::Latency,
        4 => ProfileType::Throughput,
        5 => ProfileType::PowerSave,
        _ => ProfileType::Balanced,
    };
    PROFILE_MANAGER.switch_profile(profile_type)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_profiles_get_current() -> u8 {
    match PROFILE_MANAGER.get_current_profile() {
        ProfileType::Balanced => 0,
        ProfileType::Desktop => 1,
        ProfileType::Server => 2,
        ProfileType::Latency => 3,
        ProfileType::Throughput => 4,
        ProfileType::PowerSave => 5,
        ProfileType::Custom => 6,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_profiles_get_sched_latency() -> u64 {
    PROFILE_MANAGER.get_current_params().sched_latency_ns
}

#[no_mangle]
pub unsafe extern "C" fn sigma_profiles_get_swappiness() -> u8 {
    PROFILE_MANAGER.get_current_params().swappiness
}

#[no_mangle]
pub unsafe extern "C" fn sigma_profiles_get_tcp_congestion_control() -> u8 {
    PROFILE_MANAGER.get_current_params().tcp_congestion_control
}

#[no_mangle]
pub unsafe extern "C" fn sigma_profiles_get_cpu_governor() -> u8 {
    PROFILE_MANAGER.get_current_params().cpu_governor
}

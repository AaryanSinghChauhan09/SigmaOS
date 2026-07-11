// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/ostree/health.rs — OSTree Health Checking
//
// Provides health checking for OSTree deployments
// Monitors system health and triggers automatic rollback if needed
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U32 = u32;
type U64 = u64;
type I32 = i32;

pub const HEALTH_OK: I32 = 0;
pub const HEALTH_ERR_FAILED: I32 = -1;
pub const HEALTH_ERR_TIMEOUT: I32 = -2;

const MAX_HEALTH_CHECKS: usize = 10;
const CHECK_NAME_LEN: usize = 64;

// ─── Health Check Status ───────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Failed,
    Unknown,
}

// ─── Health Check Structure ───────────────────────────────────────────────────

#[repr(C)]
pub struct HealthCheck {
    pub name: [U8; CHECK_NAME_LEN],
    pub status: HealthStatus,
    pub last_run: U64,
    pub duration_ms: U32,
    pub failure_count: U32,
    pub critical: bool,
}

impl HealthCheck {
    pub const fn empty() -> Self {
        Self {
            name: [0; CHECK_NAME_LEN],
            status: HealthStatus::Unknown,
            last_run: 0,
            duration_ms: 0,
            failure_count: 0,
            critical: false,
        }
    }
}

// ─── Health Check Result ─────────────────────────────────────────────────────

#[repr(C)]
pub struct HealthCheckResult {
    pub success: bool,
    pub message: [U8; 256],
    pub duration_ms: U32,
}

impl HealthCheckResult {
    pub const fn success(message: &[U8], duration: U32) -> Self {
        let mut result = Self {
            success: true,
            message: [0; 256],
            duration_ms: duration,
        };
        let len = message.len().min(256);
        for i in 0..len {
            result.message[i] = message[i];
        }
        result
    }

    pub const fn failure(message: &[U8], duration: U32) -> Self {
        let mut result = Self {
            success: false,
            message: [0; 256],
            duration_ms: duration,
        };
        let len = message.len().min(256);
        for i in 0..len {
            result.message[i] = message[i];
        }
        result
    }
}

// ─── Health Check Function Type ───────────────────────────────────────────────

type HealthCheckFn = unsafe extern "C" fn() -> HealthCheckResult;

// ─── Health Manager ───────────────────────────────────────────────────────────

pub struct HealthManager {
    pub health_checks: [HealthCheck; MAX_HEALTH_CHECKS],
    pub overall_status: HealthStatus,
    pub last_check_time: U64,
    pub auto_rollback_enabled: bool,
    pub max_failures: U32,
}

impl HealthManager {
    pub const fn new() -> Self {
        Self {
            health_checks: [HealthCheck::empty(); MAX_HEALTH_CHECKS],
            overall_status: HealthStatus::Unknown,
            last_check_time: 0,
            auto_rollback_enabled: true,
            max_failures: 3,
        }
    }

    /// Initialize health manager
    pub unsafe fn init(&mut self) -> I32 {
        // Register default health checks
        self.register_check(b"boot_success", true);
        self.register_check(b"kernel_integrity", true);
        self.register_check(b"filesystem_mount", true);
        self.register_check(b"network_available", false);
        self.register_check(b"services_running", false);

        HEALTH_OK
    }

    /// Register a health check
    pub unsafe fn register_check(&mut self, name: &[U8], critical: bool) -> I32 {
        let slot = self.find_free_slot();
        if slot < 0 {
            return HEALTH_ERR_FAILED;
        }

        let check = &mut self.health_checks[slot as usize];

        let len = name.len().min(CHECK_NAME_LEN);
        for i in 0..len {
            check.name[i] = name[i];
        }

        check.critical = critical;
        check.status = HealthStatus::Unknown;

        HEALTH_OK
    }

    /// Run all health checks
    pub unsafe fn run_all_checks(&mut self) -> I32 {
        let mut critical_failed = false;
        let mut any_failed = false;

        for i in 0..MAX_HEALTH_CHECKS {
            if self.health_checks[i].name[0] != 0 {
                let result = self.run_check(i);
                if !result.success {
                    any_failed = true;
                    if self.health_checks[i].critical {
                        critical_failed = true;
                    }
                }
            }
        }

        // Update overall status
        self.overall_status = if critical_failed {
            HealthStatus::Failed
        } else if any_failed {
            HealthStatus::Degraded
        } else {
            HealthStatus::Healthy
        };

        self.last_check_time = self.get_timestamp();

        // Trigger auto-rollback if critical failures and enabled
        if critical_failed && self.auto_rollback_enabled {
            self.trigger_auto_rollback();
        }

        HEALTH_OK
    }

    /// Run a specific health check
    unsafe fn run_check(&mut self, index: usize) -> HealthCheckResult {
        let check = &mut self.health_checks[index];
        let start_time = self.get_timestamp();

        // In real implementation, would call the actual health check function
        // For now, simulate based on check name
        let result = self.simulate_check(&check.name);

        let duration = (self.get_timestamp() - start_time) as U32;

        check.last_run = start_time;
        check.duration_ms = duration;

        if result.success {
            check.status = HealthStatus::Healthy;
            check.failure_count = 0;
        } else {
            check.status = HealthStatus::Failed;
            check.failure_count += 1;
        }

        result
    }

    /// Simulate health check (stub)
    fn simulate_check(&self, name: &[U8]) -> HealthCheckResult {
        let name_str = core::str::from_utf8(name).unwrap_or("unknown");
        
        match name_str {
            "boot_success" => HealthCheckResult::success(b"Boot successful", 10),
            "kernel_integrity" => HealthCheckResult::success(b"Kernel integrity verified", 50),
            "filesystem_mount" => HealthCheckResult::success(b"Filesystems mounted", 100),
            "network_available" => HealthCheckResult::success(b"Network available", 20),
            "services_running" => HealthCheckResult::success(b"Critical services running", 30),
            _ => HealthCheckResult::success(b"Check passed", 10),
        }
    }

    /// Get overall health status
    pub fn get_overall_status(&self) -> HealthStatus {
        self.overall_status
    }

    /// Get health check results
    pub fn get_health_checks(&self) -> &[HealthCheck] {
        &self.health_checks
    }

    /// Enable/disable auto-rollback
    pub fn set_auto_rollback(&mut self, enabled: bool) {
        self.auto_rollback_enabled = enabled;
    }

    /// Set max failures before rollback
    pub fn set_max_failures(&mut self, max: U32) {
        self.max_failures = max;
    }

    /// Trigger automatic rollback
    unsafe fn trigger_auto_rollback(&self) {
        // In real implementation, would call rollback manager
        // For now, stub
    }

    /// Find free health check slot
    fn find_free_slot(&self) -> isize {
        for i in 0..MAX_HEALTH_CHECKS {
            if self.health_checks[i].name[0] == 0 {
                return i as isize;
            }
        }
        -1
    }

    /// Get current timestamp
    fn get_timestamp(&self) -> U64 {
        // In real implementation, get from RTC
        0
    }
}

// ─── Global Health Manager ───────────────────────────────────────────────────

static mut HEALTH_MANAGER: HealthManager = HealthManager::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_health_init() -> I32 {
    HEALTH_MANAGER.init()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_health_run_all() -> I32 {
    HEALTH_MANAGER.run_all_checks()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_health_register_check(name: *const U8, name_len: U32, critical: bool) -> I32 {
    let name_slice = core::slice::from_raw_parts(name, name_len as usize);
    HEALTH_MANAGER.register_check(name_slice, critical)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_health_get_status() -> I32 {
    match HEALTH_MANAGER.get_overall_status() {
        HealthStatus::Healthy => 0,
        HealthStatus::Degraded => 1,
        HealthStatus::Failed => 2,
        HealthStatus::Unknown => 3,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_health_set_auto_rollback(enabled: bool) {
    HEALTH_MANAGER.set_auto_rollback(enabled);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_health_set_max_failures(max: U32) {
    HEALTH_MANAGER.set_max_failures(max);
}

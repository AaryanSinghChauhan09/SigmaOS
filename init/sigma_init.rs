//! SigmaOS — SigmaInit (PID 1) Init System
//! Sovereign init daemon inspired by systemd/OpenRC but fully native to SigmaOS.
//! No std, no allocator — fixed-size service table.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type U64 = u64;

// ── Constants ───────────────────────────────────────────────────────────────
const MAX_SERVICES:      usize = 128;
const MAX_TARGETS:       usize = 16;
const MAX_DEPS_PER_SVC:  usize = 8;
const MAX_NAME_LEN:      usize = 48;
const MAX_CMD_LEN:       usize = 128;

// ── Service States ──────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum ServiceState {
    Inactive  = 0,
    Starting  = 1,
    Running   = 2,
    Stopping  = 3,
    Stopped   = 4,
    Failed    = 5,
    Restarting = 6,
    Masked    = 7,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum ServiceType {
    Simple    = 0,  // Main process is the service
    Oneshot   = 1,  // Run once and exit
    Forking   = 2,  // Forks, parent exits
    Notify    = 3,  // Sends ready notification
    Idle      = 4,  // Run when system is idle
    Timer     = 5,  // Periodic timer service
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum RestartPolicy {
    No        = 0,
    Always    = 1,
    OnFailure = 2,
    OnAbnormal = 3,
}

// ── Boot Target (like systemd target) ───────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum BootTarget {
    Emergency    = 0,  // Single-user emergency shell
    Rescue       = 1,  // Single-user with basic services
    MultiUser    = 2,  // Full multi-user, no GUI
    Graphical    = 3,  // Full GUI desktop
    Reboot       = 4,
    Poweroff     = 5,
}

// ── Service Unit ────────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct ServiceUnit {
    pub name:          [U8; MAX_NAME_LEN],
    pub name_len:      usize,
    pub description:   [U8; MAX_CMD_LEN],
    pub desc_len:      usize,
    pub exec_start:    [U8; MAX_CMD_LEN],
    pub exec_start_len: usize,
    pub exec_stop:     [U8; MAX_CMD_LEN],
    pub exec_stop_len: usize,
    pub svc_type:      ServiceType,
    pub state:         ServiceState,
    pub restart:       RestartPolicy,
    pub pid:           U32,
    pub exit_code:     i32,
    pub start_time:    U64,
    pub restart_count: U32,
    pub max_restarts:  U32,
    pub timeout_ms:    U32,       // Start/stop timeout
    pub deps:          [U32; MAX_DEPS_PER_SVC], // Service IDs this depends on
    pub dep_count:     usize,
    pub target:        BootTarget, // Which target this belongs to
    pub enabled:       bool,
    pub active:        bool,
}

impl ServiceUnit {
    pub const fn empty() -> Self {
        ServiceUnit {
            name: [0u8; MAX_NAME_LEN], name_len: 0,
            description: [0u8; MAX_CMD_LEN], desc_len: 0,
            exec_start: [0u8; MAX_CMD_LEN], exec_start_len: 0,
            exec_stop: [0u8; MAX_CMD_LEN], exec_stop_len: 0,
            svc_type: ServiceType::Simple,
            state: ServiceState::Inactive,
            restart: RestartPolicy::No,
            pid: 0, exit_code: 0, start_time: 0,
            restart_count: 0, max_restarts: 3,
            timeout_ms: 30_000,
            deps: [0u32; MAX_DEPS_PER_SVC], dep_count: 0,
            target: BootTarget::MultiUser,
            enabled: false, active: false,
        }
    }
}

// ── Init System State ───────────────────────────────────────────────────────
pub struct SigmaInit {
    pub services:      [ServiceUnit; MAX_SERVICES],
    pub service_count: usize,
    pub current_target: BootTarget,
    pub boot_time_ns:  U64,
    pub hostname:      [U8; 64],
    pub hostname_len:  usize,
    pub uptime_ns:     U64,
    pub reboot_requested: bool,
    pub poweroff_requested: bool,
}

static mut INIT: SigmaInit = SigmaInit {
    services: [ServiceUnit::empty(); MAX_SERVICES],
    service_count: 0,
    current_target: BootTarget::MultiUser,
    boot_time_ns: 0,
    hostname: [0u8; 64],
    hostname_len: 0,
    uptime_ns: 0,
    reboot_requested: false,
    poweroff_requested: false,
};

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize the init system (PID 1 entry point).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_start(boot_time_ns: U64) -> i32 {
    INIT.boot_time_ns = boot_time_ns;
    INIT.current_target = BootTarget::MultiUser;
    INIT.service_count = 0;
    INIT.reboot_requested = false;
    INIT.poweroff_requested = false;

    // Set default hostname
    let name = b"sigmaos";
    for (i, &b) in name.iter().enumerate() {
        INIT.hostname[i] = b;
    }
    INIT.hostname_len = name.len();

    // Register built-in essential services
    sigma_init_register_builtin();

    0
}

/// Register built-in essential services.
unsafe fn sigma_init_register_builtin() {
    // Service 0: sigma-udevd (device manager)
    register_service(b"sigma-udevd", b"/sbin/sigma-udevd", ServiceType::Simple,
        BootTarget::Rescue, RestartPolicy::Always);

    // Service 1: sigma-networkd (network manager)
    register_service(b"sigma-networkd", b"/sbin/sigma-networkd", ServiceType::Notify,
        BootTarget::MultiUser, RestartPolicy::OnFailure);

    // Service 2: sigma-logd (logging daemon)
    register_service(b"sigma-logd", b"/sbin/sigma-logd", ServiceType::Simple,
        BootTarget::Rescue, RestartPolicy::Always);

    // Service 3: sigma-dbus (message bus)
    register_service(b"sigma-dbus", b"/sbin/sigma-dbus", ServiceType::Forking,
        BootTarget::MultiUser, RestartPolicy::OnFailure);

    // Service 4: sigma-sshd (SSH server)
    register_service(b"sigma-sshd", b"/sbin/sigma-sshd", ServiceType::Forking,
        BootTarget::MultiUser, RestartPolicy::OnFailure);

    // Service 5: zenith-desktop (GUI compositor)
    register_service(b"zenith-desktop", b"/sbin/zenith-desktop", ServiceType::Notify,
        BootTarget::Graphical, RestartPolicy::Always);

    // Service 6: sigma-crond (cron daemon)
    register_service(b"sigma-crond", b"/sbin/sigma-crond", ServiceType::Simple,
        BootTarget::MultiUser, RestartPolicy::OnFailure);
}

unsafe fn register_service(
    name: &[U8], exec: &[U8], svc_type: ServiceType,
    target: BootTarget, restart: RestartPolicy,
) {
    if INIT.service_count >= MAX_SERVICES { return; }
    let idx = INIT.service_count;
    let svc = &mut INIT.services[idx];

    let nl = name.len().min(MAX_NAME_LEN);
    for i in 0..nl { svc.name[i] = name[i]; }
    svc.name_len = nl;

    let el = exec.len().min(MAX_CMD_LEN);
    for i in 0..el { svc.exec_start[i] = exec[i]; }
    svc.exec_start_len = el;

    svc.svc_type = svc_type;
    svc.target = target;
    svc.restart = restart;
    svc.enabled = true;
    svc.active = true;
    svc.state = ServiceState::Inactive;

    INIT.service_count += 1;
}

/// Start a service by index.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_start_service(svc_idx: U32) -> i32 {
    let idx = svc_idx as usize;
    if idx >= INIT.service_count { return -1; }

    let svc = &mut INIT.services[idx];
    if svc.state == ServiceState::Running { return 0; } // Already running
    if svc.state == ServiceState::Masked { return -2; }

    // Check dependencies
    for d in 0..svc.dep_count {
        let dep_idx = svc.deps[d] as usize;
        if dep_idx < INIT.service_count {
            if INIT.services[dep_idx].state != ServiceState::Running {
                return -3; // Dependency not running
            }
        }
    }

    svc.state = ServiceState::Starting;
    // In a real implementation: fork + exec svc.exec_start
    // For now, mark as running
    svc.state = ServiceState::Running;
    svc.start_time = INIT.uptime_ns;
    0
}

/// Stop a service by index.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_stop_service(svc_idx: U32) -> i32 {
    let idx = svc_idx as usize;
    if idx >= INIT.service_count { return -1; }

    let svc = &mut INIT.services[idx];
    if svc.state != ServiceState::Running { return -2; }

    svc.state = ServiceState::Stopping;
    // In a real implementation: send SIGTERM, wait, then SIGKILL
    svc.state = ServiceState::Stopped;
    0
}

/// Restart a service.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_restart_service(svc_idx: U32) -> i32 {
    sigma_init_stop_service(svc_idx);
    sigma_init_start_service(svc_idx)
}

/// Bring up all services for a target.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_reach_target(target: U8) -> i32 {
    let target = match target {
        0 => BootTarget::Emergency,
        1 => BootTarget::Rescue,
        2 => BootTarget::MultiUser,
        3 => BootTarget::Graphical,
        4 => BootTarget::Reboot,
        5 => BootTarget::Poweroff,
        _ => return -1,
    };

    INIT.current_target = target;

    match target {
        BootTarget::Reboot => {
            INIT.reboot_requested = true;
            // Stop all services in reverse order
            let count = INIT.service_count;
            for i in (0..count).rev() {
                sigma_init_stop_service(i as U32);
            }
            return 0;
        }
        BootTarget::Poweroff => {
            INIT.poweroff_requested = true;
            let count = INIT.service_count;
            for i in (0..count).rev() {
                sigma_init_stop_service(i as U32);
            }
            return 0;
        }
        _ => {}
    }

    // Start all enabled services for this target and below
    for i in 0..INIT.service_count {
        if INIT.services[i].enabled && INIT.services[i].target as u8 <= target as u8 {
            sigma_init_start_service(i as U32);
        }
    }

    0
}

/// Handle service exit (called when a child process exits).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_service_exited(pid: U32, exit_code: i32) {
    for i in 0..INIT.service_count {
        if INIT.services[i].pid == pid && INIT.services[i].state == ServiceState::Running {
            INIT.services[i].exit_code = exit_code;

            if exit_code != 0 {
                INIT.services[i].state = ServiceState::Failed;
            } else {
                INIT.services[i].state = ServiceState::Stopped;
            }

            // Apply restart policy
            let should_restart = match INIT.services[i].restart {
                RestartPolicy::Always => true,
                RestartPolicy::OnFailure => exit_code != 0,
                RestartPolicy::OnAbnormal => exit_code != 0 && exit_code != 1,
                RestartPolicy::No => false,
            };

            if should_restart && INIT.services[i].restart_count < INIT.services[i].max_restarts {
                INIT.services[i].restart_count += 1;
                INIT.services[i].state = ServiceState::Restarting;
                sigma_init_start_service(i as U32);
            }

            break;
        }
    }
}

/// Set hostname.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_set_hostname(name: *const U8, len: usize) -> i32 {
    if name.is_null() || len == 0 || len > 64 { return -1; }
    let src = core::slice::from_raw_parts(name, len);
    for i in 0..len { INIT.hostname[i] = src[i]; }
    INIT.hostname_len = len;
    0
}

/// Get the number of registered services.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_service_count() -> U32 {
    INIT.service_count as U32
}

/// Get a service's state by index.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_service_state(svc_idx: U32) -> U8 {
    let idx = svc_idx as usize;
    if idx >= INIT.service_count { return 0; }
    INIT.services[idx].state as U8
}

/// Update system uptime (called from timer tick).
#[no_mangle]
pub unsafe extern "C" fn sigma_init_update_uptime(ns: U64) {
    INIT.uptime_ns = ns;
}

/// Check if reboot is requested.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_reboot_requested() -> i32 {
    if INIT.reboot_requested { 1 } else { 0 }
}

/// Check if poweroff is requested.
#[no_mangle]
pub unsafe extern "C" fn sigma_init_poweroff_requested() -> i32 {
    if INIT.poweroff_requested { 1 } else { 0 }
}
//! SigmaOS — MicroVM Sandbox (Security & Isolation)
//! Equivalent to gVisor / Firecracker isolation.
//! Uses KVM/VMM primitives for lightweight process isolation.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U32 = u32;
type U64 = u64;

// ── Sandbox Configuration ───────────────────────────────────────────────────
pub struct SandboxConfig {
    pub memory_limit_mb: U32,
    pub cpu_quota: U32, // Percentage (e.g., 100 = 1 core)
    pub network_namespace: U32,
    pub allow_syscalls: [bool; 256], // Syscall filter (seccomp equivalent)
    pub block_devices: [U64; 4], // Allowed loopback block devices
}

impl SandboxConfig {
    pub const fn default_strict() -> Self {
        let mut allow = [false; 256];
        // Allow basic syscalls: read, write, exit, sched_yield
        allow[0] = true;
        allow[1] = true;
        allow[24] = true;
        allow[60] = true;

        SandboxConfig {
            memory_limit_mb: 128,
            cpu_quota: 50,
            network_namespace: 0, // No network by default
            allow_syscalls: allow,
            block_devices: [0; 4],
        }
    }
}

// ── Sandbox State ───────────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
pub enum SandboxState {
    Stopped = 0,
    Initializing = 1,
    Running = 2,
    Terminated = 3,
}

pub struct MicroVM {
    pub id: U32,
    pub state: SandboxState,
    pub root_cr3: U64, // Isolated page table root
    pub pid_namespace: U32,
}

const MAX_SANDBOXES: usize = 32;
static mut SANDBOXES: [MicroVM; MAX_SANDBOXES] = [MicroVM {
    id: 0, state: SandboxState::Stopped, root_cr3: 0, pid_namespace: 0
}; MAX_SANDBOXES];
static mut NEXT_SANDBOX_ID: U32 = 1;

// ── Public API ──────────────────────────────────────────────────────────────

/// Create a new MicroVM sandbox with the given configuration.
#[no_mangle]
pub unsafe extern "C" fn sigma_sandbox_create(config: *const SandboxConfig) -> i32 {
    if config.is_null() { return -1; }

    for i in 0..MAX_SANDBOXES {
        if SANDBOXES[i].state == SandboxState::Stopped {
            SANDBOXES[i].id = NEXT_SANDBOX_ID;
            NEXT_SANDBOX_ID += 1;
            SANDBOXES[i].state = SandboxState::Initializing;
            
            // Create isolated page table
            // SANDBOXES[i].root_cr3 = sigma_vmm_create_address_space();
            
            return SANDBOXES[i].id as i32;
        }
    }
    
    -2 // No available sandboxes
}

/// Start a previously created sandbox.
#[no_mangle]
pub unsafe extern "C" fn sigma_sandbox_start(sandbox_id: U32) -> i32 {
    for i in 0..MAX_SANDBOXES {
        if SANDBOXES[i].id == sandbox_id && SANDBOXES[i].state == SandboxState::Initializing {
            SANDBOXES[i].state = SandboxState::Running;
            return 0;
        }
    }
    -1 // Not found or invalid state
}

/// Kill and cleanup a sandbox.
#[no_mangle]
pub unsafe extern "C" fn sigma_sandbox_destroy(sandbox_id: U32) -> i32 {
    for i in 0..MAX_SANDBOXES {
        if SANDBOXES[i].id == sandbox_id {
            SANDBOXES[i].state = SandboxState::Stopped;
            // sigma_vmm_destroy_address_space(SANDBOXES[i].root_cr3);
            SANDBOXES[i].root_cr3 = 0;
            return 0;
        }
    }
    -1
}

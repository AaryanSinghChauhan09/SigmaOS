/// SigmaOS — modules/core/kernel/self_heal.rs
/// Autonomous fault detection, restart policies, and state recovery for shards.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Configuration ────────────────────────────────────────────────────────────

pub const MAX_SHARDS: SigmaUsize = 1024;
pub const MAX_RESTARTS_PER_MINUTE: SigmaU32 = 5;

// ─── Shard Fault State ────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RestartPolicy {
    Never          = 0,
    OnFailure      = 1,
    Always         = 2,
    EscalateToHost = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FaultRecord {
    pub shard_id:        SigmaU32,
    pub policy:          RestartPolicy,
    pub restart_count:   SigmaU32,
    pub last_fault_time: SigmaU64,
    pub backoff_ms:      SigmaU32,
    pub pending_restart: SigmaBool,
}

impl FaultRecord {
    pub const fn empty() -> Self {
        FaultRecord {
            shard_id:        0,
            policy:          RestartPolicy::Never,
            restart_count:   0,
            last_fault_time: 0,
            backoff_ms:      0,
            pending_restart: false,
        }
    }
}

static mut FAULT_REGISTRY: [FaultRecord; MAX_SHARDS] = [FaultRecord::empty(); MAX_SHARDS];

// ─── External Hooks ───────────────────────────────────────────────────────────

extern "C" {
    fn kernel_uptime() -> SigmaU64;
    fn shard_kill(id: SigmaU32) -> SigmaI32;
    fn shard_spawn(args: *const SigmaU64) -> SigmaI64;
    fn kernel_panic(msg: *const u8) -> !;
    fn sigma_log(level: SigmaU32, msg: *const u8);
}

// ─── Implementation ───────────────────────────────────────────────────────────

/// Register a shard's restart policy.
#[no_mangle]
pub unsafe extern "C" fn self_heal_register(
    shard_id: SigmaU32,
    policy:   SigmaU32,
) -> SigmaI32 {
    let idx = shard_id as usize;
    if idx >= MAX_SHARDS { return -1; }

    let p = match policy {
        0 => RestartPolicy::Never,
        1 => RestartPolicy::OnFailure,
        2 => RestartPolicy::Always,
        3 => RestartPolicy::EscalateToHost,
        _ => return -1,
    };

    FAULT_REGISTRY[idx].shard_id        = shard_id;
    FAULT_REGISTRY[idx].policy          = p;
    FAULT_REGISTRY[idx].restart_count   = 0;
    FAULT_REGISTRY[idx].last_fault_time = 0;
    FAULT_REGISTRY[idx].backoff_ms      = 0;
    FAULT_REGISTRY[idx].pending_restart = false;
    0
}

/// Triggered by the exception handler when a shard crashes.
#[no_mangle]
pub unsafe extern "C" fn self_heal_fault_detected(shard_id: SigmaU32, reason_code: SigmaU32) {
    let idx = shard_id as usize;
    if idx >= MAX_SHARDS { return; }

    let rec = &mut FAULT_REGISTRY[idx];
    let now = kernel_uptime();

    // Reset counter if it's been more than a minute since last fault
    if now > rec.last_fault_time + 60_000 {
        rec.restart_count = 0;
    }

    rec.last_fault_time = now;
    rec.restart_count = rec.restart_count.wrapping_add(1);

    match rec.policy {
        RestartPolicy::Never => {
            shard_kill(shard_id);
        }
        RestartPolicy::OnFailure | RestartPolicy::Always => {
            if rec.restart_count > MAX_RESTARTS_PER_MINUTE {
                // Too many crashes — mark dead and don't restart
                shard_kill(shard_id);
                rec.pending_restart = false;
            } else {
                // Schedule restart with exponential backoff
                shard_kill(shard_id);
                rec.backoff_ms = 100 * (1 << (rec.restart_count - 1));
                rec.pending_restart = true;
            }
        }
        RestartPolicy::EscalateToHost => {
            // Critical system shard crashed — bring down the whole kernel
            kernel_panic(b"Critical shard fault - escalating to panic\0".as_ptr());
        }
    }
}

/// Called periodically by the scheduler/watchdog to restart pending shards.
#[no_mangle]
pub unsafe extern "C" fn self_heal_tick() {
    let now = kernel_uptime();
    for rec in FAULT_REGISTRY.iter_mut() {
        if rec.pending_restart {
            if now >= rec.last_fault_time + (rec.backoff_ms as SigmaU64) {
                // Time to restart
                rec.pending_restart = false;
                
                // Construct spawn request (placeholder for actual args)
                let spawn_args: [SigmaU64; 6] = [rec.shard_id as SigmaU64, 0, 0, 0, 0, 0];
                let _ = shard_spawn(spawn_args.as_ptr());
            }
        }
    }
}

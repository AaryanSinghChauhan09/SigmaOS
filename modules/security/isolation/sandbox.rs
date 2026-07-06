/// SigmaOS — modules/security/isolation/sandbox.rs
/// Sovereign Sandbox Configuration and Lifecycle.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaU64   = u64;
type SigmaBool  = bool;
type SigmaUsize = usize;

pub const MAX_SANDBOXES: SigmaUsize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SandboxConfig {
    pub max_memory_kb: SigmaU64,
    pub max_cpu_time:  SigmaU64,
    pub allowed_caps:  SigmaU64,
    pub allow_net:     SigmaBool,
    pub allow_fs:      SigmaBool,
}

impl SandboxConfig {
    pub const fn default() -> Self {
        SandboxConfig {
            max_memory_kb: 16384, // 16MB default
            max_cpu_time:  0,     // unlimited
            allowed_caps:  0,
            allow_net:     false,
            allow_fs:      false,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SandboxState {
    pub id:       SigmaU32,
    pub active:   SigmaBool,
    pub config:   SandboxConfig,
    pub root_pid: SigmaU32, // The shard ID running inside
}

impl SandboxState {
    pub const fn empty() -> Self {
        SandboxState {
            id:       0,
            active:   false,
            config:   SandboxConfig::default(),
            root_pid: 0,
        }
    }
}

static mut SANDBOXES: [SandboxState; MAX_SANDBOXES] = [SandboxState::empty(); MAX_SANDBOXES];
static mut NEXT_SB_ID: SigmaU32 = 1;

extern "C" {
    fn shard_kill(id: SigmaU32) -> SigmaI32;
}

#[no_mangle]
pub unsafe extern "C" fn init_security_isolation() -> SigmaI32 {
    for sb in SANDBOXES.iter_mut() { sb.active = false; }
    0
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_create(cfg_ptr: *const u8) -> SigmaI32 {
    if cfg_ptr.is_null() { return -1; }
    
    let cfg = &*(cfg_ptr as *const SandboxConfig);
    
    for i in 0..MAX_SANDBOXES {
        if !SANDBOXES[i].active {
            let id = NEXT_SB_ID;
            NEXT_SB_ID = NEXT_SB_ID.wrapping_add(1);
            
            SANDBOXES[i].id     = id;
            SANDBOXES[i].active = true;
            SANDBOXES[i].config = *cfg;
            
            return id as SigmaI32;
        }
    }
    -12 // ENOMEM
}

#[no_mangle]
pub unsafe extern "C" fn sandbox_destroy(id: SigmaU32) -> SigmaI32 {
    for i in 0..MAX_SANDBOXES {
        if SANDBOXES[i].active && SANDBOXES[i].id == id {
            // Kill all shards associated with this sandbox.
            // Simplified: we just kill the root PID for now.
            if SANDBOXES[i].root_pid != 0 {
                shard_kill(SANDBOXES[i].root_pid);
            }
            SANDBOXES[i].active = false;
            return 0;
        }
    }
    -4 // ENOENT
}

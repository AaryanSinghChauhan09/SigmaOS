/// SigmaOS: usr/security/sigma_sandbox.rs
/// CLI API for wrapping executables in Sovereign Sandboxes.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU32   = u32;
type SigmaI32   = i32;
type SigmaU64   = u64;
type SigmaUsize = usize;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SandboxConfig {
    pub max_memory_kb: SigmaU64,
    pub max_cpu_time:  SigmaU64,
    pub allowed_caps:  SigmaU64,
    pub allow_net:     bool,
    pub allow_fs:      bool,
}

extern "C" {
    fn sandbox_create(cfg_ptr: *const u8) -> SigmaI32;
    fn sandbox_destroy(id: SigmaU32) -> SigmaI32;
    fn shard_spawn(args: *const SigmaU64) -> i64;
}

#[no_mangle]
pub unsafe extern "C" fn cmd_sandbox_run(
    executable_path: *const u8,
    memory_limit_kb: SigmaU64,
    allow_network: bool,
) -> SigmaI32 {
    let mut cfg = SandboxConfig {
        max_memory_kb: memory_limit_kb,
        max_cpu_time:  0,
        allowed_caps:  0,
        allow_net:     allow_network,
        allow_fs:      false, // strict by default
    };
    
    // Call kernel to instantiate sandbox container
    let sb_id = sandbox_create(&cfg as *const _ as *const u8);
    if sb_id < 0 { return sb_id; }
    
    // Spawn the shard inside the sandbox.
    // In production, we pass sb_id as part of the spawn request to link them.
    let spawn_args: [SigmaU64; 6] = [0, 0, 0, 0, 0, sb_id as SigmaU64];
    let shard_id = shard_spawn(spawn_args.as_ptr());
    
    if shard_id < 0 {
        sandbox_destroy(sb_id as u32);
        return shard_id as i32;
    }
    
    sb_id
}

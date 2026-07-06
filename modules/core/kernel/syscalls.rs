/// SigmaOS — modules/core/kernel/syscalls.rs
/// Sovereign Syscall Dispatcher: numbered ABI, argument validation, audit hook.
/// no_std | no alloc | no external crates.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

type SigmaU64   = u64;
type SigmaU32   = u32;
type SigmaI64   = i64;
type SigmaI32   = i32;
type SigmaUsize = usize;

// ─── Syscall Numbers ──────────────────────────────────────────────────────────

pub const SYS_SIGMA_NOOP:        SigmaU64 = 0;
pub const SYS_SIGMA_EXIT:        SigmaU64 = 1;
pub const SYS_SIGMA_SHARD_SPAWN: SigmaU64 = 2;
pub const SYS_SIGMA_SHARD_KILL:  SigmaU64 = 3;
pub const SYS_SIGMA_IPC_OPEN:    SigmaU64 = 4;
pub const SYS_SIGMA_IPC_SEND:    SigmaU64 = 5;
pub const SYS_SIGMA_IPC_RECV:    SigmaU64 = 6;
pub const SYS_SIGMA_IPC_CLOSE:   SigmaU64 = 7;
pub const SYS_SIGMA_VFS_OPEN:    SigmaU64 = 8;
pub const SYS_SIGMA_VFS_READ:    SigmaU64 = 9;
pub const SYS_SIGMA_VFS_WRITE:   SigmaU64 = 10;
pub const SYS_SIGMA_VFS_CLOSE:   SigmaU64 = 11;
pub const SYS_SIGMA_VFS_SEEK:    SigmaU64 = 12;
pub const SYS_SIGMA_VFS_STAT:    SigmaU64 = 13;
pub const SYS_SIGMA_VFS_MKDIR:   SigmaU64 = 14;
pub const SYS_SIGMA_VFS_UNLINK:  SigmaU64 = 15;
pub const SYS_SIGMA_NET_SOCKET:  SigmaU64 = 16;
pub const SYS_SIGMA_NET_BIND:    SigmaU64 = 17;
pub const SYS_SIGMA_NET_CONNECT: SigmaU64 = 18;
pub const SYS_SIGMA_NET_SEND:    SigmaU64 = 19;
pub const SYS_SIGMA_NET_RECV:    SigmaU64 = 20;
pub const SYS_SIGMA_NET_CLOSE:   SigmaU64 = 21;
pub const SYS_SIGMA_MEM_ALLOC:   SigmaU64 = 22;
pub const SYS_SIGMA_MEM_FREE:    SigmaU64 = 23;
pub const SYS_SIGMA_MEM_MAP:     SigmaU64 = 24;
pub const SYS_SIGMA_CAP_DERIVE:  SigmaU64 = 25;
pub const SYS_SIGMA_CAP_REVOKE:  SigmaU64 = 26;
pub const SYS_SIGMA_UPTIME:      SigmaU64 = 27;
pub const SYS_SIGMA_AUDIT_LOG:   SigmaU64 = 28;
pub const SYS_SIGMA_SANDBOX_NEW: SigmaU64 = 29;
pub const SYS_SIGMA_SANDBOX_EXEC:SigmaU64 = 30;
pub const SYS_SIGMA_YIELD:       SigmaU64 = 31;
pub const SYS_SIGMA_REBOOT:      SigmaU64 = 32;

pub const SYS_SIGMA_MAX:         SigmaU64 = 33;

// ─── Syscall Result Codes ─────────────────────────────────────────────────────

pub const ESIGMA_OK:         SigmaI64 = 0;
pub const ESIGMA_INVAL:      SigmaI64 = -1;
pub const ESIGMA_PERM:       SigmaI64 = -3;
pub const ESIGMA_NOSYS:      SigmaI64 = -38;
pub const ESIGMA_FAULT:      SigmaI64 = -14;
pub const ESIGMA_BUSY:       SigmaI64 = -16;

// ─── Syscall Statistics ───────────────────────────────────────────────────────

#[repr(C)]
pub struct SyscallStats {
    pub total_calls:   SigmaU64,
    pub denied_calls:  SigmaU64,
    pub invalid_calls: SigmaU64,
    /// Per-syscall invocation counter (index = syscall number)
    pub per_call:      [SigmaU64; SYS_SIGMA_MAX as SigmaUsize],
}

static mut SYSCALL_STATS: SyscallStats = SyscallStats {
    total_calls:   0,
    denied_calls:  0,
    invalid_calls: 0,
    per_call:      [0u64; SYS_SIGMA_MAX as SigmaUsize],
};

// ─── External handlers (implemented in other modules) ─────────────────────────

extern "C" {
    fn shard_spawn(args: *const SigmaU64) -> SigmaI64;
    fn shard_kill(id: SigmaU64)           -> SigmaI64;
    fn ipc_open(owner: SigmaU32, peer: SigmaU32, cap: SigmaU64) -> SigmaI32;
    fn ipc_send(ch: SigmaU32, msg: *const u8) -> SigmaI32;
    fn ipc_recv(ch: SigmaU32, out: *mut u8)   -> SigmaI32;
    fn ipc_close(ch: SigmaU32)                -> SigmaI32;
    fn vfs_open(path: *const u8, flags: SigmaU32, mode: SigmaU32) -> SigmaI32;
    fn vfs_read(fd: SigmaI32, buf: *mut u8, len: SigmaUsize) -> SigmaI64;
    fn vfs_write(fd: SigmaI32, buf: *const u8, len: SigmaUsize) -> SigmaI64;
    fn vfs_close(fd: SigmaI32) -> SigmaI32;
    fn sigma_net_socket(domain: SigmaI32, kind: SigmaI32, proto: SigmaI32) -> SigmaI32;
    fn sigma_alloc_pages(order: SigmaU32) -> SigmaI64;
    fn sigma_free_pages(addr: SigmaU64, order: SigmaU32);
    fn kernel_uptime() -> SigmaU64;
    fn sigma_audit_append(entry: *const u8) -> SigmaI32;
    fn sandbox_create(cfg: *const u8) -> SigmaI32;
}

// ─── Syscall Dispatcher ───────────────────────────────────────────────────────

/// Central syscall entry point — called from the interrupt gate (int 0x80 / SYSCALL).
/// `nr` = syscall number, `args` = pointer to array of up to 6 u64 arguments.
/// Returns the result value (negative = error code).
#[no_mangle]
pub unsafe extern "C" fn sigma_syscall(nr: SigmaU64, args: *const SigmaU64) -> SigmaI64 {
    // Accounting
    SYSCALL_STATS.total_calls = SYSCALL_STATS.total_calls.wrapping_add(1);

    if nr >= SYS_SIGMA_MAX {
        SYSCALL_STATS.invalid_calls = SYSCALL_STATS.invalid_calls.wrapping_add(1);
        return ESIGMA_NOSYS;
    }

    SYSCALL_STATS.per_call[nr as SigmaUsize] =
        SYSCALL_STATS.per_call[nr as SigmaUsize].wrapping_add(1);

    // Safe arg helpers (return 0 when args is null)
    let a = |i: SigmaUsize| -> SigmaU64 {
        if args.is_null() { 0 } else { *args.add(i) }
    };

    match nr {
        SYS_SIGMA_NOOP         => ESIGMA_OK,
        SYS_SIGMA_EXIT         => { /* signal scheduler to clean up shard */ ESIGMA_OK }
        SYS_SIGMA_SHARD_SPAWN  => shard_spawn(args),
        SYS_SIGMA_SHARD_KILL   => shard_kill(a(0)),
        SYS_SIGMA_IPC_OPEN     => ipc_open(a(0) as SigmaU32, a(1) as SigmaU32, a(2)) as SigmaI64,
        SYS_SIGMA_IPC_SEND     => ipc_send(a(0) as SigmaU32, a(1) as *const u8) as SigmaI64,
        SYS_SIGMA_IPC_RECV     => ipc_recv(a(0) as SigmaU32, a(1) as *mut u8) as SigmaI64,
        SYS_SIGMA_IPC_CLOSE    => ipc_close(a(0) as SigmaU32) as SigmaI64,
        SYS_SIGMA_VFS_OPEN     => vfs_open(a(0) as *const u8, a(1) as SigmaU32, a(2) as SigmaU32) as SigmaI64,
        SYS_SIGMA_VFS_READ     => vfs_read(a(0) as SigmaI32, a(1) as *mut u8, a(2) as SigmaUsize),
        SYS_SIGMA_VFS_WRITE    => vfs_write(a(0) as SigmaI32, a(1) as *const u8, a(2) as SigmaUsize),
        SYS_SIGMA_VFS_CLOSE    => vfs_close(a(0) as SigmaI32) as SigmaI64,
        SYS_SIGMA_NET_SOCKET   => sigma_net_socket(a(0) as SigmaI32, a(1) as SigmaI32, a(2) as SigmaI32) as SigmaI64,
        SYS_SIGMA_MEM_ALLOC    => sigma_alloc_pages(a(0) as SigmaU32),
        SYS_SIGMA_MEM_FREE     => { sigma_free_pages(a(0), a(1) as SigmaU32); ESIGMA_OK }
        SYS_SIGMA_UPTIME       => kernel_uptime() as SigmaI64,
        SYS_SIGMA_AUDIT_LOG    => sigma_audit_append(a(0) as *const u8) as SigmaI64,
        SYS_SIGMA_SANDBOX_NEW  => sandbox_create(a(0) as *const u8) as SigmaI64,
        SYS_SIGMA_YIELD        => { ESIGMA_OK  /* scheduler handles actual yield */ }
        _                      => ESIGMA_NOSYS,
    }
}

// ─── Init / Process Queue ─────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn syscall_init() {
    SYSCALL_STATS.total_calls   = 0;
    SYSCALL_STATS.denied_calls  = 0;
    SYSCALL_STATS.invalid_calls = 0;
    for c in SYSCALL_STATS.per_call.iter_mut() { *c = 0; }
}

/// Drain any queued (deferred) syscall requests from a shard's pending list.
/// In production this is called by the scheduler between shard switches.
#[no_mangle]
pub unsafe extern "C" fn syscall_process_queue() {
    // Deferred-syscall queue processing would go here.
    // Phase 2: integrate with shard scheduler run-queue.
}

/// Read total syscall invocation count (for telemetry).
#[no_mangle]
pub unsafe extern "C" fn syscall_total_count() -> SigmaU64 {
    SYSCALL_STATS.total_calls
}

/// Read invocation count for a specific syscall number.
#[no_mangle]
pub unsafe extern "C" fn syscall_count_for(nr: SigmaU64) -> SigmaU64 {
    if nr >= SYS_SIGMA_MAX { return 0; }
    SYSCALL_STATS.per_call[nr as SigmaUsize]
}

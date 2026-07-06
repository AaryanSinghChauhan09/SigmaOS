/// SigmaOS — modules/core/kernel/kernel.rs
/// Sovereign Microkernel: bootstrap, state machine, subsystem wiring.
/// no_std | no alloc | no external crates | C-ABI exports.

#![no_std]
#![allow(dead_code)]
#![allow(unused_variables)]

// ─── Primitive Types ────────────────────────────────────────────────────────

type SigmaU8    = u8;
type SigmaU16   = u16;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaI64   = i64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Kernel Result ──────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum KernelError {
    Ok             =  0,
    InvalidArg     = -1,
    OutOfMemory    = -2,
    PermissionDeny = -3,
    NotFound       = -4,
    AlreadyExists  = -5,
    Timeout        = -6,
    HardwareFault  = -7,
}

pub type KernelResult = KernelError;

// ─── Kernel Boot Phase ───────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum BootPhase {
    PreInit       = 0,  // Before any subsystem is ready
    HalReady      = 1,  // HAL initialised (interrupts, paging)
    MemReady      = 2,  // Physical memory allocator online
    IpcReady      = 3,  // IPC channels initialised
    SecurityReady = 4,  // MAC / capabilities active
    FsReady       = 5,  // VFS mounted
    NetReady      = 6,  // Network stack online
    UserlandReady = 7,  // First user shard running
}

// ─── Kernel Global State ─────────────────────────────────────────────────────

#[repr(C)]
pub struct KernelState {
    pub boot_phase:        BootPhase,
    pub shard_count:       SigmaU32,
    pub free_pages:        SigmaU64,
    pub total_pages:       SigmaU64,
    pub uptime_ticks:      SigmaU64,
    pub panic_count:       SigmaU32,
    pub watchdog_enabled:  SigmaBool,
    pub mac_enabled:       SigmaBool,
    pub audit_enabled:     SigmaBool,
}

static mut KERNEL_STATE: KernelState = KernelState {
    boot_phase:       BootPhase::PreInit,
    shard_count:      0,
    free_pages:       0,
    total_pages:      0,
    uptime_ticks:     0,
    panic_count:      0,
    watchdog_enabled: false,
    mac_enabled:      false,
    audit_enabled:    false,
};

// ─── Subsystem Init Order ────────────────────────────────────────────────────

/// Phase table: each entry is (description, init_fn pointer, phase marker).
/// Executed sequentially by `kernel_main` during boot.
type InitFn = unsafe fn() -> KernelResult;

struct BootEntry {
    name:    &'static str,
    phase:   BootPhase,
    init_fn: InitFn,
}

extern "C" {
    fn hal_init()               -> KernelResult;
    fn res_alloc_init()         -> KernelResult;
    fn interrupts_init()        -> KernelResult;
    fn ipc_init()               -> KernelResult;
    fn init_security_isolation()-> KernelResult;
    fn vfs_init()               -> KernelResult;
    fn init_core_net()          -> KernelResult;
    fn watchdog_init()          -> KernelResult;
}

// ─── kernel_main ─────────────────────────────────────────────────────────────

/// Entry point called from the bootloader after paging is set up.
/// Runs the ordered boot sequence then drops into the idle loop.
#[no_mangle]
pub unsafe extern "C" fn kernel_main() {
    // Phase 1: HAL (interrupts, LAPIC, TSC)
    if hal_init() != KernelError::Ok {
        kernel_panic(b"HAL init failed\0".as_ptr());
        return;
    }
    KERNEL_STATE.boot_phase = BootPhase::HalReady;

    // Phase 2: Physical memory
    if res_alloc_init() != KernelError::Ok {
        kernel_panic(b"Memory init failed\0".as_ptr());
        return;
    }
    KERNEL_STATE.boot_phase = BootPhase::MemReady;

    // Phase 3: Interrupt descriptor table
    let _ = interrupts_init();

    // Phase 4: IPC
    if ipc_init() != KernelError::Ok {
        kernel_panic(b"IPC init failed\0".as_ptr());
        return;
    }
    KERNEL_STATE.boot_phase = BootPhase::IpcReady;

    // Phase 5: Security / sandbox
    let _ = init_security_isolation();
    KERNEL_STATE.mac_enabled    = true;
    KERNEL_STATE.audit_enabled  = true;
    KERNEL_STATE.boot_phase     = BootPhase::SecurityReady;

    // Phase 6: VFS
    if vfs_init() != KernelError::Ok {
        kernel_panic(b"VFS init failed\0".as_ptr());
        return;
    }
    KERNEL_STATE.boot_phase = BootPhase::FsReady;

    // Phase 7: Network
    let _ = init_core_net();
    KERNEL_STATE.boot_phase = BootPhase::NetReady;

    // Phase 8: Watchdog — arm before handing off to userland
    let _ = watchdog_init();
    KERNEL_STATE.watchdog_enabled = true;

    // Mark boot complete
    KERNEL_STATE.boot_phase = BootPhase::UserlandReady;

    // Idle loop — real systems hand off to the scheduler here
    idle_loop();
}

// ─── Idle Loop ───────────────────────────────────────────────────────────────

#[inline(never)]
unsafe fn idle_loop() -> ! {
    loop {
        KERNEL_STATE.uptime_ticks = KERNEL_STATE.uptime_ticks.wrapping_add(1);
        // HLT on x86_64 to yield until next interrupt
        #[cfg(target_arch = "x86_64")]
        core::arch::asm!("hlt");
    }
}

// ─── Kernel Panic ────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn kernel_panic(msg: *const SigmaU8) -> ! {
    KERNEL_STATE.panic_count = KERNEL_STATE.panic_count.wrapping_add(1);
    // Disable interrupts and halt all cores
    #[cfg(target_arch = "x86_64")]
    {
        core::arch::asm!("cli");
        loop { core::arch::asm!("hlt"); }
    }
    #[allow(unreachable_code)]
    loop {}
}

// ─── Exported Queries ────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn kernel_get_state() -> *const KernelState {
    &KERNEL_STATE as *const KernelState
}

#[no_mangle]
pub unsafe extern "C" fn kernel_uptime() -> SigmaU64 {
    KERNEL_STATE.uptime_ticks
}

#[no_mangle]
pub unsafe extern "C" fn kernel_shard_count() -> SigmaU32 {
    KERNEL_STATE.shard_count
}

#[no_mangle]
pub unsafe extern "C" fn kernel_inc_shard_count() {
    KERNEL_STATE.shard_count = KERNEL_STATE.shard_count.wrapping_add(1);
}

#[no_mangle]
pub unsafe extern "C" fn kernel_dec_shard_count() {
    if KERNEL_STATE.shard_count > 0 {
        KERNEL_STATE.shard_count -= 1;
    }
}

// ─── Scheduler hook ──────────────────────────────────────────────────────────

/// Called by the timer interrupt to increment the tick counter.
#[no_mangle]
pub unsafe extern "C" fn schedule() {
    KERNEL_STATE.uptime_ticks = KERNEL_STATE.uptime_ticks.wrapping_add(1);
}

/// Re-export the init entry point used by boot sequence validation.
#[no_mangle]
pub unsafe extern "C" fn init_core_kernel() -> KernelResult {
    KernelError::Ok
}

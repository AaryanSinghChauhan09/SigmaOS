/// SigmaOS: Sovereign Non-POSIX Syscall ABI and Dispatcher
/// Built in Rust — no_std, no alloc, no external dependencies.
/// Implements a fully custom syscall interface with OOP registration.

#![no_std]
#![allow(dead_code)]

type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;
type SigmaI64 = i64;
type SigmaBool = bool;
type SigmaUsize = usize;

// ─── Syscall Table Vectors ──────────────────────────────────────────────────
pub const SYS_SOVEREIGN_YIELD: SigmaU32  = 0x100;
pub const SYS_SOVEREIGN_ALLOC: SigmaU32  = 0x101;
pub const SYS_SOVEREIGN_FREE: SigmaU32   = 0x102;
pub const SYS_SOVEREIGN_IPC_SEND: SigmaU32 = 0x103;
pub const SYS_SOVEREIGN_IPC_RECV: SigmaU32 = 0x104;
pub const SYS_SOVEREIGN_SHARD_REG: SigmaU32 = 0x105;
pub const SYS_SOVEREIGN_AUDIT: SigmaU32    = 0x106;

// ─── Syscall Handler Trait (OOP Interface) ──────────────────────────────────
pub trait SyscallHandler {
    fn handle(&mut self, arg1: SigmaU64, arg2: SigmaU64, arg3: SigmaU64) -> SigmaI64;
    fn get_name(&self) -> &'static str;
}

// ─── Syscall Registry ───────────────────────────────────────────────────────
pub struct SyscallRegistry {
    handlers: [Option<&'static mut dyn SyscallHandler>; 256],
}

impl SyscallRegistry {
    pub const fn new() -> Self {
        Self {
            handlers: [None; 256],
        }
    }

    pub fn register(&mut self, vector: SigmaU32, handler: &'static mut dyn SyscallHandler) -> SigmaBool {
        let index = (vector & 0xFF) as usize;
        if index < 256 {
            self.handlers[index] = Some(handler);
            true
        } else {
            false
        }
    }

    pub fn dispatch(&mut self, vector: SigmaU32, arg1: SigmaU64, arg2: SigmaU64, arg3: SigmaU64) -> SigmaI64 {
        let index = (vector & 0xFF) as usize;
        if index < 256 {
            if let Some(ref mut handler) = self.handlers[index] {
                return handler.handle(arg1, arg2, arg3);
            }
        }
        -1 // Err: Invalid Syscall
    }
}

// ─── Default Handlers ───────────────────────────────────────────────────────
struct YieldHandler;
impl SyscallHandler for YieldHandler {
    fn handle(&mut self, _a1: SigmaU64, _a2: SigmaU64, _a3: SigmaU64) -> SigmaI64 {
        // Yield execution to the scheduler (simulated)
        0
    }
    fn get_name(&self) -> &'static str { "SYS_SOVEREIGN_YIELD" }
}

struct AuditHandler;
impl SyscallHandler for AuditHandler {
    fn handle(&mut self, event_id: SigmaU64, severity: SigmaU64, _a3: SigmaU64) -> SigmaI64 {
        // Log auditing info (simulated)
        (event_id + severity) as SigmaI64
    }
    fn get_name(&self) -> &'static str { "SYS_SOVEREIGN_AUDIT" }
}

// ─── Global State ──────────────────────────────────────────────────────────
static mut REGISTRY: SyscallRegistry = SyscallRegistry::new();
static mut YIELD_H: YieldHandler = YieldHandler;
static mut AUDIT_H: AuditHandler = AuditHandler;

// ─── Entry & Dispatch Bridge ────────────────────────────────────────────────
#[no_mangle]
pub unsafe extern "C" fn sovereign_abi_init() {
    REGISTRY.register(SYS_SOVEREIGN_YIELD, &mut YIELD_H);
    REGISTRY.register(SYS_SOVEREIGN_AUDIT, &mut AUDIT_H);
}

#[no_mangle]
pub unsafe extern "C" fn sovereign_syscall_dispatch(
    vector: SigmaU32,
    arg1: SigmaU64,
    arg2: SigmaU64,
    arg3: SigmaU64,
) -> SigmaI64 {
    REGISTRY.dispatch(vector, arg1, arg2, arg3)
}

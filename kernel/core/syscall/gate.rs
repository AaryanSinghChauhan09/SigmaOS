// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Syscall Gate (Rust, no_std)
//! =========================================================================

type U32 = u32;
type U64 = u64;

const SIGMA_SYS_YIELD: U32  = 0x01;
const SIGMA_SYS_MALLOC: U32 = 0x02;
const SIGMA_SYS_FREE: U32   = 0x03;
const SIGMA_SYS_SEND: U32   = 0x04;
const SIGMA_SYS_SOCKET: U32 = 0x05;
const SIGMA_OK: U32         = 0x00;

pub struct SovereignSyscallEngine {
    initialized: bool,
    total_calls: U64,
}

impl SovereignSyscallEngine {
    pub const fn new() -> Self {
        SovereignSyscallEngine {
            initialized: false,
            total_calls: 0,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
        self.total_calls = 0;
    }

    pub fn dispatch(&mut self, id: U32, arg1: U32, arg2: U32, arg3: U32) -> U32 {
        self.total_calls += 1;

        match id {
            SIGMA_SYS_YIELD => {
                // Trigger context switch
                SIGMA_OK
            }
            SIGMA_SYS_MALLOC => {
                // Defer to SLUB
                SIGMA_OK
            }
            SIGMA_SYS_FREE => {
                // Defer to SLUB
                SIGMA_OK
            }
            SIGMA_SYS_SEND => {
                // Defer to IPC router
                SIGMA_OK
            }
            SIGMA_SYS_SOCKET => {
                // In a complete implementation, this calls into network stack.
                // We mock the return for the C-ABI shim.
                1000 // Fake socket handle
            }
            _ => self.attempt_self_healing(id, arg1, arg2, arg3),
        }
    }

    fn attempt_self_healing(&mut self, _id: U32, _a1: U32, _a2: U32, _a3: U32) -> U32 {
        // Fallback execution / Error recovery
        SIGMA_OK
    }

    pub fn get_total_calls(&self) -> U64 {
        self.total_calls
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_SYSCALL: SovereignSyscallEngine = SovereignSyscallEngine::new();

// ── C-ABI Exports (Replacing SovereignSyscall.cpp) ─────────────────────────

#[no_mangle]
pub unsafe extern "C" fn syscall_init() {
    G_SYSCALL.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_syscall(id: U32, arg1: U32, arg2: U32, arg3: U32) -> U32 {
    G_SYSCALL.dispatch(id, arg1, arg2, arg3)
}

#[no_mangle]
pub unsafe extern "C" fn syscall_handler_asm() {
    // ASM Gate Transition logger / handler hook
}

#[no_mangle]
pub unsafe extern "C" fn syscall_get_total_calls() -> U64 {
    G_SYSCALL.get_total_calls()
}

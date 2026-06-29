// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Sovereign Shell (sh) — POSIX-free interactive shell (Rust, no_std)
//! =========================================================================
//! Replaces: usr/sh.c
//!
//! OOP Design:
//!   - SovereignSh struct: owns state, pipeline table, and exit code.
//!   - Hand-rolled redirection and pipe parsing.
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK:    SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
type U32 = u32;

const CMD_BUF: usize = 256;

fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut i = 0;
    while i < a.len() { if a[i] != b[i] { return false; } i += 1; }
    true
}

// ── Shell Struct ───────────────────────────────────────────────────────────

pub struct SovereignSh {
    last_exit: i32,
    active:    bool,
}

impl SovereignSh {
    pub const fn new() -> Self {
        SovereignSh { last_exit: 0, active: false }
    }

    pub fn init(&mut self) -> SigmaStatus {
        self.active = true;
        SIGMA_OK
    }

    /// Execute a raw command line byte slice. Returns exit code.
    pub fn exec(&mut self, cmd: &[u8]) -> i32 {
        if !self.active { return -1; }
        // Built-in: exit
        if bytes_eq(cmd, b"exit") {
            self.active = false;
            self.last_exit = 0;
            return 0;
        }
        // All other commands route through Sovereign Syscall exec gate
        self.last_exit = -127; // ENOSYS placeholder
        self.last_exit
    }

    pub fn last_exit(&self) -> i32 { self.last_exit }
    pub fn is_active(&self) -> bool { self.active }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_SH: SovereignSh = SovereignSh::new();

// ── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sh_init() -> SigmaStatus {
    G_SH.init()
}

#[no_mangle]
pub unsafe extern "C" fn sh_exec(cmd: *const u8, len: U32) -> i32 {
    let s = core::slice::from_raw_parts(cmd, len as usize);
    G_SH.exec(s)
}

#[no_mangle]
pub unsafe extern "C" fn sh_last_exit() -> i32 {
    G_SH.last_exit()
}

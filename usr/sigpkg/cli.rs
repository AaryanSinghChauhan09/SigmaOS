// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: sigpkg — CLI Interface (Rust, no_std)
//! =========================================================================
//!
//! CLI argument dispatch struct: SigPkgCli
//! Replaces: usr/sigma_pkg.c
//!
//! Commands:
//!   install  <name>   — Install a package
//!   remove   <name>   — Remove a package
//!   verify   <name>   — Verify package signature
//!   count             — Print number of installed packages
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK:    SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

type U32 = u32;

// ── Hand-rolled byte-slice comparison (no stdlib) ─────────────────────────

fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] { return false; }
        i += 1;
    }
    true
}

// ── CLI Struct (OOP) ───────────────────────────────────────────────────────

pub struct SigPkgCli {
    ready: bool,
}

impl SigPkgCli {
    pub const fn new() -> Self {
        SigPkgCli { ready: true }
    }

    /// Dispatch a CLI command.
    /// cmd  - null-free byte slice for the command verb
    /// arg  - optional argument (package name), may be empty
    /// Returns SIGMA_OK or SIGMA_ERROR
    pub fn dispatch(&self, cmd: &[u8], arg: &[u8]) -> SigmaStatus {
        if !self.ready { return SIGMA_ERROR; }

        if bytes_eq(cmd, b"install") {
            // Delegate to C-ABI (wires into G_PKG_MGR singleton)
            unsafe {
                let dummy_hash = [0u8; 32];
                crate::manager::sigpkg_install(
                    arg.as_ptr(),
                    arg.len() as U32,
                    0x0001_0000, // version 1.0.0
                    dummy_hash.as_ptr(),
                    1, // assume sig valid for now
                )
            }
        } else if bytes_eq(cmd, b"remove") {
            unsafe { crate::manager::sigpkg_remove(arg.as_ptr(), arg.len() as U32) }
        } else if bytes_eq(cmd, b"verify") {
            unsafe { crate::manager::sigpkg_verify(arg.as_ptr(), arg.len() as U32) }
        } else if bytes_eq(cmd, b"count") {
            let _n = unsafe { crate::manager::sigpkg_count() };
            SIGMA_OK
        } else if bytes_eq(cmd, b"list") {
            let mut names = [[0u8; 64]; 512];
            let _count = unsafe { crate::manager::sigpkg_list(names.as_mut_ptr()) };
            SIGMA_OK
        } else if bytes_eq(cmd, b"search") {
            let _count = unsafe { crate::manager::sigpkg_search(arg.as_ptr(), arg.len() as U32) };
            SIGMA_OK
        } else if bytes_eq(cmd, b"update") {
            unsafe { crate::manager::sigpkg_update(arg.as_ptr(), arg.len() as U32, 0x0002_0000) }
        } else {
            SIGMA_ERROR
        }
    }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_CLI: SigPkgCli = SigPkgCli::new();

// ── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigpkg_cli_dispatch(
    cmd:     *const u8,
    cmd_len: U32,
    arg:     *const u8,
    arg_len: U32,
) -> SigmaStatus {
    let cmd_slice = core::slice::from_raw_parts(cmd, cmd_len as usize);
    let arg_slice = core::slice::from_raw_parts(arg, arg_len as usize);
    G_CLI.dispatch(cmd_slice, arg_slice)
}

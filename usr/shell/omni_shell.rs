// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: OmniShell — Sovereign Interactive Shell (Rust, no_std)
//! =========================================================================
//! Replaces: usr/omni_shell.c  +  usr/omni_shell.cpp
//!
//! Design (OOP):
//!   - OmniShell struct owns a static line buffer and command dispatch table.
//!   - Commands are matched via hand-rolled byte comparison (no stdlib).
//!   - History ring-buffer (static, 64 entries).
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK:    SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;

type U32 = u32;

const LINE_BUF: usize  = 512;
const HIST_LEN: usize  = 64;
const MAX_ARGS: usize  = 32;

// ── Line / History Buffers ─────────────────────────────────────────────────

struct HistoryEntry {
    data: [u8; LINE_BUF],
    len:  usize,
}

impl HistoryEntry {
    const fn empty() -> Self {
        HistoryEntry { data: [0u8; LINE_BUF], len: 0 }
    }
}

// ── Byte-slice utilities (no stdlib) ──────────────────────────────────────

fn bytes_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() { return false; }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] { return false; }
        i += 1;
    }
    true
}

fn copy_bytes(dst: &mut [u8], src: &[u8]) -> usize {
    let n = if src.len() < dst.len() { src.len() } else { dst.len() };
    let mut i = 0;
    while i < n { dst[i] = src[i]; i += 1; }
    n
}

// ── Argument Tokeniser ────────────────────────────────────────────────────

struct ArgList<'a> {
    args: [&'a [u8]; MAX_ARGS],
    count: usize,
}

impl<'a> ArgList<'a> {
    fn parse(line: &'a [u8]) -> Self {
        let mut al = ArgList { args: [&[]; MAX_ARGS], count: 0 };
        let mut start = 0;
        let mut in_word = false;
        let mut i = 0;
        while i <= line.len() {
            let ch = if i < line.len() { line[i] } else { b' ' };
            if ch == b' ' || ch == b'\t' || ch == b'\n' || ch == b'\0' {
                if in_word && al.count < MAX_ARGS {
                    al.args[al.count] = &line[start..i];
                    al.count += 1;
                    in_word = false;
                }
            } else {
                if !in_word { start = i; in_word = true; }
            }
            i += 1;
        }
        al
    }
}

// ── OmniShell Struct ───────────────────────────────────────────────────────

pub struct OmniShell {
    line_buf:  [u8; LINE_BUF],
    line_len:  usize,
    history:   [HistoryEntry; HIST_LEN],
    hist_head: usize,
    hist_count: usize,
    running:   bool,
}

impl OmniShell {
    pub const fn new() -> Self {
        const EMPTY_HIST: HistoryEntry = HistoryEntry::empty();
        OmniShell {
            line_buf:   [0u8; LINE_BUF],
            line_len:   0,
            history:    [EMPTY_HIST; HIST_LEN],
            hist_head:  0,
            hist_count: 0,
            running:    false,
        }
    }

    pub fn start(&mut self) -> SigmaStatus {
        self.running = true;
        SIGMA_OK
    }

    /// Feed a line of input (byte slice) to the shell.
    pub fn feed_line(&mut self, line: &[u8]) -> SigmaStatus {
        if !self.running { return SIGMA_ERROR; }
        // Store in history
        let slot = self.hist_head % HIST_LEN;
        self.history[slot].len = copy_bytes(&mut self.history[slot].data, line);
        self.hist_head = self.hist_head.wrapping_add(1);
        if self.hist_count < HIST_LEN { self.hist_count += 1; }
        // Dispatch
        self.dispatch(line)
    }

    fn dispatch(&mut self, line: &[u8]) -> SigmaStatus {
        let args = ArgList::parse(line);
        if args.count == 0 { return SIGMA_OK; }
        let cmd = args.args[0];
        if bytes_eq(cmd, b"exit") || bytes_eq(cmd, b"quit") {
            self.running = false;
            SIGMA_OK
        } else if bytes_eq(cmd, b"history") {
            // History listing — output handled by caller via get_history()
            SIGMA_OK
        } else if bytes_eq(cmd, b"clear") {
            SIGMA_OK
        } else {
            // Delegate to Sovereign Exec syscall (no posix)
            SIGMA_ERROR
        }
    }

    pub fn is_running(&self) -> bool { self.running }
    pub fn history_count(&self) -> usize { self.hist_count }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_OMNI_SHELL: OmniShell = OmniShell::new();

// ── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn omni_shell_start() -> SigmaStatus {
    G_OMNI_SHELL.start()
}

#[no_mangle]
pub unsafe extern "C" fn omni_shell_feed(line: *const u8, len: U32) -> SigmaStatus {
    let s = core::slice::from_raw_parts(line, len as usize);
    G_OMNI_SHELL.feed_line(s)
}

#[no_mangle]
pub unsafe extern "C" fn omni_shell_running() -> u8 {
    G_OMNI_SHELL.is_running() as u8
}

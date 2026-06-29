// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SIGMAOS: Log Daemon (logd) — Sovereign Structured Logging (Rust, no_std)
//! =========================================================================
//! Replaces: usr/logd.cpp
//!
//! OOP Design:
//!   - LogDaemon struct: circular static log ring (no heap).
//!   - Severity-based filtering (DEBUG, INFO, WARN, ERROR, CRIT).
//!   - Outputs via Sovereign Syscall write gate (no libc write()).
//! =========================================================================

pub type SigmaStatus = i32;
pub const SIGMA_OK:    SigmaStatus = 0;
pub const SIGMA_ERROR: SigmaStatus = -1;
type U32 = u32;

const LOG_MSG_LEN: usize = 256;
const LOG_RING:    usize = 512;

// ── Log Level ─────────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum LogLevel {
    Debug = 0,
    Info  = 1,
    Warn  = 2,
    Error = 3,
    Crit  = 4,
}

// ── Log Entry ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
struct LogEntry {
    level: LogLevel,
    msg:   [u8; LOG_MSG_LEN],
    len:   usize,
    seq:   U32,
}

impl LogEntry {
    const fn empty() -> Self {
        LogEntry {
            level: LogLevel::Debug,
            msg:   [0u8; LOG_MSG_LEN],
            len:   0,
            seq:   0,
        }
    }
}

// ── LogDaemon Struct ───────────────────────────────────────────────────────

pub struct LogDaemon {
    ring:      [LogEntry; LOG_RING],
    head:      usize,
    count:     usize,
    seq:       U32,
    min_level: LogLevel,
    active:    bool,
}

impl LogDaemon {
    pub const fn new() -> Self {
        const E: LogEntry = LogEntry::empty();
        LogDaemon {
            ring:      [E; LOG_RING],
            head:      0,
            count:     0,
            seq:       0,
            min_level: LogLevel::Info,
            active:    false,
        }
    }

    pub fn start(&mut self) -> SigmaStatus {
        self.active = true;
        SIGMA_OK
    }

    pub fn set_min_level(&mut self, level: LogLevel) {
        self.min_level = level;
    }

    fn copy_msg(dst: &mut [u8; LOG_MSG_LEN], src: &[u8]) -> usize {
        let n = if src.len() < LOG_MSG_LEN { src.len() } else { LOG_MSG_LEN - 1 };
        let mut i = 0;
        while i < n { dst[i] = src[i]; i += 1; }
        n
    }

    /// Write a log message.
    pub fn log(&mut self, level: LogLevel, msg: &[u8]) -> SigmaStatus {
        if !self.active { return SIGMA_ERROR; }
        if (level as u8) < (self.min_level as u8) { return SIGMA_OK; }

        let slot = self.head % LOG_RING;
        self.ring[slot].level = level;
        self.ring[slot].len   = Self::copy_msg(&mut self.ring[slot].msg, msg);
        self.ring[slot].seq   = self.seq;
        self.seq = self.seq.wrapping_add(1);
        self.head = self.head.wrapping_add(1);
        if self.count < LOG_RING { self.count += 1; }
        SIGMA_OK
    }

    pub fn entry_count(&self) -> usize { self.count }
}

// ── Global Singleton ───────────────────────────────────────────────────────
static mut G_LOGD: LogDaemon = LogDaemon::new();

// ── C-ABI Exports ──────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn logd_start() -> SigmaStatus {
    G_LOGD.start()
}

#[no_mangle]
pub unsafe extern "C" fn logd_write(level: u8, msg: *const u8, len: U32) -> SigmaStatus {
    let lv = match level {
        0 => LogLevel::Debug,
        1 => LogLevel::Info,
        2 => LogLevel::Warn,
        3 => LogLevel::Error,
        _ => LogLevel::Crit,
    };
    let s = core::slice::from_raw_parts(msg, len as usize);
    G_LOGD.log(lv, s)
}

#[no_mangle]
pub unsafe extern "C" fn logd_count() -> U32 {
    G_LOGD.entry_count() as U32
}

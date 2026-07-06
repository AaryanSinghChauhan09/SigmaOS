// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_logrotate — Sovereign Log Rotation
//! Size/time-based rotation policies with compression and retention.
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;

pub const LOGROTATE_MAX_RULES: usize = 32;
pub const LOGROTATE_NAME_LEN:  usize = 64;
pub const LOGROTATE_PATH_LEN:  usize = 256;

// ─── Rotation Trigger ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum RotateTrigger {
    Size       = 0,   // rotate when file exceeds max_size_kb
    Daily      = 1,
    Weekly     = 2,
    Monthly    = 3,
    Hourly     = 4,
}

/// Compression method for rotated logs
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum CompressMethod {
    None       = 0,
    Gzip       = 1,
    Zstd       = 2,
    Xz         = 3,
    Lz4        = 4,
}

/// A logrotate rule for one log file or pattern
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LogRotateRule {
    pub name:          [u8; LOGROTATE_NAME_LEN],
    pub path:          [u8; LOGROTATE_PATH_LEN],
    pub trigger:       RotateTrigger,
    pub max_size_kb:   SigmaU64,       // for Size trigger
    pub rotate_count:  SigmaU32,       // keep this many old copies
    pub compress:      CompressMethod,
    pub delay_compress: SigmaBool,     // compress on next rotation, not immediately
    pub copy_truncate: SigmaBool,      // copy then truncate (for live files)
    pub missing_ok:    SigmaBool,      // don't error if file doesn't exist
    pub not_if_empty:  SigmaBool,      // skip if file is empty
    pub create_mode:   SigmaU32,       // mode for new log file (e.g., 0o644)
    pub create_uid:    SigmaU32,
    pub create_gid:    SigmaU32,
    pub post_rotate:   SigmaU64,       // fn() callback after rotation
    pub active:        SigmaBool,
    // State tracking
    pub current_size_kb: SigmaU64,
    pub last_rotated:    SigmaU64,     // timestamp of last rotation
    pub rotation_count:  SigmaU32,     // how many times rotated
}

impl LogRotateRule {
    pub const fn empty() -> Self {
        Self {
            name: [0u8; LOGROTATE_NAME_LEN],
            path: [0u8; LOGROTATE_PATH_LEN],
            trigger: RotateTrigger::Size,
            max_size_kb: 10240,   // 10 MB default
            rotate_count: 5,
            compress: CompressMethod::Gzip,
            delay_compress: false,
            copy_truncate: false,
            missing_ok: true,
            not_if_empty: true,
            create_mode: 0o644,
            create_uid: 0,
            create_gid: 0,
            post_rotate: 0,
            active: false,
            current_size_kb: 0,
            last_rotated: 0,
            rotation_count: 0,
        }
    }
}

/// Rotation event record
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RotationEvent {
    pub rule_idx:     SigmaU32,
    pub timestamp:    SigmaU64,
    pub old_size_kb:  SigmaU64,
    pub compressed:   SigmaBool,
    pub files_removed: SigmaU32,  // old files pruned by retention
}

// ─── Global State ───────────────────────────────────────────────────────────

struct LogRotateState {
    rules:       [LogRotateRule; LOGROTATE_MAX_RULES],
    rule_count:  SigmaU32,
    initialized: SigmaBool,
}

static mut LOGROTATE: LogRotateState = LogRotateState {
    rules:       [LogRotateRule::empty(); LOGROTATE_MAX_RULES],
    rule_count:  0,
    initialized: false,
};

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn lr_strncpy(dst: *mut u8, src: *const u8, n: usize) {
    let mut i = 0;
    while i < n { let b = *src.add(i); *dst.add(i) = b; if b == 0 { return; } i += 1; }
    if n > 0 { *dst.add(n - 1) = 0; }
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Initialize logrotate subsystem
#[no_mangle]
pub unsafe extern "C" fn sigma_logrotate_init() -> SigmaI32 {
    let lr = &mut LOGROTATE;
    lr.rule_count  = 0;
    lr.initialized = true;
    0
}

/// Add a logrotate rule
#[no_mangle]
pub unsafe extern "C" fn sigma_logrotate_add_rule(
    name:          *const u8,
    path:          *const u8,
    trigger:       RotateTrigger,
    max_size_kb:   SigmaU64,
    rotate_count:  SigmaU32,
    compress:      CompressMethod,
    copy_truncate: SigmaBool,
) -> SigmaI32 {
    let lr = &mut LOGROTATE;
    if lr.rule_count as usize >= LOGROTATE_MAX_RULES { return -1; }

    let idx = lr.rule_count as usize;
    let r = &mut lr.rules[idx];
    *r = LogRotateRule::empty();
    lr_strncpy(r.name.as_mut_ptr(), name, LOGROTATE_NAME_LEN);
    lr_strncpy(r.path.as_mut_ptr(), path, LOGROTATE_PATH_LEN);
    r.trigger       = trigger;
    r.max_size_kb   = max_size_kb;
    r.rotate_count  = rotate_count;
    r.compress      = compress;
    r.copy_truncate = copy_truncate;
    r.active        = true;

    lr.rule_count += 1;
    idx as SigmaI32
}

/// Check if a rule should trigger rotation based on current state
unsafe fn should_rotate(rule: &LogRotateRule, current_time: SigmaU64) -> bool {
    if !rule.active { return false; }

    match rule.trigger {
        RotateTrigger::Size => {
            rule.current_size_kb >= rule.max_size_kb
        }
        RotateTrigger::Daily => {
            current_time.saturating_sub(rule.last_rotated) >= 86400
        }
        RotateTrigger::Weekly => {
            current_time.saturating_sub(rule.last_rotated) >= 604800
        }
        RotateTrigger::Monthly => {
            current_time.saturating_sub(rule.last_rotated) >= 2592000
        }
        RotateTrigger::Hourly => {
            current_time.saturating_sub(rule.last_rotated) >= 3600
        }
    }
}

/// Execute rotation for a single rule
#[no_mangle]
pub unsafe extern "C" fn sigma_logrotate_rotate(
    rule_idx:     SigmaU32,
    current_time: SigmaU64,
) -> SigmaI32 {
    let lr = &mut LOGROTATE;
    if rule_idx >= lr.rule_count { return -1; }

    let r = &mut lr.rules[rule_idx as usize];
    if !r.active { return -2; }

    // In real implementation:
    // 1. Check not_if_empty: skip if file is empty
    // 2. If copy_truncate: copy file content, then truncate original
    //    else: rename log -> log.1, log.1 -> log.2, etc.
    // 3. Prune old rotated files beyond rotate_count
    // 4. If compress && !delay_compress: compress the just-rotated file
    // 5. If delay_compress: compress the previously-rotated file
    // 6. Create new empty log file with create_mode/uid/gid
    // 7. Call post_rotate callback if set
    // 8. Update state

    r.rotation_count += 1;
    r.last_rotated    = current_time;
    r.current_size_kb = 0;

    0
}

/// Run all logrotate rules (called periodically by cron/timer)
#[no_mangle]
pub unsafe extern "C" fn sigma_logrotate_run_all(current_time: SigmaU64) -> SigmaI32 {
    let lr = &LOGROTATE;
    let count = lr.rule_count;
    let mut rotated = 0i32;

    for i in 0..count {
        if should_rotate(&LOGROTATE.rules[i as usize], current_time) {
            if sigma_logrotate_rotate(i, current_time) == 0 {
                rotated += 1;
            }
        }
    }
    rotated
}

/// Update the current file size for a rule (called by VFS write hooks)
#[no_mangle]
pub unsafe extern "C" fn sigma_logrotate_update_size(
    rule_idx:  SigmaU32,
    size_kb:   SigmaU64,
) -> SigmaI32 {
    let lr = &mut LOGROTATE;
    if rule_idx >= lr.rule_count { return -1; }
    lr.rules[rule_idx as usize].current_size_kb = size_kb;
    0
}

/// Get the status of a logrotate rule
#[no_mangle]
pub unsafe extern "C" fn sigma_logrotate_status(
    rule_idx:       SigmaU32,
    rotation_count: *mut SigmaU32,
    last_rotated:   *mut SigmaU64,
    current_size:   *mut SigmaU64,
) -> SigmaBool {
    let lr = &LOGROTATE;
    if rule_idx >= lr.rule_count { return false; }
    let r = &lr.rules[rule_idx as usize];
    *rotation_count = r.rotation_count;
    *last_rotated   = r.last_rotated;
    *current_size   = r.current_size_kb;
    true
}

/// Get total rule count
#[no_mangle]
pub unsafe extern "C" fn sigma_logrotate_rule_count() -> SigmaU32 {
    unsafe { LOGROTATE.rule_count }
}

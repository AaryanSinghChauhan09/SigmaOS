// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS sigma_dmesg — Sovereign Kernel Ring Buffer
//! Fixed-size ring buffer for kernel log messages with severity levels.
//! no_std, no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ──────────────────────────────────────────────────────────────
pub const DMESG_RING_SIZE:  usize = 1024;  // number of log entries
pub const DMESG_MSG_LEN:    usize = 256;   // max message length per entry
pub const DMESG_FACILITY_LEN: usize = 16;

// ─── Log Levels (matching syslog/kernel priorities) ─────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum LogLevel {
    Emergency = 0,   // KERN_EMERG   — system is unusable
    Alert     = 1,   // KERN_ALERT   — action must be taken immediately
    Critical  = 2,   // KERN_CRIT    — critical conditions
    Error     = 3,   // KERN_ERR     — error conditions
    Warning   = 4,   // KERN_WARNING — warning conditions
    Notice    = 5,   // KERN_NOTICE  — normal but significant
    Info      = 6,   // KERN_INFO    — informational
    Debug     = 7,   // KERN_DEBUG   — debug-level messages
}

/// Log facility (syslog-style)
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum LogFacility {
    Kernel    = 0,
    UserSpace = 1,
    Driver    = 2,
    Network   = 3,
    FileSystem = 4,
    Security  = 5,
    Boot      = 6,
    Scheduler = 7,
    Memory    = 8,
    Ipc       = 9,
}

/// A single kernel log message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct DmesgEntry {
    pub seqnum:    SigmaU64,            // monotonic sequence number
    pub timestamp: SigmaU64,            // microseconds since boot
    pub level:     LogLevel,
    pub facility:  LogFacility,
    pub source:    [u8; DMESG_FACILITY_LEN], // subsystem name (e.g., "pci", "net")
    pub message:   [u8; DMESG_MSG_LEN],
    pub msg_len:   SigmaU32,
    pub valid:     SigmaBool,
}

impl DmesgEntry {
    pub const fn empty() -> Self {
        Self {
            seqnum:    0,
            timestamp: 0,
            level:     LogLevel::Info,
            facility:  LogFacility::Kernel,
            source:    [0u8; DMESG_FACILITY_LEN],
            message:   [0u8; DMESG_MSG_LEN],
            msg_len:   0,
            valid:     false,
        }
    }
}

// ─── Ring Buffer State ──────────────────────────────────────────────────────

struct DmesgRingBuffer {
    entries:     [DmesgEntry; DMESG_RING_SIZE],
    head:        SigmaU32,        // oldest entry
    tail:        SigmaU32,        // next write position
    count:       SigmaU32,        // total entries (capped at RING_SIZE)
    total_written: SigmaU64,      // total messages ever logged
    next_seqnum: SigmaU64,
    console_level: LogLevel,      // messages <= this level go to console
    initialized: SigmaBool,
}

static mut DMESG: DmesgRingBuffer = DmesgRingBuffer {
    entries:       [DmesgEntry::empty(); DMESG_RING_SIZE],
    head:          0,
    tail:          0,
    count:         0,
    total_written: 0,
    next_seqnum:   1,
    console_level: LogLevel::Warning,
    initialized:   false,
};

// ─── Helpers ────────────────────────────────────────────────────────────────

unsafe fn dmesg_strncpy(dst: *mut u8, src: *const u8, n: usize) -> SigmaU32 {
    let mut i = 0u32;
    while (i as usize) < n {
        let b = *src.add(i as usize);
        *dst.add(i as usize) = b;
        if b == 0 { return i; }
        i += 1;
    }
    if n > 0 { *dst.add(n - 1) = 0; }
    i
}

// ─── C-ABI Exports ──────────────────────────────────────────────────────────

/// Initialize the kernel ring buffer
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_init() -> SigmaI32 {
    let d = &mut DMESG;
    d.head          = 0;
    d.tail          = 0;
    d.count         = 0;
    d.total_written = 0;
    d.next_seqnum   = 1;
    d.console_level = LogLevel::Warning;
    d.initialized   = true;

    // Log our own initialization
    sigma_klog(LogLevel::Info, LogFacility::Kernel,
        b"kernel\0".as_ptr(), b"SigmaOS kernel ring buffer initialized\0".as_ptr());
    0
}

/// Log a kernel message (primary logging entry point)
#[no_mangle]
pub unsafe extern "C" fn sigma_klog(
    level:    LogLevel,
    facility: LogFacility,
    source:   *const u8,
    message:  *const u8,
) {
    let d = &mut DMESG;
    if !d.initialized { return; }

    let idx = (d.tail % DMESG_RING_SIZE as u32) as usize;

    d.entries[idx].seqnum    = d.next_seqnum;
    d.entries[idx].timestamp = 0; // filled by arch timer in real impl
    d.entries[idx].level     = level;
    d.entries[idx].facility  = facility;
    d.entries[idx].valid     = true;

    dmesg_strncpy(d.entries[idx].source.as_mut_ptr(), source, DMESG_FACILITY_LEN);
    let len = dmesg_strncpy(d.entries[idx].message.as_mut_ptr(), message, DMESG_MSG_LEN);
    d.entries[idx].msg_len = len;

    d.next_seqnum   += 1;
    d.total_written += 1;
    d.tail = d.tail.wrapping_add(1);

    if d.count < DMESG_RING_SIZE as u32 {
        d.count += 1;
    } else {
        d.head = d.head.wrapping_add(1); // drop oldest
    }

    // If level <= console_level, would write to console in real impl
}

/// Convenience wrappers for each log level
#[no_mangle]
pub unsafe extern "C" fn sigma_klog_emerg(source: *const u8, msg: *const u8) {
    sigma_klog(LogLevel::Emergency, LogFacility::Kernel, source, msg);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_klog_err(source: *const u8, msg: *const u8) {
    sigma_klog(LogLevel::Error, LogFacility::Kernel, source, msg);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_klog_warn(source: *const u8, msg: *const u8) {
    sigma_klog(LogLevel::Warning, LogFacility::Kernel, source, msg);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_klog_info(source: *const u8, msg: *const u8) {
    sigma_klog(LogLevel::Info, LogFacility::Kernel, source, msg);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_klog_debug(source: *const u8, msg: *const u8) {
    sigma_klog(LogLevel::Debug, LogFacility::Kernel, source, msg);
}

/// Read all entries (dmesg command equivalent)
/// Returns number of entries written to `out`
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_read_all(
    out: *mut DmesgEntry,
    max: SigmaU32,
) -> SigmaU32 {
    let d = &DMESG;
    if !d.initialized { return 0; }

    let mut count = 0u32;
    let total = if d.count < max { d.count } else { max };

    for i in 0..total {
        let idx = ((d.head + i) % DMESG_RING_SIZE as u32) as usize;
        if d.entries[idx].valid {
            *out.add(count as usize) = d.entries[idx];
            count += 1;
        }
    }
    count
}

/// Read entries filtered by log level (only messages <= specified level)
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_read_level(
    max_level: LogLevel,
    out:       *mut DmesgEntry,
    max:       SigmaU32,
) -> SigmaU32 {
    let d = &DMESG;
    if !d.initialized { return 0; }

    let mut count = 0u32;
    for i in 0..d.count {
        if count >= max { break; }
        let idx = ((d.head + i) % DMESG_RING_SIZE as u32) as usize;
        if d.entries[idx].valid && (d.entries[idx].level as u8) <= (max_level as u8) {
            *out.add(count as usize) = d.entries[idx];
            count += 1;
        }
    }
    count
}

/// Read entries filtered by facility
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_read_facility(
    facility: LogFacility,
    out:      *mut DmesgEntry,
    max:      SigmaU32,
) -> SigmaU32 {
    let d = &DMESG;
    if !d.initialized { return 0; }

    let mut count = 0u32;
    for i in 0..d.count {
        if count >= max { break; }
        let idx = ((d.head + i) % DMESG_RING_SIZE as u32) as usize;
        if d.entries[idx].valid && d.entries[idx].facility == facility {
            *out.add(count as usize) = d.entries[idx];
            count += 1;
        }
    }
    count
}

/// Read entries since a specific sequence number (for tailing)
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_read_since(
    since_seqnum: SigmaU64,
    out:          *mut DmesgEntry,
    max:          SigmaU32,
) -> SigmaU32 {
    let d = &DMESG;
    if !d.initialized { return 0; }

    let mut count = 0u32;
    for i in 0..d.count {
        if count >= max { break; }
        let idx = ((d.head + i) % DMESG_RING_SIZE as u32) as usize;
        if d.entries[idx].valid && d.entries[idx].seqnum > since_seqnum {
            *out.add(count as usize) = d.entries[idx];
            count += 1;
        }
    }
    count
}

/// Clear the ring buffer (dmesg -C)
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_clear() {
    let d = &mut DMESG;
    d.head  = 0;
    d.tail  = 0;
    d.count = 0;
    // Don't reset total_written or next_seqnum
}

/// Set the console log level
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_set_console_level(level: LogLevel) {
    DMESG.console_level = level;
}

/// Get current console log level
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_get_console_level() -> LogLevel {
    unsafe { DMESG.console_level }
}

/// Get buffer statistics
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_stats(
    count:         *mut SigmaU32,
    total_written: *mut SigmaU64,
    next_seqnum:   *mut SigmaU64,
) {
    let d = &DMESG;
    *count         = d.count;
    *total_written = d.total_written;
    *next_seqnum   = d.next_seqnum;
}

/// Get the ring buffer capacity
#[no_mangle]
pub unsafe extern "C" fn sigma_dmesg_capacity() -> SigmaU32 {
    DMESG_RING_SIZE as SigmaU32
}

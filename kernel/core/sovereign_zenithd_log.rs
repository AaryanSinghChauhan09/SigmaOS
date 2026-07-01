// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Logging Daemon (Rust, no_std)
//! Replaces: kernel/core/sigma_zenithd_log.c
//! OOP Principles: Structs, methods, traits, encapsulation, type safety.
//! Zero libraries, zero predefined functions.
//! =========================================================================

#![no_std]

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use core::cell::UnsafeCell;

// ---- Severity Levels ----
pub const ZEN_TRACE: u32 = 0;
pub const ZEN_DEBUG: u32 = 1;
pub const ZEN_INFO: u32 = 2;
pub const ZEN_WARN: u32 = 3;
pub const ZEN_ERROR: u32 = 4;
pub const ZEN_CRIT: u32 = 5;
pub const ZEN_PANIC: u32 = 6;

const SEVERITY_NAMES: [&str; 7] = ["TRACE", "DEBUG", "INFO ", "WARN ", "ERROR", "CRIT ", "PANIC"];

// ---- Configuration ----
pub const ZENITH_LOG_RING_SIZE: usize = 4096; // Must be power of 2
pub const ZENITH_LOG_MSG_LEN: usize = 112;
pub const ZENITH_LOG_COMP_LEN: usize = 16;

// ---- Log Entry ----
#[repr(C)]
pub struct zenith_log_entry_t {
    pub timestamp_tsc: u64,
    pub severity: u32,
    pub error_code: u32,
    pub cpu_id: u32,
    pub correlation_id: u32,
    pub component: [u8; ZENITH_LOG_COMP_LEN],
    pub message: [u8; ZENITH_LOG_MSG_LEN],
}

// ---- Statistics ----
#[repr(C)]
pub struct zenith_log_stats_t {
    pub total_emitted: u64,
    pub total_dropped: u64,
    pub ring_wraps: u64,
    pub current_severity: u32,
    pub ring_write_idx: u32,
}

// ---- C External Symbols ----
extern "C" {
    pub fn serial_puts(s: *const u8);
    pub fn serial_putc(c: u8);
    pub fn cpu_rdtsc() -> u64;
    pub fn cpu_get_id() -> u32;
}

// ---- OOP: Base Trait for Logs ----
pub trait LoggingEngine {
    fn init(&mut self);
    fn emit(&mut self, severity: u32, error_code: u32, component: *const u8, message: *const u8, cid: u32);
    fn set_severity(&mut self, min_severity: u32);
    fn get_stats(&self) -> zenith_log_stats_t;
    fn get_entry(&self, index: u32) -> *const zenith_log_entry_t;
    fn dump(&self);
}

// ---- OOP: Logger Class Struct ----
pub struct ZenithLogger {
    ring: [zenith_log_entry_t; ZENITH_LOG_RING_SIZE],
    write_idx: AtomicU32,
    min_severity: AtomicU32,
    total_emitted: AtomicU64,
    total_dropped: AtomicU64,
    ring_wraps: AtomicU64,
    initialized: bool,
}

// Helper to copy C string into fixed-size byte array safely (equivalent to strcpy_bounded)
fn copy_c_str(dest: &mut [u8], src: *const u8) {
    if src.is_null() {
        dest[0] = 0;
        return;
    }
    let mut i = 0;
    let limit = dest.len() - 1;
    unsafe {
        while i < limit && *src.add(i) != 0 {
            dest[i] = *src.add(i);
            i += 1;
        }
    }
    dest[i] = 0;
}

impl ZenithLogger {
    pub const fn new() -> Self {
        // Rust const initializer trick for large arrays
        // Since we can't easily initialize a 4096-sized array of non-Copy or complex structs,
        // we utilize a transparent zeroed array via unsafe transmute or raw initialization helper.
        // For freestanding simplicity and compile safety:
        let entry_template = zenith_log_entry_t {
            timestamp_tsc: 0,
            severity: 0,
            error_code: 0,
            cpu_id: 0,
            correlation_id: 0,
            component: [0; ZENITH_LOG_COMP_LEN],
            message: [0; ZENITH_LOG_MSG_LEN],
        };
        
        Self {
            ring: [entry_template; ZENITH_LOG_RING_SIZE],
            write_idx: AtomicU32::new(0),
            min_severity: AtomicU32::new(ZEN_INFO),
            total_emitted: AtomicU64::new(0),
            total_dropped: AtomicU64::new(0),
            ring_wraps: AtomicU64::new(0),
            initialized: false,
        }
    }
}

// Implement the LoggingEngine trait for our OOP class ZenithLogger
impl LoggingEngine for ZenithLogger {
    fn init(&mut self) {
        // Zero all values
        for i in 0..ZENITH_LOG_RING_SIZE {
            self.ring[i].timestamp_tsc = 0;
            self.ring[i].severity = 0;
            self.ring[i].error_code = 0;
            self.ring[i].cpu_id = 0;
            self.ring[i].correlation_id = 0;
            self.ring[i].component = [0; ZENITH_LOG_COMP_LEN];
            self.ring[i].message = [0; ZENITH_LOG_MSG_LEN];
        }
        self.write_idx.store(0, Ordering::SeqCst);
        self.min_severity.store(ZEN_INFO, Ordering::SeqCst);
        self.total_emitted.store(0, Ordering::SeqCst);
        self.total_dropped.store(0, Ordering::SeqCst);
        self.ring_wraps.store(0, Ordering::SeqCst);
        self.initialized = true;
    }

    fn emit(&mut self, severity: u32, error_code: u32, component: *const u8, message: *const u8, cid: u32) {
        let min_sev = self.min_severity.load(Ordering::Relaxed);
        if severity < min_sev {
            self.total_dropped.fetch_add(1, Ordering::Relaxed);
            return;
        }

        // Claim slot in the ring buffer using atomic CAS loop
        let mut idx = self.write_idx.load(Ordering::Relaxed);
        let mut next;
        loop {
            next = (idx + 1) & (ZENITH_LOG_RING_SIZE as u32 - 1);
            match self.write_idx.compare_exchange_weak(idx, next, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => break,
                Err(actual) => idx = actual,
            }
        }

        // Track wrapping
        if next == 0 {
            self.ring_wraps.fetch_add(1, Ordering::Relaxed);
        }

        // Fill entry in buffer
        let entry = &mut self.ring[idx as usize];
        entry.timestamp_tsc = unsafe { cpu_rdtsc() };
        entry.severity = severity;
        entry.error_code = error_code;
        entry.cpu_id = unsafe { cpu_get_id() };
        entry.correlation_id = cid;
        copy_c_str(&mut entry.component, component);
        copy_c_str(&mut entry.message, message);

        self.total_emitted.fetch_add(1, Ordering::Relaxed);

        // Immediate high-severity serial mirroring (WARN+)
        if severity >= ZEN_WARN {
            unsafe {
                serial_puts(b"[TSC:\x00".as_ptr());
                // Print TSC (upper 32-bits hex for speed/simplicity)
                let tsc_hi = (entry.timestamp_tsc >> 32) as u32;
                let mut hex_buf = [0u8; 9];
                let hex_chars = b"0123456789abcdef";
                let mut val = tsc_hi;
                for i in (0..8).rev() {
                    hex_buf[i] = hex_chars[(val & 0xF) as usize];
                    val >>= 4;
                }
                hex_buf[8] = 0;
                serial_puts(hex_buf.as_ptr());

                serial_puts(b"] [CPU:\x00".as_ptr());
                serial_putc(b'0' + (entry.cpu_id % 10) as u8);
                serial_puts(b"] [\x00".as_ptr());
                let sev_idx = if severity <= ZEN_PANIC { severity as usize } else { ZEN_PANIC as usize };
                
                // Print severity name by casting to C string (requires raw pointers)
                let name = SEVERITY_NAMES[sev_idx];
                let mut name_buf = [0u8; 8];
                let mut name_i = 0;
                for b in name.bytes() {
                    if name_i < 7 {
                        name_buf[name_i] = b;
                        name_i += 1;
                    }
                }
                name_buf[name_i] = 0;
                serial_puts(name_buf.as_ptr());

                serial_puts(b"] \x00".as_ptr());

                if error_code != 0 {
                    serial_puts(b"[0x\x00".as_ptr());
                    let mut code_buf = [0u8; 9];
                    let mut ec = error_code;
                    for i in (0..8).rev() {
                        code_buf[i] = hex_chars[(ec & 0xF) as usize];
                        ec >>= 4;
                    }
                    code_buf[8] = 0;
                    serial_puts(code_buf.as_ptr());
                    serial_puts(b"] \x00".as_ptr());
                }

                serial_puts(b"[\x00".as_ptr());
                serial_puts(entry.component.as_ptr());
                serial_puts(b"] \x00".as_ptr());
                serial_puts(entry.message.as_ptr());
                serial_puts(b"\n\x00".as_ptr());
            }
        }
    }

    fn set_severity(&mut self, min_severity: u32) {
        if min_severity <= ZEN_PANIC {
            self.min_severity.store(min_severity, Ordering::SeqCst);
        }
    }

    fn get_stats(&self) -> zenith_log_stats_t {
        zenith_log_stats_t {
            total_emitted: self.total_emitted.load(Ordering::Relaxed),
            total_dropped: self.total_dropped.load(Ordering::Relaxed),
            ring_wraps: self.ring_wraps.load(Ordering::Relaxed),
            current_severity: self.min_severity.load(Ordering::Relaxed),
            ring_write_idx: self.write_idx.load(Ordering::Relaxed),
        }
    }

    fn get_entry(&self, index: u32) -> *const zenith_log_entry_t {
        &self.ring[(index & (ZENITH_LOG_RING_SIZE as u32 - 1)) as usize]
    }

    fn dump(&self) {
        unsafe {
            serial_puts(b"\n=== ZENITHD LOG DUMP ===\n\x00".as_ptr());
            let emitted = self.total_emitted.load(Ordering::Relaxed);
            let count = if emitted < ZENITH_LOG_RING_SIZE as u64 {
                emitted as u32
            } else {
                ZENITH_LOG_RING_SIZE as u32
            };
            let write_pos = self.write_idx.load(Ordering::Relaxed);
            let start = (write_pos - count) & (ZENITH_LOG_RING_SIZE as u32 - 1);

            for i in 0..count {
                let idx = (start + i) & (ZENITH_LOG_RING_SIZE as u32 - 1);
                let e = &self.ring[idx as usize];
                let sev_idx = if e.severity <= ZEN_PANIC { e.severity as usize } else { ZEN_PANIC as usize };
                let sev_name = SEVERITY_NAMES[sev_idx];

                serial_puts(b"[\x00".as_ptr());
                let mut sev_buf = [0u8; 8];
                let mut name_i = 0;
                for b in sev_name.bytes() {
                    if name_i < 7 {
                        sev_buf[name_i] = b;
                        name_i += 1;
                    }
                }
                sev_buf[name_i] = 0;
                serial_puts(sev_buf.as_ptr());
                
                serial_puts(b"] [\x00".as_ptr());
                serial_puts(e.component.as_ptr());
                serial_puts(b"] \x00".as_ptr());
                serial_puts(e.message.as_ptr());
                serial_puts(b"\n\x00".as_ptr());
            }
            serial_puts(b"=== END DUMP ===\n\x00".as_ptr());
        }
    }
}

// Thread-safe wrapper struct using UnsafeCell
struct SafeZenithLogger {
    inner: UnsafeCell<ZenithLogger>,
}

unsafe impl Sync for SafeZenithLogger {}

static LOGGER: SafeZenithLogger = SafeZenithLogger {
    inner: UnsafeCell::new(ZenithLogger::new()),
};

// ---- C Compatible Entry Points (ABI Shims) ----

#[no_mangle]
pub unsafe extern "C" fn zenith_log_init() {
    let l = &mut *LOGGER.inner.get();
    l.init();
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_emit(
    severity: u32,
    error_code: u32,
    component: *const u8,
    message: *const u8,
    cid: u32,
) {
    let l = &mut *LOGGER.inner.get();
    l.emit(severity, error_code, component, message, cid);
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_structured(
    code: u32,
    comp: *const u8,
    desc: *const u8,
    cid: u32,
) {
    let mut severity = ZEN_ERROR;
    if code >= 0xB000 && code <= 0xBFFF {
        severity = ZEN_PANIC;
    } else if code >= 0xA000 && code <= 0xAFFF {
        severity = ZEN_CRIT;
    } else if code >= 0xD000 && code <= 0xFFFF {
        severity = ZEN_ERROR;
    }
    let l = &mut *LOGGER.inner.get();
    l.emit(severity, code, comp, desc, cid);
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_set_severity(min_severity: u32) {
    let l = &mut *LOGGER.inner.get();
    l.set_severity(min_severity);
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_get_stats() -> zenith_log_stats_t {
    let l = &*LOGGER.inner.get();
    l.get_stats()
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_entry_at(index: u32) -> *const zenith_log_entry_t {
    let l = &*LOGGER.inner.get();
    l.get_entry(index)
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_dump() {
    let l = &*LOGGER.inner.get();
    l.dump();
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_flush_to_disk(path: *const u8) {
    // VFS not yet available
    serial_puts(b"[zenithd] flush_to_disk: VFS not yet available\n\x00".as_ptr());
}

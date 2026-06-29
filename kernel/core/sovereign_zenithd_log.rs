// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign Logging Daemon (Rust, no_std)
//! Replaces: kernel/core/sigma_zenithd_log.c
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

pub const LOG_BUFFER_SIZE: usize = 16384;

pub struct ZenithLogger {
    buffer: [u8; LOG_BUFFER_SIZE],
    head: usize,
    tail: usize,
}

impl ZenithLogger {
    pub const fn new() -> Self {
        Self {
            buffer: [0; LOG_BUFFER_SIZE],
            head: 0,
            tail: 0,
        }
    }
}

struct SafeZenithLogger {
    inner: UnsafeCell<ZenithLogger>,
}

unsafe impl Sync for SafeZenithLogger {}

static LOGGER: SafeZenithLogger = SafeZenithLogger {
    inner: UnsafeCell::new(ZenithLogger::new()),
};

extern "C" {
    fn serial_puts(s: *const u8);
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_init() {
    let l = &mut *LOGGER.inner.get();
    l.head = 0;
    l.tail = 0;
}

#[no_mangle]
pub unsafe extern "C" fn zenith_log_write(msg_ptr: *const u8) {
    let l = &mut *LOGGER.inner.get();
    let mut idx = 0;
    while *msg_ptr.add(idx) != 0 {
        let next_tail = (l.tail + 1) % LOG_BUFFER_SIZE;
        if next_tail == l.head {
            // Buffer full, drop/overwrite older logs
            l.head = (l.head + 1) % LOG_BUFFER_SIZE;
        }
        l.buffer[l.tail] = *msg_ptr.add(idx);
        l.tail = next_tail;
        idx += 1;
    }
    serial_puts(msg_ptr);
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// runtime/compat/win32/sigma_handle_table.rs — Win32 HANDLE table
//
// Windows uses kernel handles (opaque integers) for all resources.
// This module maps Win32 HANDLEs to SigmaOS file descriptors, thread IDs,
// mutex IDs, event IDs, and memory section descriptors.
//
// Design:
//   - Fixed-size table of 1024 slots (matches Wine/ReactOS default)
//   - Handle values are (index << 2) | 0 — always 4-byte aligned, bit 0 clear
//   - INVALID_HANDLE_VALUE = -1 (0xFFFFFFFFFFFFFFFF)
//   - Thread-safe: spinlock guards the table
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub const INVALID_HANDLE_VALUE: usize = usize::MAX;
pub const MAX_HANDLES: usize = 1024;

// ── Handle types ──────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HandleKind {
    Free      = 0x000,
    File      = 0x001,
    Thread    = 0x100,
    Process   = 0x200,
    Event     = 0x300,
    Mutex     = 0x400,
    Semaphore = 0x500,
    Section   = 0x600, // memory-mapped file / shared memory
    Key       = 0x700, // registry key
    Timer     = 0x800,
    Unknown   = 0xFFF,
}

impl HandleKind {
    pub fn from_u32(v: u32) -> Self {
        match v {
            0x001 => Self::File,
            0x100 => Self::Thread,
            0x200 => Self::Process,
            0x300 => Self::Event,
            0x400 => Self::Mutex,
            0x500 => Self::Semaphore,
            0x600 => Self::Section,
            0x700 => Self::Key,
            0x800 => Self::Timer,
            _     => Self::Unknown,
        }
    }
}

// ── Handle slot ───────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HandleSlot {
    pub kind:       u32,    // HandleKind as u32
    pub flags:      u8,     // HANDLE_FLAG_*
    pub ref_count:  u16,
    pub _pad:       u8,
    pub data:       u64,    // fd / tid / event_id / etc.
}

pub const HANDLE_FLAG_INHERIT:            u8 = 0x01;
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: u8 = 0x02;

impl HandleSlot {
    pub const fn empty() -> Self {
        Self { kind: 0, flags: 0, ref_count: 0, _pad: 0, data: 0 }
    }
    pub fn is_free(&self) -> bool { self.kind == 0 }
}

// ── Spinlock ──────────────────────────────────────────────────────────────
struct Spinlock(AtomicBool);
impl Spinlock {
    const fn new() -> Self { Self(AtomicBool::new(false)) }
    fn lock(&self) {
        while self.0.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            core::hint::spin_loop();
        }
    }
    fn unlock(&self) { self.0.store(false, Ordering::Release); }
}

// ── Handle table ──────────────────────────────────────────────────────────
pub struct Win32HandleTable {
    slots:     [HandleSlot; MAX_HANDLES],
    count:     AtomicU32,
    lock:      Spinlock,
}

impl Win32HandleTable {
    pub const fn new() -> Self {
        Self {
            slots:  [const { HandleSlot::empty() }; MAX_HANDLES],
            count:  AtomicU32::new(0),
            lock:   Spinlock::new(),
        }
    }

    /// Allocate a new handle and return the HANDLE value (index << 2).
    pub fn alloc(&mut self, kind: HandleKind, data: u64) -> usize {
        self.lock.lock();
        let result = (|| {
            for i in 1..MAX_HANDLES {  // index 0 = NULL handle, skip
                if self.slots[i].is_free() {
                    self.slots[i] = HandleSlot {
                        kind: kind as u32,
                        flags: 0,
                        ref_count: 1,
                        _pad: 0,
                        data,
                    };
                    self.count.fetch_add(1, Ordering::Relaxed);
                    return i << 2;  // handle = index * 4
                }
            }
            INVALID_HANDLE_VALUE
        })();
        self.lock.unlock();
        result
    }

    /// Increment reference count on a handle.
    pub fn add_ref(&mut self, handle: usize) -> bool {
        let idx = handle >> 2;
        if idx == 0 || idx >= MAX_HANDLES { return false; }
        self.lock.lock();
        let result = if !self.slots[idx].is_free() {
            self.slots[idx].ref_count = self.slots[idx].ref_count.saturating_add(1);
            true
        } else { false };
        self.lock.unlock();
        result
    }

    /// Decrement reference count; free if it hits zero.
    pub fn release(&mut self, handle: usize) -> i32 {
        let idx = handle >> 2;
        if idx == 0 || idx >= MAX_HANDLES { return -1; }
        self.lock.lock();
        let result = if !self.slots[idx].is_free() {
            self.slots[idx].ref_count -= 1;
            if self.slots[idx].ref_count == 0 {
                self.slots[idx] = HandleSlot::empty();
                self.count.fetch_sub(1, Ordering::Relaxed);
            }
            0
        } else { -1 };
        self.lock.unlock();
        result
    }

    /// Get the raw data payload for a handle (e.g. fd, tid).
    pub fn get_data(&self, handle: usize, expected_kind: HandleKind) -> Option<u64> {
        let idx = handle >> 2;
        if idx == 0 || idx >= MAX_HANDLES { return None; }
        let slot = &self.slots[idx];
        if slot.is_free() { return None; }
        if expected_kind != HandleKind::Unknown && slot.kind != expected_kind as u32 {
            return None;
        }
        Some(slot.data)
    }

    /// Get slot by handle value (read-only).
    pub fn get(&self, handle: usize) -> Option<&HandleSlot> {
        let idx = handle >> 2;
        if idx == 0 || idx >= MAX_HANDLES { return None; }
        let slot = &self.slots[idx];
        if slot.is_free() { None } else { Some(slot) }
    }

    pub fn active_count(&self) -> u32 {
        self.count.load(Ordering::Relaxed)
    }
}

// ── Global handle table ───────────────────────────────────────────────────
static mut G_HANDLE_TABLE: Win32HandleTable = Win32HandleTable::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sigma_handle_alloc(kind: u32, data: u64) -> usize {
    G_HANDLE_TABLE.alloc(HandleKind::from_u32(kind), data)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_handle_free(handle: usize) -> i32 {
    G_HANDLE_TABLE.release(handle)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_handle_get_data(handle: usize, kind: u32) -> u64 {
    G_HANDLE_TABLE.get_data(handle, HandleKind::from_u32(kind)).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_handle_addref(handle: usize) -> i32 {
    if G_HANDLE_TABLE.add_ref(handle) { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_handle_active_count() -> u32 {
    G_HANDLE_TABLE.active_count()
}

// Standard Win32 pseudo-handles
pub const CURRENT_PROCESS: usize = !0usize;     // (HANDLE)-1
pub const CURRENT_THREAD:  usize = !1usize;     // (HANDLE)-2

#[no_mangle]
pub unsafe extern "C" fn GetCurrentProcess() -> usize { CURRENT_PROCESS }
#[no_mangle]
pub unsafe extern "C" fn GetCurrentThread()  -> usize { CURRENT_THREAD  }

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop { unsafe { core::arch::asm!("cli; hlt", options(nomem, nostack)); } }
}

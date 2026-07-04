// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/core/hotplug_manager.rs — Device hotplug event manager
//
// Handles runtime device attach/detach events (USB, Thunderbolt, PCIe
// hotplug, ACPI _EJ0 eject notifications).
//
// Design:
//   - Event queue (ring buffer, lock-free SPSC)
//   - Per-device probe/remove callbacks routed through SDF
//   - sigma-bus notifications to userspace on every event
//   - Supports both kernel-ring-0 and ring-3 isolated drivers
//
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicUsize, Ordering};

// ── Hotplug event types ───────────────────────────────────────────────────
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HotplugEventKind {
    Attached  = 1,
    Detached  = 2,
    Suspended = 3,
    Resumed   = 4,
    Error     = 5,
}

/// A single hotplug event
#[repr(C)]
#[derive(Copy, Clone)]
pub struct HotplugEvent {
    pub kind:      HotplugEventKind,
    pub bus:       u8,   // PCI bus / USB bus number
    pub dev:       u8,   // PCI device / USB address
    pub func:      u8,   // PCI function
    pub vendor_id: u16,
    pub device_id: u16,
    pub irq:       u8,
    pub mmio_base: u64,
    pub _pad:      [u8; 5],
}

impl HotplugEvent {
    pub const fn zeroed() -> Self {
        Self {
            kind: HotplugEventKind::Error,
            bus: 0, dev: 0, func: 0,
            vendor_id: 0, device_id: 0,
            irq: 0, mmio_base: 0,
            _pad: [0u8; 5],
        }
    }
}

// ── Lock-free SPSC ring buffer ────────────────────────────────────────────
const RING_SIZE: usize = 64; // must be power of 2

pub struct EventRing {
    buf:   [HotplugEvent; RING_SIZE],
    head:  AtomicUsize, // consumer reads from head
    tail:  AtomicUsize, // producer writes to tail
}

impl EventRing {
    pub const fn new() -> Self {
        Self {
            buf:  [const { HotplugEvent::zeroed() }; RING_SIZE],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn push(&mut self, event: HotplugEvent) -> bool {
        let tail = self.tail.load(Ordering::Relaxed);
        let next = (tail + 1) & (RING_SIZE - 1);
        if next == self.head.load(Ordering::Acquire) {
            return false; // full
        }
        self.buf[tail] = event;
        self.tail.store(next, Ordering::Release);
        true
    }

    pub fn pop(&mut self) -> Option<HotplugEvent> {
        let head = self.head.load(Ordering::Relaxed);
        if head == self.tail.load(Ordering::Acquire) {
            return None; // empty
        }
        let event = self.buf[head];
        self.head.store((head + 1) & (RING_SIZE - 1), Ordering::Release);
        Some(event)
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Relaxed) == self.tail.load(Ordering::Relaxed)
    }
}

// ── Hotplug listener callback type ───────────────────────────────────────
pub type HotplugCallback = unsafe extern "C" fn(event: *const HotplugEvent);

const MAX_LISTENERS: usize = 16;

// ── Hotplug manager ───────────────────────────────────────────────────────
pub struct HotplugManager {
    ring:         EventRing,
    listeners:    [Option<HotplugCallback>; MAX_LISTENERS],
    listener_cnt: usize,
    initialized:  bool,
    total_events: u64,
}

impl HotplugManager {
    pub const fn new() -> Self {
        Self {
            ring:         EventRing::new(),
            listeners:    [const { None }; MAX_LISTENERS],
            listener_cnt: 0,
            initialized:  false,
            total_events: 0,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
    }

    /// Register a callback to be called on every hotplug event.
    pub fn register_listener(&mut self, cb: HotplugCallback) -> bool {
        if self.listener_cnt >= MAX_LISTENERS { return false; }
        self.listeners[self.listener_cnt] = Some(cb);
        self.listener_cnt += 1;
        true
    }

    /// Post a new hotplug event (called from IRQ handler or ACPI).
    pub fn post_event(&mut self, event: HotplugEvent) {
        self.total_events += 1;
        self.ring.push(event);
    }

    /// Process all pending events — called from kernel work queue.
    pub unsafe fn process_pending(&mut self) {
        while let Some(event) = self.ring.pop() {
            // Notify all listeners
            for i in 0..self.listener_cnt {
                if let Some(cb) = self.listeners[i] {
                    cb(&event as *const HotplugEvent);
                }
            }
            // Notify userspace via sigma-bus channel 0x10 (HOTPLUG)
            extern "C" {
                fn sigma_bus_send(ch: u32, data: *const u8, len: usize) -> i32;
            }
            sigma_bus_send(
                0x10,
                &event as *const HotplugEvent as *const u8,
                core::mem::size_of::<HotplugEvent>(),
            );
        }
    }

    pub fn total_events(&self) -> u64 { self.total_events }
}

static mut G_HOTPLUG: HotplugManager = HotplugManager::new();

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn hotplug_init() {
    G_HOTPLUG.init();
}

#[no_mangle]
pub unsafe extern "C" fn hotplug_register_listener(cb: HotplugCallback) -> i32 {
    if G_HOTPLUG.register_listener(cb) { 0 } else { -12 }
}

#[no_mangle]
pub unsafe extern "C" fn hotplug_post_event(event: *const HotplugEvent) -> i32 {
    if event.is_null() { return -22; }
    G_HOTPLUG.post_event(*event);
    0
}

#[no_mangle]
pub unsafe extern "C" fn hotplug_process_pending() {
    G_HOTPLUG.process_pending();
}

#[no_mangle]
pub unsafe extern "C" fn hotplug_total_events() -> u64 {
    G_HOTPLUG.total_events()
}

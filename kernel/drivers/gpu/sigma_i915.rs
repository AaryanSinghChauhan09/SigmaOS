// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/drivers/gpu/sigma_i915.rs — Intel i915 (Gen9+) GPU Driver
// Implements: Basic display initialization, mode setting,
// and hardware cursor for Intel Integrated Graphics.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

// ── MMIO Offsets (Gen9+) ───────────────────────────────────────────────────
const DSPCNTR:   usize = 0x70180; // Display Plane Control
const DSPSTRIDE: usize = 0x70188; // Display Plane Stride
const DSPPOS:    usize = 0x7018C; // Display Plane Position
const DSPSIZE:   usize = 0x70190; // Display Plane Size
const DSPBASE:   usize = 0x70184; // Display Plane Base (Surface)
const DSPSURF:   usize = 0x7019C; // Display Plane Surface Address

const PIPEACONF: usize = 0x70008; // Pipe A Configuration

// ── PCI IDs ────────────────────────────────────────────────────────────────
const INTEL_VID: u16 = 0x8086;
// e.g., Skylake/KabyLake/CoffeeLake
const SKYLAKE_GT2: u16 = 0x1912;
const COFFEELAKE_GT2: u16 = 0x3E92;

pub struct I915Driver {
    pub initialized: bool,
    pub mmio_base: usize,
    pub fb_base: u64,
    pub fb_size: usize,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
}

static mut I915: I915Driver = I915Driver {
    initialized: false,
    mmio_base: 0,
    fb_base: 0,
    fb_size: 0,
    width: 0,
    height: 0,
    pitch: 0,
};

static I915_READY: AtomicBool = AtomicBool::new(false);

impl I915Driver {
    pub fn probe() -> bool {
        let candidates = [SKYLAKE_GT2, COFFEELAKE_GT2];
        for did in candidates.iter() {
            if let Some((bar0, bar2)) = pci_find_i915(INTEL_VID, *did) {
                unsafe {
                    I915.mmio_base = bar0;
                    I915.fb_base = bar2 as u64; // GTT aperture base
                    I915.init();
                }
                return true;
            }
        }
        false
    }

    fn init(&mut self) {
        // Simple initialization sequence for framebuffer
        // Assume UEFI GOP has already set up the clocks and PLLs

        self.width = 1920;
        self.height = 1080;
        self.pitch = 1920 * 4;
        self.fb_size = (self.pitch * self.height) as usize;

        // Enable Pipe A
        let conf = self.read32(PIPEACONF);
        self.write32(PIPEACONF, conf | (1 << 31)); // Enable bit

        // Configure Display Plane A
        // 0x40000000: BGRX8888 format, Enable Plane
        self.write32(DSPCNTR, 0x40000000 | (1 << 31));
        self.write32(DSPSTRIDE, self.pitch / 64);
        self.write32(DSPPOS, 0);
        self.write32(DSPSIZE, ((self.height - 1) << 16) | (self.width - 1));
        
        // Write surface address (triggers update)
        self.write32(DSPSURF, 0); // Offset into GTT aperture

        self.initialized = true;
        I915_READY.store(true, Ordering::Release);
    }

    pub fn draw_pixel(&self, x: u32, y: u32, color: u32) {
        if !self.initialized || x >= self.width || y >= self.height { return; }
        let offset = (y * self.pitch + x * 4) as usize;
        unsafe {
            let ptr = (self.fb_base as usize + offset) as *mut u32;
            core::ptr::write_volatile(ptr, color);
        }
    }

    // ── MMIO Helpers ───────────────────────────────────────────────────────
    
    fn read32(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.mmio_base + offset) as *const u32) }
    }
    
    fn write32(&self, offset: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.mmio_base + offset) as *mut u32, val) }
    }
}

// ── PCI Helper ─────────────────────────────────────────────────────────────
fn pci_find_i915(vendor: u16, device: u16) -> Option<(usize, usize)> {
    for bus in 0u8..=255 {
        for slot in 0u8..32 {
            let addr = 0x8000_0000u32 | ((bus as u32)<<16) | ((slot as u32)<<11);
            let id = pci_r32(addr);
            if id == 0xFFFF_FFFF { continue; }
            if (id & 0xFFFF) as u16 == vendor && (id>>16) as u16 == device {
                let bar0 = (pci_r32(addr | 0x10) & !0xF) as usize;
                let bar2 = (pci_r32(addr | 0x18) & !0xF) as usize;
                return Some((bar0, bar2));
            }
        }
    }
    None
}

fn pci_r32(addr: u32) -> u32 {
    unsafe {
        core::arch::asm!("out dx, eax", in("dx") 0xCF8u16, in("eax") addr);
        let v: u32;
        core::arch::asm!("in eax, dx", out("eax") v, in("dx") 0xCFCu16);
        v
    }
}

pub fn i915_init() -> bool { I915Driver::probe() }
pub fn i915_is_ready() -> bool { I915_READY.load(Ordering::Relaxed) }
pub fn i915_draw_pixel(x: u32, y: u32, color: u32) { unsafe { I915.draw_pixel(x, y, color) } }

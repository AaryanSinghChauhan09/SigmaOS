// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// arch/arm64/sigma_gic.rs — ARM Generic Interrupt Controller (GIC) v2/v3
// Implements: Initialization, IRQ enabling, EOI (End of Interrupt),
// and interrupt acknowledgment for ARM64 platforms (e.g. Raspberry Pi 4).

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

// ── GICv2 Register Offsets ─────────────────────────────────────────────────
const GICD_CTLR:   usize = 0x000;
const GICD_TYPER:  usize = 0x004;
const GICD_IGROUPR:usize = 0x080;
const GICD_ISENABLER: usize = 0x100;
const GICD_ICENABLER: usize = 0x180;
const GICD_ISPENDR: usize = 0x200;
const GICD_ICPENDR: usize = 0x280;
const GICD_IPRIORITYR: usize = 0x400;
const GICD_ITARGETSR: usize = 0x800;
const GICD_ICFGR:  usize = 0xC00;
const GICD_SGIR:   usize = 0xF00;

const GICC_CTLR:   usize = 0x000;
const GICC_PMR:    usize = 0x004;
const GICC_BPR:    usize = 0x008;
const GICC_IAR:    usize = 0x00C;
const GICC_EOIR:   usize = 0x010;
const GICC_RPR:    usize = 0x014;
const GICC_HPIR:   usize = 0x018;
const GICC_IIDR:   usize = 0x0FC;

// ── GIC State ──────────────────────────────────────────────────────────────
pub struct Gic {
    pub gicd_base: usize,
    pub gicc_base: usize,
    pub max_irqs: u32,
    pub initialized: bool,
}

static mut GIC: Gic = Gic {
    gicd_base: 0,
    gicc_base: 0,
    max_irqs: 0,
    initialized: false,
};

static GIC_READY: AtomicBool = AtomicBool::new(false);

impl Gic {
    pub fn init(&mut self, gicd_base: usize, gicc_base: usize) {
        self.gicd_base = gicd_base;
        self.gicc_base = gicc_base;

        // 1. Disable Distributor
        self.write_dist(GICD_CTLR, 0);

        // 2. Read max IRQs
        let typer = self.read_dist(GICD_TYPER);
        self.max_irqs = 32 * ((typer & 0x1F) + 1);

        // 3. Configure all SPIs (Shared Peripheral Interrupts)
        for i in (32..self.max_irqs).step_by(32) {
            self.write_dist(GICD_ICENABLER + (i as usize / 32) * 4, 0xFFFFFFFF);
            self.write_dist(GICD_ICPENDR + (i as usize / 32) * 4, 0xFFFFFFFF);
        }

        // Set priority to default (0xA0)
        for i in (32..self.max_irqs).step_by(4) {
            self.write_dist(GICD_IPRIORITYR + (i as usize), 0xA0A0A0A0);
        }

        // Target all SPIs to CPU 0
        for i in (32..self.max_irqs).step_by(4) {
            self.write_dist(GICD_ITARGETSR + (i as usize), 0x01010101);
        }

        // Configure all SPIs as level-sensitive
        for i in (32..self.max_irqs).step_by(16) {
            self.write_dist(GICD_ICFGR + (i as usize / 16) * 4, 0x00000000);
        }

        // 4. Enable Distributor
        self.write_dist(GICD_CTLR, 1);

        // 5. Configure CPU Interface
        // Disable first
        self.write_cpu(GICC_CTLR, 0);
        
        // Set priority mask to allow all interrupts
        self.write_cpu(GICC_PMR, 0xF0);
        
        // Enable CPU Interface
        self.write_cpu(GICC_CTLR, 1);

        self.initialized = true;
        GIC_READY.store(true, Ordering::Release);
    }

    pub fn enable_irq(&self, irq: u32) {
        if irq >= self.max_irqs { return; }
        let reg = GICD_ISENABLER + (irq as usize / 32) * 4;
        let bit = 1 << (irq % 32);
        self.write_dist(reg, bit);
    }

    pub fn disable_irq(&self, irq: u32) {
        if irq >= self.max_irqs { return; }
        let reg = GICD_ICENABLER + (irq as usize / 32) * 4;
        let bit = 1 << (irq % 32);
        self.write_dist(reg, bit);
    }

    pub fn acknowledge_irq(&self) -> u32 {
        self.read_cpu(GICC_IAR) & 0x3FF
    }

    pub fn end_of_interrupt(&self, irq: u32) {
        self.write_cpu(GICC_EOIR, irq);
    }

    // ── MMIO Helpers ───────────────────────────────────────────────────────
    
    fn read_dist(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.gicd_base + offset) as *const u32) }
    }
    
    fn write_dist(&self, offset: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.gicd_base + offset) as *mut u32, val) }
    }
    
    fn read_cpu(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile((self.gicc_base + offset) as *const u32) }
    }
    
    fn write_cpu(&self, offset: usize, val: u32) {
        unsafe { core::ptr::write_volatile((self.gicc_base + offset) as *mut u32, val) }
    }
}

pub fn gic_init(gicd_base: usize, gicc_base: usize) {
    unsafe { GIC.init(gicd_base, gicc_base); }
}

pub fn gic_enable_irq(irq: u32) {
    unsafe { GIC.enable_irq(irq); }
}

pub fn gic_disable_irq(irq: u32) {
    unsafe { GIC.disable_irq(irq); }
}

pub fn gic_ack() -> u32 {
    unsafe { GIC.acknowledge_irq() }
}

pub fn gic_eoi(irq: u32) {
    unsafe { GIC.end_of_interrupt(irq); }
}

// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// arch/arm64/sigma_bcm2711.rs — BCM2711 (Raspberry Pi 4) Platform Initialization
// Implements: MMIO base setup, Mini UART (PL011/AUX) initialization,
// and basic Mailbox interface for VideoCore IV/VI communication.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicBool, Ordering};

// ── MMIO Base ──────────────────────────────────────────────────────────────
// On BCM2711, peripheral base is at 0xFE00_0000 by default.
pub const MMIO_BASE: usize = 0xFE00_0000;

// ── UART (Mini UART / PL011) ───────────────────────────────────────────────
const GPIO_BASE: usize = MMIO_BASE + 0x200000;
const AUX_BASE:  usize = MMIO_BASE + 0x215000;

// GPIO Registers
const GPFSEL1:   usize = GPIO_BASE + 0x04;
const GPPUD:     usize = GPIO_BASE + 0x94; // For Pi 3/4 GPPUD/GPPUDCLK0 (legacy)
const GPPUDCLK0: usize = GPIO_BASE + 0x98;
const GPIO_PUP_PDN_CNTRL_REG0: usize = GPIO_BASE + 0xE4; // BCM2711 specific pull-up/down

// AUX UART Registers
const AUX_ENABLES:     usize = AUX_BASE + 0x04;
const AUX_MU_IO_REG:   usize = AUX_BASE + 0x40;
const AUX_MU_IER_REG:  usize = AUX_BASE + 0x44;
const AUX_MU_IIR_REG:  usize = AUX_BASE + 0x48;
const AUX_MU_LCR_REG:  usize = AUX_BASE + 0x4C;
const AUX_MU_MCR_REG:  usize = AUX_BASE + 0x50;
const AUX_MU_LSR_REG:  usize = AUX_BASE + 0x54;
const AUX_MU_MSR_REG:  usize = AUX_BASE + 0x58;
const AUX_MU_SCRATCH:  usize = AUX_BASE + 0x5C;
const AUX_MU_CNTL_REG: usize = AUX_BASE + 0x60;
const AUX_MU_STAT_REG: usize = AUX_BASE + 0x64;
const AUX_MU_BAUD_REG: usize = AUX_BASE + 0x68;

// ── Mailbox ────────────────────────────────────────────────────────────────
const MBOX_BASE: usize = MMIO_BASE + 0xB880;
const MBOX_READ:   usize = MBOX_BASE + 0x00;
const MBOX_POLL:   usize = MBOX_BASE + 0x10;
const MBOX_SENDER: usize = MBOX_BASE + 0x14;
const MBOX_STATUS: usize = MBOX_BASE + 0x18;
const MBOX_CONFIG: usize = MBOX_BASE + 0x1C;
const MBOX_WRITE:  usize = MBOX_BASE + 0x20;

const MBOX_FULL:  u32 = 0x80000000;
const MBOX_EMPTY: u32 = 0x40000000;

pub const MBOX_CH_PROP: u8 = 8; // Property tags (ARM <-> VC)

pub struct Bcm2711 {
    pub initialized: bool,
}

static mut BCM2711: Bcm2711 = Bcm2711 {
    initialized: false,
};

static PLATFORM_READY: AtomicBool = AtomicBool::new(false);

impl Bcm2711 {
    pub fn init(&mut self) {
        self.uart_init();
        self.initialized = true;
        PLATFORM_READY.store(true, Ordering::Release);
    }

    fn uart_init(&self) {
        // 1. Enable Mini UART
        self.write32(AUX_ENABLES, 1);
        
        // 2. Disable TX/RX during configuration
        self.write32(AUX_MU_CNTL_REG, 0);
        
        // 3. Set data size to 8 bits
        self.write32(AUX_MU_LCR_REG, 3);
        
        // 4. Set RTS line to be always high
        self.write32(AUX_MU_MCR_REG, 0);
        
        // 5. Disable interrupts
        self.write32(AUX_MU_IER_REG, 0);
        
        // 6. Clear FIFOs
        self.write32(AUX_MU_IIR_REG, 0xC6);
        
        // 7. Set baud rate (115200 for 500MHz system clock)
        self.write32(AUX_MU_BAUD_REG, 270);
        
        // 8. Map UART to GPIO pins 14 (TX) and 15 (RX)
        let mut r = self.read32(GPFSEL1);
        r &= !((7 << 12) | (7 << 15)); // Clear bits
        r |= (2 << 12) | (2 << 15);    // ALT5 mode
        self.write32(GPFSEL1, r);
        
        // 9. Disable pull-up/down for pins 14/15 on BCM2711
        let mut pup = self.read32(GPIO_PUP_PDN_CNTRL_REG0);
        pup &= !((3 << 28) | (3 << 30)); // 14 and 15
        // 00 = No resistor
        self.write32(GPIO_PUP_PDN_CNTRL_REG0, pup);
        
        // 10. Enable TX/RX
        self.write32(AUX_MU_CNTL_REG, 3);
    }

    pub fn uart_putc(&self, c: u8) {
        // Wait until transmitter is empty
        while (self.read32(AUX_MU_LSR_REG) & 0x20) == 0 {
            core::hint::spin_loop();
        }
        self.write32(AUX_MU_IO_REG, c as u32);
    }

    pub fn mbox_call(&self, ch: u8, buffer_addr: u32) -> bool {
        // buffer_addr must be 16-byte aligned and converted to bus address
        let r = (buffer_addr & !0xF) | (ch as u32 & 0xF);

        // Wait until mailbox is not full
        while (self.read32(MBOX_STATUS) & MBOX_FULL) != 0 {
            core::hint::spin_loop();
        }

        // Write address to mailbox
        self.write32(MBOX_WRITE, r);

        // Wait for response
        loop {
            while (self.read32(MBOX_STATUS) & MBOX_EMPTY) != 0 {
                core::hint::spin_loop();
            }

            let res = self.read32(MBOX_READ);
            if (res & 0xF) == (ch as u32) {
                // Return success if buffer indicates success
                // Usually checked in the buffer itself, but we return true here
                return true;
            }
        }
    }

    // ── MMIO Helpers ───────────────────────────────────────────────────────
    
    fn read32(&self, offset: usize) -> u32 {
        unsafe { core::ptr::read_volatile(offset as *const u32) }
    }
    
    fn write32(&self, offset: usize, val: u32) {
        unsafe { core::ptr::write_volatile(offset as *mut u32, val) }
    }
}

pub fn bcm2711_init() {
    unsafe { BCM2711.init(); }
}

pub fn bcm2711_uart_putc(c: u8) {
    unsafe { BCM2711.uart_putc(c); }
}

pub fn bcm2711_uart_puts(s: &str) {
    for b in s.bytes() {
        if b == b'\n' {
            unsafe { BCM2711.uart_putc(b'\r'); }
        }
        unsafe { BCM2711.uart_putc(b); }
    }
}

pub fn bcm2711_mbox_call(ch: u8, buffer_addr: u32) -> bool {
    unsafe { BCM2711.mbox_call(ch, buffer_addr) }
}

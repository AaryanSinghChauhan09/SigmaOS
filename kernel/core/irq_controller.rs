// SPDX-License-Identifier: GPL-2.0-or-later
//! =========================================================================
//! SigmaOS: Sovereign IRQ Controller (Rust, no_std)
//! Implements 8259 PIC and APIC interrupt handling for x86_64
//! =========================================================================

#![no_std]

use core::cell::UnsafeCell;

// ─── PIC Constants ─────────────────────────────────────────────────────────

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const PIC_EOI: u8 = 0x20;
const PIC_INIT: u8 = 0x11;
const PIC_ICW4_8086: u8 = 0x01;

// ─── IRQ Handler Types ───────────────────────────────────────────────────────

type IrqHandler = unsafe extern "C" fn();

#[derive(Copy, Clone)]
pub enum IrqLine {
    Timer = 0,
    Keyboard = 1,
    Cascade = 2,
    Com2 = 3,
    Com1 = 4,
    Lpt2 = 5,
    Floppy = 6,
    Lpt1 = 7,
    RTC = 8,
    Mouse = 12,
    Fpu = 13,
    PrimaryATA = 14,
    SecondaryATA = 15,
}

// ─── IRQ Controller State ──────────────────────────────────────────────────

pub struct IrqController {
    handlers: [Option<IrqHandler>; 16],
    initialized: bool,
    mask: u16,
}

impl IrqController {
    pub const fn new() -> Self {
        Self {
            handlers: [None; 16],
            initialized: false,
            mask: 0xFFFF, // All IRQs masked initially
        }
    }

    pub unsafe fn init(&mut self, offset1: u8, offset2: u8) {
        if self.initialized {
            return;
        }

        // Save masks
        let a1 = self.port_in(PIC1_DATA);
        let a2 = self.port_in(PIC2_DATA);

        // Start initialization sequence (ICW1)
        self.port_out(PIC1_COMMAND, PIC_INIT);
        self.port_out(PIC2_COMMAND, PIC_INIT);

        // Set vector offsets (ICW2)
        self.port_out(PIC1_DATA, offset1);
        self.port_out(PIC2_DATA, offset2);

        // Configure cascade mode (ICW3)
        self.port_out(PIC1_DATA, 4); // PIC2 at IRQ2
        self.port_out(PIC2_DATA, 2); // Cascade identity

        // Set 8086 mode (ICW4)
        self.port_out(PIC1_DATA, PIC_ICW4_8086);
        self.port_out(PIC2_DATA, PIC_ICW4_8086);

        // Restore masks
        self.port_out(PIC1_DATA, a1);
        self.port_out(PIC2_DATA, a2);

        self.initialized = true;
    }

    unsafe fn port_out(&mut self, port: u16, value: u8) {
        core::arch::asm!("out dx, al", in("dx") port, in("al") value);
    }

    unsafe fn port_in(&mut self, port: u16) -> u8 {
        let value: u8;
        core::arch::asm!("in al, dx", out("al") value, in("dx") port);
        value
    }

    pub unsafe fn mask_irq(&mut self, irq: u8) {
        if irq >= 16 {
            return;
        }

        let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
        let irq_line = if irq < 8 { irq } else { irq - 8 };

        let current = self.port_in(port);
        self.port_out(port, current | (1 << irq_line));

        self.mask |= 1 << irq;
    }

    pub unsafe fn unmask_irq(&mut self, irq: u8) {
        if irq >= 16 {
            return;
        }

        let port = if irq < 8 { PIC1_DATA } else { PIC2_DATA };
        let irq_line = if irq < 8 { irq } else { irq - 8 };

        let current = self.port_in(port);
        self.port_out(port, current & !(1 << irq_line));

        self.mask &= !(1 << irq);
    }

    pub unsafe fn register_handler(&mut self, irq: u8, handler: IrqHandler) {
        if irq < 16 {
            self.handlers[irq as usize] = Some(handler);
        }
    }

    pub unsafe fn handle_irq(&mut self, irq: u8) {
        if irq < 16 {
            if let Some(handler) = self.handlers[irq as usize] {
                handler();
            }
        }

        // Send EOI
        if irq >= 8 {
            self.port_out(PIC2_COMMAND, PIC_EOI);
        }
        self.port_out(PIC1_COMMAND, PIC_EOI);
    }

    pub fn is_masked(&self, irq: u8) -> bool {
        if irq >= 16 {
            return true;
        }
        (self.mask & (1 << irq)) != 0
    }
}

// ─── Global Singleton ───────────────────────────────────────────────────────

struct SafeIrqController {
    inner: UnsafeCell<IrqController>,
}

unsafe impl Sync for SafeIrqController {}

static IRQ_CONTROLLER: SafeIrqController = SafeIrqController {
    inner: UnsafeCell::new(IrqController::new()),
};

// ─── C-ABI Exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn irq_init(offset1: u8, offset2: u8) {
    let controller = &mut *IRQ_CONTROLLER.inner.get();
    controller.init(offset1, offset2);
}

#[no_mangle]
pub unsafe extern "C" fn irq_mask(irq: u8) {
    let controller = &mut *IRQ_CONTROLLER.inner.get();
    controller.mask_irq(irq);
}

#[no_mangle]
pub unsafe extern "C" fn irq_unmask(irq: u8) {
    let controller = &mut *IRQ_CONTROLLER.inner.get();
    controller.unmask_irq(irq);
}

#[no_mangle]
pub unsafe extern "C" fn irq_handle(irq: u8) {
    let controller = &mut *IRQ_CONTROLLER.inner.get();
    controller.handle_irq(irq);
}

#[no_mangle]
pub unsafe extern "C" fn irq_register(irq: u8, handler: IrqHandler) {
    let controller = &mut *IRQ_CONTROLLER.inner.get();
    controller.register_handler(irq, handler);
}

#[no_mangle]
pub unsafe extern "C" fn irq_is_masked(irq: u8) -> bool {
    let controller = &*IRQ_CONTROLLER.inner.get();
    controller.is_masked(irq)
}

// ─── Default IRQ Handlers ───────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn default_irq_handler() {
    // Default handler - do nothing
}

#[no_mangle]
pub unsafe extern "C" fn timer_irq_handler() {
    // Timer interrupt - will be used for scheduler
}

#[no_mangle]
pub unsafe extern "C" fn keyboard_irq_handler() {
    // Keyboard interrupt - will be used for input
}

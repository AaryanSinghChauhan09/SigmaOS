//! Interrupt Controller Management
//!
//! Inspired by Linux IRQ subsystem and BSD interrupt handling.
//! Provides interrupt registration, handling, and routing.
//!
//! # Features
//! - PIC (8259) legacy interrupt controller
//! - APIC (Advanced Programmable Interrupt Controller)
//! - Interrupt vector allocation
//! - IRQ sharing and chaining
//! - MSI/MSI-X support preparation
//!
//! # Linux Inspiration
//! - `kernel/irq/` - Generic IRQ handling
//! - `arch/x86/kernel/apic/` - APIC management
//! - `kernel/irq/chip.c` - IRQ chip abstraction
//!
//! # FreeBSD Inspiration
//! - `sys/kern/kern_intr.c` - Interrupt management
//! - `sys/x86/x86/local_apic.c` - Local APIC driver

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use core::arch::asm;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// PIC (8259) I/O ports
const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

/// PIC commands
const PIC_EOI: u8 = 0x20; // End of Interrupt

/// Interrupt vector range
const IRQ_VECTOR_BASE: u8 = 0x20;
const IRQ_VECTOR_MAX: u8 = 0xFF;

/// Interrupt handler function type
pub type InterruptHandler = fn(vector: u8);

/// IRQ line number (0-15 for legacy PIC)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IrqLine(pub u8);

impl IrqLine {
    pub const TIMER: Self = Self(0);
    pub const KEYBOARD: Self = Self(1);
    pub const CASCADE: Self = Self(2);
    pub const COM2: Self = Self(3);
    pub const COM1: Self = Self(4);
    pub const LPT2: Self = Self(5);
    pub const FLOPPY: Self = Self(6);
    pub const LPT1: Self = Self(7);
    pub const RTC: Self = Self(8);
    pub const MOUSE: Self = Self(12);
    pub const COPROCESSOR: Self = Self(13);
    pub const PRIMARY_ATA: Self = Self(14);
    pub const SECONDARY_ATA: Self = Self(15);
}

/// Interrupt descriptor
#[derive(Clone)]
pub struct InterruptDescriptor {
    pub vector: u8,
    pub irq_line: Option<IrqLine>,
    pub handler: Option<InterruptHandler>,
    pub enabled: AtomicBool,
    pub count: AtomicU64,
}

impl InterruptDescriptor {
    pub fn new(vector: u8) -> Self {
        Self {
            vector,
            irq_line: None,
            handler: None,
            enabled: AtomicBool::new(false),
            count: AtomicU64::new(0),
        }
    }

    pub fn increment_count(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_count(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
}

/// Interrupt Controller
pub struct InterruptController {
    descriptors: Vec<InterruptDescriptor>,
    pic_initialized: bool,
}

impl InterruptController {
    pub fn new() -> Self {
        let mut descriptors = Vec::new();
        for vector in IRQ_VECTOR_BASE..=IRQ_VECTOR_MAX {
            descriptors.push(InterruptDescriptor::new(vector));
        }

        Self {
            descriptors,
            pic_initialized: false,
        }
    }

    /// Initialize 8259 PIC (legacy mode)
    /// Linux: `arch/x86/kernel/i8259.c:init_8259A()`
    /// FreeBSD: `sys/x86/isa/atpic.c:atpic_init()`
    pub fn init_pic(&mut self) {
        unsafe {
            // ICW1: Start initialization sequence
            Self::outb(PIC1_COMMAND, 0x11);
            Self::outb(PIC2_COMMAND, 0x11);

            // ICW2: Set vector offsets
            Self::outb(PIC1_DATA, IRQ_VECTOR_BASE); // Master PIC: vectors 0x20-0x27
            Self::outb(PIC2_DATA, IRQ_VECTOR_BASE + 8); // Slave PIC: vectors 0x28-0x2F

            // ICW3: Configure cascading
            Self::outb(PIC1_DATA, 0x04); // Master has slave at IRQ2
            Self::outb(PIC2_DATA, 0x02); // Slave is at IRQ2 of master

            // ICW4: Set mode
            Self::outb(PIC1_DATA, 0x01); // 8086 mode
            Self::outb(PIC2_DATA, 0x01); // 8086 mode

            // Mask all interrupts initially
            Self::outb(PIC1_DATA, 0xFF);
            Self::outb(PIC2_DATA, 0xFF);
        }

        self.pic_initialized = true;
    }

    /// Disable PIC (when using APIC)
    /// Linux: `arch/x86/kernel/apic/apic.c:disable_pic()`
    pub fn disable_pic(&mut self) {
        unsafe {
            // Mask all interrupts
            Self::outb(PIC1_DATA, 0xFF);
            Self::outb(PIC2_DATA, 0xFF);
        }
    }

    /// Register interrupt handler
    /// Linux: `kernel/irq/manage.c:request_irq()`
    pub fn register_handler(
        &mut self,
        irq_line: IrqLine,
        handler: InterruptHandler,
    ) -> Result<u8, IrqError> {
        let vector = IRQ_VECTOR_BASE + irq_line.0;
        let index = (vector - IRQ_VECTOR_BASE) as usize;

        if index >= self.descriptors.len() {
            return Err(IrqError::InvalidVector);
        }

        let desc = &mut self.descriptors[index];
        desc.irq_line = Some(irq_line);
        desc.handler = Some(handler);
        desc.enabled.store(true, Ordering::Release);

        Ok(vector)
    }

    /// Enable IRQ line
    /// Linux: `kernel/irq/chip.c:irq_enable()`
    pub fn enable_irq(&mut self, irq_line: IrqLine) {
        if !self.pic_initialized {
            return;
        }

        unsafe {
            let irq = irq_line.0;
            if irq < 8 {
                // Master PIC
                let mut mask = Self::inb(PIC1_DATA);
                mask &= !(1 << irq);
                Self::outb(PIC1_DATA, mask);
            } else {
                // Slave PIC
                let mut mask = Self::inb(PIC2_DATA);
                mask &= !(1 << (irq - 8));
                Self::outb(PIC2_DATA, mask);

                // Also enable cascade line on master
                let mut master_mask = Self::inb(PIC1_DATA);
                master_mask &= !(1 << 2);
                Self::outb(PIC1_DATA, master_mask);
            }
        }
    }

    /// Disable IRQ line
    /// Linux: `kernel/irq/chip.c:irq_disable()`
    pub fn disable_irq(&mut self, irq_line: IrqLine) {
        if !self.pic_initialized {
            return;
        }

        unsafe {
            let irq = irq_line.0;
            if irq < 8 {
                // Master PIC
                let mut mask = Self::inb(PIC1_DATA);
                mask |= 1 << irq;
                Self::outb(PIC1_DATA, mask);
            } else {
                // Slave PIC
                let mut mask = Self::inb(PIC2_DATA);
                mask |= 1 << (irq - 8);
                Self::outb(PIC2_DATA, mask);
            }
        }
    }

    /// Send End of Interrupt (EOI) signal
    /// Linux: `arch/x86/kernel/apic/apic.c:ack_APIC_irq()`
    pub fn send_eoi(&self, irq_line: IrqLine) {
        unsafe {
            if irq_line.0 >= 8 {
                // Send EOI to slave PIC
                Self::outb(PIC2_COMMAND, PIC_EOI);
            }
            // Always send EOI to master PIC
            Self::outb(PIC1_COMMAND, PIC_EOI);
        }
    }

    /// Handle interrupt (called from IDT handler)
    /// Linux: `kernel/irq/handle.c:handle_irq()`
    pub fn handle_interrupt(&self, vector: u8) {
        let index = (vector - IRQ_VECTOR_BASE) as usize;
        
        if index >= self.descriptors.len() {
            return;
        }

        let desc = &self.descriptors[index];
        
        if !desc.enabled.load(Ordering::Acquire) {
            return;
        }

        desc.increment_count();

        if let Some(handler) = desc.handler {
            handler(vector);
        }

        // Send EOI if this is a PIC interrupt
        if let Some(irq_line) = desc.irq_line {
            self.send_eoi(irq_line);
        }
    }

    /// Get interrupt statistics
    pub fn get_stats(&self, irq_line: IrqLine) -> Option<u64> {
        let vector = IRQ_VECTOR_BASE + irq_line.0;
        let index = (vector - IRQ_VECTOR_BASE) as usize;
        
        self.descriptors.get(index).map(|d| d.get_count())
    }

    /// I/O port access
    #[cfg(target_arch = "x86_64")]
    unsafe fn inb(port: u16) -> u8 {
        let value: u8;
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") value,
            options(nomem, nostack, preserves_flags)
        );
        value
    }

    #[cfg(target_arch = "x86_64")]
    unsafe fn outb(port: u16, value: u8) {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }

    #[cfg(not(target_arch = "x86_64"))]
    unsafe fn inb(_port: u16) -> u8 {
        0
    }

    #[cfg(not(target_arch = "x86_64"))]
    unsafe fn outb(_port: u16, _value: u8) {
        // Stub
    }
}

/// IRQ Error Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqError {
    InvalidVector,
    AlreadyRegistered,
    NotInitialized,
}

/// Global interrupt controller
static mut INTERRUPT_CONTROLLER: Option<InterruptController> = None;

/// Initialize global interrupt controller
pub fn init_interrupts() {
    unsafe {
        let mut ic = InterruptController::new();
        ic.init_pic();
        INTERRUPT_CONTROLLER = Some(ic);
    }
}

/// Register interrupt handler (global interface)
pub fn register_irq_handler(
    irq_line: IrqLine,
    handler: InterruptHandler,
) -> Result<u8, IrqError> {
    unsafe {
        INTERRUPT_CONTROLLER
            .as_mut()
            .ok_or(IrqError::NotInitialized)?
            .register_handler(irq_line, handler)
    }
}

/// Enable IRQ (global interface)
pub fn enable_irq(irq_line: IrqLine) {
    unsafe {
        if let Some(ic) = INTERRUPT_CONTROLLER.as_mut() {
            ic.enable_irq(irq_line);
        }
    }
}

/// Disable IRQ (global interface)
pub fn disable_irq(irq_line: IrqLine) {
    unsafe {
        if let Some(ic) = INTERRUPT_CONTROLLER.as_mut() {
            ic.disable_irq(irq_line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irq_line_constants() {
        assert_eq!(IrqLine::TIMER.0, 0);
        assert_eq!(IrqLine::KEYBOARD.0, 1);
        assert_eq!(IrqLine::RTC.0, 8);
    }

    #[test]
    fn test_interrupt_descriptor() {
        let desc = InterruptDescriptor::new(0x20);
        assert_eq!(desc.vector, 0x20);
        assert_eq!(desc.get_count(), 0);
        
        desc.increment_count();
        assert_eq!(desc.get_count(), 1);
    }
}

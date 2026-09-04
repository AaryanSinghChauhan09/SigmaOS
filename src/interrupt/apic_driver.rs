// SPDX-License-Identifier: MIT
// SigmaOS APIC & I/O APIC Driver Implementation
// x86_64 Advanced Programmable Interrupt Controller (APIC) & I/O APIC hardware abstraction

use core::sync::atomic::{AtomicU32, Ordering};
use alloc::vec::Vec;

// ============================================================================
// APIC Memory-Mapped I/O Registers (x86_64)
// ============================================================================

pub const APIC_BASE: u64 = 0xfee0_0000; // Local APIC Base Address (default)
pub const IO_APIC_BASE: u64 = 0xfec0_0000; // I/O APIC Base Address (default)

// Local APIC Register Offsets (from APIC_BASE)
pub const APIC_ID: u32 = 0x20;          // Local APIC ID
pub const APIC_VERSION: u32 = 0x30;     // Local APIC Version
pub const APIC_TPR: u32 = 0x80;         // Task Priority Register
pub const APIC_PPR: u32 = 0xa0;         // Processor Priority Register
pub const APIC_EOI: u32 = 0xb0;         // End of Interrupt
pub const APIC_REMOTE_READ: u32 = 0xc0; // Remote Read
pub const APIC_LOGICAL_DEST: u32 = 0xd0; // Logical Destination
pub const APIC_DEST_FORMAT: u32 = 0xe0; // Destination Format
pub const APIC_SPURIOUS_INT: u32 = 0xf0; // Spurious Interrupt Vector
pub const APIC_ISR_BASE: u32 = 0x100;   // In-Service Register (32 bytes)
pub const APIC_TMR_BASE: u32 = 0x180;   // Trigger Mode Register (32 bytes)
pub const APIC_IRR_BASE: u32 = 0x200;   // Interrupt Request Register (32 bytes)
pub const APIC_ERROR_STATUS: u32 = 0x280; // Error Status Register
pub const APIC_CMCI: u32 = 0x2f0;       // Corrected Machine Check Interrupt
pub const APIC_ICR_LOW: u32 = 0x300;    // Interrupt Command Register (low 32-bit)
pub const APIC_ICR_HIGH: u32 = 0x310;   // Interrupt Command Register (high 32-bit)
pub const APIC_LVT_TIMER: u32 = 0x320;  // LVT Timer
pub const APIC_LVT_THERMAL: u32 = 0x330; // LVT Thermal
pub const APIC_LVT_PERFORMANCE: u32 = 0x340; // LVT Performance Counter
pub const APIC_LVT_LINT0: u32 = 0x350;  // LVT LINT0 (Local Interrupt 0)
pub const APIC_LVT_LINT1: u32 = 0x360;  // LVT LINT1 (Local Interrupt 1)
pub const APIC_LVT_ERROR: u32 = 0x370;  // LVT Error
pub const APIC_TIMER_INITIAL: u32 = 0x380; // Timer Initial Count
pub const APIC_TIMER_CURRENT: u32 = 0x390; // Timer Current Count
pub const APIC_TIMER_DIVIDE: u32 = 0x3e0; // Timer Divide Configuration

// I/O APIC Register Offsets
pub const IO_APIC_INDEX: u32 = 0x00;    // Register Select Index
pub const IO_APIC_DATA: u32 = 0x10;     // Register Data
pub const IO_APIC_ID: u32 = 0x00;       // I/O APIC ID
pub const IO_APIC_VERSION: u32 = 0x01;  // I/O APIC Version
pub const IO_APIC_ARB_ID: u32 = 0x02;   // I/O APIC Arbitration ID
pub const IO_APIC_REDIR_BASE: u32 = 0x10; // Redirection Table Entries start at 0x10

// Delivery Modes (bits 8-10 of ICR low or Redirection Entry low)
pub const DELIVERY_MODE_FIXED: u32 = 0x0;
pub const DELIVERY_MODE_LOWEST_PRIORITY: u32 = 0x1;
pub const DELIVERY_MODE_SMI: u32 = 0x2;
pub const DELIVERY_MODE_NMI: u32 = 0x4;
pub const DELIVERY_MODE_INIT: u32 = 0x5;
pub const DELIVERY_MODE_SIPI: u32 = 0x6;

// Destination Modes (bit 11)
pub const DEST_MODE_PHYSICAL: u32 = 0x0;
pub const DEST_MODE_LOGICAL: u32 = 0x1;

// Interrupt Trigger Modes (bit 15 of Redirection Entry)
pub const TRIGGER_MODE_EDGE: u32 = 0x0;
pub const TRIGGER_MODE_LEVEL: u32 = 0x1;

// Interrupt Pin Polarities (bit 13)
pub const POLARITY_ACTIVE_HIGH: u32 = 0x0;
pub const POLARITY_ACTIVE_LOW: u32 = 0x1;

// Vector Numbering for Common Interrupts
pub const VECTOR_TIMER: u8 = 32;
pub const VECTOR_KEYBOARD: u8 = 33;
pub const VECTOR_SERIAL: u8 = 36;
pub const VECTOR_NETWORK: u8 = 37;
pub const VECTOR_DISK: u8 = 38;
pub const VECTOR_ERROR: u8 = 127;
pub const VECTOR_SPURIOUS: u8 = 255;

// ============================================================================
// Local APIC Driver
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct LocalApicId {
    id: u8,
}

impl LocalApicId {
    pub const fn new(id: u8) -> Self {
        LocalApicId { id }
    }

    pub const fn value(&self) -> u8 {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApicTimerMode {
    OneShot,
    Periodic,
    TSCDeadline,
}

pub struct LocalApic {
    base_addr: u64,
    apic_id: LocalApicId,
    version: u32,
    is_enabled: bool,
}

impl LocalApic {
    pub fn new(base_addr: u64) -> Self {
        LocalApic {
            base_addr,
            apic_id: LocalApicId::new(0),
            version: 0,
            is_enabled: false,
        }
    }

    // MMIO read/write helpers
    #[inline]
    unsafe fn read_register(&self, offset: u32) -> u32 {
        let addr = (self.base_addr + (offset as u64)) as *const u32;
        core::ptr::read_volatile(addr)
    }

    #[inline]
    unsafe fn write_register(&self, offset: u32, value: u32) {
        let addr = (self.base_addr + (offset as u64)) as *mut u32;
        core::ptr::write_volatile(addr, value);
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        unsafe {
            // Read version to determine max LVT entries
            self.version = self.read_register(APIC_VERSION);

            // Read APIC ID
            let id_reg = self.read_register(APIC_ID);
            self.apic_id = LocalApicId::new((id_reg >> 24) as u8);

            // Enable the APIC and set spurious interrupt vector
            let spurious = self.read_register(APIC_SPURIOUS_INT);
            self.write_register(APIC_SPURIOUS_INT, (spurious & 0xffffff00) | 0xff | (1 << 8));

            // Map LINT0 to IRQ0 (keyboard)
            self.write_register(
                APIC_LVT_LINT0,
                DELIVERY_MODE_FIXED | (VECTOR_KEYBOARD as u32) | (TRIGGER_MODE_EDGE << 15),
            );

            // Map LINT1 to NMI
            self.write_register(
                APIC_LVT_LINT1,
                DELIVERY_MODE_NMI | (TRIGGER_MODE_LEVEL << 15) | (1 << 16), // masked
            );

            // Disable performance counter interrupts
            self.write_register(APIC_LVT_PERFORMANCE, 1 << 16); // mask

            // Set task priority to 0 (accept all interrupts)
            self.write_register(APIC_TPR, 0);

            self.is_enabled = true;
        }
        Ok(())
    }

    pub fn send_ipi(&self, target_apic_id: u8, vector: u8, delivery_mode: u32) -> Result<(), &'static str> {
        unsafe {
            // Write ICR high (destination)
            self.write_register(APIC_ICR_HIGH, (target_apic_id as u32) << 24);

            // Write ICR low (command)
            let icr_low = (delivery_mode & 0x7) << 8 | (vector as u32);
            self.write_register(APIC_ICR_LOW, icr_low);

            // Wait for send to complete
            let mut timeout = 1000;
            loop {
                let status = self.read_register(APIC_ICR_LOW);
                if (status & (1 << 12)) == 0 {
                    // Delivery status cleared = send complete
                    break;
                }
                timeout -= 1;
                if timeout == 0 {
                    return Err("IPI send timeout");
                }
            }
        }
        Ok(())
    }

    pub fn setup_timer(&self, vector: u8, mode: ApicTimerMode, initial_count: u32) -> Result<(), &'static str> {
        unsafe {
            let mode_bits = match mode {
                ApicTimerMode::OneShot => 0,
                ApicTimerMode::Periodic => 1,
                ApicTimerMode::TSCDeadline => 2,
            };

            let lvt_timer = (vector as u32) | (mode_bits << 17) | (0 << 16); // enable
            self.write_register(APIC_LVT_TIMER, lvt_timer);

            // Set divide configuration (divide by 16)
            self.write_register(APIC_TIMER_DIVIDE, 0x3);

            // Set initial count
            self.write_register(APIC_TIMER_INITIAL, initial_count);
        }
        Ok(())
    }

    pub fn eoi(&self) {
        unsafe {
            self.write_register(APIC_EOI, 0);
        }
    }

    pub fn get_isr_vector(&self) -> Option<u8> {
        unsafe {
            // Read ISR (In-Service Register) to find highest priority vector being serviced
            for i in (0..8).rev() {
                let isr = self.read_register(APIC_ISR_BASE + (i * 4));
                if isr != 0 {
                    // Find highest bit set
                    for j in (0..32).rev() {
                        if (isr & (1 << j)) != 0 {
                            return Some(((i as u8) * 32 + j as u8));
                        }
                    }
                }
            }
        }
        None
    }

    pub fn get_irr_vector(&self) -> Option<u8> {
        unsafe {
            // Read IRR (Interrupt Request Register)
            for i in (0..8).rev() {
                let irr = self.read_register(APIC_IRR_BASE + (i * 4));
                if irr != 0 {
                    // Find highest bit set
                    for j in (0..32).rev() {
                        if (irr & (1 << j)) != 0 {
                            return Some(((i as u8) * 32 + j as u8));
                        }
                    }
                }
            }
        }
        None
    }

    pub fn id(&self) -> LocalApicId {
        self.apic_id
    }

    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }
}

// ============================================================================
// I/O APIC Driver
// ============================================================================

#[derive(Debug, Clone)]
pub struct IoApicRedirectionEntry {
    pub vector: u8,
    pub delivery_mode: u32,
    pub dest_mode: u32,
    pub delivery_status: bool,
    pub polarity: u32,
    pub remote_irr: bool,
    pub trigger_mode: u32,
    pub masked: bool,
    pub destination: u8,
}

impl IoApicRedirectionEntry {
    pub fn to_u64(&self) -> u64 {
        let low = (self.vector as u64)
            | ((self.delivery_mode as u64) << 8)
            | ((self.dest_mode as u64) << 11)
            | ((self.polarity as u64) << 13)
            | ((self.trigger_mode as u64) << 15)
            | (if self.masked { 1 << 16 } else { 0 });

        let high = (self.destination as u64) << 56;
        low | high
    }

    pub fn from_u64(val: u64) -> Self {
        IoApicRedirectionEntry {
            vector: (val & 0xff) as u8,
            delivery_mode: ((val >> 8) & 0x7) as u32,
            dest_mode: ((val >> 11) & 0x1) as u32,
            delivery_status: ((val >> 12) & 0x1) != 0,
            polarity: ((val >> 13) & 0x1) as u32,
            remote_irr: ((val >> 14) & 0x1) != 0,
            trigger_mode: ((val >> 15) & 0x1) as u32,
            masked: ((val >> 16) & 0x1) != 0,
            destination: ((val >> 56) & 0xff) as u8,
        }
    }
}

pub struct IoApic {
    base_addr: u64,
    apic_id: u8,
    max_redir_entries: u8,
    is_enabled: bool,
}

impl IoApic {
    pub fn new(base_addr: u64) -> Self {
        IoApic {
            base_addr,
            apic_id: 0,
            max_redir_entries: 24, // Most common default
            is_enabled: false,
        }
    }

    #[inline]
    unsafe fn read_reg(&self, reg: u32) -> u32 {
        let index_addr = (self.base_addr) as *mut u32;
        let data_addr = (self.base_addr + 0x10) as *mut u32;

        core::ptr::write_volatile(index_addr, reg);
        core::ptr::read_volatile(data_addr)
    }

    #[inline]
    unsafe fn write_reg(&self, reg: u32, value: u32) {
        let index_addr = (self.base_addr) as *mut u32;
        let data_addr = (self.base_addr + 0x10) as *mut u32;

        core::ptr::write_volatile(index_addr, reg);
        core::ptr::write_volatile(data_addr, value);
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        unsafe {
            // Read version to determine max entries
            let version = self.read_reg(IO_APIC_VERSION);
            self.max_redir_entries = (((version >> 16) & 0xff) + 1) as u8;

            // Read APIC ID
            let id_reg = self.read_reg(IO_APIC_ID);
            self.apic_id = ((id_reg >> 24) & 0xf) as u8;

            self.is_enabled = true;
        }
        Ok(())
    }

    pub fn set_irq_routing(&self, irq: u8, vector: u8, dest_apic_id: u8) -> Result<(), &'static str> {
        if irq as u32 >= self.max_redir_entries as u32 {
            return Err("IRQ out of range");
        }

        unsafe {
            let redir_reg_index = IO_APIC_REDIR_BASE + (irq as u32) * 2;

            // Read current entry
            let low = self.read_reg(redir_reg_index);
            let high = self.read_reg(redir_reg_index + 1);

            // Construct new entry
            let new_low = (low & 0xffff0000) | ((vector as u32) & 0xff);
            let new_high = (dest_apic_id as u32) << 24;

            // Write back
            self.write_reg(redir_reg_index, new_low);
            self.write_reg(redir_reg_index + 1, new_high);
        }
        Ok(())
    }

    pub fn enable_irq(&self, irq: u8) -> Result<(), &'static str> {
        if irq as u32 >= self.max_redir_entries as u32 {
            return Err("IRQ out of range");
        }

        unsafe {
            let redir_reg_index = IO_APIC_REDIR_BASE + (irq as u32) * 2;
            let low = self.read_reg(redir_reg_index);
            // Clear mask bit (bit 16)
            self.write_reg(redir_reg_index, low & !(1 << 16));
        }
        Ok(())
    }

    pub fn disable_irq(&self, irq: u8) -> Result<(), &'static str> {
        if irq as u32 >= self.max_redir_entries as u32 {
            return Err("IRQ out of range");
        }

        unsafe {
            let redir_reg_index = IO_APIC_REDIR_BASE + (irq as u32) * 2;
            let low = self.read_reg(redir_reg_index);
            // Set mask bit (bit 16)
            self.write_reg(redir_reg_index, low | (1 << 16));
        }
        Ok(())
    }

    pub fn get_max_redir_entries(&self) -> u8 {
        self.max_redir_entries
    }
}

// ============================================================================
// Interrupt Dispatch Table
// ============================================================================

pub type InterruptHandler = fn(vector: u8);

pub struct InterruptDispatchTable {
    handlers: Vec<Option<InterruptHandler>>,
}

impl InterruptDispatchTable {
    pub fn new() -> Self {
        let mut handlers = Vec::new();
        for _ in 0..256 {
            handlers.push(None);
        }
        InterruptDispatchTable { handlers }
    }

    pub fn register_handler(&mut self, vector: u8, handler: InterruptHandler) -> Result<(), &'static str> {
        if (vector as usize) >= self.handlers.len() {
            return Err("Vector out of range");
        }
        self.handlers[vector as usize] = Some(handler);
        Ok(())
    }

    pub fn dispatch(&self, vector: u8) -> Result<(), &'static str> {
        if (vector as usize) >= self.handlers.len() {
            return Err("Vector out of range");
        }

        if let Some(handler) = self.handlers[vector as usize] {
            handler(vector);
            Ok(())
        } else {
            Err("No handler registered")
        }
    }
}

// ============================================================================
// Unified APIC Manager
// ============================================================================

pub struct ApicManager {
    local_apic: LocalApic,
    io_apic: IoApic,
    dispatch_table: InterruptDispatchTable,
}

impl ApicManager {
    pub fn new() -> Self {
        ApicManager {
            local_apic: LocalApic::new(APIC_BASE),
            io_apic: IoApic::new(IO_APIC_BASE),
            dispatch_table: InterruptDispatchTable::new(),
        }
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.local_apic.init()?;
        self.io_apic.init()?;
        Ok(())
    }

    pub fn register_interrupt_handler(&mut self, vector: u8, handler: InterruptHandler) -> Result<(), &'static str> {
        self.dispatch_table.register_handler(vector, handler)
    }

    pub fn route_irq(&self, irq: u8, vector: u8, dest_apic_id: u8) -> Result<(), &'static str> {
        self.io_apic.set_irq_routing(irq, vector, dest_apic_id)
    }

    pub fn enable_irq(&self, irq: u8) -> Result<(), &'static str> {
        self.io_apic.enable_irq(irq)
    }

    pub fn disable_irq(&self, irq: u8) -> Result<(), &'static str> {
        self.io_apic.disable_irq(irq)
    }

    pub fn send_ipi(&self, target_apic_id: u8, vector: u8) -> Result<(), &'static str> {
        self.local_apic
            .send_ipi(target_apic_id, vector, DELIVERY_MODE_FIXED)
    }

    pub fn eoi(&self) {
        self.local_apic.eoi();
    }

    pub fn dispatch_interrupt(&self, vector: u8) -> Result<(), &'static str> {
        self.dispatch_table.dispatch(vector)
    }

    pub fn local_apic_id(&self) -> LocalApicId {
        self.local_apic.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_apic_creation() {
        let apic = LocalApic::new(APIC_BASE);
        assert!(!apic.is_enabled());
    }

    #[test]
    fn test_io_apic_creation() {
        let io_apic = IoApic::new(IO_APIC_BASE);
        assert!(!io_apic.is_enabled);
    }

    #[test]
    fn test_apic_manager_creation() {
        let _manager = ApicManager::new();
    }

    #[test]
    fn test_redir_entry_encoding() {
        let entry = IoApicRedirectionEntry {
            vector: 32,
            delivery_mode: DELIVERY_MODE_FIXED,
            dest_mode: DEST_MODE_PHYSICAL,
            delivery_status: false,
            polarity: POLARITY_ACTIVE_HIGH,
            remote_irr: false,
            trigger_mode: TRIGGER_MODE_EDGE,
            masked: false,
            destination: 0,
        };

        let encoded = entry.to_u64();
        let decoded = IoApicRedirectionEntry::from_u64(encoded);

        assert_eq!(decoded.vector, entry.vector);
        assert_eq!(decoded.delivery_mode, entry.delivery_mode);
    }
}

// SPDX-License-Identifier: MIT
// SigmaOS Interrupt Subsystem
// Interrupt handling and management inspired by Linux IRQ subsystem

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};

/// Interrupt number
pub type IrqNumber = u32;

/// Interrupt vector
pub type IrqVector = u32;

/// Interrupt handler type
pub type IrqHandler = fn(IrqNumber) -> Result<(), &'static str>;

/// Interrupt flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IrqFlags {
    pub disabled: bool,
    pub shared: bool,
    pub level_triggered: bool,
}

impl IrqFlags {
    pub fn new(disabled: bool, shared: bool, level_triggered: bool) -> Self {
        IrqFlags {
            disabled,
            shared,
            level_triggered,
        }
    }
}

/// Interrupt descriptor
#[derive(Debug)]
pub struct IrqDescriptor {
    pub irq: IrqNumber,
    pub vector: IrqVector,
    pub handler: Option<IrqHandler>,
    pub flags: IrqFlags,
    pub name: String,
    pub count: AtomicU64,
    pub spurious_count: AtomicU32,
}

impl IrqDescriptor {
    pub fn new(irq: IrqNumber, vector: IrqVector, name: String, flags: IrqFlags) -> Self {
        IrqDescriptor {
            irq,
            vector,
            handler: None,
            flags,
            name,
            count: AtomicU64::new(0),
            spurious_count: AtomicU32::new(0),
        }
    }

    pub fn set_handler(&mut self, handler: IrqHandler) {
        self.handler = Some(handler);
    }

    pub fn increment_count(&self) {
        self.count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn increment_spurious(&self) {
        self.spurious_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_count(&self) -> u64 {
        self.count.load(Ordering::SeqCst)
    }

    pub fn get_spurious_count(&self) -> u32 {
        self.spurious_count.load(Ordering::SeqCst)
    }
}

/// Interrupt controller
#[derive(Debug)]
pub struct InterruptController {
    pub name: String,
    pub irq_base: IrqNumber,
    pub irq_count: u32,
    pub enabled: AtomicU32, // 0 = disabled, 1 = enabled
}

impl InterruptController {
    pub fn new(name: String, irq_base: IrqNumber, irq_count: u32) -> Self {
        InterruptController {
            name,
            irq_base,
            irq_count,
            enabled: AtomicU32::new(1),
        }
    }

    pub fn enable(&self) {
        self.enabled.store(1, Ordering::SeqCst);
    }

    pub fn disable(&self) {
        self.enabled.store(0, Ordering::SeqCst);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }

    pub fn handles_irq(&self, irq: IrqNumber) -> bool {
        irq >= self.irq_base && irq < self.irq_base + self.irq_count
    }
}

/// Interrupt subsystem
#[derive(Debug)]
pub struct InterruptSubsystem {
    irqs: BTreeMap<IrqNumber, IrqDescriptor>,
    controllers: BTreeMap<String, InterruptController>,
    next_vector: AtomicU32,
    global_enable: AtomicU32, // 0 = disabled, 1 = enabled
}

impl InterruptSubsystem {
    pub fn new() -> Self {
        InterruptSubsystem {
            irqs: BTreeMap::new(),
            controllers: BTreeMap::new(),
            next_vector: AtomicU32::new(32),
            global_enable: AtomicU32::new(1),
        }
    }

    /// Register an interrupt controller
    pub fn register_controller(&mut self, controller: InterruptController) {
        self.controllers.insert(controller.name.clone(), controller);
    }

    /// Request an IRQ
    pub fn request_irq(&mut self, irq: IrqNumber, name: String, flags: IrqFlags) -> Result<IrqVector, &'static str> {
        if self.irqs.contains_key(&irq) {
            return Err("IRQ already in use");
        }

        let vector = self.next_vector.fetch_add(1, Ordering::SeqCst);
        let descriptor = IrqDescriptor::new(irq, vector, name, flags);
        self.irqs.insert(irq, descriptor);
        Ok(vector)
    }

    /// Free an IRQ
    pub fn free_irq(&mut self, irq: IrqNumber) -> Result<(), &'static str> {
        self.irqs.remove(&irq).ok_or("IRQ not found")?;
        Ok(())
    }

    /// Set IRQ handler
    pub fn set_irq_handler(&mut self, irq: IrqNumber, handler: IrqHandler) -> Result<(), &'static str> {
        let descriptor = self.irqs.get_mut(&irq).ok_or("IRQ not found")?;
        descriptor.set_handler(handler);
        Ok(())
    }

    /// Enable IRQ
    pub fn enable_irq(&mut self, irq: IrqNumber) -> Result<(), &'static str> {
        let descriptor = self.irqs.get_mut(&irq).ok_or("IRQ not found")?;
        descriptor.flags.disabled = false;
        Ok(())
    }

    /// Disable IRQ
    pub fn disable_irq(&mut self, irq: IrqNumber) -> Result<(), &'static str> {
        let descriptor = self.irqs.get_mut(&irq).ok_or("IRQ not found")?;
        descriptor.flags.disabled = true;
        Ok(())
    }

    /// Handle interrupt
    pub fn handle_interrupt(&mut self, irq: IrqNumber) -> Result<(), &'static str> {
        if !self.is_global_enabled() {
            return Err("Interrupts globally disabled");
        }

        let descriptor = self.irqs.get(&irq).ok_or("IRQ not found")?;
        
        if descriptor.flags.disabled {
            return Err("IRQ is disabled");
        }

        descriptor.increment_count();

        if let Some(handler) = descriptor.handler {
            handler(irq)?;
        }

        Ok(())
    }

    /// Enable all interrupts globally
    pub fn enable_all(&self) {
        self.global_enable.store(1, Ordering::SeqCst);
    }

    /// Disable all interrupts globally
    pub fn disable_all(&self) {
        self.global_enable.store(0, Ordering::SeqCst);
    }

    /// Check if interrupts are globally enabled
    pub fn is_global_enabled(&self) -> bool {
        self.global_enable.load(Ordering::SeqCst) == 1
    }

    /// Get IRQ count
    pub fn irq_count(&self) -> usize {
        self.irqs.len()
    }

    /// Get controller count
    pub fn controller_count(&self) -> usize {
        self.controllers.len()
    }

    /// Get IRQ statistics
    pub fn get_irq_stats(&self, irq: IrqNumber) -> Option<(u64, u32)> {
        let descriptor = self.irqs.get(&irq)?;
        Some((descriptor.get_count(), descriptor.get_spurious_count()))
    }
}

impl Default for InterruptSubsystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interrupt_controller_registration() {
        let mut subsystem = InterruptSubsystem::new();
        
        let controller = InterruptController::new("PIC".to_string(), 0, 16);
        subsystem.register_controller(controller);
        
        assert_eq!(subsystem.controller_count(), 1);
    }

    #[test]
    fn test_irq_request() {
        let mut subsystem = InterruptSubsystem::new();
        
        let flags = IrqFlags::new(false, false, true);
        let vector = subsystem.request_irq(1, "test".to_string(), flags).unwrap();
        
        assert!(vector >= 32);
        assert_eq!(subsystem.irq_count(), 1);
    }

    #[test]
    fn test_irq_handler() {
        let mut subsystem = InterruptSubsystem::new();
        
        let flags = IrqFlags::new(false, false, true);
        subsystem.request_irq(1, "test".to_string(), flags).unwrap();
        
        let handler: IrqHandler = |_irq| Ok(());
        subsystem.set_irq_handler(1, handler).unwrap();
    }

    #[test]
    fn test_irq_enable_disable() {
        let mut subsystem = InterruptSubsystem::new();
        
        let flags = IrqFlags::new(false, false, true);
        subsystem.request_irq(1, "test".to_string(), flags).unwrap();
        
        subsystem.disable_irq(1).unwrap();
        subsystem.enable_irq(1).unwrap();
    }

    #[test]
    fn test_interrupt_handling() {
        let mut subsystem = InterruptSubsystem::new();
        
        let flags = IrqFlags::new(false, false, true);
        subsystem.request_irq(1, "test".to_string(), flags).unwrap();
        
        let handler: IrqHandler = |_irq| Ok(());
        subsystem.set_irq_handler(1, handler).unwrap();
        
        subsystem.handle_interrupt(1).unwrap();
        
        let stats = subsystem.get_irq_stats(1).unwrap();
        assert_eq!(stats.0, 1); // count
    }

    #[test]
    fn test_global_enable_disable() {
        let subsystem = InterruptSubsystem::new();
        
        subsystem.disable_all();
        assert!(!subsystem.is_global_enabled());
        
        subsystem.enable_all();
        assert!(subsystem.is_global_enabled());
    }
}

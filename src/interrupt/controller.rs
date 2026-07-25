#![no_std]
#![no_main]

/// OOP-based Interrupt/IRQ Controller for SigmaOS
/// Based on Roadmap Item: Interrupt/IRQ Controller + APIC/GIC Support

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type IRQNumber = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IRQState { Disabled = 0, Enabled = 1, Pending = 2, InService = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ControllerType { APIC = 0, GIC = 1, PLIC = 2 }

pub trait InterruptController {
    fn controller_type(&self) -> ControllerType;
    fn enable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
    fn disable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
    fn get_irq_state(&self, irq: IRQNumber) -> IRQState;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IRQError { Success = 0, InvalidIRQ = 1, ControllerError = 2 }

#[repr(C)]
pub struct SimpleInterruptController {
    pub controller_type: ControllerType,
    pub irq_states: [AtomicUsize; 256],
}

impl SimpleInterruptController {
    pub fn new(controller_type: ControllerType) -> Self {
        let mut irq_states = [AtomicUsize::new(IRQState::Disabled as usize); 256];
        SimpleInterruptController { controller_type, irq_states }
    }
}

impl InterruptController for SimpleInterruptController {
    fn controller_type(&self) -> ControllerType { self.controller_type }
    fn enable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError> {
        if irq >= 256 { return Err(IRQError::InvalidIRQ); }
        self.irq_states[irq].store(IRQState::Enabled as usize, Ordering::SeqCst);
        Ok(())
    }
    fn disable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError> {
        if irq >= 256 { return Err(IRQError::InvalidIRQ); }
        self.irq_states[irq].store(IRQState::Disabled as usize, Ordering::SeqCst);
        Ok(())
    }
    fn get_irq_state(&self, irq: IRQNumber) -> IRQState {
        if irq >= 256 { return IRQState::Disabled; }
        unsafe { core::mem::transmute(self.irq_states[irq].load(Ordering::SeqCst)) }
    }
}

pub trait IRQHandler {
    fn handle_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
}

#[repr(C)]
pub struct SimpleIRQHandler {
    pub controller: SimpleInterruptController,
}

impl SimpleIRQHandler {
    pub fn new(controller_type: ControllerType) -> Self {
        SimpleIRQHandler { controller: SimpleInterruptController::new(controller_type) }
    }
}

impl IRQHandler for SimpleIRQHandler {
    fn handle_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError> {
        self.controller.irq_states[irq].store(IRQState::InService as usize, Ordering::SeqCst);
        self.controller.irq_states[irq].store(IRQState::Enabled as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait APICSupport {
    fn init_apic(&mut self) -> Result<(), IRQError>;
    fn send_ipi(&mut self, target: usize, vector: usize) -> Result<(), IRQError>;
}

impl APICSupport for SimpleInterruptController {
    fn init_apic(&mut self) -> Result<(), IRQError> {
        for i in 0..256 {
            self.irq_states[i].store(IRQState::Disabled as usize, Ordering::SeqCst);
        }
        Ok(())
    }
    fn send_ipi(&mut self, _target: usize, _vector: usize) -> Result<(), IRQError> {
        Ok(())
    }
}

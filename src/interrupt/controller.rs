#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// OOP-based Interrupt/IRQ Controller for SigmaOS
// Based on APIC/GIC Support specifications.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type IRQNumber = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IRQState {
    Disabled = 0,
    Enabled = 1,
    Pending = 2,
    InService = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerType {
    APIC = 0,
    GIC = 1,
    PLIC = 2,
}

pub trait InterruptController {
    fn controller_type(&self) -> ControllerType;
    fn enable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
    fn disable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
    fn get_irq_state(&self, irq: IRQNumber) -> IRQState;
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IRQError {
    Success = 0,
    InvalidIRQ = 1,
    ControllerError = 2,
}

pub struct SimpleInterruptController {
    pub controller_type: ControllerType,
    pub irq_states: Vec<AtomicUsize>,
}

impl SimpleInterruptController {
    pub fn new(controller_type: ControllerType) -> Self {
        let mut irq_states = Vec::new();
        for _ in 0..256 {
            irq_states.push(AtomicUsize::new(IRQState::Disabled as usize));
        }
        SimpleInterruptController {
            controller_type,
            irq_states,
        }
    }
}

impl InterruptController for SimpleInterruptController {
    fn controller_type(&self) -> ControllerType {
        self.controller_type
    }

    fn enable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError> {
        if irq >= 256 {
            return Err(IRQError::InvalidIRQ);
        }
        self.irq_states[irq].store(IRQState::Enabled as usize, Ordering::SeqCst);
        Ok(())
    }

    fn disable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError> {
        if irq >= 256 {
            return Err(IRQError::InvalidIRQ);
        }
        self.irq_states[irq].store(IRQState::Disabled as usize, Ordering::SeqCst);
        Ok(())
    }

    fn get_irq_state(&self, irq: IRQNumber) -> IRQState {
        if irq >= 256 {
            return IRQState::Disabled;
        }
        match self.irq_states[irq].load(Ordering::SeqCst) {
            0 => IRQState::Disabled,
            1 => IRQState::Enabled,
            2 => IRQState::Pending,
            _ => IRQState::InService,
        }
    }
}

pub trait IRQHandler {
    fn handle_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
}

pub struct SimpleIRQHandler {
    pub controller: SimpleInterruptController,
}

impl SimpleIRQHandler {
    pub fn new(controller_type: ControllerType) -> Self {
        SimpleIRQHandler {
            controller: SimpleInterruptController::new(controller_type),
        }
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
        for state in &self.irq_states {
            state.store(IRQState::Disabled as usize, Ordering::SeqCst);
        }
        Ok(())
    }

    fn send_ipi(&mut self, _target: usize, _vector: usize) -> Result<(), IRQError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irq_controller_flows() {
        let mut controller = SimpleInterruptController::new(ControllerType::APIC);
        assert_eq!(controller.get_irq_state(42), IRQState::Disabled);

        controller.enable_irq(42).unwrap();
        assert_eq!(controller.get_irq_state(42), IRQState::Enabled);

        controller.disable_irq(42).unwrap();
        assert_eq!(controller.get_irq_state(42), IRQState::Disabled);
    }
}

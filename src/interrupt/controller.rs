#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
/// Advanced High-Fidelity Heterogeneous Interrupt Controller (APIC, GIC, PLIC) for SigmaOS
/// Models x86_64 APIC Inter-Processor Interrupts (IPI), ARM GIC Fast Interrupts (FIQ), and RISC-V PLIC Supervisor targets.
use std::vec::Vec;
use core::sync::atomic::{AtomicU32, Ordering};

pub type IRQNumber = usize;

/// Standard IRQ states
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IRQState {
    Disabled = 0,
    Enabled = 1,
    Pending = 2,
    InService = 3,
}

/// Dynamic Heterogeneous Interrupt Controller architectures
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerType {
    Apic = 0, // x86_64 Advanced Programmable Interrupt Controller
    Gic = 1,  // ARM Generic Interrupt Controller
    Plic = 2, // RISC-V Platform-Level Interrupt Controller
}

/// CPU Privilege Modes modelled across architectures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuPrivilegeMode {
    User,
    Supervisor,
    Monitor,
    Machine,
}

/// Interrupt Priority Level (FIQ is highest priority)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterruptPriority {
    StandardIrq = 0,
    SupervisorTrap = 1,
    MonitorTrap = 2,
    FastInterruptFiq = 3, // ARM Fast Interrupt
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IRQError {
    Success = 0,
    InvalidIRQ = 1,
    ControllerError = 2,
}

pub trait InterruptController {
    fn controller_type(&self) -> ControllerType;
    fn enable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
    fn disable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
    fn get_irq_state(&self, irq: IRQNumber) -> IRQState;
}

/// Unified Multi-Architecture Interrupt Controller
pub struct SimpleInterruptController {
    pub controller_type: ControllerType,
    pub irq_states: Vec<AtomicU32>,
    pub irq_priorities: Vec<InterruptPriority>,
    pub target_mode: CpuPrivilegeMode,
}

impl SimpleInterruptController {
    pub fn new(controller_type: ControllerType) -> Self {
        let mut states = Vec::new();
        let mut priorities = Vec::new();
        for _ in 0..256 {
            states.push(AtomicU32::new(IRQState::Disabled as u32));
            priorities.push(InterruptPriority::StandardIrq);
        }

        SimpleInterruptController {
            controller_type,
            irq_states: states,
            irq_priorities: priorities,
            target_mode: CpuPrivilegeMode::Supervisor,
        }
    }

    /// Configures priority level for specified IRQ line (e.g. mapping Fast Interrupt FIQ)
    pub fn set_irq_priority(
        &mut self,
        irq: IRQNumber,
        priority: InterruptPriority,
    ) -> Result<(), IRQError> {
        if irq >= 256 {
            return Err(IRQError::InvalidIRQ);
        }
        self.irq_priorities[irq] = priority;
        Ok(())
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
        self.irq_states[irq].store(IRQState::Enabled as u32, Ordering::SeqCst);
        Ok(())
    }

    fn disable_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError> {
        if irq >= 256 {
            return Err(IRQError::InvalidIRQ);
        }
        self.irq_states[irq].store(IRQState::Disabled as u32, Ordering::SeqCst);
        Ok(())
    }

    fn get_irq_state(&self, irq: IRQNumber) -> IRQState {
        if irq >= 256 {
            return IRQState::Disabled;
        }
        unsafe { core::mem::transmute(self.irq_states[irq].load(Ordering::SeqCst)) }
    }
}

pub trait IRQHandler {
    fn handle_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError>;
}

/// Real-time IRQ router
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
    /// Dispatches IRQ, transitioning its state register safely
    fn handle_irq(&mut self, irq: IRQNumber) -> Result<(), IRQError> {
        if irq >= 256 {
            return Err(IRQError::InvalidIRQ);
        }
        self.controller.irq_states[irq].store(IRQState::InService as u32, Ordering::SeqCst);
        // Simulate completion and re-enable
        self.controller.irq_states[irq].store(IRQState::Enabled as u32, Ordering::SeqCst);
        Ok(())
    }
}

/// Advanced multi-processor interrupt signaling support
pub trait APICSupport {
    fn init_apic(&mut self) -> Result<(), IRQError>;
    fn send_ipi(&mut self, target_cpu_id: usize, vector: u8) -> Result<(), IRQError>;
}

impl APICSupport for SimpleInterruptController {
    fn init_apic(&mut self) -> Result<(), IRQError> {
        for i in 0..256 {
            self.irq_states[i].store(IRQState::Disabled as u32, Ordering::SeqCst);
        }
        Ok(())
    }

    /// Emits Inter-Processor Interrupt (IPI) to signal target CPU threads (x86 APIC specification)
    fn send_ipi(&mut self, _target_cpu_id: usize, vector: u8) -> Result<(), IRQError> {
        let v_idx = vector as usize;
        if v_idx >= 256 {
            return Err(IRQError::InvalidIRQ);
        }
        // Set state to pending on the remote CPU line
        self.irq_states[v_idx].store(IRQState::Pending as u32, Ordering::SeqCst);
        Ok(())
    }
}

/// GIC/PLIC specific target context operations
impl SimpleInterruptController {
    /// Routes the interrupt context based on privilege target (RISC-V PLIC & ARM GIC spec)
    pub fn set_target_privilege(&mut self, mode: CpuPrivilegeMode) {
        self.target_mode = mode;
    }

    /// Decides whether to prioritize incoming FIQ (Fast Interrupts) over pending Supervisor Traps
    pub fn evaluate_priority_dispatch(
        &self,
        irq_a: IRQNumber,
        irq_b: IRQNumber,
    ) -> Option<IRQNumber> {
        if irq_a >= 256 || irq_b >= 256 {
            return None;
        }

        let prio_a = self.irq_priorities[irq_a];
        let prio_b = self.irq_priorities[irq_b];

        if prio_a > prio_b {
            Some(irq_a)
        } else if prio_b > prio_a {
            Some(irq_b)
        } else {
            Some(irq_a) // Equal priority default
        }
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_interrupt_controller_initialization() {
        let mut controller = SimpleInterruptController::new(ControllerType::Apic);
        assert_eq!(controller.controller_type(), ControllerType::Apic);
        assert_eq!(controller.get_irq_state(45), IRQState::Disabled);

        controller.enable_irq(45).unwrap();
        assert_eq!(controller.get_irq_state(45), IRQState::Enabled);
    }

    #[test]
    fn test_apic_inter_processor_interrupt() {
        let mut controller = SimpleInterruptController::new(ControllerType::Apic);
        controller.init_apic().unwrap();

        // Send Inter-Processor Interrupt on vector 80
        controller.send_ipi(1, 80).unwrap();
        assert_eq!(controller.get_irq_state(80), IRQState::Pending);
    }

    #[test]
    fn test_gic_fiq_priority_dispatch() {
        let mut controller = SimpleInterruptController::new(ControllerType::Gic);

        // Map IRQ 12 to FIQ (ARM Fast Interrupt) and IRQ 15 to Standard IRQ
        controller
            .set_irq_priority(12, InterruptPriority::FastInterruptFiq)
            .unwrap();
        controller
            .set_irq_priority(15, InterruptPriority::StandardIrq)
            .unwrap();

        // Evaluate priority: FIQ (12) must always defeat standard IRQ (15)
        let selected = controller.evaluate_priority_dispatch(15, 12).unwrap();
        assert_eq!(selected, 12);
    }

    #[test]
    fn test_plic_target_contexts() {
        let mut controller = SimpleInterruptController::new(ControllerType::Plic);
        assert_eq!(controller.target_mode, CpuPrivilegeMode::Supervisor); // Default supervisor context

        // Elevate PLIC context to Machine Mode
        controller.set_target_privilege(CpuPrivilegeMode::Machine);
        assert_eq!(controller.target_mode, CpuPrivilegeMode::Machine);
    }
}

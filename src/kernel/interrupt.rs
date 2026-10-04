// Interrupt Handling Framework
// Inspired by Linux and BSD interrupt handling with IDT and IRQ management

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering};

/// Interrupt vector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptVector {
    pub number: u8,
    pub type_id: InterruptType,
}

/// Interrupt type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptType {
    Exception,
    Irq,
    SoftwareInterrupt,
    Trap,
}

/// Interrupt handler function type
pub type InterruptHandler = fn(InterruptVector, u64) -> ();

/// Interrupt descriptor
#[derive(Debug)]
pub struct InterruptDescriptor {
    pub vector: InterruptVector,
    pub handler: Option<InterruptHandler>,
    pub handler_data: u64,
    pub enabled: bool,
    pub count: AtomicU32,
}

/// IRQ line
#[derive(Debug, Clone)]
pub struct IrqLine {
    pub number: u32,
    pub trigger_type: IrqTriggerType,
    pub handler: Option<InterruptHandler>,
    pub handler_data: u64,
    pub enabled: bool,
    pub pending: bool,
}

/// IRQ trigger type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrqTriggerType {
    Edge,
    Level,
}

/// Interrupt controller
pub struct InterruptController {
    next_vector: AtomicU8,
    descriptors: HashMap<u8, InterruptDescriptor>,
    irq_lines: HashMap<u32, IrqLine>,
    interrupt_count: AtomicU64,
}

impl InterruptController {
    pub fn new() -> Self {
        Self {
            next_vector: AtomicU8::new(32), // Start from 32 (exceptions use 0-31)
            descriptors: HashMap::new(),
            irq_lines: HashMap::new(),
            interrupt_count: AtomicU64::new(0),
        }
    }

    /// Allocate an interrupt vector
    pub fn allocate_vector(&self, int_type: InterruptType) -> InterruptVector {
        let number = self.next_vector.fetch_add(1, Ordering::SeqCst);

        InterruptVector {
            number,
            type_id: int_type,
        }
    }

    /// Register an interrupt handler
    pub fn register_handler(
        &mut self,
        vector: InterruptVector,
        handler: InterruptHandler,
        data: u64,
    ) -> Result<(), &'static str> {
        let descriptor = InterruptDescriptor {
            vector,
            handler: Some(handler),
            handler_data: data,
            enabled: true,
            count: AtomicU32::new(0),
        };

        self.descriptors.insert(vector.number, descriptor);
        Ok(())
    }

    /// Unregister an interrupt handler
    pub fn unregister_handler(&mut self, vector: InterruptVector) -> Result<(), &'static str> {
        if let Some(desc) = self.descriptors.get_mut(&vector.number) {
            desc.handler = None;
            desc.enabled = false;
            Ok(())
        } else {
            Err("Interrupt descriptor not found")
        }
    }

    /// Enable an interrupt
    pub fn enable_interrupt(&mut self, vector: InterruptVector) -> Result<(), &'static str> {
        if let Some(desc) = self.descriptors.get_mut(&vector.number) {
            desc.enabled = true;
            Ok(())
        } else {
            Err("Interrupt descriptor not found")
        }
    }

    /// Disable an interrupt
    pub fn disable_interrupt(&mut self, vector: InterruptVector) -> Result<(), &'static str> {
        if let Some(desc) = self.descriptors.get_mut(&vector.number) {
            desc.enabled = false;
            Ok(())
        } else {
            Err("Interrupt descriptor not found")
        }
    }

    /// Register an IRQ line
    pub fn register_irq(
        &mut self,
        irq: u32,
        trigger_type: IrqTriggerType,
    ) -> Result<(), &'static str> {
        if self.irq_lines.contains_key(&irq) {
            return Err("IRQ already registered");
        }

        let irq_line = IrqLine {
            number: irq,
            trigger_type,
            handler: None,
            handler_data: 0,
            enabled: false,
            pending: false,
        };

        self.irq_lines.insert(irq, irq_line);
        Ok(())
    }

    /// Register IRQ handler
    pub fn register_irq_handler(
        &mut self,
        irq: u32,
        handler: InterruptHandler,
        data: u64,
    ) -> Result<(), &'static str> {
        if let Some(irq_line) = self.irq_lines.get_mut(&irq) {
            irq_line.handler = Some(handler);
            irq_line.handler_data = data;
            Ok(())
        } else {
            Err("IRQ not found")
        }
    }

    /// Enable IRQ
    pub fn enable_irq(&mut self, irq: u32) -> Result<(), &'static str> {
        if let Some(irq_line) = self.irq_lines.get_mut(&irq) {
            irq_line.enabled = true;
            Ok(())
        } else {
            Err("IRQ not found")
        }
    }

    /// Disable IRQ
    pub fn disable_irq(&mut self, irq: u32) -> Result<(), &'static str> {
        if let Some(irq_line) = self.irq_lines.get_mut(&irq) {
            irq_line.enabled = false;
            Ok(())
        } else {
            Err("IRQ not found")
        }
    }

    /// Handle an interrupt
    pub fn handle_interrupt(&mut self, vector: InterruptVector) -> Result<(), &'static str> {
        self.interrupt_count.fetch_add(1, Ordering::SeqCst);

        if let Some(desc) = self.descriptors.get_mut(&vector.number) {
            if !desc.enabled {
                return Err("Interrupt disabled");
            }

            desc.count.fetch_add(1, Ordering::SeqCst);

            if let Some(handler) = desc.handler {
                handler(vector, desc.handler_data);
            }

            Ok(())
        } else {
            Err("Interrupt descriptor not found")
        }
    }

    /// Handle an IRQ
    pub fn handle_irq(&mut self, irq: u32) -> Result<(), &'static str> {
        if let Some(irq_line) = self.irq_lines.get_mut(&irq) {
            if !irq_line.enabled {
                return Err("IRQ disabled");
            }

            irq_line.pending = false;

            if let Some(handler) = irq_line.handler {
                handler(
                    InterruptVector {
                        number: irq as u8,
                        type_id: InterruptType::Irq,
                    },
                    irq_line.handler_data,
                );
            }

            Ok(())
        } else {
            Err("IRQ not found")
        }
    }

    /// Get interrupt count
    pub fn interrupt_count(&self) -> u64 {
        self.interrupt_count.load(Ordering::SeqCst)
    }

    /// Get descriptor by vector
    pub fn get_descriptor(&self, vector: u8) -> Option<&InterruptDescriptor> {
        self.descriptors.get(&vector)
    }

    /// Get IRQ line
    pub fn get_irq_line(&self, irq: u32) -> Option<&IrqLine> {
        self.irq_lines.get(&irq)
    }

    /// Get descriptor count
    pub fn descriptor_count(&self) -> usize {
        self.descriptors.len()
    }

    /// Get IRQ count
    pub fn irq_count(&self) -> usize {
        self.irq_lines.len()
    }
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_vector() {
        let controller = InterruptController::new();

        let vector = controller.allocate_vector(InterruptType::Irq);
        assert_eq!(vector.number, 32);
    }

    #[test]
    fn test_register_handler() {
        let mut controller = InterruptController::new();

        let vector = controller.allocate_vector(InterruptType::Irq);
        let handler: InterruptHandler = |_, _| {};

        assert!(controller.register_handler(vector, handler, 0).is_ok());
        assert_eq!(controller.descriptor_count(), 1);
    }

    #[test]
    fn test_enable_disable_interrupt() {
        let mut controller = InterruptController::new();

        let vector = controller.allocate_vector(InterruptType::Irq);
        let handler: InterruptHandler = |_, _| {};

        controller.register_handler(vector, handler, 0).unwrap();
        assert!(controller.disable_interrupt(vector).is_ok());
        assert!(controller.enable_interrupt(vector).is_ok());
    }

    #[test]
    fn test_register_irq() {
        let mut controller = InterruptController::new();

        assert!(controller.register_irq(1, IrqTriggerType::Edge).is_ok());
        assert_eq!(controller.irq_count(), 1);
    }

    #[test]
    fn test_irq_handler() {
        let mut controller = InterruptController::new();

        controller.register_irq(1, IrqTriggerType::Edge).unwrap();
        let handler: InterruptHandler = |_, _| {};

        assert!(controller.register_irq_handler(1, handler, 0).is_ok());
        assert!(controller.enable_irq(1).is_ok());
    }

    #[test]
    fn test_handle_interrupt() {
        let mut controller = InterruptController::new();

        let vector = controller.allocate_vector(InterruptType::Irq);
        let handler: InterruptHandler = |_, _| {};

        controller.register_handler(vector, handler, 0).unwrap();
        assert!(controller.handle_interrupt(vector).is_ok());

        assert_eq!(controller.interrupt_count(), 1);
    }
}

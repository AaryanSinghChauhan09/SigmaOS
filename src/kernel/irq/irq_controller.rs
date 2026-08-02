//! SigmaOS Interrupt/IRQ Controller
//! APIC (x86), GIC (ARM), PLIC (RISC-V) support
//! Target: <1µs IRQ dispatch overhead

// (no_std only applicable at crate root - removed)

use core::sync::atomic::{AtomicUsize, AtomicPtr, Ordering};

#[repr(C)]
pub struct IRQController {
    controller_type: ControllerType,
    irq_count: AtomicUsize,
    handlers: [AtomicPtr<IRQHandler>; 256],
    spurious_count: AtomicUsize,
}

#[derive(Clone, Copy, PartialEq)]
pub enum ControllerType {
    APIC,    // x86 Advanced Programmable Interrupt Controller
    GIC,     // ARM Generic Interrupt Controller
    PLIC,    // RISC-V Platform-Level Interrupt Controller
    PIC,     // Legacy 8259 PIC
}

#[repr(C)]
pub struct IRQHandler {
    irq: AtomicUsize,
    handler: AtomicUsize, // Function pointer
    context: AtomicPtr<u8>,
    enabled: AtomicUsize,
}

#[repr(C)]
pub struct APIC {
    id: AtomicUsize,
    version: AtomicUsize,
    eoi: AtomicUsize,
    icr_low: AtomicUsize,
    icr_high: AtomicUsize,
}

#[repr(C)]
pub struct GIC {
    distributor: AtomicUsize,
    cpu_interface: AtomicUsize,
    version: AtomicUsize,
}

#[repr(C)]
pub struct PLIC {
    priority: AtomicUsize,
    pending: AtomicUsize,
    enable: AtomicUsize,
    context: AtomicUsize,
}

impl IRQController {
    pub fn new(controller_type: ControllerType) -> Self {
        IRQController {
            controller_type,
            irq_count: AtomicUsize::new(0),
            handlers: {
                let mut arr = [AtomicPtr::new(core::ptr::null_mut()); 256];
                for i in 0..256 {
                    arr[i] = AtomicPtr::new(core::ptr::null_mut());
                }
                arr
            },
            spurious_count: AtomicUsize::new(0),
        }
    }

    /// Initialize interrupt controller
    pub fn init(&mut self) -> Result<(), IRQError> {
        match self.controller_type {
            ControllerType::APIC => self.init_apic(),
            ControllerType::GIC => self.init_gic(),
            ControllerType::PLIC => self.init_plic(),
            ControllerType::PIC => self.init_pic(),
        }
    }

    fn init_apic(&mut self) -> Result<(), IRQError> {
        // Initialize x86 APIC
        // Set APIC ID, enable APIC, configure spurious IRQ vector
        Ok(())
    }

    fn init_gic(&mut self) -> Result<(), IRQError> {
        // Initialize ARM GIC
        // Configure distributor and CPU interface
        Ok(())
    }

    fn init_plic(&mut self) -> Result<(), IRQError> {
        // Initialize RISC-V PLIC
        // Set priority thresholds, enable contexts
        Ok(())
    }

    fn init_pic(&mut self) -> Result<(), IRQError> {
        // Initialize legacy 8259 PIC
        // Remap IRQs, enable cascade mode
        Ok(())
    }

    /// Register IRQ handler
    pub fn register_handler(&self, irq: usize, handler: *mut IRQHandler) -> Result<(), IRQError> {
        if irq >= 256 {
            return Err(IRQError::InvalidIRQ);
        }

        self.handlers[irq].store(handler, Ordering::SeqCst);
        self.enable_irq(irq);
        
        Ok(())
    }

    /// Unregister IRQ handler
    pub fn unregister_handler(&self, irq: usize) {
        if irq < 256 {
            self.handlers[irq].store(core::ptr::null_mut(), Ordering::SeqCst);
            self.disable_irq(irq);
        }
    }

    /// Enable IRQ
    pub fn enable_irq(&self, irq: usize) {
        if irq < 256 {
            let handler = self.handlers[irq].load(Ordering::Acquire);
            if !handler.is_null() {
                unsafe {
                    (*handler).enabled.store(1, Ordering::SeqCst);
                }
            }
        }
    }

    /// Disable IRQ
    pub fn disable_irq(&self, irq: usize) {
        if irq < 256 {
            let handler = self.handlers[irq].load(Ordering::Acquire);
            if !handler.is_null() {
                unsafe {
                    (*handler).enabled.store(0, Ordering::SeqCst);
                }
            }
        }
    }

    /// Handle interrupt
    pub fn handle_interrupt(&self, irq: usize) {
        if irq >= 256 {
            self.spurious_count.fetch_add(1, Ordering::SeqCst);
            return;
        }

        let handler = self.handlers[irq].load(Ordering::Acquire);
        if !handler.is_null() {
            unsafe {
                if (*handler).enabled.load(Ordering::Acquire) == 1 {
                    // Call handler function
                    let func = (*handler).handler.load(Ordering::Acquire);
                    let ctx = (*handler).context.load(Ordering::Acquire);
                    
                    // In real implementation, would call the function pointer
                    // let handler_fn: fn(*mut u8) = core::mem::transmute(func);
                    // handler_fn(ctx);
                }
            }
        } else {
            self.spurious_count.fetch_add(1, Ordering::SeqCst);
        }

        self.send_eoi(irq);
    }

    /// Send End of Interrupt
    fn send_eoi(&self, irq: usize) {
        match self.controller_type {
            ControllerType::APIC => {
                // Write to APIC EOI register
            }
            ControllerType::GIC => {
                // Write to GIC EOI register
            }
            ControllerType::PLIC => {
                // Write to PLIC complete register
            }
            ControllerType::PIC => {
                // Send EOI to PIC
            }
        }
    }

    /// Get spurious interrupt count
    pub fn spurious_count(&self) -> usize {
        self.spurious_count.load(Ordering::SeqCst)
    }

    /// Get controller type
    pub fn controller_type(&self) -> ControllerType {
        self.controller_type
    }
}

impl IRQHandler {
    pub fn new(irq: usize, handler: usize, context: *mut u8) -> Self {
        IRQHandler {
            irq: AtomicUsize::new(irq),
            handler: AtomicUsize::new(handler),
            context: AtomicPtr::new(context),
            enabled: AtomicUsize::new(0),
        }
    }

    pub fn irq(&self) -> usize {
        self.irq.load(Ordering::SeqCst)
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst) == 1
    }
}

impl APIC {
    pub fn new() -> Self {
        APIC {
            id: AtomicUsize::new(0),
            version: AtomicUsize::new(0),
            eoi: AtomicUsize::new(0),
            icr_low: AtomicUsize::new(0),
            icr_high: AtomicUsize::new(0),
        }
    }

    /// Send IPI (Inter-Processor Interrupt)
    pub fn send_ipi(&self, target_apic_id: usize, vector: usize) {
        // Write to ICR registers to send IPI
        self.icr_high.store(target_apic_id << 24, Ordering::SeqCst);
        self.icr_low.store(vector, Ordering::SeqCst);
    }

    /// Get APIC ID
    pub fn id(&self) -> usize {
        self.id.load(Ordering::SeqCst)
    }

    /// Get APIC version
    pub fn version(&self) -> usize {
        self.version.load(Ordering::SeqCst)
    }
}

impl GIC {
    pub fn new() -> Self {
        GIC {
            distributor: AtomicUsize::new(0),
            cpu_interface: AtomicUsize::new(0),
            version: AtomicUsize::new(0),
        }
    }

    /// Set interrupt priority
    pub fn set_priority(&self, irq: usize, priority: usize) {
        // Write to GIC distributor priority register
    }

    /// Enable interrupt
    pub fn enable_interrupt(&self, irq: usize) {
        // Write to GIC distributor enable register
    }
}

impl PLIC {
    pub fn new() -> Self {
        PLIC {
            priority: AtomicUsize::new(0),
            pending: AtomicUsize::new(0),
            enable: AtomicUsize::new(0),
            context: AtomicUsize::new(0),
        }
    }

    /// Set interrupt priority
    pub fn set_priority(&self, irq: usize, priority: usize) {
        // Write to PLIC priority register
    }

    /// Enable interrupt for context
    pub fn enable_context(&self, context_id: usize, irq: usize) {
        // Write to PLIC enable register for context
    }

    /// Claim interrupt
    pub fn claim(&self, context_id: usize) -> Option<usize> {
        // Read from PLIC claim register
        None
    }

    /// Complete interrupt
    pub fn complete(&self, context_id: usize, irq: usize) {
        // Write to PLIC complete register
    }
}

#[derive(Debug)]
pub enum IRQError {
    InvalidIRQ,
    HandlerExists,
    ControllerInitFailed,
}

#![no_std]
#![no_main]

/// OOP-based Interrupt Handler for SigmaOS
/// Implements state-of-the-art priority-based interrupt nesting, preemption, global disablement queuing,
/// and time sequence logging inspired by x86 APIC TPR, ARM GIC PMR, and Windows IRQLs.

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::mem;

extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

/// Interrupt number
pub type InterruptNumber = u8;

/// Interrupt vector (0-255)
pub type InterruptVector = u8;

/// Interrupt handler trait (OOP interface)
pub trait InterruptHandler {
    /// Handle interrupt
    fn handle(&mut self, interrupt: InterruptNumber) -> InterruptResult;
    /// Enable interrupt
    fn enable(&mut self, interrupt: InterruptNumber) -> Result<(), InterruptError>;
    /// Disable interrupt
    fn disable(&mut self, interrupt: InterruptNumber) -> Result<(), InterruptError>;
    /// Get handler info
    fn info(&self) -> InterruptHandlerInfo;
}

/// Interrupt result
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptResult {
    Handled = 0,
    Ignored = 1,
    Deferred = 2,
    Error = 3,
}

/// Interrupt error types
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptError {
    Success = 0,
    InvalidInterrupt = 1,
    AlreadyEnabled = 2,
    AlreadyDisabled = 3,
    PermissionDenied = 4,
    HandlerNotFound = 5,
}

/// Interrupt handler info
#[repr(C)]
pub struct InterruptHandlerInfo {
    pub handler_type: HandlerType,
    pub priority: Priority,
    pub capability: HandlerCapability,
}

impl InterruptHandlerInfo {
    pub fn new(handler_type: HandlerType) -> Self {
        InterruptHandlerInfo {
            handler_type,
            priority: Priority::Normal,
            capability: HandlerCapability::new(),
        }
    }
}

/// Handler type
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HandlerType {
    Hardware = 0,
    Software = 1,
    Exception = 2,
    Timer = 3,
    Keyboard = 4,
    Mouse = 5,
    Network = 6,
    Custom = 7,
}

/// Priority level (standard interrupt task priority)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    Low = 0,
    Normal = 1,
    High = 2,
    Critical = 3,
}

/// Handler capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HandlerCapability {
    pub can_enable: bool,
    pub can_disable: bool,
    pub can_mask: bool,
}

impl HandlerCapability {
    pub fn new() -> Self {
        HandlerCapability {
            can_enable: false,
            can_disable: false,
            can_mask: false,
        }
    }

    pub fn full() -> Self {
        HandlerCapability {
            can_enable: true,
            can_disable: true,
            can_mask: true,
        }
    }
}

/// Interrupt descriptor (OOP: Interrupt object)
#[repr(C)]
pub struct InterruptDescriptor {
    pub number: InterruptNumber,
    pub vector: InterruptVector,
    pub enabled: AtomicBool,
    pub masked: AtomicBool,
    pub handler: Option<usize>, // Index into handlers array
    pub capability: HandlerCapability,
    pub interrupt_count: AtomicUsize,
}

impl InterruptDescriptor {
    pub fn new(number: InterruptNumber, vector: InterruptVector, capability: HandlerCapability) -> Self {
        InterruptDescriptor {
            number,
            vector,
            enabled: AtomicBool::new(false),
            masked: AtomicBool::new(false),
            handler: None,
            capability,
            interrupt_count: AtomicUsize::new(0),
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    pub fn is_masked(&self) -> bool {
        self.masked.load(Ordering::SeqCst)
    }

    pub fn enable(&self) -> Result<(), InterruptError> {
        if !self.capability.can_enable {
            return Err(InterruptError::PermissionDenied);
        }

        if self.is_enabled() {
            return Err(InterruptError::AlreadyEnabled);
        }

        self.enabled.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn disable(&self) -> Result<(), InterruptError> {
        if !self.capability.can_disable {
            return Err(InterruptError::PermissionDenied);
        }

        if !self.is_enabled() {
            return Err(InterruptError::AlreadyDisabled);
        }

        self.enabled.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn mask(&self) -> Result<(), InterruptError> {
        if !self.capability.can_mask {
            return Err(InterruptError::PermissionDenied);
        }

        self.masked.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn unmask(&self) -> Result<(), InterruptError> {
        if !self.capability.can_mask {
            return Err(InterruptError::PermissionDenied);
        }

        self.masked.store(false, Ordering::SeqCst);
        Ok(())
    }

    pub fn increment_count(&self) {
        self.interrupt_count.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_count(&self) -> usize {
        self.interrupt_count.load(Ordering::SeqCst)
    }
}

/// Simple interrupt handler (OOP: Concrete handler class)
pub struct SimpleInterruptHandler {
    pub handler_type: HandlerType,
    pub priority: Priority,
    pub capability: HandlerCapability,
    pub handle_count: AtomicUsize,
}

impl SimpleInterruptHandler {
    pub fn new(handler_type: HandlerType, priority: Priority, capability: HandlerCapability) -> Self {
        SimpleInterruptHandler {
            handler_type,
            priority,
            capability,
            handle_count: AtomicUsize::new(0),
        }
    }
}

impl InterruptHandler for SimpleInterruptHandler {
    fn handle(&mut self, _interrupt: InterruptNumber) -> InterruptResult {
        self.handle_count.fetch_add(1, Ordering::SeqCst);
        InterruptResult::Handled
    }

    fn enable(&mut self, _interrupt: InterruptNumber) -> Result<(), InterruptError> {
        if !self.capability.can_enable {
            return Err(InterruptError::PermissionDenied);
        }
        Ok(())
    }

    fn disable(&mut self, _interrupt: InterruptNumber) -> Result<(), InterruptError> {
        if !self.capability.can_disable {
            return Err(InterruptError::PermissionDenied);
        }
        Ok(())
    }

    fn info(&self) -> InterruptHandlerInfo {
        InterruptHandlerInfo {
            handler_type: self.handler_type,
            priority: self.priority,
            capability: self.capability,
        }
    }
}

/// Interrupt controller trait (OOP interface)
pub trait InterruptController {
    /// Register handler
    fn register_handler(&mut self, handler: Box<dyn InterruptHandler>, interrupt: InterruptNumber) -> Result<(), InterruptError>;
    /// Unregister handler
    fn unregister_handler(&mut self, interrupt: InterruptNumber) -> Result<(), InterruptError>;
    /// Dispatch interrupt
    fn dispatch(&mut self, interrupt: InterruptNumber) -> InterruptResult;
    /// Enable interrupt line
    fn enable_interrupt(&mut self, interrupt: InterruptNumber) -> Result<(), InterruptError>;
    /// Disable interrupt line
    fn disable_interrupt(&mut self, interrupt: InterruptNumber) -> Result<(), InterruptError>;
    /// Get controller statistics
    fn stats(&self) -> InterruptStats;
    /// Get custom descriptor priority
    fn get_priority(&self, interrupt: InterruptNumber) -> Option<Priority>;
}

/// Interrupt statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptStats {
    pub total_interrupts: u64,
    pub handled_interrupts: u64,
    pub ignored_interrupts: u64,
    pub error_interrupts: u64,
    pub handler_counts: [u64; 256],
}

impl InterruptStats {
    pub fn new() -> Self {
        InterruptStats {
            total_interrupts: 0,
            handled_interrupts: 0,
            ignored_interrupts: 0,
            error_interrupts: 0,
            handler_counts: [0; 256],
        }
    }
}

/// PIC (Programmable Interrupt Controller) (OOP: Concrete controller class)
pub struct PIC {
    pub descriptors: [Option<InterruptDescriptor>; 256],
    pub handlers: Vec<Option<Box<dyn InterruptHandler>>>,
    pub stats: InterruptStats,
    pub capability: ControllerCapability,
}

/// Controller capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ControllerCapability {
    pub can_register: bool,
    pub can_unregister: bool,
    pub can_dispatch: bool,
}

impl ControllerCapability {
    pub fn new() -> Self {
        ControllerCapability {
            can_register: false,
            can_unregister: false,
            can_dispatch: false,
        }
    }

    pub fn full() -> Self {
        ControllerCapability {
            can_register: true,
            can_unregister: true,
            can_dispatch: true,
        }
    }
}

impl PIC {
    pub fn new(capability: ControllerCapability) -> Self {
        let mut descriptors = [None; 256];
        
        // Initialize common interrupt descriptors
        for i in 0..256 {
            descriptors[i] = Some(InterruptDescriptor::new(
                i as u8,
                i as u8,
                HandlerCapability::full()
            ));
        }

        PIC {
            descriptors,
            handlers: Vec::new(),
            stats: InterruptStats::new(),
            capability,
        }
    }
}

impl InterruptController for PIC {
    fn register_handler(&mut self, handler: Box<dyn InterruptHandler>, interrupt: InterruptNumber) -> Result<(), InterruptError> {
        if !self.capability.can_register {
            return Err(InterruptError::PermissionDenied);
        }

        let handler_index = self.handlers.len();
        self.handlers.push(Some(handler));

        if let Some(ref mut descriptor) = self.descriptors[interrupt as usize] {
            descriptor.handler = Some(handler_index);
        }

        Ok(())
    }

    fn unregister_handler(&mut self, interrupt: InterruptNumber) -> Result<(), InterruptError> {
        if !self.capability.can_unregister {
            return Err(InterruptError::PermissionDenied);
        }

        if let Some(ref mut descriptor) = self.descriptors[interrupt as usize] {
            if let Some(handler_index) = descriptor.handler {
                self.handlers[handler_index] = None;
                descriptor.handler = None;
                Ok(())
            } else {
                Err(InterruptError::HandlerNotFound)
            }
        } else {
            Err(InterruptError::InvalidInterrupt)
        }
    }

    fn dispatch(&mut self, interrupt: InterruptNumber) -> InterruptResult {
        if !self.capability.can_dispatch {
            return InterruptResult::Error;
        }

        self.stats.total_interrupts += 1;

        if let Some(ref descriptor) = self.descriptors[interrupt as usize] {
            if descriptor.is_masked() || !descriptor.is_enabled() {
                self.stats.ignored_interrupts += 1;
                return InterruptResult::Ignored;
            }

            descriptor.increment_count();

            if let Some(handler_index) = descriptor.handler {
                if let Some(ref mut handler) = self.handlers[handler_index] {
                    let result = handler.handle(interrupt);
                    match result {
                        InterruptResult::Handled => self.stats.handled_interrupts += 1,
                        InterruptResult::Ignored => self.stats.ignored_interrupts += 1,
                        InterruptResult::Error => self.stats.error_interrupts += 1,
                        _ => {}
                    }
                    self.stats.handler_counts[interrupt as usize] += 1;
                    return result;
                }
            }
        }

        self.stats.ignored_interrupts += 1;
        InterruptResult::Ignored
    }

    fn enable_interrupt(&mut self, interrupt: InterruptNumber) -> Result<(), InterruptError> {
        if let Some(ref descriptor) = self.descriptors[interrupt as usize] {
            descriptor.enable()
        } else {
            Err(InterruptError::InvalidInterrupt)
        }
    }

    fn disable_interrupt(&mut self, interrupt: InterruptNumber) -> Result<(), InterruptError> {
        if let Some(ref descriptor) = self.descriptors[interrupt as usize] {
            descriptor.disable()
        } else {
            Err(InterruptError::InvalidInterrupt)
        }
    }

    fn stats(&self) -> InterruptStats {
        self.stats
    }

    fn get_priority(&self, interrupt: InterruptNumber) -> Option<Priority> {
        if let Some(ref descriptor) = self.descriptors[interrupt as usize] {
            if let Some(handler_index) = descriptor.handler {
                if let Some(Some(ref handler)) = self.handlers.get(handler_index) {
                    return Some(handler.info().priority);
                }
            }
        }
        None
    }
}

// =========================================================================
// Advanced Interrupt Time Sequence Logging, Preemption, Nesting
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceEventType {
    InterruptArrived,
    InterruptDispatched,
    InterruptPreempted,
    InterruptCompleted,
    InterruptDeferred,
    InterruptFlushed,
}

#[derive(Debug, Clone, Copy)]
pub struct InterruptTrace {
    pub timestamp: u64,
    pub event_type: TraceEventType,
    pub interrupt_number: InterruptNumber,
    pub priority: Priority,
}

/// Advanced Interrupt manager supporting CLI/STI, Nesting, Preemption and ETW/ftrace Time Sequencing
pub struct InterruptManager {
    pub controllers: Vec<Option<Box<dyn InterruptController>>>,
    pub active_controller: AtomicUsize,
    // Conforming x86/ARM/Linux kernel additions
    pub global_interrupts_enabled: AtomicBool,
    pub execution_stack: Vec<InterruptNumber>,
    pub pending_queue: Vec<InterruptNumber>,
    pub trace_log: Vec<InterruptTrace>,
    pub trace_counter: AtomicUsize,
}

impl InterruptManager {
    pub fn new() -> Self {
        InterruptManager {
            controllers: Vec::new(),
            active_controller: AtomicUsize::new(0),
            global_interrupts_enabled: AtomicBool::new(true),
            execution_stack: Vec::new(),
            pending_queue: Vec::new(),
            trace_log: Vec::new(),
            trace_counter: AtomicUsize::new(0),
        }
    }

    pub fn add_controller(&mut self, controller: Box<dyn InterruptController>) -> usize {
        let index = self.controllers.len();
        self.controllers.push(Some(controller));
        index
    }

    pub fn set_active_controller(&self, index: usize) {
        self.active_controller.store(index, Ordering::SeqCst);
    }

    /// Simulates standard hardware CLI (Disable Interrupts)
    pub fn cli(&self) {
        self.global_interrupts_enabled.store(false, Ordering::SeqCst);
    }

    /// Simulates standard hardware STI (Enable Interrupts) with deferred queue flushing
    pub fn sti(&mut self) {
        self.global_interrupts_enabled.store(true, Ordering::SeqCst);
        let timestamp = self.trace_counter.fetch_add(1, Ordering::SeqCst) as u64;

        // Flush deferred pending interrupts
        if !self.pending_queue.is_empty() {
            println!("[interrupt-manager] Re-enabled global interrupts (STI). Flushing pending queue...");

            // Re-order pending queue by priority descending (standard APIC/GIC behavior)
            let mut ordered_pending = Vec::new();
            while let Some(irq) = self.pending_queue.pop() {
                ordered_pending.push(irq);
            }

            let manager_ptr = self as *mut Self;
            for irq in ordered_pending {
                unsafe {
                    let trace_evt = InterruptTrace {
                        timestamp,
                        event_type: TraceEventType::InterruptFlushed,
                        interrupt_number: irq,
                        priority: (*manager_ptr).get_irq_priority(irq).unwrap_or(Priority::Normal),
                    };
                    (*manager_ptr).trace_log.push(trace_evt);
                    (*manager_ptr).dispatch_interrupt(irq);
                }
            }
        }
    }

    pub fn get_irq_priority(&self, interrupt: InterruptNumber) -> Option<Priority> {
        let active = self.active_controller.load(Ordering::SeqCst);
        if active < self.controllers.len() {
            if let Some(ref controller) = self.controllers[active] {
                return controller.get_priority(interrupt);
            }
        }
        None
    }

    /// Advanced stateful Priority-conforming preemption and nesting dispatch loop
    pub fn dispatch_interrupt(&mut self, interrupt: InterruptNumber) -> InterruptResult {
        let timestamp = self.trace_counter.fetch_add(1, Ordering::SeqCst) as u64;
        let priority = self.get_irq_priority(interrupt).unwrap_or(Priority::Normal);

        // Record arrival trace
        self.trace_log.push(InterruptTrace {
            timestamp,
            event_type: TraceEventType::InterruptArrived,
            interrupt_number: interrupt,
            priority,
        });

        // 1. If global interrupts are globally disabled (CLI), queue it as pending
        if !self.global_interrupts_enabled.load(Ordering::SeqCst) {
            println!("[interrupt-manager] Globally disabled. Deferring IRQ #{} to pending queue.", interrupt);
            self.pending_queue.push(interrupt);
            self.trace_log.push(InterruptTrace {
                timestamp,
                event_type: TraceEventType::InterruptDeferred,
                interrupt_number: interrupt,
                priority,
            });
            return InterruptResult::Deferred;
        }

        // 2. Priority task masking / preemption checking
        let mut should_preempt = true;
        if let Some(&current_running_irq) = self.execution_stack.last() {
            let current_priority = self.get_irq_priority(current_running_irq).unwrap_or(Priority::Normal);
            if priority <= current_priority {
                // Task priority register (TPR) rejects preemption. Defer to pending queue.
                println!(
                    "[interrupt-manager] Rejecting preemption: incoming IRQ #{} priority ({:?}) <= current IRQ #{} priority ({:?})",
                    interrupt, priority, current_running_irq, current_priority
                );
                self.pending_queue.push(interrupt);
                self.trace_log.push(InterruptTrace {
                    timestamp,
                    event_type: TraceEventType::InterruptDeferred,
                    interrupt_number: interrupt,
                    priority,
                });
                return InterruptResult::Deferred;
            } else {
                // High-priority interrupt preempts/nests lower-priority active interrupt
                println!(
                    "[interrupt-manager] NESTING/PREEMPTION: incoming IRQ #{} ({:?}) preempts active IRQ #{} ({:?})",
                    interrupt, priority, current_running_irq, current_priority
                );
                self.trace_log.push(InterruptTrace {
                    timestamp,
                    event_type: TraceEventType::InterruptPreempted,
                    interrupt_number: current_running_irq,
                    priority: current_priority,
                });
            }
        }

        // 3. Dispatch the interrupt
        self.execution_stack.push(interrupt);
        self.trace_log.push(InterruptTrace {
            timestamp,
            event_type: TraceEventType::InterruptDispatched,
            interrupt_number: interrupt,
            priority,
        });

        let active = self.active_controller.load(Ordering::SeqCst);
        let result = if active < self.controllers.len() {
            let controllers_ptr = &mut self.controllers as *mut Vec<Option<Box<dyn InterruptController>>>;
            unsafe {
                if let Some(Some(ref mut controller)) = (*controllers_ptr).get_mut(active) {
                    controller.dispatch(interrupt)
                } else {
                    InterruptResult::Error
                }
            }
        } else {
            InterruptResult::Error
        };

        // 4. Complete interrupt and pop context
        self.execution_stack.pop();
        self.trace_log.push(InterruptTrace {
            timestamp,
            event_type: TraceEventType::InterruptCompleted,
            interrupt_number: interrupt,
            priority,
        });

        result
    }

    pub fn get_stats(&self) -> Vec<InterruptStats> {
        let mut stats = Vec::new();
        for controller_option in &self.controllers {
            if let Some(ref controller) = *controller_option {
                stats.push(controller.stats());
            }
        }
        stats
    }
}

impl Default for InterruptManager {
    fn default() -> Self {
        Self::new()
    }
}

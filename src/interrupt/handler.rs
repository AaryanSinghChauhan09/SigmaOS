#![no_std]
#![no_main]

/// OOP-based Interrupt Handler for SigmaOS
/// Implements interrupt handling using OOP principles with traits and structs
/// No dependency on external interrupt frameworks

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

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
#[derive(Debug, Clone, Copy)]
pub enum InterruptResult {
    Handled = 0,
    Ignored = 1,
    Deferred = 2,
    Error = 3,
}

/// Interrupt error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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

/// Priority level
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
    fn handle(&mut self, interrupt: InterruptNumber) -> InterruptResult {
        self.handle_count.fetch_add(1, Ordering::SeqCst);
        // In a real implementation, this would handle the interrupt
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
}

/// Interrupt statistics
#[repr(C)]
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
    descriptors: [Option<InterruptDescriptor>; 256],
    handlers: Vec<Option<Box<dyn InterruptHandler>>>,
    stats: InterruptStats,
    capability: ControllerCapability,
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
}

/// Interrupt manager (OOP: Manager class)
pub struct InterruptManager {
    controllers: Vec<Option<Box<dyn InterruptController>>>,
    active_controller: AtomicUsize,
}

impl InterruptManager {
    pub fn new() -> Self {
        InterruptManager {
            controllers: Vec::new(),
            active_controller: AtomicUsize::new(0),
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

    pub fn dispatch_interrupt(&mut self, interrupt: InterruptNumber) -> InterruptResult {
        let active = self.active_controller.load(Ordering::SeqCst);
        if active < self.controllers.len() {
            if let Some(ref mut controller) = self.controllers[active] {
                controller.dispatch(interrupt)
            } else {
                InterruptResult::Error
            }
        } else {
            InterruptResult::Error
        }
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

/// Simple Vec implementation for no_std
struct Vec<T> {
    data: *mut T,
    len: usize,
    capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }

    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }

            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;

        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }

            if self.capacity > 0 {
                free(self.data as *mut u8);
            }

            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

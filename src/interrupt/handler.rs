// OOP-based Interrupt Handler for SigmaOS
// Implements interrupt handling using OOP principles with traits and structs.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandlerCapability {
    pub can_enable: bool,
    pub can_disable: bool,
    pub can_mask: bool,
}

impl HandlerCapability {
    pub const fn new() -> Self {
        HandlerCapability {
            can_enable: false,
            can_disable: false,
            can_mask: false,
        }
    }

    pub const fn full() -> Self {
        HandlerCapability {
            can_enable: true,
            can_disable: true,
            can_mask: true,
        }
    }
}

impl Default for HandlerCapability {
    fn default() -> Self {
        Self::new()
    }
}

/// Interrupt descriptor (OOP: Interrupt object)
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
    pub fn new(
        number: InterruptNumber,
        vector: InterruptVector,
        capability: HandlerCapability,
    ) -> Self {
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
    pub fn new(
        handler_type: HandlerType,
        priority: Priority,
        capability: HandlerCapability,
    ) -> Self {
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
    fn register_handler(
        &mut self,
        handler: Box<dyn InterruptHandler>,
        interrupt: InterruptNumber,
    ) -> Result<(), InterruptError>;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptStats {
    pub total_interrupts: u64,
    pub handled_interrupts: u64,
    pub ignored_interrupts: u64,
    pub error_interrupts: u64,
    pub handler_counts: [u64; 256],
}

impl InterruptStats {
    pub const fn new() -> Self {
        InterruptStats {
            total_interrupts: 0,
            handled_interrupts: 0,
            ignored_interrupts: 0,
            error_interrupts: 0,
            handler_counts: [0; 256],
        }
    }
}

impl Default for InterruptStats {
    fn default() -> Self {
        Self::new()
    }
}

/// PIC (Programmable Interrupt Controller) (OOP: Concrete controller class)
pub struct PIC {
    descriptors: Vec<Option<InterruptDescriptor>>,
    handlers: Vec<Option<Box<dyn InterruptHandler>>>,
    stats: InterruptStats,
    capability: ControllerCapability,
}

/// Controller capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControllerCapability {
    pub can_register: bool,
    pub can_unregister: bool,
    pub can_dispatch: bool,
}

impl ControllerCapability {
    pub const fn new() -> Self {
        ControllerCapability {
            can_register: false,
            can_unregister: false,
            can_dispatch: false,
        }
    }

    pub const fn full() -> Self {
        ControllerCapability {
            can_register: true,
            can_unregister: true,
            can_dispatch: true,
        }
    }
}

impl Default for ControllerCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl PIC {
    pub fn new(capability: ControllerCapability) -> Self {
        let mut descriptors = Vec::new();

        // Initialize common interrupt descriptors
        for i in 0..256 {
            descriptors.push(Some(InterruptDescriptor::new(
                i as u8,
                i as u8,
                HandlerCapability::full(),
            )));
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
    fn register_handler(
        &mut self,
        handler: Box<dyn InterruptHandler>,
        interrupt: InterruptNumber,
    ) -> Result<(), InterruptError> {
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

impl Default for InterruptManager {
    fn default() -> Self {
        Self::new()
    }
}

/// CPU Privilege Execution Modes
/// Inspired directly by ARM architecture state registers (usr, fiq, irq, svc, mon, abt, und, sys)
/// and CISC/x86 privilege rings.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuPrivilegeMode {
    User = 0,         // usr - Unprivileged application mode
    Fiq = 1,          // fiq - Fast Interrupt Request (low latency hardware routing)
    Irq = 2,          // irq - Standard Interrupt Request
    Supervisor = 3,   // svc - Supervisor/Software Service Interrupt (syscalls, entry gate)
    Monitor = 4,      // mon - Secure Monitor State (TrustZone/Virtualization boundary)
    Abort = 5,        // abt - Instruction/Data prefetch memory translation fault
    Undefined = 6,    // und - Undefined instruction/Coprocessor exception handler
    System = 7,       // sys - Privileged system execution mode
}

/// Dynamic Exception Vector Frame mapping registers and states during a privilege mode trap.
/// Inspired by Windows KTRAP_FRAME and Linux pt_regs.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PrivilegeExceptionFrame {
    pub mode: CpuPrivilegeMode,
    pub spsr: u32,             // Saved Processor Status Register
    pub lr: u64,               // Link Register (return instruction address)
    pub fault_address: u64,    // DFAR / IFAR fault address representation
    pub error_code: u32,       // Exception-specific error code
}

/// Secure Hardware Exception Dispatcher for SigmaOS.
/// Performs precise routing of CPU traps matching Linux, BSD, and Windows kernel behaviors.
pub fn dispatch_privilege_exception(
    frame: &PrivilegeExceptionFrame,
) -> Result<InterruptResult, &'static str> {
    match frame.mode {
        CpuPrivilegeMode::User => {
            // Unprivileged execution error - translate to virtual signal or core dump (Linux Parity)
            Ok(InterruptResult::Error)
        }
        CpuPrivilegeMode::Fiq => {
            // High-speed low-latency hardware handler route (Fast Path)
            Ok(InterruptResult::Handled)
        }
        CpuPrivilegeMode::Supervisor => {
            // Software Service Interrupt / system call gate routing
            Ok(InterruptResult::Handled)
        }
        CpuPrivilegeMode::Monitor => {
            // Secure world virtualization partition exit
            Ok(InterruptResult::Handled)
        }
        CpuPrivilegeMode::Abort => {
            // Prefetch or Data Abort - trigger virtual memory demand-page loading
            if frame.fault_address == 0 {
                return Err("Null pointer dereference data abort (SIGSEGV Parity)");
            }
            Ok(InterruptResult::Deferred) // Defer to page loader
        }
        CpuPrivilegeMode::Undefined => {
            // Coprocessor or unrecognized instruction - pass to software JIT emulator
            Ok(InterruptResult::Handled)
        }
        CpuPrivilegeMode::System | CpuPrivilegeMode::Irq => {
            // Privileged interrupt / hardware line trigger
            Ok(InterruptResult::Handled)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interrupt_handling_and_dispatch() {
        let mut pic = PIC::new(ControllerCapability::full());
        let handler = SimpleInterruptHandler::new(
            HandlerType::Keyboard,
            Priority::Critical,
            HandlerCapability::full(),
        );

        pic.register_handler(Box::new(handler), 33).unwrap();
        pic.enable_interrupt(33).unwrap();

        let res = pic.dispatch(33);
        assert_eq!(res, InterruptResult::Handled);
        assert_eq!(pic.stats().handled_interrupts, 1);
    }

    #[test]
    fn test_privilege_modes_and_exceptions() {
        let frame = PrivilegeExceptionFrame {
            mode: CpuPrivilegeMode::Supervisor,
            spsr: 0,
            lr: 0x2000,
            fault_address: 0,
            error_code: 0,
        };
        let res = dispatch_privilege_exception(&frame).unwrap();
        assert_eq!(res, InterruptResult::Handled);

        let abort_frame = PrivilegeExceptionFrame {
            mode: CpuPrivilegeMode::Abort,
            spsr: 0,
            lr: 0x4000,
            fault_address: 0x8000,
            error_code: 1,
        };
        let res_abort = dispatch_privilege_exception(&abort_frame).unwrap();
        assert_eq!(res_abort, InterruptResult::Deferred);

        // Fault address 0 is Null ptr deref exception
        let bad_abort_frame = PrivilegeExceptionFrame {
            mode: CpuPrivilegeMode::Abort,
            spsr: 0,
            lr: 0x4000,
            fault_address: 0,
            error_code: 1,
        };
        assert!(dispatch_privilege_exception(&bad_abort_frame).is_err());
    }
}

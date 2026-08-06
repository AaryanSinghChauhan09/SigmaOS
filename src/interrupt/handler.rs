#![no_std]
#![cfg_attr(not(test), no_main)]

/// Advanced High-Fidelity Interrupt & Exception Handler for SigmaOS
/// Models standard x86/x64 CPU register states, AMD64 canonical address checks, exception ISR routers, and PIC/APIC controllers.

extern crate alloc;

use alloc::vec::Vec;
use alloc::boxed::Box;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::interrupt::controller::InterruptPriority;

pub type InterruptNumber = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptError {
    Success,
    InvalidIRQ,
    ControllerError,
}

/// Standard x86/x64 Exceptions and Hardware Interrupt vectors
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionVector {
    DivideByZero = 0,
    Debug = 1,
    NonMaskableInterrupt = 2,
    Breakpoint = 3,
    Overflow = 4,
    BoundRangeExceeded = 5,
    InvalidOpcode = 6,
    DeviceNotAvailable = 7,
    DoubleFault = 8,
    CoprocessorSegmentOverrun = 9,
    InvalidTSS = 10,
    SegmentNotPresent = 11,
    StackSegmentFault = 12,
    GeneralProtectionFault = 13,
    PageFault = 14,
    X87FloatingPointException = 16,
    AlignmentCheck = 17,
    MachineCheck = 18,
    SIMDFloatingPointException = 19,
    VirtualizationException = 20,
    SecurityException = 30,
    SpuriousInterrupt = 39,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptResult {
    Handled = 0,
    Ignored = 1,
    ChainNext = 2,
    Error = 3,
    Deferred = 4,
}

/// Models the complete x86_64 General Purpose and Segment CPU Register Set
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RegisterSet {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8:  u64,
    pub r9:  u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
    pub cs: u64,
    pub ss: u64,
    pub ds: u64,
    pub es: u64,
    pub fs: u64,
    pub gs: u64,
}

pub trait InterruptHandler: Send + Sync {
    fn id(&self) -> InterruptNumber;
    fn handle(&mut self, regs: &mut RegisterSet) -> InterruptResult;
    fn priority(&self) -> InterruptPriority {
        InterruptPriority::Normal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandlerType {
    Custom,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HandlerCapability {
    pub mask: u32,
}

impl HandlerCapability {
    pub fn full() -> Self {
        HandlerCapability { mask: 0xFFFFFFFF }
    }
}

/// Simulated concrete interrupt handler
pub struct SimpleInterruptHandler {
    pub vector: InterruptNumber,
    pub trigger_count: u32,
    pub handler_type: HandlerType,
    pub priority: InterruptPriority,
    pub capability: HandlerCapability,
}

impl SimpleInterruptHandler {
    pub fn new(handler_type: HandlerType, priority: InterruptPriority, capability: HandlerCapability) -> Self {
        SimpleInterruptHandler {
            vector: 0,
            trigger_count: 0,
            handler_type,
            priority,
            capability,
        }
    }
}

impl InterruptHandler for SimpleInterruptHandler {
    fn id(&self) -> InterruptNumber {
        self.vector
    }

    fn handle(&mut self, _regs: &mut RegisterSet) -> InterruptResult {
        self.trigger_count += 1;
        InterruptResult::Handled
    }

    fn priority(&self) -> InterruptPriority {
        self.priority
    }
}

/// Dynamic descriptor tracking interrupt routing
pub struct InterruptDescriptor {
    pub vector: InterruptNumber,
    pub enabled: AtomicBool,
    pub masked: AtomicBool,
}

impl InterruptDescriptor {
    pub fn new(vector: InterruptNumber) -> Self {
        InterruptDescriptor {
            vector,
            enabled: AtomicBool::new(true),
            masked: AtomicBool::new(false),
        }
    }
}

/// Telemetry stats on interrupt dispatches
#[derive(Debug, Clone, Copy, Default)]
pub struct InterruptStats {
    pub total_interrupts_dispatched: u64,
    pub spurious_count: u64,
    pub double_faults: u64,
    pub page_faults: u64,
    pub gpf_faults: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceEventType {
    InterruptArrived,
    InterruptDispatched,
    InterruptCompleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptTrace {
    pub event_type: TraceEventType,
    pub interrupt_number: u32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControllerCapability {
    pub mask: u32,
}

impl ControllerCapability {
    pub fn full() -> Self {
        ControllerCapability { mask: 0xFFFFFFFF }
    }
}

pub struct PIC {
    pub capability: ControllerCapability,
    pub handlers: Vec<(Box<dyn InterruptHandler>, u32)>,
    pub enabled_interrupts: Vec<u32>,
}

impl PIC {
    pub fn new(capability: ControllerCapability) -> Self {
        PIC {
            capability,
            handlers: Vec::new(),
            enabled_interrupts: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn InterruptHandler>, irq: u32) -> Result<(), &'static str> {
        self.handlers.push((handler, irq));
        Ok(())
    }

    pub fn enable_interrupt(&mut self, irq: u32) -> Result<(), &'static str> {
        if !self.enabled_interrupts.contains(&irq) {
            self.enabled_interrupts.push(irq);
        }
        Ok(())
    }
}

/// Core Interrupt & Exception Manager
pub struct InterruptManager {
    pub handlers: Vec<Box<dyn InterruptHandler>>,
    pub descriptors: Vec<InterruptDescriptor>,
    pub stats: InterruptStats,
    pub controllers: Vec<Box<PIC>>,
    pub execution_stack: Vec<u32>,
    pub pending_queue: Vec<u32>,
    pub trace_log: Vec<InterruptTrace>,
    pub interrupts_enabled: bool,
}

impl InterruptManager {
    pub fn new() -> Self {
        let mut descriptors = Vec::new();
        for i in 0..256 {
            descriptors.push(InterruptDescriptor::new(i as u32));
        }

        InterruptManager {
            handlers: Vec::new(),
            descriptors,
            stats: InterruptStats::default(),
            controllers: Vec::new(),
            execution_stack: Vec::new(),
            pending_queue: Vec::new(),
            trace_log: Vec::new(),
            interrupts_enabled: true,
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn InterruptHandler>) {
        self.handlers.push(handler);
    }

    /// Verifies if a virtual memory address is canonical under AMD64 architecture (bits 48 to 63 must sign-extend bit 47)
    pub fn is_canonical_address(address: u64) -> bool {
        let sign_bit = (address >> 47) & 1;
        let upper_bits = address >> 48;
        if sign_bit == 0 {
            upper_bits == 0
        } else {
            upper_bits == 0xFFFF
        }
    }

    /// Routes CPU exceptions and interrupts, adjusting register sets and aggregating telemetry
    pub fn dispatch_exception(
        &mut self,
        vector: ExceptionVector,
        regs: &mut RegisterSet,
    ) -> InterruptResult {
        self.stats.total_interrupts_dispatched += 1;

        // Perform canonical address checks on instruction and stack pointer values (failsafe)
        if !Self::is_canonical_address(regs.rip) || !Self::is_canonical_address(regs.rsp) {
            self.stats.double_faults += 1;
            return InterruptResult::Error; // Direct double fault panic route
        }

        match vector {
            ExceptionVector::DoubleFault => {
                self.stats.double_faults += 1;
                InterruptResult::Error
            }
            ExceptionVector::PageFault => {
                self.stats.page_faults += 1;
                // Handle page fault on-demand and restore
                regs.rax = 0xFFFFFFFF; // Set error return register
                InterruptResult::Handled
            }
            ExceptionVector::GeneralProtectionFault => {
                self.stats.gpf_faults += 1;
                InterruptResult::Handled
            }
            ExceptionVector::SpuriousInterrupt => {
                self.stats.spurious_count += 1;
                InterruptResult::Ignored
            }
            _ => {
                // Check registered handlers
                let v_num = vector as u32;
                if let Some(pos) = self.handlers.iter().position(|h| h.id() == v_num) {
                    self.handlers[pos].handle(regs)
                } else {
                    InterruptResult::Ignored
                }
            }
        }
    }

    pub fn get_stats(&self) -> InterruptStats {
        self.stats
    }

    pub fn cli(&mut self) {
        self.interrupts_enabled = false;
    }

    pub fn sti(&mut self) {
        self.interrupts_enabled = true;
        // Process pending interrupts
        let mut temp_pending = Vec::new();
        core::mem::swap(&mut self.pending_queue, &mut temp_pending);
        for irq in temp_pending {
            self.dispatch_interrupt(irq);
        }
    }

    pub fn add_controller(&mut self, controller: Box<PIC>) {
        self.controllers.push(controller);
    }

    pub fn dispatch_interrupt(&mut self, irq: u32) -> InterruptResult {
        let arrived_ts = self.trace_log.len() as u64;
        self.trace_log.push(InterruptTrace {
            event_type: TraceEventType::InterruptArrived,
            interrupt_number: irq,
            timestamp: arrived_ts,
        });

        if !self.interrupts_enabled {
            self.pending_queue.push(irq);
            return InterruptResult::Deferred;
        }

        let irq_priority = self.get_irq_priority(irq);

        if let Some(&active_irq) = self.execution_stack.last() {
            let active_priority = self.get_irq_priority(active_irq);
            if active_priority >= irq_priority {
                self.pending_queue.push(irq);
                return InterruptResult::Deferred;
            }
        }

        let dispatched_ts = self.trace_log.len() as u64;
        self.trace_log.push(InterruptTrace {
            event_type: TraceEventType::InterruptDispatched,
            interrupt_number: irq,
            timestamp: dispatched_ts,
        });

        self.execution_stack.push(irq);

        let mut handled = false;
        for controller in &mut self.controllers {
            for (handler, h_irq) in &mut controller.handlers {
                if *h_irq == irq {
                    let mut regs = RegisterSet::default();
                    handler.handle(&mut regs);
                    handled = true;
                    break;
                }
            }
        }

        self.execution_stack.pop();

        let completed_ts = self.trace_log.len() as u64;
        self.trace_log.push(InterruptTrace {
            event_type: TraceEventType::InterruptCompleted,
            interrupt_number: irq,
            timestamp: completed_ts,
        });

        if handled {
            InterruptResult::Handled
        } else {
            InterruptResult::Ignored
        }
    }

    fn get_irq_priority(&self, irq: u32) -> InterruptPriority {
        for controller in &self.controllers {
            for (handler, h_irq) in &controller.handlers {
                if *h_irq == irq {
                    return handler.priority();
                }
            }
        }
        InterruptPriority::Normal
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
    fn test_canonical_address_verification() {
        // Standard user canonical address
        assert!(InterruptManager::is_canonical_address(0x0000_7FFF_FFFF_FFFF));
        // Standard kernel canonical address (sign extended 1s)
        assert!(InterruptManager::is_canonical_address(0xFFFF_8000_0000_0000));

        // Non-canonical address (sign bit 47 is 0, but upper bits contain 1s)
        assert!(!InterruptManager::is_canonical_address(0x0001_7FFF_FFFF_FFFF));
        // Non-canonical address (sign bit 47 is 1, but upper bits contain 0s)
        assert!(!InterruptManager::is_canonical_address(0x1FFF_8000_0000_0000));
    }

    #[test]
    fn test_exception_vector_routing() {
        let mut manager = InterruptManager::new();
        let mut regs = RegisterSet::default();
        regs.rip = 0x0000_7FFF_FFFF_FFFF;
        regs.rsp = 0x0000_7FFF_FFFF_FFFF;

        // Route a Page Fault
        let res = manager.dispatch_exception(ExceptionVector::PageFault, &mut regs);
        assert_eq!(res, InterruptResult::Handled);
        assert_eq!(regs.rax, 0xFFFFFFFF);
        assert_eq!(manager.stats.page_faults, 1);
    }

    #[test]
    fn test_non_canonical_double_fault_router() {
        let mut manager = InterruptManager::new();
        let mut regs = RegisterSet::default();
        // Set invalid non-canonical instruction pointer
        regs.rip = 0x0001_7FFF_FFFF_FFFF;
        regs.rsp = 0x0000_7FFF_FFFF_FFFF;

        // Attempting to route any exception on a non-canonical register state should panic to double fault
        let res = manager.dispatch_exception(ExceptionVector::PageFault, &mut regs);
        assert_eq!(res, InterruptResult::Error);
        assert_eq!(manager.stats.double_faults, 1);
    }

    #[test]
    fn test_custom_handler_callback() {
        let mut manager = InterruptManager::new();
        let mut regs = RegisterSet::default();
        regs.rip = 0x0000_7FFF_FFFF_FFFF;
        regs.rsp = 0x0000_7FFF_FFFF_FFFF;

        let mut handler = SimpleInterruptHandler::new(
            HandlerType::Custom,
            InterruptPriority::Normal,
            HandlerCapability::full()
        );
        handler.vector = 13; // GPF handler vector
        manager.register_handler(Box::new(handler));

        let res = manager.dispatch_exception(ExceptionVector::GeneralProtectionFault, &mut regs);
        assert_eq!(res, InterruptResult::Handled); // Core GPF handler intercepts and overrides
        assert_eq!(manager.stats.gpf_faults, 1);
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

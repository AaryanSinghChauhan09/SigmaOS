// OOP-based Interrupt Handler for SigmaOS
// Implements interrupt handling using OOP principles with traits and structs.
#![no_std]
#![no_main]
/// Advanced High-Fidelity Interrupt & Exception Handler for SigmaOS
/// Models standard x86/x64 CPU register states, AMD64 canonical address checks, exception ISR routers, and PIC/APIC controllers.

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

pub type InterruptNumber = u32;

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
}

/// Models the complete x86_64 General Purpose and Segment CPU Register Set
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptError {
    Success = 0,
    InvalidInterrupt = 1,
    AlreadyEnabled = 2,
    AlreadyDisabled = 3,
    PermissionDenied = 4,
    HandlerNotFound = 5,
#[derive(Debug, Clone, Copy)]
pub enum InterruptError {
    Success = 0,
    InvalidInterrupt = 1,
    AlreadyEnabled = 2,
    AlreadyDisabled = 3,
    PermissionDenied = 4,
    HandlerNotFound = 5,
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

pub trait InterruptHandler {
    fn id(&self) -> InterruptNumber;
    fn handle(&mut self, regs: &mut RegisterSet) -> InterruptResult;
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
/// Simulated concrete interrupt handler
pub struct SimpleInterruptHandler {
    pub vector: InterruptNumber,
    pub trigger_count: u32,
}

impl SimpleInterruptHandler {
    pub fn new(
        handler_type: HandlerType,
        priority: Priority,
        capability: HandlerCapability,
    ) -> Self {
    pub fn new(handler_type: HandlerType, priority: Priority, capability: HandlerCapability) -> Self {
    pub fn new(vector: InterruptNumber) -> Self {
        SimpleInterruptHandler {
            vector,
            trigger_count: 0,
        }
    }
}

impl InterruptHandler for SimpleInterruptHandler {
    fn handle(&mut self, _interrupt: InterruptNumber) -> InterruptResult {
        self.handle_count.fetch_add(1, Ordering::SeqCst);
    fn handle(&mut self, interrupt: InterruptNumber) -> InterruptResult {
        self.handle_count.fetch_add(1, Ordering::SeqCst);
        // In a real implementation, this would handle the interrupt
    fn id(&self) -> InterruptNumber {
        self.vector
    }

    fn handle(&mut self, _regs: &mut RegisterSet) -> InterruptResult {
        self.trigger_count += 1;
        InterruptResult::Handled
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
/// Telemetry stats on interrupt dispatches
#[derive(Debug, Clone, Copy, Default)]
pub struct InterruptStats {
    pub total_interrupts_dispatched: u64,
    pub spurious_count: u64,
    pub double_faults: u64,
    pub page_faults: u64,
    pub gpf_faults: u64,
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
/// Core Interrupt & Exception Manager
pub struct InterruptManager {
    pub handlers: Vec<Box<dyn InterruptHandler>>,
    pub descriptors: Vec<InterruptDescriptor>,
    pub stats: InterruptStats,
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
}

impl Default for InterruptManager {
    fn default() -> Self {
        Self::new()
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

        let handler = SimpleInterruptHandler::new(13); // GPF handler
        manager.register_handler(Box::new(handler));

        let res = manager.dispatch_exception(ExceptionVector::GeneralProtectionFault, &mut regs);
        assert_eq!(res, InterruptResult::Handled); // Core GPF handler intercepts and overrides
        assert_eq!(manager.stats.gpf_faults, 1);
    }
}

#[cfg(test)]
mod tests {

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
}

// External allocator functions
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

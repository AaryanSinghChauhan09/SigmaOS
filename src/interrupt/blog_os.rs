// GDT, IDT, and VGA Text Buffer architectures for SigmaOS
// Integrates core bare-metal concepts from phil-opp/blog_os under `#![no_std]`.

extern crate alloc;

use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// ==========================================
// 1. GLOBAL DESCRIPTOR TABLE & TSS
// ==========================================

/// Task State Segment (TSS) containing double fault stacks to prevent triple faults
pub struct TaskStateSegment {
    pub interrupt_stack_table: [u64; 7],
}

impl TaskStateSegment {
    pub fn new() -> Self {
        Self {
            interrupt_stack_table: [0; 7],
        }
    }
}

/// Global Descriptor Table (GDT) mapping code, data, and system segments
pub struct GDT {
    pub code_selector: u16,
    pub tss_selector: u16,
}

impl GDT {
    pub fn new() -> Self {
        Self {
            code_selector: 8,
            tss_selector: 16,
        }
    }

    pub fn load(&self, _tss: &TaskStateSegment) -> bool {
        // Simulates loading GDT and TSS registers securely
        true
    }
}

// ==========================================
// 2. INTERRUPT DESCRIPTOR TABLE & EXCEPTIONS
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionType {
    DivideByZero = 0,
    Breakpoint = 3,
    DoubleFault = 8,
    PageFault = 14,
}

/// Interrupt Descriptor Table (IDT) defining entry gates for exceptions and IRQs
pub struct IDT {
    pub divide_by_zero_handler: Option<fn()>,
    pub breakpoint_handler: Option<fn()>,
    pub double_fault_handler: Option<fn()>,
    pub page_fault_handler: Option<fn()>,
    pub hardware_irqs: [Option<fn()>; 16],
}

impl IDT {
    pub fn new() -> Self {
        Self {
            divide_by_zero_handler: None,
            breakpoint_handler: None,
            double_fault_handler: None,
            page_fault_handler: None,
            hardware_irqs: [None; 16],
        }
    }

    pub fn set_exception_handler(&mut self, exc: ExceptionType, handler: fn()) {
        match exc {
            ExceptionType::DivideByZero => self.divide_by_zero_handler = Some(handler),
            ExceptionType::Breakpoint => self.breakpoint_handler = Some(handler),
            ExceptionType::DoubleFault => self.double_fault_handler = Some(handler),
            ExceptionType::PageFault => self.page_fault_handler = Some(handler),
        }
    }

    pub fn set_hardware_irq_handler(&mut self, irq: usize, handler: fn()) {
        if irq < 16 {
            self.hardware_irqs[irq] = Some(handler);
        }
    }
}

// ==========================================
// 3. VGA TEXT BUFFER DRIVER
// ==========================================

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: VGAColor, background: VGAColor) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ScreenChar {
    pub ascii_character: u8,
    pub color_code: u8,
}

pub struct VGATextBuffer {
    pub buffer: [[ScreenChar; 80]; 25],
    pub row: usize,
    pub col: usize,
    pub current_color: ColorCode,
}

impl VGATextBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [[ScreenChar {
                ascii_character: b' ',
                color_code: 0x07,
            }; 80]; 25],
            row: 0,
            col: 0,
            current_color: ColorCode::new(VGAColor::LightGray, VGAColor::Black),
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.col >= 80 {
                    self.new_line();
                }

                let row = self.row;
                let col = self.col;

                self.buffer[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code: self.current_color.0,
                };
                self.col += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }

    fn new_line(&mut self) {
        if self.row < 24 {
            self.row += 1;
            self.col = 0;
        } else {
            // Scroll buffer up by one row
            for r in 1..25 {
                self.buffer[r - 1] = self.buffer[r];
            }
            // Clear last row
            self.buffer[24] = [ScreenChar {
                ascii_character: b' ',
                color_code: self.current_color.0,
            }; 80];
            self.col = 0;
        }
    }
}

// ==========================================
// 4. x86_64, ARM, LINUX, BSD & WINDOWS ARCHITECTURES
// ==========================================

/// x86_64 Page Fault Error Code flags (Present, Write, User, Reserved, Instruction)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFaultErrorCode {
    pub present: bool,
    pub write: bool,
    pub user: bool,
    pub reserved_write: bool,
    pub instruction_fetch: bool,
}

impl PageFaultErrorCode {
    pub const fn from_bits(bits: u64) -> Self {
        Self {
            present: (bits & (1 << 0)) != 0,
            write: (bits & (1 << 1)) != 0,
            user: (bits & (1 << 2)) != 0,
            reserved_write: (bits & (1 << 3)) != 0,
            instruction_fetch: (bits & (1 << 4)) != 0,
        }
    }
}

/// x86_64 Exception CPU Context Frame (CR2 Register + Stack Frame)
#[derive(Debug, Clone, Copy)]
pub struct X86_64ExceptionContext {
    pub ip: u64,
    pub cs: u64,
    pub flags: u64,
    pub sp: u64,
    pub ss: u64,
    pub cr2: u64, // Faulting linear memory address (Page Fault specific)
    pub error_code: Option<u64>,
}

/// ARM Exception Levels (EL0 = User, EL1 = OS Kernel, EL2 = Hypervisor, EL3 = Secure Monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArmExceptionLevel {
    EL0 = 0,
    EL1 = 1,
    EL2 = 2,
    EL3 = 3,
}

/// ARM Exception Vector Types (Synchronous, IRQ, FIQ, SError)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArmVectorType {
    Synchronous,
    IRQ,
    FIQ,
    SError,
}

/// ARM Vector Base Address Register (VBAR) & Exception Table Context
pub struct ArmVectorTable {
    pub vbar_address: u64,
    pub current_el: ArmExceptionLevel,
}

impl ArmVectorTable {
    pub fn new(vbar_address: u64, current_el: ArmExceptionLevel) -> Self {
        Self {
            vbar_address,
            current_el,
        }
    }

    /// Simulates dispatching an exception vector based on EL and Vector Type
    pub fn route_exception(&self, _vector: ArmVectorType, target_el: ArmExceptionLevel) -> bool {
        // Enforces that exceptions always route upwards or maintain exception level (no downward routing)
        if target_el < self.current_el {
            false
        } else {
            // Securely routed to target EL handler
            true
        }
    }
}

/// Linux Kernel-style SoftIRQ type definitions (Top-half / Bottom-half deferred processing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SoftirqType {
    Hi = 0,
    Timer = 1,
    NetTx = 2,
    NetRx = 3,
    Block = 4,
    IrqPoll = 5,
    Tasklet = 6,
    Sched = 7,
    Hrtimer = 8,
    Rcu = 9,
}

/// Linux Kernel Bottom Half (Deferred Execution Queue)
pub struct LinuxBottomHalf {
    pub softirq_pending: u32, // Bitmask of pending SoftIRQs
    pub tasklets_scheduled: Vec<u8>, // Simulating scheduled tasklet ids
}

impl LinuxBottomHalf {
    pub fn new() -> Self {
        Self {
            softirq_pending: 0,
            tasklets_scheduled: Vec::new(),
        }
    }

    pub fn raise_softirq(&mut self, softirq: SoftirqType) {
        self.softirq_pending |= 1 << (softirq as u8);
    }

    pub fn run_softirqs(&mut self, executed: &mut Vec<SoftirqType>) {
        // Processes all raised SoftIRQs sequentially from highest priority (Hi) to lowest (Rcu)
        for i in 0..10 {
            if (self.softirq_pending & (1 << i)) != 0 {
                let softirq = match i {
                    0 => SoftirqType::Hi,
                    1 => SoftirqType::Timer,
                    2 => SoftirqType::NetTx,
                    3 => SoftirqType::NetRx,
                    4 => SoftirqType::Block,
                    5 => SoftirqType::IrqPoll,
                    6 => SoftirqType::Tasklet,
                    7 => SoftirqType::Sched,
                    8 => SoftirqType::Hrtimer,
                    _ => SoftirqType::Rcu,
                };
                executed.push(softirq);
            }
        }
        // Clear all processed softirqs
        self.softirq_pending = 0;
    }
}

/// Windows/BSD-style Interrupt Request Level (IRQL) preemption architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InterruptRequestLevel {
    PassiveLevel = 0, // User mode & non-critical threads
    ApcLevel = 1,     // Asynchronous Procedure Calls
    DispatchLevel = 2, // Scheduler, DPCs and bottom halves (No page faults allowed!)
    DirqlLevel = 3,   // Device Interrupt Request Level
    HighLevel = 4,    // Machine checks, hardware clocks, non-maskable events
}

/// Windows/BSD IRQL Controller
pub struct WindowsIrqlController {
    pub current_irql: InterruptRequestLevel,
}

impl WindowsIrqlController {
    pub fn new() -> Self {
        Self {
            current_irql: InterruptRequestLevel::PassiveLevel,
        }
    }

    /// Elevates current IRQL, returning the old level. (Enforces upward elevation only)
    pub fn raise_irql(&mut self, new_irql: InterruptRequestLevel) -> Result<InterruptRequestLevel, &'static str> {
        if new_irql < self.current_irql {
            Err("Cannot raise to a lower IRQL level")
        } else {
            let old_irql = self.current_irql;
            self.current_irql = new_irql;
            Ok(old_irql)
        }
    }

    /// Lowers current IRQL to a previously saved level.
    pub fn lower_irql(&mut self, saved_irql: InterruptRequestLevel) -> Result<(), &'static str> {
        if saved_irql > self.current_irql {
            Err("Cannot lower to a higher IRQL level")
        } else {
            self.current_irql = saved_irql;
            Ok(())
        }
    }

    /// Evaluates whether an incoming hardware interrupt can preempt the current CPU execution
    pub fn can_preempt(&self, incoming_irql: InterruptRequestLevel) -> bool {
        incoming_irql > self.current_irql
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXCEPTION_TRIGGERED: AtomicUsize = AtomicUsize::new(0);

    fn mock_breakpoint_handler() {
        EXCEPTION_TRIGGERED.fetch_add(1, Ordering::SeqCst);
    }

    #[test]
    fn test_gdt_and_tss_loading() {
        let tss = TaskStateSegment::new();
        let gdt = GDT::new();
        assert!(gdt.load(&tss));
    }

    #[test]
    fn test_idt_handling() {
        let mut idt = IDT::new();
        idt.set_exception_handler(ExceptionType::Breakpoint, mock_breakpoint_handler);

        assert!(idt.breakpoint_handler.is_some());

        // Simulates triggering the registered exception handler
        if let Some(handler) = idt.breakpoint_handler {
            handler();
        }
        assert_eq!(EXCEPTION_TRIGGERED.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_vga_writing_and_scrolling() {
        let mut vga = VGATextBuffer::new();
        vga.write_string("Hello SigmaOS bare-metal!\n");
        assert_eq!(vga.row, 1);
        assert_eq!(vga.col, 0);
        assert_eq!(vga.buffer[0][0].ascii_character, b'H');

        // Write 25 rows to trigger scroll
        for _i in 0..25 {
            vga.write_string("Scroll test row\n");
        }
        assert_eq!(vga.row, 24);
        assert_eq!(vga.buffer[23][0].ascii_character, b'S');
    }

    #[test]
    fn test_x86_64_exception_context() {
        let error_code = PageFaultErrorCode::from_bits(0b10111); // Present, Write, User, Instruction
        assert!(error_code.present);
        assert!(error_code.write);
        assert!(error_code.user);
        assert!(!error_code.reserved_write);
        assert!(error_code.instruction_fetch);

        let context = X86_64ExceptionContext {
            ip: 0x00401000,
            cs: 8,
            flags: 0x200202,
            sp: 0x7fffffffe000,
            ss: 16,
            cr2: 0xdeadbeefc000,
            error_code: Some(0b10111),
        };

        assert_eq!(context.ip, 0x00401000);
        assert_eq!(context.cr2, 0xdeadbeefc000);
        assert!(context.error_code.is_some());
    }

    #[test]
    fn test_arm_vector_table_dispatch() {
        let vbar = ArmVectorTable::new(0xffff0000, ArmExceptionLevel::EL1);
        assert_eq!(vbar.current_el, ArmExceptionLevel::EL1);

        // Routing from EL1 to EL2 (Hypervisor) should succeed (upward routing)
        assert!(vbar.route_exception(ArmVectorType::Synchronous, ArmExceptionLevel::EL2));

        // Routing from EL1 to EL0 (User space exception) should fail (no downward exception routing)
        assert!(!vbar.route_exception(ArmVectorType::IRQ, ArmExceptionLevel::EL0));
    }

    #[test]
    fn test_linux_softirq_queue() {
        let mut bh = LinuxBottomHalf::new();
        assert_eq!(bh.softirq_pending, 0);

        bh.raise_softirq(SoftirqType::Timer);
        bh.raise_softirq(SoftirqType::Rcu);

        assert_ne!(bh.softirq_pending, 0);

        let mut executed = Vec::new();
        bh.run_softirqs(&mut executed);

        assert_eq!(executed.len(), 2);
        assert_eq!(executed[0], SoftirqType::Timer);
        assert_eq!(executed[1], SoftirqType::Rcu);
        assert_eq!(bh.softirq_pending, 0);
    }

    #[test]
    fn test_windows_irql_preemption() {
        let mut ctrl = WindowsIrqlController::new();
        assert_eq!(ctrl.current_irql, InterruptRequestLevel::PassiveLevel);

        // Raise IRQL to DispatchLevel (scheduler)
        let saved = ctrl.raise_irql(InterruptRequestLevel::DispatchLevel).unwrap();
        assert_eq!(saved, InterruptRequestLevel::PassiveLevel);
        assert_eq!(ctrl.current_irql, InterruptRequestLevel::DispatchLevel);

        // DIRQL should be able to preempt DispatchLevel
        assert!(ctrl.can_preempt(InterruptRequestLevel::DirqlLevel));

        // PassiveLevel should not be able to preempt DispatchLevel
        assert!(!ctrl.can_preempt(InterruptRequestLevel::PassiveLevel));

        // Lower IRQL back to saved PassiveLevel
        ctrl.lower_irql(saved).unwrap();
        assert_eq!(ctrl.current_irql, InterruptRequestLevel::PassiveLevel);
    }
}

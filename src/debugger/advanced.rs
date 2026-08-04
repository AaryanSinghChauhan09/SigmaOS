//! SigmaOS Advanced Debugger Subsystem
//!
//! Models and implements advanced debugger user interfaces, mathematical and bitwise expression
//! evaluations, process & thread control models, and trace exception handling (Handled vs Not Handled).
//! Highly inspired by low-level hardware debugging interfaces (x86 DR0-DR7, ARM EL registers)
//! and production kernels (Linux ptrace, Windows Dbgsrv/WinDbg).

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// =========================================================================
// 1. DEBUGGER WINDOWS MANAGEMENT
// =========================================================================

/// Types of specialized debugger panels/windows
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugWindowType {
    Registers,
    Disassembly,
    CallStack,
    WatchExpressions,
    ThreadList,
    MemoryDump,
    Console,
}

/// Models a single active debugger window with console positioning coordinates
pub struct DebugWindow {
    pub window_type: DebugWindowType,
    pub title: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub is_focused: bool,
    pub cached_output: Vec<String>,
}

impl DebugWindow {
    pub fn new(w_type: DebugWindowType, title: String, x: u32, y: u32, w: u32, h: u32) -> Self {
        Self {
            window_type: w_type,
            title,
            x,
            y,
            width: w,
            height: h,
            is_focused: false,
            cached_output: Vec::new(),
        }
    }

    pub fn refresh_content(&mut self, content: Vec<String>) {
        self.cached_output = content;
    }
}

pub struct DebugWindowManager {
    pub windows: Vec<DebugWindow>,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl DebugWindowManager {
    pub fn new() -> Self {
        let mut manager = Self {
            windows: Vec::new(),
            screen_width: 120,
            screen_height: 40,
        };
        manager.initialize_default_layout();
        manager
    }

    fn initialize_default_layout(&mut self) {
        // Registers Window (Top Left)
        self.windows.push(DebugWindow::new(DebugWindowType::Registers, String::from("Registers [CPU]"), 0, 0, 40, 15));
        // Disassembly Window (Top Right)
        self.windows.push(DebugWindow::new(DebugWindowType::Disassembly, String::from("Disassembly [RIP]"), 40, 0, 80, 15));
        // Watch Window (Middle Left)
        self.windows.push(DebugWindow::new(DebugWindowType::WatchExpressions, String::from("Watch Expressions"), 0, 15, 40, 15));
        // CallStack Window (Middle Right)
        self.windows.push(DebugWindow::new(DebugWindowType::CallStack, String::from("Call Stack"), 40, 15, 80, 15));
        // Console Window (Bottom Full)
        self.windows.push(DebugWindow::new(DebugWindowType::Console, String::from("Debugger Command Console"), 0, 30, 120, 10));
    }

    pub fn set_focus(&mut self, window_type: DebugWindowType) {
        for win in &mut self.windows {
            win.is_focused = win.window_type == window_type;
        }
    }
}

// =========================================================================
// 2. EXPRESSION EVALUATION ENGINE (USEFUL OPERATORS)
// =========================================================================

/// Mathematical and bitwise expression tree nodes for debugger variable/address evaluation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionNode {
    Literal(u64),
    Register(String),
    Dereference(Box<ExpressionNode>), // *ptr dereference
    Add(Box<ExpressionNode>, Box<ExpressionNode>),
    Subtract(Box<ExpressionNode>, Box<ExpressionNode>),
    Multiply(Box<ExpressionNode>, Box<ExpressionNode>),
    Divide(Box<ExpressionNode>, Box<ExpressionNode>),
    BitwiseAnd(Box<ExpressionNode>, Box<ExpressionNode>),
    BitwiseOr(Box<ExpressionNode>, Box<ExpressionNode>),
    BitwiseXor(Box<ExpressionNode>, Box<ExpressionNode>),
    BitwiseNot(Box<ExpressionNode>),
}

pub struct EvaluationEngine {
    pub mock_registers: Vec<(String, u64)>,
    pub mock_memory: Vec<(u64, u64)>, // Address -> Value
}

impl EvaluationEngine {
    pub fn new() -> Self {
        Self {
            mock_registers: vec![
                (String::from("rip"), 0x00100400),
                (String::from("rsp"), 0x000F0000),
                (String::from("rax"), 0x0000002A), // 42 decimal
                (String::from("rbx"), 0x00000003),
            ],
            mock_memory: vec![
                (0x000F0000, 0x00100500),
                (0x00100500, 0x00000100),
            ],
        }
    }

    /// Evaluates an ExpressionNode parsing variable math, bitwise gates, and pointers
    pub fn evaluate(&self, node: &ExpressionNode) -> Result<u64, &'static str> {
        match node {
            ExpressionNode::Literal(val) => Ok(*val),
            ExpressionNode::Register(reg) => {
                self.mock_registers.iter()
                    .find(|(r, _)| r == reg)
                    .map(|(_, val)| *val)
                    .ok_or("Register not found")
            }
            ExpressionNode::Dereference(inner) => {
                let address = self.evaluate(inner)?;
                self.mock_memory.iter()
                    .find(|(addr, _)| *addr == address)
                    .map(|(_, val)| *val)
                    .ok_or("Memory segmentation fault or invalid read pointer")
            }
            ExpressionNode::Add(left, right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                Ok(l.wrapping_add(r))
            }
            ExpressionNode::Subtract(left, right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                Ok(l.wrapping_sub(r))
            }
            ExpressionNode::Multiply(left, right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                Ok(l.wrapping_mul(r))
            }
            ExpressionNode::Divide(left, right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                if r == 0 { return Err("Division by zero exception"); }
                Ok(l / r)
            }
            ExpressionNode::BitwiseAnd(left, right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                Ok(l & r)
            }
            ExpressionNode::BitwiseOr(left, right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                Ok(l | r)
            }
            ExpressionNode::BitwiseXor(left, right) => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                Ok(l ^ r)
            }
            ExpressionNode::BitwiseNot(inner) => {
                let val = self.evaluate(inner)?;
                Ok(!val)
            }
        }
    }
}

// =========================================================================
// 3. PROCESS & THREAD CONTROLS
// =========================================================================

/// Thread execution state inside debugger trace controls
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugExecutionState {
    Running,
    Frozen,
    SingleStepping,
    ExceptionSuspended,
}

/// Represents registers and metadata for a thread tracked by the debugger
pub struct ThreadDebugState {
    pub tid: usize,
    pub execution_state: DebugExecutionState,
    pub rip: u64,
    pub rsp: u64,
    pub rflags: u64,
}

impl ThreadDebugState {
    pub fn new(tid: usize, start_rip: u64, start_rsp: u64) -> Self {
        Self {
            tid,
            execution_state: DebugExecutionState::Frozen,
            rip: start_rip,
            rsp: start_rsp,
            rflags: 0x202, // Standard default IF enabled
        }
    }
}

/// Process-level control manager tracking and synchronizing execution across threads
pub struct ProcessDebugContainer {
    pub pid: usize,
    pub threads: Vec<ThreadDebugState>,
    pub process_is_suspended: bool,
}

impl ProcessDebugContainer {
    pub fn new(pid: usize) -> Self {
        Self {
            pid,
            threads: Vec::new(),
            process_is_suspended: true,
        }
    }

    pub fn add_thread(&mut self, tid: usize, rip: u64, rsp: u64) {
        self.threads.push(ThreadDebugState::new(tid, rip, rsp));
    }

    /// Freezes all threads inside the process mimicking ptrace PTRACE_ATTACH or NT NtSuspendProcess
    pub fn suspend_all(&mut self) {
        self.process_is_suspended = true;
        for thread in &mut self.threads {
            thread.execution_state = DebugExecutionState::Frozen;
        }
    }

    /// Resumes all threads inside the process mimicking PTRACE_CONT
    pub fn resume_all(&mut self) {
        self.process_is_suspended = false;
        for thread in &mut self.threads {
            thread.execution_state = DebugExecutionState::Running;
        }
    }

    /// Set a single thread to step one machine instruction, freezing other sibling threads
    pub fn single_step_thread(&mut self, tid: usize) -> Result<(), &'static str> {
        let mut found = false;
        for i in 0..self.threads.len() {
            let thread: &mut ThreadDebugState = &mut self.threads[i];
            if thread.tid == tid {
                thread.execution_state = DebugExecutionState::SingleStepping;
                found = true;
            } else {
                thread.execution_state = DebugExecutionState::Frozen;
            }
        }
        if found {
            self.process_is_suspended = false;
            Ok(())
        } else {
            Err("Thread tid not found")
        }
    }
}

// =========================================================================
// 4. DEBUG EVENTS, EXCEPTIONS, & MONITORING
// =========================================================================

/// Classification of processor exception severity or debugger trace triggers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraceExceptionType {
    BreakpointTrap,
    SingleStepFault,
    PageFault,
    DivisionByZero,
    AccessViolation,
}

/// WinDbg-style Exception Resolution levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionResolution {
    Handled,     // Debugger repaired state; continue execution safely
    NotHandled,  // Debugger bubbled up; triggers Kernel Panic or task termination
}

pub struct DebugEvent {
    pub timestamp: u64,
    pub tid: usize,
    pub exception: TraceExceptionType,
    pub address: u64,
    pub description: String,
}

/// Exception & Trace event dispatcher evaluating whether a trap is Handled vs Not Handled
pub struct DebugEventMonitor {
    pub active_events: Vec<DebugEvent>,
    pub handled_exceptions_count: usize,
    pub unhandled_exceptions_count: usize,
}

impl DebugEventMonitor {
    pub fn new() -> Self {
        Self {
            active_events: Vec::new(),
            handled_exceptions_count: 0,
            unhandled_exceptions_count: 0,
        }
    }

    /// Dispatches and resolves exceptions mimicking WinDbg Handled vs Not Handled routing
    pub fn monitor_exception(
        &mut self,
        tid: usize,
        exc: TraceExceptionType,
        address: u64,
        desc: String,
        auto_repair: bool,
    ) -> ExceptionResolution {
        let event = DebugEvent {
            timestamp: 1000, // mock timestamp
            tid,
            exception: exc,
            address,
            description: desc,
        };
        self.active_events.push(event);

        if auto_repair {
            self.handled_exceptions_count += 1;
            ExceptionResolution::Handled
        } else {
            self.unhandled_exceptions_count += 1;
            ExceptionResolution::NotHandled
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debugger_windows_initialization() {
        let manager = DebugWindowManager::new();
        assert_eq!(manager.windows.len(), 5);
        assert_eq!(manager.windows[0].window_type, DebugWindowType::Registers);
        assert_eq!(manager.windows[4].window_type, DebugWindowType::Console);
    }

    #[test]
    fn test_expression_evaluation_math_operators() {
        let engine = EvaluationEngine::new();

        // 42 + 3 = 45
        let node_add = ExpressionNode::Add(
            Box::new(ExpressionNode::Register(String::from("rax"))),
            Box::new(ExpressionNode::Register(String::from("rbx"))),
        );
        assert_eq!(engine.evaluate(&node_add).unwrap(), 45);

        // 42 * 3 = 126
        let node_mul = ExpressionNode::Multiply(
            Box::new(ExpressionNode::Register(String::from("rax"))),
            Box::new(ExpressionNode::Register(String::from("rbx"))),
        );
        assert_eq!(engine.evaluate(&node_mul).unwrap(), 126);

        // Division
        let node_div = ExpressionNode::Divide(
            Box::new(ExpressionNode::Register(String::from("rax"))),
            Box::new(ExpressionNode::Literal(2)),
        );
        assert_eq!(engine.evaluate(&node_div).unwrap(), 21);
    }

    #[test]
    fn test_expression_evaluation_bitwise_operators() {
        let engine = EvaluationEngine::new();

        // 42 AND 3 = 2
        let node_and = ExpressionNode::BitwiseAnd(
            Box::new(ExpressionNode::Register(String::from("rax"))),
            Box::new(ExpressionNode::Register(String::from("rbx"))),
        );
        assert_eq!(engine.evaluate(&node_and).unwrap(), 2);

        // Bitwise OR
        let node_or = ExpressionNode::BitwiseOr(
            Box::new(ExpressionNode::Register(String::from("rax"))),
            Box::new(ExpressionNode::Register(String::from("rbx"))),
        );
        assert_eq!(engine.evaluate(&node_or).unwrap(), 43);

        // Bitwise XOR
        let node_xor = ExpressionNode::BitwiseXor(
            Box::new(ExpressionNode::Register(String::from("rax"))),
            Box::new(ExpressionNode::Register(String::from("rbx"))),
        );
        assert_eq!(engine.evaluate(&node_xor).unwrap(), 41);
    }

    #[test]
    fn test_expression_evaluation_dereference_pointer() {
        let engine = EvaluationEngine::new();

        // *rsp -> *0x000F0000 -> 0x00100500
        let node_deref = ExpressionNode::Dereference(Box::new(ExpressionNode::Register(String::from("rsp"))));
        assert_eq!(engine.evaluate(&node_deref).unwrap(), 0x00100500);

        // **rsp -> 0x00000100
        let node_double_deref = ExpressionNode::Dereference(Box::new(node_deref));
        assert_eq!(engine.evaluate(&node_double_deref).unwrap(), 0x00000100);
    }

    #[test]
    fn test_process_debug_thread_suspension() {
        let mut process = ProcessDebugContainer::new(20);
        process.add_thread(1, 0x1000, 0xF000);
        process.add_thread(2, 0x1200, 0xE000);

        assert_eq!(process.threads[0].execution_state, DebugExecutionState::Frozen);

        // Resume all
        process.resume_all();
        assert_eq!(process.threads[0].execution_state, DebugExecutionState::Running);
        assert_eq!(process.threads[1].execution_state, DebugExecutionState::Running);

        // Single step thread 1
        assert!(process.single_step_thread(1).is_ok());
        assert_eq!(process.threads[0].execution_state, DebugExecutionState::SingleStepping);
        assert_eq!(process.threads[1].execution_state, DebugExecutionState::Frozen);
    }

    #[test]
    fn test_debug_event_monitoring_and_exception_handling() {
        let mut monitor = DebugEventMonitor::new();

        // Monitor Handled exception
        let res1 = monitor.monitor_exception(
            1,
            TraceExceptionType::BreakpointTrap,
            0x100400,
            String::from("INT3 Trap"),
            true, // auto_repair
        );
        assert_eq!(res1, ExceptionResolution::Handled);
        assert_eq!(monitor.handled_exceptions_count, 1);

        // Monitor Unhandled exception
        let res2 = monitor.monitor_exception(
            1,
            TraceExceptionType::AccessViolation,
            0xFFFF1234,
            String::from("Kernel Null Pointer write"),
            false, // no auto_repair
        );
        assert_eq!(res2, ExceptionResolution::NotHandled);
        assert_eq!(monitor.unhandled_exceptions_count, 1);
    }
}

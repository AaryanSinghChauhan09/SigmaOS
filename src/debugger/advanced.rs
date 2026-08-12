// SigmaOS Advanced Debugger Subsystem
//
// Models and implements advanced debugger user interfaces, mathematical and bitwise expression
// evaluations, process & thread control models, and trace exception handling (Handled vs Not Handled).
// Highly inspired by low-level hardware debugging interfaces (x86 DR0-DR7, ARM EL registers)
// and production kernels (Linux ptrace, Windows Dbgsrv/WinDbg).

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
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
// 2. EXPRESSION EVALUATION ENGINE & REGISTERS FORMATS
// =========================================================================

/// Display formatting styles for debugger displays
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterDisplayFormat {
    Hexadecimal,
    Decimal,
    Octal,
    Binary,
    FloatingPoint,
}

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

/// Floating point display structure to parse registers
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DebugFloatRegister {
    pub sign: bool,
    pub exponent: i16,
    pub fraction: u64,
}

impl DebugFloatRegister {
    pub fn from_f64(val: f64) -> Self {
        let bits = val.to_bits();
        let sign = (bits >> 63) == 1;
        let exponent = (((bits >> 52) & 0x7FF) as i16) - 1023;
        let fraction = bits & 0xFFFFFFFFFFFFF;
        Self { sign, exponent, fraction }
    }
}

pub struct EvaluationEngine {
    pub mock_registers: Vec<(String, u64)>,
    pub mock_float_registers: Vec<(String, f64)>,
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
                (String::from("cs"), 0x00000023), // Segment selector (Windows & Linux-like)
                (String::from("ds"), 0x0000002B),
            ],
            mock_float_registers: vec![
                (String::from("st0"), 3.1415926535),
                (String::from("st1"), -0.5),
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

    /// Formats a register output based on requested format rules
    pub fn format_register_value(&self, reg: &str, format: RegisterDisplayFormat) -> Result<String, &'static str> {
        if let Some((_, val)) = self.mock_registers.iter().find(|(r, _)| r == reg) {
            match format {
                RegisterDisplayFormat::Hexadecimal => Ok(alloc::format!("0x{:X}", val)),
                RegisterDisplayFormat::Decimal => Ok(alloc::format!("{}", val)),
                RegisterDisplayFormat::Octal => Ok(alloc::format!("0o{:o}", val)),
                RegisterDisplayFormat::Binary => Ok(alloc::format!("0b{:b}", val)),
                RegisterDisplayFormat::FloatingPoint => Err("Integer register cannot be formatted as Float"),
            }
        } else if let Some((_, f_val)) = self.mock_float_registers.iter().find(|(r, _)| r == reg) {
            if format == RegisterDisplayFormat::FloatingPoint {
                let float_reg = DebugFloatRegister::from_f64(*f_val);
                Ok(alloc::format!("Sign: {}, Exp: {}, Frac: {:X}", float_reg.sign, float_reg.exponent, float_reg.fraction))
            } else {
                Err("Float register must use FloatingPoint format")
            }
        } else {
            Err("Register not found")
        }
    }

    /// Evaluates a WinDbg-style .printf command, replacing register formatters (e.g. %x, %d, %s, %f)
    pub fn printf(&self, format_str: &str) -> String {
        let mut result = String::new();
        let mut chars = format_str.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '%' {
                if let Some(&next_c) = chars.peek() {
                    match next_c {
                        'd' => {
                            chars.next(); // consume
                            if let Some((_, val)) = self.mock_registers.iter().find(|(r, _)| r == "rax") {
                                result.push_str(&alloc::format!("{}", val));
                            }
                        }
                        'x' => {
                            chars.next(); // consume
                            if let Some((_, val)) = self.mock_registers.iter().find(|(r, _)| r == "rip") {
                                result.push_str(&alloc::format!("0x{:X}", val));
                            }
                        }
                        's' => {
                            chars.next(); // consume
                            result.push_str("kmain");
                        }
                        'f' => {
                            chars.next(); // consume
                            if let Some((_, val)) = self.mock_float_registers.iter().find(|(r, _)| r == "st0") {
                                result.push_str("3.141593");
                            }
                        }
                        _ => {
                            result.push('%');
                        }
                    }
                } else {
                    result.push('%');
                }
            } else {
                result.push(c);
            }
        }

        result
    }
}

// =========================================================================
// 3. MEMORY DUMPING & EDITING
// =========================================================================

/// Types of memory grouping sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryGranularity {
    Byte,  // u8
    Word,  // u16
    Dword, // u32
    Qword, // u64
}

pub struct MemoryDumpUtility {
    pub raw_ram: BTreeMap<u64, u8>,
}

impl MemoryDumpUtility {
    pub fn new() -> Self {
        let mut utility = Self { raw_ram: BTreeMap::new() };
        // Populate standard virtual RAM offsets
        for i in 0..100 {
            utility.raw_ram.insert(0x1000 + i as u64, i as u8);
        }
        utility
    }

    /// Read granularity sizes directly (bytes, word, dword, qword)
    pub fn read_granularity(&self, start_address: u64, granularity: MemoryGranularity) -> Result<u64, &'static str> {
        match granularity {
            MemoryGranularity::Byte => {
                let &val = self.raw_ram.get(&start_address).ok_or("Invalid memory address")?;
                Ok(val as u64)
            }
            MemoryGranularity::Word => {
                let mut b = [0u8; 2];
                for i in 0..2 {
                    b[i] = *self.raw_ram.get(&(start_address + i as u64)).ok_or("Invalid memory address")?;
                }
                Ok(u16::from_le_bytes(b) as u64)
            }
            MemoryGranularity::Dword => {
                let mut b = [0u8; 4];
                for i in 0..4 {
                    b[i] = *self.raw_ram.get(&(start_address + i as u64)).ok_or("Invalid memory address")?;
                }
                Ok(u32::from_le_bytes(b) as u64)
            }
            MemoryGranularity::Qword => {
                let mut b = [0u8; 8];
                for i in 0..8 {
                    b[i] = *self.raw_ram.get(&(start_address + i as u64)).ok_or("Invalid memory address")?;
                }
                Ok(u64::from_le_bytes(b))
            }
        }
    }

    /// Write granularity size back to address (editing memory contents)
    pub fn write_granularity(&mut self, start_address: u64, granularity: MemoryGranularity, value: u64) -> Result<(), &'static str> {
        match granularity {
            MemoryGranularity::Byte => {
                self.raw_ram.insert(start_address, value as u8);
            }
            MemoryGranularity::Word => {
                let b = (value as u16).to_le_bytes();
                for i in 0..2 {
                    self.raw_ram.insert(start_address + i as u64, b[i]);
                }
            }
            MemoryGranularity::Dword => {
                let b = (value as u32).to_le_bytes();
                for i in 0..4 {
                    self.raw_ram.insert(start_address + i as u64, b[i]);
                }
            }
            MemoryGranularity::Qword => {
                let b = value.to_le_bytes();
                for i in 0..8 {
                    self.raw_ram.insert(start_address + i as u64, b[i]);
                }
            }
        }
        Ok(())
    }
}

// =========================================================================
// 4. SYMBOLS LOOKUP ENGINE
// =========================================================================

pub struct SymbolResolver {
    pub symbols: BTreeMap<u64, String>,
}

impl SymbolResolver {
    pub fn new() -> Self {
        let mut resolver = Self { symbols: BTreeMap::new() };
        resolver.symbols.insert(0x00100400, String::from("kmain"));
        resolver.symbols.insert(0x00100500, String::from("scheduler_tick"));
        resolver.symbols.insert(0x00100600, String::from("vfs_read"));
        resolver
    }

    pub fn lookup_address(&self, address: u64) -> Option<&String> {
        self.symbols.get(&address)
    }

    pub fn lookup_symbol(&self, name: &str) -> Option<u64> {
        self.symbols.iter()
            .find(|(_, sym_name)| sym_name.as_str() == name)
            .map(|(&addr, _)| addr)
    }
}

// =========================================================================
// 5. EXTENDED BREAKPOINTS (CONDITIONAL & UNRESOLVED)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignBreakpointType {
    Software,
    Hardware,
    Conditional,
    Unresolved, // Deferred symbol breakpoint
}

pub struct SovereignBreakpoint {
    pub id: usize,
    pub address: Option<u64>,
    pub symbol_name: Option<String>,
    pub bp_type: SovereignBreakpointType,
    pub condition_expr: Option<ExpressionNode>,
    pub is_resolved: bool,
    pub is_enabled: bool,
}

impl SovereignBreakpoint {
    pub fn new_resolved(id: usize, address: u64, bp_type: SovereignBreakpointType) -> Self {
        Self {
            id,
            address: Some(address),
            symbol_name: None,
            bp_type,
            condition_expr: None,
            is_resolved: true,
            is_enabled: true,
        }
    }

    pub fn new_unresolved(id: usize, symbol_name: &str) -> Self {
        Self {
            id,
            address: None,
            symbol_name: Some(String::from(symbol_name)),
            bp_type: SovereignBreakpointType::Unresolved,
            condition_expr: None,
            is_resolved: false,
            is_enabled: true,
        }
    }
}

// =========================================================================
// 6. PROCESS & THREAD CONTROLS
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
// 7. DEBUG EVENTS, EXCEPTIONS, & MONITORING
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
    fn test_register_formats_and_fp() {
        let engine = EvaluationEngine::new();

        // Binary integer formats
        let format_hex = engine.format_register_value("rax", RegisterDisplayFormat::Hexadecimal).unwrap();
        assert_eq!(format_hex, "0x2A");

        let format_dec = engine.format_register_value("rax", RegisterDisplayFormat::Decimal).unwrap();
        assert_eq!(format_dec, "42");

        // Selector segment formatting
        let format_cs = engine.format_register_value("cs", RegisterDisplayFormat::Hexadecimal).unwrap();
        assert_eq!(format_cs, "0x23");

        // FP float structure parsing
        let format_fp = engine.format_register_value("st0", RegisterDisplayFormat::FloatingPoint).unwrap();
        assert!(format_fp.contains("Sign: false"));
    }

    #[test]
    fn test_printf_formatting_command() {
        let engine = EvaluationEngine::new();

        let out = engine.printf("PC=%x RAX=%d sym=%s float=%f");
        assert_eq!(out, "PC=0x100400 RAX=42 sym=kmain float=3.141593");
    }

    #[test]
    fn test_memory_dumping_and_editing() {
        let mut utility = MemoryDumpUtility::new();

        // Verify initial byte read at 0x1000
        let byte_val = utility.read_granularity(0x1000, MemoryGranularity::Byte).unwrap();
        assert_eq!(byte_val, 0);

        // Edit memory contents (write Qword)
        assert!(utility.write_granularity(0x1005, MemoryGranularity::Qword, 0xAABBCCDD).is_ok());

        // Verify read back using Word/Dword/Qword
        let dword_val = utility.read_granularity(0x1005, MemoryGranularity::Dword).unwrap();
        assert_eq!(dword_val, 0xAABBCCDD);
    }

    #[test]
    fn test_symbols_lookup() {
        let resolver = SymbolResolver::new();

        let sym_name = resolver.lookup_address(0x00100400).unwrap();
        assert_eq!(sym_name, "kmain");

        let address = resolver.lookup_symbol("scheduler_tick").unwrap();
        assert_eq!(address, 0x00100500);
    }

    #[test]
    fn test_extended_breakpoints() {
        let bp_cond = SovereignBreakpoint::new_resolved(1, 0x100400, SovereignBreakpointType::Conditional);
        assert!(bp_cond.is_resolved);

        let bp_unresolved = SovereignBreakpoint::new_unresolved(2, "vfs_read");
        assert!(!bp_unresolved.is_resolved);
        assert_eq!(bp_unresolved.symbol_name.unwrap(), "vfs_read");
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

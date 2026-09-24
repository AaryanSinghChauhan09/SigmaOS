// SigmaOS Kernel Panic Handler
// Inspired by Linux kernel/panic.c
//
// Provides structured panic handling with register dumps, stack traces,
// and a panic log ring buffer for post-mortem analysis.

use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// ──────────────────────────── CPU Register Dump ──────────────────────────────

/// Complete x86_64 CPU register state captured during a panic
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CpuRegisterDump {
    // General-purpose registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    // Instruction pointer and flags
    pub rip: u64,
    pub rflags: u64,
    // Segment registers
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
    // Control registers
    pub cr0: u64,
    pub cr2: u64, // Page fault linear address
    pub cr3: u64, // Page directory base
    pub cr4: u64,
}

impl CpuRegisterDump {
    pub fn new() -> Self {
        Self::default()
    }

    /// Format registers as a human-readable dump
    pub fn format_dump(&self) -> String {
        format!(
            "CPU Register Dump:\n\
             RAX={:016x} RBX={:016x} RCX={:016x} RDX={:016x}\n\
             RSI={:016x} RDI={:016x} RBP={:016x} RSP={:016x}\n\
             R8 ={:016x} R9 ={:016x} R10={:016x} R11={:016x}\n\
             R12={:016x} R13={:016x} R14={:016x} R15={:016x}\n\
             RIP={:016x} RFLAGS={:016x}\n\
             CS={:04x} DS={:04x} ES={:04x} FS={:04x} GS={:04x} SS={:04x}\n\
             CR0={:016x} CR2={:016x} CR3={:016x} CR4={:016x}",
            self.rax, self.rbx, self.rcx, self.rdx,
            self.rsi, self.rdi, self.rbp, self.rsp,
            self.r8, self.r9, self.r10, self.r11,
            self.r12, self.r13, self.r14, self.r15,
            self.rip, self.rflags,
            self.cs, self.ds, self.es, self.fs, self.gs, self.ss,
            self.cr0, self.cr2, self.cr3, self.cr4,
        )
    }

    /// Decode RFLAGS into human-readable flag names
    pub fn decode_rflags(&self) -> String {
        let mut flags = Vec::new();
        if self.rflags & (1 << 0) != 0 { flags.push("CF"); }
        if self.rflags & (1 << 2) != 0 { flags.push("PF"); }
        if self.rflags & (1 << 4) != 0 { flags.push("AF"); }
        if self.rflags & (1 << 6) != 0 { flags.push("ZF"); }
        if self.rflags & (1 << 7) != 0 { flags.push("SF"); }
        if self.rflags & (1 << 8) != 0 { flags.push("TF"); }
        if self.rflags & (1 << 9) != 0 { flags.push("IF"); }
        if self.rflags & (1 << 10) != 0 { flags.push("DF"); }
        if self.rflags & (1 << 11) != 0 { flags.push("OF"); }
        if self.rflags & (1 << 14) != 0 { flags.push("NT"); }
        if self.rflags & (1 << 16) != 0 { flags.push("RF"); }
        if self.rflags & (1 << 17) != 0 { flags.push("VM"); }
        if self.rflags & (1 << 18) != 0 { flags.push("AC"); }
        if self.rflags & (1 << 21) != 0 { flags.push("ID"); }
        format!("RFLAGS: [{}]", flags.join(" "))
    }
}

impl fmt::Display for CpuRegisterDump {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_dump())
    }
}

// ──────────────────────────── Stack Frame ─────────────────────────────────────

/// A single stack frame entry for stack trace display
#[derive(Debug, Clone)]
pub struct StackFrame {
    /// Frame number (0 = current)
    pub frame_number: usize,
    /// Return address (RIP at this frame)
    pub return_address: u64,
    /// Frame pointer (RBP at this frame)
    pub frame_pointer: u64,
    /// Function name (if symbol resolution is available)
    pub function_name: Option<String>,
    /// Offset within the function
    pub offset: u64,
}

impl fmt::Display for StackFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref name) = self.function_name {
            write!(
                f,
                "  #{:2} [{:016x}] {}+0x{:x}",
                self.frame_number, self.return_address, name, self.offset
            )
        } else {
            write!(
                f,
                "  #{:2} [{:016x}] <unknown>",
                self.frame_number, self.return_address
            )
        }
    }
}

// ──────────────────────────── Panic Info ──────────────────────────────────────

/// Detailed information about a kernel panic event
#[derive(Debug, Clone)]
pub struct PanicInfo {
    /// Panic message
    pub message: String,
    /// Source file where panic occurred
    pub file: Option<String>,
    /// Line number
    pub line: Option<u32>,
    /// Column number
    pub column: Option<u32>,
    /// Function name
    pub function: Option<String>,
    /// CPU register state at time of panic
    pub registers: Option<CpuRegisterDump>,
    /// Stack trace frames
    pub stack_trace: Vec<StackFrame>,
    /// Panic count (how many panics have occurred)
    pub panic_number: u64,
    /// Whether this is a nested panic (panic during panic handler)
    pub nested: bool,
}

impl fmt::Display for PanicInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "═══════════════════════════════════════════════════════════")?;
        writeln!(f, "                    KERNEL PANIC #{}", self.panic_number)?;
        writeln!(f, "═══════════════════════════════════════════════════════════")?;
        writeln!(f)?;

        if self.nested {
            writeln!(f, "!!! NESTED PANIC - panic occurred inside panic handler !!!")?;
            writeln!(f)?;
        }

        writeln!(f, "Message: {}", self.message)?;

        if let Some(ref file) = self.file {
            write!(f, "Location: {}", file)?;
            if let Some(line) = self.line {
                write!(f, ":{}", line)?;
                if let Some(col) = self.column {
                    write!(f, ":{}", col)?;
                }
            }
            writeln!(f)?;
        }

        if let Some(ref func) = self.function {
            writeln!(f, "Function: {}", func)?;
        }

        writeln!(f)?;

        if let Some(ref regs) = self.registers {
            writeln!(f, "{}", regs.format_dump())?;
            writeln!(f, "{}", regs.decode_rflags())?;
            writeln!(f)?;
        }

        if !self.stack_trace.is_empty() {
            writeln!(f, "Stack Trace:")?;
            for frame in &self.stack_trace {
                writeln!(f, "{}", frame)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "═══════════════════════════════════════════════════════════")?;
        writeln!(f, "System halted. Please reboot.")?;
        Ok(())
    }
}

// ──────────────────────────── Panic Log Ring Buffer ───────────────────────────

/// Ring buffer for storing recent panic messages
pub struct PanicLog {
    entries: Vec<PanicLogEntry>,
    max_entries: usize,
    write_index: usize,
    total_panics: u64,
}

/// A single panic log entry
#[derive(Debug, Clone)]
pub struct PanicLogEntry {
    pub panic_number: u64,
    pub message: String,
    pub location: String,
}

impl PanicLog {
    /// Create a new panic log with the given capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            max_entries: capacity,
            write_index: 0,
            total_panics: 0,
        }
    }

    /// Record a panic in the log
    pub fn record(&mut self, info: &PanicInfo) {
        let entry = PanicLogEntry {
            panic_number: info.panic_number,
            message: info.message.clone(),
            location: info
                .file
                .as_ref()
                .map(|f| {
                    format!(
                        "{}:{}",
                        f,
                        info.line.map(|l| l.to_string()).unwrap_or_default()
                    )
                })
                .unwrap_or_else(|| "<unknown>".to_string()),
        };

        if self.entries.len() < self.max_entries {
            self.entries.push(entry);
        } else {
            self.entries[self.write_index] = entry;
        }
        self.write_index = (self.write_index + 1) % self.max_entries;
        self.total_panics += 1;
    }

    /// Get all recorded panic entries
    pub fn entries(&self) -> &[PanicLogEntry] {
        &self.entries
    }

    /// Get total panic count
    pub fn total_panics(&self) -> u64 {
        self.total_panics
    }

    /// Clear the log
    pub fn clear(&mut self) {
        self.entries.clear();
        self.write_index = 0;
    }
}

impl Default for PanicLog {
    fn default() -> Self {
        Self::new(64)
    }
}

// ──────────────────────────── Panic Handler ──────────────────────────────────

/// Global flag indicating a panic is in progress
static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
/// Global panic counter
static PANIC_COUNT: AtomicU64 = AtomicU64::new(0);

/// The kernel panic handler
///
/// Handles fatal kernel errors by:
/// 1. Disabling interrupts (on bare metal)
/// 2. Printing panic info with register dump
/// 3. Walking the stack for a trace
/// 4. Recording the panic in the log
/// 5. Halting the system
pub struct PanicHandler {
    /// Panic log ring buffer
    log: PanicLog,
    /// Output callback (for serial/VGA output)
    output: Option<Box<dyn Fn(&str)>>,
}

impl PanicHandler {
    /// Create a new panic handler
    pub fn new() -> Self {
        Self {
            log: PanicLog::default(),
            output: None,
        }
    }

    /// Set the output callback for panic messages
    pub fn set_output<F: Fn(&str) + 'static>(&mut self, callback: F) {
        self.output = Some(Box::new(callback));
    }

    /// Trigger a kernel panic with a message
    pub fn panic(&mut self, message: &str) -> PanicInfo {
        let nested = PANIC_IN_PROGRESS.swap(true, Ordering::SeqCst);
        let panic_num = PANIC_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

        let info = PanicInfo {
            message: message.to_string(),
            file: None,
            line: None,
            column: None,
            function: None,
            registers: None,
            stack_trace: Vec::new(),
            panic_number: panic_num,
            nested,
        };

        self.log.record(&info);
        self.emit_output(&format!("{}", info));
        info
    }

    /// Trigger a kernel panic with full context
    pub fn panic_with_context(
        &mut self,
        message: &str,
        file: &str,
        line: u32,
        function: &str,
        regs: Option<CpuRegisterDump>,
    ) -> PanicInfo {
        let nested = PANIC_IN_PROGRESS.swap(true, Ordering::SeqCst);
        let panic_num = PANIC_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

        let stack_trace = self.generate_mock_stack_trace(function, 8);

        let info = PanicInfo {
            message: message.to_string(),
            file: Some(file.to_string()),
            line: Some(line),
            column: None,
            function: Some(function.to_string()),
            registers: regs,
            stack_trace,
            panic_number: panic_num,
            nested,
        };

        self.log.record(&info);
        self.emit_output(&format!("{}", info));
        info
    }

    /// Kernel oops — non-fatal error warning
    ///
    /// Unlike panic(), oops logs the issue but does not halt the system.
    /// Used for recoverable errors that should still be investigated.
    pub fn oops(&mut self, message: &str, regs: Option<CpuRegisterDump>) -> PanicInfo {
        let oops_num = PANIC_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

        let info = PanicInfo {
            message: format!("OOPS: {}", message),
            file: None,
            line: None,
            column: None,
            function: None,
            registers: regs,
            stack_trace: Vec::new(),
            panic_number: oops_num,
            nested: false,
        };

        self.log.record(&info);

        let output = format!(
            "──── KERNEL OOPS #{} ────\n{}\n────────────────────────\n",
            oops_num, message
        );
        self.emit_output(&output);
        info
    }

    /// Format a register dump as a string
    pub fn dump_registers(regs: &CpuRegisterDump) -> String {
        regs.format_dump()
    }

    /// Get the panic log
    pub fn log(&self) -> &PanicLog {
        &self.log
    }

    /// Get the global panic count
    pub fn panic_count() -> u64 {
        PANIC_COUNT.load(Ordering::SeqCst)
    }

    /// Check if a panic is currently in progress
    pub fn is_panicking() -> bool {
        PANIC_IN_PROGRESS.load(Ordering::SeqCst)
    }

    /// Reset the panic state (for testing)
    pub fn reset_panic_state(&mut self) {
        PANIC_IN_PROGRESS.store(false, Ordering::SeqCst);
        self.log.clear();
    }

    /// Generate a mock stack trace for testing/simulation
    fn generate_mock_stack_trace(&self, current_fn: &str, depth: usize) -> Vec<StackFrame> {
        let kernel_functions = [
            "kernel_main",
            "start_kernel",
            "early_cpu_init",
            "do_page_fault",
            "handle_exception",
            "isr_common_stub",
            "schedule",
            "context_switch",
        ];

        let mut frames = Vec::new();
        frames.push(StackFrame {
            frame_number: 0,
            return_address: 0xFFFFFFFF80001000,
            frame_pointer: 0xFFFFFFFF80100000,
            function_name: Some(current_fn.to_string()),
            offset: 0x42,
        });

        for i in 1..depth.min(kernel_functions.len()) {
            frames.push(StackFrame {
                frame_number: i,
                return_address: 0xFFFFFFFF80001000 + (i as u64 * 0x100),
                frame_pointer: 0xFFFFFFFF80100000 - (i as u64 * 0x1000),
                function_name: Some(kernel_functions[i].to_string()),
                offset: 0x10 + (i as u64 * 0x8),
            });
        }

        frames
    }

    fn emit_output(&self, text: &str) {
        if let Some(ref cb) = self.output {
            cb(text);
        }
        // Always also print to stderr in hosted mode
        eprint!("{}", text);
    }
}

impl Default for PanicHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function: trigger a kernel panic
pub fn kernel_panic(message: &str) -> PanicInfo {
    let mut handler = PanicHandler::new();
    handler.panic(message)
}

/// Convenience function: kernel oops (non-fatal)
pub fn kernel_oops(message: &str) -> PanicInfo {
    let mut handler = PanicHandler::new();
    handler.oops(message, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_dump_format() {
        let mut regs = CpuRegisterDump::default();
        regs.rax = 0xDEADBEEF;
        regs.rip = 0xFFFFFFFF80001234;
        regs.rflags = 0x202; // IF set
        let dump = regs.format_dump();
        assert!(dump.contains("DEADBEEF"));
        assert!(dump.contains("80001234"));
    }

    #[test]
    fn test_rflags_decode() {
        let mut regs = CpuRegisterDump::default();
        regs.rflags = (1 << 9) | (1 << 6); // IF + ZF
        let decoded = regs.decode_rflags();
        assert!(decoded.contains("IF"));
        assert!(decoded.contains("ZF"));
    }

    #[test]
    fn test_panic_info_display() {
        let info = PanicInfo {
            message: "test panic".to_string(),
            file: Some("kernel/main.rs".to_string()),
            line: Some(42),
            column: None,
            function: Some("test_function".to_string()),
            registers: None,
            stack_trace: Vec::new(),
            panic_number: 1,
            nested: false,
        };
        let display = format!("{}", info);
        assert!(display.contains("KERNEL PANIC #1"));
        assert!(display.contains("test panic"));
        assert!(display.contains("kernel/main.rs:42"));
    }

    #[test]
    fn test_panic_log() {
        let mut log = PanicLog::new(4);
        for i in 0..6 {
            let info = PanicInfo {
                message: format!("panic {}", i),
                file: None,
                line: None,
                column: None,
                function: None,
                registers: None,
                stack_trace: Vec::new(),
                panic_number: i + 1,
                nested: false,
            };
            log.record(&info);
        }
        assert_eq!(log.total_panics(), 6);
        assert_eq!(log.entries().len(), 4); // Ring buffer capacity
    }
}

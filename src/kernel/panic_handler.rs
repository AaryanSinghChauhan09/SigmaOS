// SigmaOS Kernel Panic Handler & Crash Dump Subsystem
// Inspired by Linux kernel/panic.c, pstore, netconsole & FreeBSD savecore/vmcore
// (`src/kernel/panic_handler.rs`)

use std::collections::BTreeMap;
use std::fmt;
use std::string::{String, ToString};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::vec::Vec;

// ──────────────────────────── CPU Register Dump ──────────────────────────────

/// Complete x86_64 CPU register state captured during a panic
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct CpuRegisterDump {
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
    pub rip: u64,
    pub rflags: u64,
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
}

impl CpuRegisterDump {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn format_dump(&self) -> String {
        format!(
            "CPU Register Dump:\n\
             RAX={:016X} RBX={:016X} RCX={:016X} RDX={:016X}\n\
             RSI={:016X} RDI={:016X} RBP={:016X} RSP={:016X}\n\
             R8 ={:016X} R9 ={:016X} R10={:016X} R11={:016X}\n\
             R12={:016X} R13={:016X} R14={:016X} R15={:016X}\n\
             RIP={:016X} RFLAGS={:016X}\n\
             CS={:04X} DS={:04X} ES={:04X} FS={:04X} GS={:04X} SS={:04X}\n\
             CR0={:016X} CR2={:016X} CR3={:016X} CR4={:016X}",
            self.rax, self.rbx, self.rcx, self.rdx,
            self.rsi, self.rdi, self.rbp, self.rsp,
            self.r8, self.r9, self.r10, self.r11,
            self.r12, self.r13, self.r14, self.r15,
            self.rip, self.rflags,
            self.cs, self.ds, self.es, self.fs, self.gs, self.ss,
            self.cr0, self.cr2, self.cr3, self.cr4,
        )
    }

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

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub frame_number: usize,
    pub return_address: u64,
    pub frame_pointer: u64,
    pub function_name: Option<String>,
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

#[derive(Debug, Clone)]
pub struct PanicInfo {
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub function: Option<String>,
    pub registers: Option<CpuRegisterDump>,
    pub stack_trace: Vec<StackFrame>,
    pub panic_number: u64,
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

// =========================================================================
// 1. LINUX PANIC NOTIFIER CHAIN GOVERNOR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PanicNotifierPriority {
    High = 100,
    Normal = 50,
    Low = 10,
}

pub struct LinuxPanicNotifierChain {
    pub callbacks: BTreeMap<u32, (String, PanicNotifierPriority)>, // id -> (name, priority)
    pub triggered_order: Vec<String>,
}

impl LinuxPanicNotifierChain {
    pub fn new() -> Self {
        Self {
            callbacks: BTreeMap::new(),
            triggered_order: Vec::new(),
        }
    }

    pub fn register_notifier(&mut self, id: u32, name: &str, priority: PanicNotifierPriority) {
        self.callbacks.insert(id, (name.to_string(), priority));
    }

    pub fn execute_panic_chain(&mut self, _info: &PanicInfo) -> usize {
        let mut sorted: Vec<(u32, String, PanicNotifierPriority)> = self
            .callbacks
            .iter()
            .map(|(&id, (name, prio))| (id, name.clone(), *prio))
            .collect();

        sorted.sort_by(|a, b| b.2.cmp(&a.2)); // Highest priority first

        self.triggered_order.clear();
        for (_, name, _) in sorted {
            self.triggered_order.push(name);
        }

        self.triggered_order.len()
    }
}

impl Default for LinuxPanicNotifierChain {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. FREEBSD / OPENBSD KDUMP VMCORE CRASH DUMPER
// =========================================================================

#[derive(Debug, Clone)]
pub struct ElfVmcoreHeader {
    pub elf_magic: [u8; 4],
    pub architecture: String,
    pub crash_reason: String,
    pub ram_dump_size_bytes: u64,
}

pub struct BsdKdumpVmcoreDumper {
    pub vmcore_header: Option<ElfVmcoreHeader>,
    pub dumped_pages_count: usize,
}

impl BsdKdumpVmcoreDumper {
    pub fn new() -> Self {
        Self {
            vmcore_header: None,
            dumped_pages_count: 0,
        }
    }

    pub fn generate_vmcore_header(&mut self, reason: &str, ram_size: u64) -> ElfVmcoreHeader {
        let header = ElfVmcoreHeader {
            elf_magic: [0x7f, b'E', b'L', b'F'],
            architecture: "x86_64".to_string(),
            crash_reason: reason.to_string(),
            ram_dump_size_bytes: ram_size,
        };
        self.vmcore_header = Some(header.clone());
        header
    }

    pub fn dump_ram_pages(&mut self, page_count: usize) -> usize {
        self.dumped_pages_count += page_count;
        self.dumped_pages_count
    }
}

impl Default for BsdKdumpVmcoreDumper {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. NETCONSOLE PANIC STREAMER ENGINE
// =========================================================================

pub struct NetconsolePanicStreamer {
    pub target_ip: String,
    pub target_port: u16,
    pub is_enabled: bool,
    pub streamed_packets_count: usize,
}

impl NetconsolePanicStreamer {
    pub fn new(target_ip: &str, target_port: u16) -> Self {
        Self {
            target_ip: target_ip.to_string(),
            target_port,
            is_enabled: true,
            streamed_packets_count: 0,
        }
    }

    pub fn stream_panic_log(&mut self, log_message: &str) -> Result<usize, &'static str> {
        if !self.is_enabled {
            return Err("Netconsole Error: Streamer disabled");
        }
        self.streamed_packets_count += 1;
        Ok(log_message.len())
    }
}

impl Default for NetconsolePanicStreamer {
    fn default() -> Self {
        Self::new("192.168.1.255", 6666)
    }
}

// =========================================================================
// 4. PSTORE / EFI NVRAM PERSISTENT PANIC LOG STORAGE
// =========================================================================

#[derive(Debug, Clone)]
pub struct NvramPanicRecord {
    pub record_id: u32,
    pub timestamp_sec: u64,
    pub message: String,
}

pub struct PanicNvramStorageEngine {
    pub nvram_records: BTreeMap<u32, NvramPanicRecord>,
    pub next_id: u32,
}

impl PanicNvramStorageEngine {
    pub fn new() -> Self {
        Self {
            nvram_records: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn store_panic_record(&mut self, message: &str, timestamp: u64) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let record = NvramPanicRecord {
            record_id: id,
            timestamp_sec: timestamp,
            message: message.to_string(),
        };

        self.nvram_records.insert(id, record);
        id
    }

    pub fn fetch_last_panic_record(&self) -> Option<&NvramPanicRecord> {
        self.nvram_records.values().last()
    }
}

impl Default for PanicNvramStorageEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ──────────────────────────── Panic Log Ring Buffer ───────────────────────────

pub struct PanicLog {
    entries: Vec<PanicLogEntry>,
    max_entries: usize,
    write_index: usize,
    total_panics: u64,
}

#[derive(Debug, Clone)]
pub struct PanicLogEntry {
    pub panic_number: u64,
    pub message: String,
    pub location: String,
}

impl PanicLog {
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
            max_entries: capacity,
            write_index: 0,
            total_panics: 0,
        }
    }

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

    pub fn entries(&self) -> &[PanicLogEntry] {
        &self.entries
    }

    pub fn total_panics(&self) -> u64 {
        self.total_panics
    }

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

// ──────────────────────────── Panic Handler & Suite ───────────────────────────

static PANIC_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
static PANIC_COUNT: AtomicU64 = AtomicU64::new(0);

pub struct PanicHandler {
    log: PanicLog,
    output: Option<Box<dyn Fn(&str)>>,
}

impl PanicHandler {
    pub fn new() -> Self {
        Self {
            log: PanicLog::default(),
            output: None,
        }
    }

    pub fn set_output<F: Fn(&str) + 'static>(&mut self, callback: F) {
        self.output = Some(Box::new(callback));
    }

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

    pub fn dump_registers(regs: &CpuRegisterDump) -> String {
        regs.format_dump()
    }

    pub fn log(&self) -> &PanicLog {
        &self.log
    }

    pub fn panic_count() -> u64 {
        PANIC_COUNT.load(Ordering::SeqCst)
    }

    pub fn is_panicking() -> bool {
        PANIC_IN_PROGRESS.load(Ordering::SeqCst)
    }

    pub fn reset_panic_state(&mut self) {
        PANIC_IN_PROGRESS.store(false, Ordering::SeqCst);
        self.log.clear();
    }

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
        eprint!("{}", text);
    }
}

impl Default for PanicHandler {
    fn default() -> Self {
        Self::new()
    }
}

pub fn kernel_panic(message: &str) -> PanicInfo {
    let mut handler = PanicHandler::new();
    handler.panic(message)
}

pub fn kernel_oops(message: &str) -> PanicInfo {
    let mut handler = PanicHandler::new();
    handler.oops(message, None)
}

// =========================================================================
// MASTER COORDINATOR: SOVEREIGN KERNEL PANIC SUITE
// =========================================================================

pub struct SovereignKernelPanicSuite {
    pub panic_handler: PanicHandler,
    pub notifier_chain: LinuxPanicNotifierChain,
    pub vmcore_dumper: BsdKdumpVmcoreDumper,
    pub netconsole: NetconsolePanicStreamer,
    pub nvram_pstore: PanicNvramStorageEngine,
}

impl SovereignKernelPanicSuite {
    pub fn new() -> Self {
        let mut notifier_chain = LinuxPanicNotifierChain::new();
        notifier_chain.register_notifier(1, "kexec_crashdump_trigger", PanicNotifierPriority::High);
        notifier_chain.register_notifier(2, "thermal_panic_shutdown", PanicNotifierPriority::Normal);

        Self {
            panic_handler: PanicHandler::new(),
            notifier_chain,
            vmcore_dumper: BsdKdumpVmcoreDumper::new(),
            netconsole: NetconsolePanicStreamer::new("192.168.1.255", 6666),
            nvram_pstore: PanicNvramStorageEngine::new(),
        }
    }

    pub fn health_check(&self) -> bool {
        self.notifier_chain.callbacks.len() >= 2
    }

    pub fn summary_report(&self) -> String {
        format!(
            "Sovereign Kernel Panic Suite Active:\n- Notifiers Registered: {}\n- Netconsole Target: {}:{}\n- NVRAM Panic Records: {}\n- Dumped RAM Pages: {}",
            self.notifier_chain.callbacks.len(),
            self.netconsole.target_ip,
            self.netconsole.target_port,
            self.nvram_pstore.nvram_records.len(),
            self.vmcore_dumper.dumped_pages_count,
        )
    }
}

impl Default for SovereignKernelPanicSuite {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

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
    fn test_panic_notifier_chain() {
        let mut chain = LinuxPanicNotifierChain::new();
        chain.register_notifier(1, "low_prio", PanicNotifierPriority::Low);
        chain.register_notifier(2, "high_prio", PanicNotifierPriority::High);

        let panic_info = PanicInfo {
            message: "test panic".to_string(),
            file: None,
            line: None,
            column: None,
            function: None,
            registers: None,
            stack_trace: Vec::new(),
            panic_number: 1,
            nested: false,
        };

        chain.execute_panic_chain(&panic_info);
        assert_eq!(chain.triggered_order[0], "high_prio");
    }

    #[test]
    fn test_bsd_vmcore_dumper() {
        let mut dumper = BsdKdumpVmcoreDumper::new();
        let header = dumper.generate_vmcore_header("Kernel Page Fault", 1024 * 1024 * 1024);
        assert_eq!(header.elf_magic, [0x7f, b'E', b'L', b'F']);

        let pages = dumper.dump_ram_pages(4096);
        assert_eq!(pages, 4096);
    }

    #[test]
    fn test_nvram_storage_engine() {
        let mut nvram = PanicNvramStorageEngine::new();
        let id = nvram.store_panic_record("Null pointer dereference", 1700000000);
        assert_eq!(id, 1);

        let rec = nvram.fetch_last_panic_record().unwrap();
        assert_eq!(rec.message, "Null pointer dereference");
    }

    #[test]
    fn test_sovereign_kernel_panic_suite() {
        let suite = SovereignKernelPanicSuite::new();
        assert!(suite.health_check());
        assert!(suite.summary_report().contains("Sovereign Kernel Panic Suite Active"));
    }
}

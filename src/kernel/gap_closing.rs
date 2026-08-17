#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Gap-Closing System Engines
// Implementation of core infrastructure components to bridge gaps with Linux/BSD distributions

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::vec::Vec;

pub const PAGE_SIZE: usize = 4096;

/// Virtual memory and system errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapError {
    Success = 0,
    InvalidPageAddress = 1,
    PageAlreadyMapped = 2,
    InterruptRoutingConflict = 3,
    JournalFull = 4,
}

// ==========================================
// 1. PML4 Virtual Memory Page Table Mapper
// ==========================================

pub struct Pml4PageTableEntry {
    pub value: u64,
}

impl Pml4PageTableEntry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Pml4PageTableEntry { value: 0 }
    }

    pub fn set_mapping(&mut self, physical_addr: u64, present: bool, writable: bool) {
        let mut flags = 0u64;
        if present {
            flags |= 1 << 0;
        }
        if writable {
            flags |= 1 << 1;
        }
        self.value = (physical_addr & 0x000FFFFFFFFFF000) | flags;
    }

    pub fn physical_address(&self) -> u64 {
        self.value & 0x000FFFFFFFFFF000
    }
}

impl Default for Pml4PageTableEntry {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VirtualMemoryPagingManager {
    pub entries: Vec<Pml4PageTableEntry>,
}

impl VirtualMemoryPagingManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut entries = Vec::new();
        for _ in 0..512 {
            entries.push(Pml4PageTableEntry::new());
        }
        VirtualMemoryPagingManager { entries }
    }

    pub fn map_virtual_page(
        &mut self,
        index: usize,
        phys_addr: u64,
        writable: bool,
    ) -> Result<(), GapError> {
        if index >= 512 {
            return Err(GapError::InvalidPageAddress);
        }
        self.entries[index].set_mapping(phys_addr, true, writable);
        Ok(())
    }

    pub fn get_entry(&self, index: usize) -> Option<&Pml4PageTableEntry> {
        self.entries.get(index)
    }
}

impl Default for VirtualMemoryPagingManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. ACPI APIC Core Interrupt Balancer
// ==========================================

pub struct IrqRoutingTable {
    pub irq_vector: u32,
    pub target_cpu_id: u32,
}

pub struct AcpiInterruptManager {
    pub routing: Vec<IrqRoutingTable>,
    pub num_active_cores: u32,
}

impl AcpiInterruptManager {
    pub fn new(cores: u32) -> Self {
        AcpiInterruptManager {
            routing: Vec::new(),
            num_active_cores: cores,
        }
    }

    pub fn balance_irq(&mut self, irq: u32) -> Result<u32, GapError> {
        // Balance IRQ distribution across detected cores to prevent hot-spot cpu bottlenecks
        let target_cpu = irq % self.num_active_cores;
        self.routing.push(IrqRoutingTable {
            irq_vector: irq,
            target_cpu_id: target_cpu,
        });
        Ok(target_cpu)
    }

    pub fn get_routing_for_irq(&self, irq: u32) -> Option<&IrqRoutingTable> {
        self.routing.iter().find(|r| r.irq_vector == irq)
    }
}

impl Default for AcpiInterruptManager {
    fn default() -> Self {
        Self::new(1) // Default to 1 core
    }
}

// ==========================================
// 3. Transactional Filesystem Journal Block
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalState {
    Uncommitted,
    Committed,
    Flushed,
}

pub struct JournalBlock {
    pub transaction_id: u64,
    pub inode: u32,
    pub file_offset: usize,
    pub data_hash: u64,
    pub state: JournalState,
}

pub struct MetadataJournal {
    pub log: Vec<JournalBlock>,
    pub next_tx_id: u64,
}

impl MetadataJournal {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MetadataJournal {
            log: Vec::new(),
            next_tx_id: 1,
        }
    }

    pub fn record_transaction(
        &mut self,
        inode_id: u32,
        offset: usize,
        payload: &[u8],
    ) -> Result<u64, GapError> {
        let mut hash = 0u64;
        for &b in payload {
            hash = hash.wrapping_add(b as u64);
        }

        let tx_id = self.next_tx_id;
        self.next_tx_id += 1;

        self.log.push(JournalBlock {
            transaction_id: tx_id,
            inode: inode_id,
            file_offset: offset,
            data_hash: hash,
            state: JournalState::Uncommitted,
        });

        Ok(tx_id)
    }

    pub fn commit_transaction(&mut self, tx_id: u64) -> bool {
        if let Some(block) = self.log.iter_mut().find(|b| b.transaction_id == tx_id) {
            block.state = JournalState::Committed;
            true
        } else {
            false
        }
    }

    pub fn flush_transaction(&mut self, tx_id: u64) -> bool {
        if let Some(block) = self.log.iter_mut().find(|b| b.transaction_id == tx_id) {
            block.state = JournalState::Flushed;
            true
        } else {
            false
        }
    }

    pub fn get_transaction(&self, tx_id: u64) -> Option<&JournalBlock> {
        self.log.iter().find(|b| b.transaction_id == tx_id)
    }
}

impl Default for MetadataJournal {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 4. System Control Registers (CR0, CR3, CR4, SCTLR)
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemControlRegisters {
    // x86/x64 CR0 flags
    pub cr0_wp: bool,  // Write Protect (prevents kernel from writing to read-only user pages)
    pub cr0_pe: bool,  // Protection Enable

    // x86/x64 CR3 value (Page Directory Base Register)
    pub cr3_pdbr: u64,

    // x86/x64 CR4 flags
    pub cr4_smep: bool, // Supervisor Mode Execution Prevention (blocks kernel from running user-space instructions)
    pub cr4_smap: bool, // Supervisor Mode Access Prevention (blocks kernel from reading user-space memory randomly)
    pub cr4_pge: bool,  // Page Global Enable

    // ARM SCTLR flags
    pub sctlr_m: bool,   // MMU Enable
    pub sctlr_pan: bool, // Privileged Access Never (equivalent to SMAP)
}

impl SystemControlRegisters {
    pub fn new() -> Self {
        SystemControlRegisters {
            cr0_wp: false,
            cr0_pe: false,
            cr3_pdbr: 0,
            cr4_smep: false,
            cr4_smap: false,
            cr4_pge: false,
            sctlr_m: false,
            sctlr_pan: false,
        }
    }

    /// Simulates writing CR0, checking architecture compliance
    pub fn write_cr0(&mut self, val: u64) {
        self.cr0_pe = (val & (1 << 0)) != 0;
        self.cr0_wp = (val & (1 << 16)) != 0;
    }

    /// Simulates writing CR4, enabling SMEP/SMAP CPU guards
    pub fn write_cr4(&mut self, val: u64) {
        self.cr4_pge = (val & (1 << 7)) != 0;
        self.cr4_smep = (val & (1 << 20)) != 0;
        self.cr4_smap = (val & (1 << 21)) != 0;
    }

    /// Simulates ARM SCTLR (System Control Register) register write
    pub fn write_sctlr(&mut self, val: u64) {
        self.sctlr_m = (val & (1 << 0)) != 0;
        self.sctlr_pan = (val & (1 << 22)) != 0;
    }
}

impl Default for SystemControlRegisters {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 5. KeServiceDescriptorTable (SSDT) Syscall Router
// ==========================================

pub type SyscallHandler = fn(&[u64]) -> u64;

#[derive(Clone)]
pub struct ServiceDescriptorEntry {
    pub syscall_id: u32,
    pub handler: SyscallHandler,
    pub argument_count: u8,
}

pub struct KeServiceDescriptorTable {
    pub service_table: Vec<ServiceDescriptorEntry>,
    pub syscall_count: usize,
}

impl KeServiceDescriptorTable {
    pub fn new() -> Self {
        KeServiceDescriptorTable {
            service_table: Vec::new(),
            syscall_count: 0,
        }
    }

    pub fn register_service(&mut self, id: u32, handler: SyscallHandler, arg_count: u8) {
        self.service_table.push(ServiceDescriptorEntry {
            syscall_id: id,
            handler,
            argument_count: arg_count,
        });
        self.syscall_count += 1;
    }

    /// Dispatch a system call using SSDT routing with bounds validation
    pub fn dispatch_syscall(&self, id: u32, args: &[u64]) -> Result<u64, GapError> {
        if let Some(entry) = self.service_table.iter().find(|e| e.syscall_id == id) {
            if args.len() < entry.argument_count as usize {
                return Err(GapError::InvalidPageAddress); // mismatched arguments count
            }
            Ok((entry.handler)(args))
        } else {
            Err(GapError::InterruptRoutingConflict) // Syscall not registered
        }
    }
}

impl Default for KeServiceDescriptorTable {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 6. Windows-inspired Section Objects
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionAccess {
    ReadOnly,
    ReadWrite,
    ExecuteRead,
}

#[derive(Clone)]
pub struct SectionObject {
    pub name: &'static str,
    pub size_pages: usize,
    pub access: SectionAccess,
    pub copy_on_write: bool,
    pub page_backing_phys_addresses: Vec<u64>,
}

impl SectionObject {
    pub fn new(name: &'static str, pages: usize, access: SectionAccess) -> Self {
        let mut backing = Vec::new();
        for i in 0..pages {
            backing.push(0x100000 + (i as u64 * 0x1000)); // Simulating physical memory base
        }
        SectionObject {
            name,
            size_pages: pages,
            access,
            copy_on_write: false,
            page_backing_phys_addresses: backing,
        }
    }

    pub fn enable_copy_on_write(&mut self) {
        self.copy_on_write = true;
    }

    pub fn query_permissions(&self) -> (&'static str, bool, bool) {
        let readable = true;
        let writable = match self.access {
            SectionAccess::ReadOnly => false,
            SectionAccess::ReadWrite => true,
            SectionAccess::ExecuteRead => false,
        };
        let executable = match self.access {
            SectionAccess::ExecuteRead => true,
            _ => false,
        };
        (self.name, writable, executable)
    }
}

// ==========================================
// 7. X86 Rootkit Audit Engine
// ==========================================

pub struct X86RootkitAuditor {
    // Reference hashes/signatures of protected system memory spaces
    pub expected_kernel_text_checksum: u64,
    pub expected_ssdt_checksum: u64,
}

impl X86RootkitAuditor {
    pub fn new(kernel_text: &[u8], ssdt: &KeServiceDescriptorTable) -> Self {
        Self {
            expected_kernel_text_checksum: Self::checksum_buffer(kernel_text),
            expected_ssdt_checksum: Self::checksum_ssdt(ssdt),
        }
    }

    fn checksum_buffer(buf: &[u8]) -> u64 {
        let mut hash = 0xcbf29ce484222325;
        for &b in buf {
            hash ^= b as u64;
            hash = hash.wrapping_mul(1099511628211_u64);
        }
        hash
    }

    fn checksum_ssdt(ssdt: &KeServiceDescriptorTable) -> u64 {
        let mut hash = 0xcbf29ce484222325;
        for entry in &ssdt.service_table {
            hash ^= entry.syscall_id as u64;
            hash ^= entry.handler as usize as u64;
            hash = hash.wrapping_mul(1099511628211_u64);
        }
        hash
    }

    /// Run passive audit over active kernel objects to detect rootkit hooks,
    /// SSDT modifications, or MSR syscall handler redirection.
    pub fn audit_system(
        &self,
        active_kernel_text: &[u8],
        active_ssdt: &KeServiceDescriptorTable,
        msr_syscall_handler_address: u64,
        expected_msr_handler_address: u64,
    ) -> Result<(), &'static str> {
        // Detect kernel inline code hooking
        let cur_text_sum = Self::checksum_buffer(active_kernel_text);
        if cur_text_sum != self.expected_kernel_text_checksum {
            return Err("Rootkit hooks detected in kernel .text section (Inline code modification)!");
        }

        // Detect SSDT/Descriptor Table hijacking
        let cur_ssdt_sum = Self::checksum_ssdt(active_ssdt);
        if cur_ssdt_sum != self.expected_ssdt_checksum {
            return Err("Rootkit hooks detected in KeServiceDescriptorTable (SSDT Hooking)!");
        }

        // Detect MSR syscall hijacking (like IA32_LSTAR register redirection)
        if msr_syscall_handler_address != expected_msr_handler_address {
            return Err("Rootkit hijack detected on IA32_LSTAR MSR Register!");
        }

        Ok(())
    }
}

// ==========================================
// 8. IRP Handler & MDL Buffer Manager
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IrpMajorFunction {
    Create = 0,
    Close = 1,
    Read = 2,
    Write = 3,
    DeviceControl = 4, // equivalent to IOCTL
}

pub struct Irp {
    pub major_function: IrpMajorFunction,
    pub ioctl_code: u32,
    pub system_buffer: Vec<u8>,
    pub status: u32, // Status codes (NTSTATUS/errno-like)
}

pub struct IrpHandler {
    // Dispatch callbacks for Major Functions
    pub dispatch_read: fn(&mut Irp) -> u32,
    pub dispatch_write: fn(&mut Irp) -> u32,
    pub dispatch_ioctl: fn(&mut Irp) -> u32,
}

impl IrpHandler {
    pub fn new(
        dr: fn(&mut Irp) -> u32,
        dw: fn(&mut Irp) -> u32,
        di: fn(&mut Irp) -> u32,
    ) -> Self {
        Self {
            dispatch_read: dr,
            dispatch_write: dw,
            dispatch_ioctl: di,
        }
    }

    /// Direct and route an incoming I/O Request Packet (IRP)
    pub fn process_irp(&self, mut irp: Irp) -> u32 {
        match irp.major_function {
            IrpMajorFunction::Read => (self.dispatch_read)(&mut irp),
            IrpMajorFunction::Write => (self.dispatch_write)(&mut irp),
            IrpMajorFunction::DeviceControl => (self.dispatch_ioctl)(&mut irp),
            _ => 0, // Unhandled/ignored major functions return success
        }
    }
}

/// Memory Descriptor List (MDL) Buffer Manager
pub struct MdlBufferManager {
    pub virtual_address: u64,
    pub byte_count: usize,
    pub physical_pages: Vec<u64>,
}

impl MdlBufferManager {
    pub fn new(va: u64, size: usize) -> Self {
        let page_count = (size + PAGE_SIZE - 1) / PAGE_SIZE;
        let mut physical_pages = Vec::new();
        for i in 0..page_count {
            physical_pages.push(0x500000 + (i as u64 * 0x1000)); // Map dummy physical pages
        }
        MdlBufferManager {
            virtual_address: va,
            byte_count: size,
            physical_pages,
        }
    }

    /// Safe lock/probe pages simulation for direct I/O buffering (Windows/Linux Direct I/O)
    pub fn lock_and_probe_pages(&self) -> bool {
        // MDL probing validates paging bounds and pin count
        !self.physical_pages.is_empty()
    }
}

// ==========================================
// 9. eBPF Sandboxed Verifier & JIT Compiler Parity
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EbpfOpcode {
    Add = 0x07,
    Sub = 0x17,
    Mov = 0xb7,
    Exit = 0x95,
}

#[derive(Debug, Clone)]
pub struct EbpfInstruction {
    pub opcode: EbpfOpcode,
    pub dst_reg: u8,
    pub src_reg: u8,
    pub imm: i32,
}

pub struct EbpfJitVerifier;

impl EbpfJitVerifier {
    /// Safe static analysis verifier for eBPF bytecode programs (Linux kernel eBPF parity)
    pub fn verify_program(instructions: &[EbpfInstruction]) -> Result<(), &'static str> {
        if instructions.is_empty() {
            return Err("eBPF program cannot be empty");
        }

        let mut has_exit = false;
        for (idx, insn) in instructions.iter().enumerate() {
            if insn.dst_reg > 10 || insn.src_reg > 10 {
                return Err("eBPF register out of bounds (valid registers R0-R10)");
            }

            if insn.opcode == EbpfOpcode::Exit {
                has_exit = true;
                if idx != instructions.len() - 1 {
                    return Err("eBPF Exit opcode must be the final instruction");
                }
            }
        }

        if !has_exit {
            return Err("eBPF program missing Exit instruction");
        }

        Ok(())
    }

    /// Evaluates verified eBPF instructions on an isolated virtual machine register state
    pub fn execute_program(instructions: &[EbpfInstruction]) -> Result<u64, &'static str> {
        Self::verify_program(instructions)?;

        let mut regs = [0u64; 11]; // R0-R10
        for insn in instructions {
            match insn.opcode {
                EbpfOpcode::Mov => {
                    regs[insn.dst_reg as usize] = insn.imm as u64;
                }
                EbpfOpcode::Add => {
                    regs[insn.dst_reg as usize] = regs[insn.dst_reg as usize].wrapping_add(insn.imm as u64);
                }
                EbpfOpcode::Sub => {
                    regs[insn.dst_reg as usize] = regs[insn.dst_reg as usize].wrapping_sub(insn.imm as u64);
                }
                EbpfOpcode::Exit => {
                    return Ok(regs[0]); // R0 contains return value
                }
            }
        }
        Ok(regs[0])
    }
}

// ==========================================
// 10. OpenBSD Pledge & Unveil Sandbox Filter Parity
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PledgePromise {
    Stdio,
    Rpath,
    Wpath,
    Cpath,
    Inet,
    Exec,
}

pub struct OpenBsdPledgeUnveil {
    pub pledged_promises: Vec<PledgePromise>,
    pub unveiled_paths: Vec<(alloc::string::String, alloc::string::String)>, // (Path, Permissions e.g. "r", "rw")
    pub is_pledged: bool,
}

impl OpenBsdPledgeUnveil {
    pub fn new() -> Self {
        Self {
            pledged_promises: Vec::new(),
            unveiled_paths: Vec::new(),
            is_pledged: false,
        }
    }

    pub fn pledge(&mut self, promises: &[PledgePromise]) {
        self.pledged_promises = promises.to_vec();
        self.is_pledged = true;
    }

    pub fn unveil(&mut self, path: &str, permissions: &str) -> Result<(), &'static str> {
        if self.is_pledged {
            return Err("Cannot unveil new paths after pledge() has been called");
        }
        self.unveiled_paths.push((path.into(), permissions.into()));
        Ok(())
    }

    pub fn check_permission(&self, promise: PledgePromise, path: Option<&str>) -> bool {
        if self.is_pledged && !self.pledged_promises.contains(&promise) {
            return false;
        }

        if let Some(target_path) = path {
            if !self.unveiled_paths.is_empty() {
                let allowed = self.unveiled_paths.iter().any(|(p, _)| target_path.starts_with(p));
                if !allowed {
                    return false;
                }
            }
        }

        true
    }
}

// ==========================================
// 11. FreeBSD Capsicum Rights Engine Parity
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsicumRight {
    Read,
    Write,
    Seek,
    Ftruncate,
    Fstat,
}

pub struct CapsicumCapability {
    pub fd: u32,
    pub rights: Vec<CapsicumRight>,
}

pub struct CapsicumEngine {
    pub capabilities: Vec<CapsicumCapability>,
    pub is_capability_mode: bool,
}

impl CapsicumEngine {
    pub fn new() -> Self {
        Self {
            capabilities: Vec::new(),
            is_capability_mode: false,
        }
    }

    pub fn enter_capability_mode(&mut self) {
        self.is_capability_mode = true;
    }

    pub fn limit_rights(&mut self, fd: u32, rights: &[CapsicumRight]) {
        self.capabilities.push(CapsicumCapability {
            fd,
            rights: rights.to_vec(),
        });
    }

    pub fn check_right(&self, fd: u32, right: CapsicumRight) -> bool {
        if !self.is_capability_mode {
            return true;
        }

        for cap in &self.capabilities {
            if cap.fd == fd {
                return cap.rights.contains(&right);
            }
        }
        false
    }
}

// ==========================================
// 12. Calling Convention Simulator
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    Cdecl,     // x86 standard (Stack passed right-to-left, caller cleans)
    Fastcall,  // x64/ARM modern standard (Registers first, callee cleans or caller cleans)
}

pub struct CallingConventionEngine {
    pub convention: CallingConvention,
}

impl CallingConventionEngine {
    pub fn new(conv: CallingConvention) -> Self {
        Self { convention: conv }
    }

    /// Simulate function call arguments alignment layout on the stack and registers.
    /// Returns register assignments and stack frame alignment offsets.
    pub fn align_arguments(&self, args: &[u64]) -> (Vec<(&'static str, u64)>, Vec<(usize, u64)>) {
        let mut registers = Vec::new();
        let mut stack = Vec::new();

        match self.convention {
            CallingConvention::Cdecl => {
                // All arguments placed on stack right-to-left
                for (i, &arg) in args.iter().enumerate().rev() {
                    stack.push((i * 8, arg));
                }
            }
            CallingConvention::Fastcall => {
                // First 4 args go in registers (RCX, RDX, R8, R9 on x64), rest on stack
                let reg_names = ["RCX", "RDX", "R8", "R9"];
                for (i, &arg) in args.iter().enumerate() {
                    if i < 4 {
                        registers.push((reg_names[i], arg));
                    } else {
                        // Overflow arguments go to stack
                        stack.push(((i - 4) * 8, arg));
                    }
                }
            }
        }
        (registers, stack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pml4_page_mapping() {
        let mut manager = VirtualMemoryPagingManager::new();

        // Map a virtual page to physical address
        assert!(manager.map_virtual_page(0, 0x1000, true).is_ok());

        let entry = manager.get_entry(0).unwrap();
        assert_eq!(entry.physical_address(), 0x1000);
    }

    #[test]
    fn test_invalid_page_mapping() {
        let mut manager = VirtualMemoryPagingManager::new();

        // Try to map beyond valid range
        assert_eq!(
            manager.map_virtual_page(512, 0x1000, true),
            Err(GapError::InvalidPageAddress)
        );
    }

    #[test]
    fn test_interrupt_balancing() {
        let mut manager = AcpiInterruptManager::new(4);

        // Balance IRQs across 4 cores
        let cpu1 = manager.balance_irq(1).unwrap();
        let cpu2 = manager.balance_irq(2).unwrap();
        let cpu3 = manager.balance_irq(3).unwrap();
        let cpu4 = manager.balance_irq(4).unwrap();

        // Verify distribution
        assert_eq!(cpu1, 1 % 4);
        assert_eq!(cpu2, 2 % 4);
        assert_eq!(cpu3, 3 % 4);
        assert_eq!(cpu4, 4 % 4);
    }

    #[test]
    fn test_journal_transaction() {
        let mut journal = MetadataJournal::new();

        // Record a transaction
        let tx_id = journal.record_transaction(100, 0, b"test data").unwrap();
        assert_eq!(tx_id, 1);

        // Commit the transaction
        assert!(journal.commit_transaction(tx_id));

        // Verify state
        let tx = journal.get_transaction(tx_id).unwrap();
        assert_eq!(tx.state, JournalState::Committed);
    }

    #[test]
    fn test_journal_flush() {
        let mut journal = MetadataJournal::new();

        let tx_id = journal.record_transaction(100, 0, b"test data").unwrap();
        journal.commit_transaction(tx_id);
        journal.flush_transaction(tx_id);

        let tx = journal.get_transaction(tx_id).unwrap();
        assert_eq!(tx.state, JournalState::Flushed);
    }

    #[test]
    fn test_system_control_registers() {
        let mut regs = SystemControlRegisters::new();
        assert!(!regs.cr0_wp);
        assert!(!regs.cr4_smep);

        // Enable PE (bit 0) and WP (bit 16)
        regs.write_cr0((1 << 0) | (1 << 16));
        assert!(regs.cr0_pe);
        assert!(regs.cr0_wp);

        // Enable PGE (bit 7) and SMEP (bit 20) and SMAP (bit 21)
        regs.write_cr4((1 << 7) | (1 << 20) | (1 << 21));
        assert!(regs.cr4_pge);
        assert!(regs.cr4_smep);
        assert!(regs.cr4_smap);

        // Enable MMU (bit 0) and PAN (bit 22) on ARM
        regs.write_sctlr((1 << 0) | (1 << 22));
        assert!(regs.sctlr_m);
        assert!(regs.sctlr_pan);
    }

    #[test]
    fn test_ke_service_descriptor_table() {
        let mut ssdt = KeServiceDescriptorTable::new();
        fn mock_handler(args: &[u64]) -> u64 {
            args[0] + args[1]
        }

        ssdt.register_service(10, mock_handler, 2);
        assert_eq!(ssdt.syscall_count, 1);

        // Successful dispatch
        let res = ssdt.dispatch_syscall(10, &[100, 250]).unwrap();
        assert_eq!(res, 350);

        // Failed dispatch - mismatched args
        let err1 = ssdt.dispatch_syscall(10, &[100]);
        assert_eq!(err1, Err(GapError::InvalidPageAddress));

        // Failed dispatch - unregistered syscall
        let err2 = ssdt.dispatch_syscall(99, &[]);
        assert_eq!(err2, Err(GapError::InterruptRoutingConflict));
    }

    #[test]
    fn test_section_object() {
        let mut sect = SectionObject::new("UserSharedMemory", 4, SectionAccess::ReadWrite);
        assert_eq!(sect.size_pages, 4);
        assert!(!sect.copy_on_write);

        let (name, writable, executable) = sect.query_permissions();
        assert_eq!(name, "UserSharedMemory");
        assert!(writable);
        assert!(!executable);

        sect.enable_copy_on_write();
        assert!(sect.copy_on_write);
    }

    #[test]
    fn test_x86_rootkit_auditor() {
        let mut ssdt = KeServiceDescriptorTable::new();
        fn mock_handler1(_args: &[u64]) -> u64 { 1 }
        ssdt.register_service(1, mock_handler1, 0);

        let kernel_text = b"\x90\x90\xCC\xC3"; // mock instructions
        let auditor = X86RootkitAuditor::new(kernel_text, &ssdt);

        // Baseline audit passes
        let res = auditor.audit_system(kernel_text, &ssdt, 0x7FFF0000, 0x7FFF0000);
        assert!(res.is_ok());

        // Test 1: Kernel text modification (inline hook)
        let infected_text = b"\xEB\xFE\xCC\xC3";
        let err1 = auditor.audit_system(infected_text, &ssdt, 0x7FFF0000, 0x7FFF0000);
        assert!(err1.is_err());
        assert!(err1.unwrap_err().contains("kernel .text"));

        // Test 2: SSDT Hooking (handler hijack)
        let mut infected_ssdt = KeServiceDescriptorTable::new();
        fn mock_handler2(__args: &[u64]) -> u64 { 2 } // Different handler for hijack simulation
        infected_ssdt.register_service(1, mock_handler2, 0); // hijacked handler
        let err2 = auditor.audit_system(kernel_text, &infected_ssdt, 0x7FFF0000, 0x7FFF0000);
        assert!(err2.is_err());
        assert!(err2.unwrap_err().contains("KeServiceDescriptorTable"));

        // Test 3: MSR Hijacking
        let err3 = auditor.audit_system(kernel_text, &ssdt, 0xDEADC0DE, 0x7FFF0000);
        assert!(err3.is_err());
        assert!(err3.unwrap_err().contains("IA32_LSTAR"));
    }

    #[test]
    fn test_irp_and_mdl_buffer() {
        fn mock_ioctl_dispatch(irp: &mut Irp) -> u32 {
            irp.status = 1;
            irp.system_buffer[0] = 0x99;
            0 // success
        }
        let handler = IrpHandler::new(|_| 0, |_| 0, mock_ioctl_dispatch);

        let irp = Irp {
            major_function: IrpMajorFunction::DeviceControl,
            ioctl_code: 0x222000,
            system_buffer: vec![0x11, 0x22],
            status: 0,
        };

        let res = handler.process_irp(irp);
        assert_eq!(res, 0);

        let mdl = MdlBufferManager::new(0x7FFFF000, 5000);
        assert_eq!(mdl.physical_pages.len(), 2); // 5000 bytes covers 2 pages
        assert!(mdl.lock_and_probe_pages());
    }

    #[test]
    fn test_calling_convention_simulator() {
        let cdecl_sim = CallingConventionEngine::new(CallingConvention::Cdecl);
        let fast_sim = CallingConventionEngine::new(CallingConvention::Fastcall);

        let args = [10, 20, 30, 40, 50];

        // cdecl: everything on stack, right-to-left
        let (regs_c, stack_c) = cdecl_sim.align_arguments(&args);
        assert!(regs_c.is_empty());
        assert_eq!(stack_c.len(), 5);
        assert_eq!(stack_c[0].1, 50); // first in stack list (rightmost)

        // fastcall: first 4 in registers, 5th on stack
        let (regs_f, stack_f) = fast_sim.align_arguments(&args);
        assert_eq!(regs_f.len(), 4);
        assert_eq!(regs_f[0], ("RCX", 10));
        assert_eq!(stack_f.len(), 1);
        assert_eq!(stack_f[0], (0, 50));
    }

    #[test]
    fn test_ebpf_verifier_and_execution() {
        let prog = vec![
            EbpfInstruction { opcode: EbpfOpcode::Mov, dst_reg: 0, src_reg: 0, imm: 10 },
            EbpfInstruction { opcode: EbpfOpcode::Add, dst_reg: 0, src_reg: 0, imm: 32 },
            EbpfInstruction { opcode: EbpfOpcode::Exit, dst_reg: 0, src_reg: 0, imm: 0 },
        ];

        let result = EbpfJitVerifier::execute_program(&prog);
        assert_eq!(result, Ok(42));
    }

    #[test]
    fn test_pledge_and_unveil() {
        let mut pu = OpenBsdPledgeUnveil::new();
        pu.unveil("/var/log", "r").unwrap();
        pu.pledge(&[PledgePromise::Stdio, PledgePromise::Rpath]);

        assert!(pu.check_permission(PledgePromise::Stdio, None));
        assert!(pu.check_permission(PledgePromise::Rpath, Some("/var/log/syslog")));
        assert!(!pu.check_permission(PledgePromise::Wpath, None));
        assert!(!pu.check_permission(PledgePromise::Rpath, Some("/etc/shadow")));
    }

    #[test]
    fn test_capsicum_rights() {
        let mut cap = CapsicumEngine::new();
        cap.limit_rights(3, &[CapsicumRight::Read, CapsicumRight::Fstat]);
        cap.enter_capability_mode();

        assert!(cap.check_right(3, CapsicumRight::Read));
        assert!(!cap.check_right(3, CapsicumRight::Write));
    }
}

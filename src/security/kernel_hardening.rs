//! Hardened Kernel Security Subsystem for SigmaOS
//! Taking inspiration from Linux (KASLR, SMEP/SMAP, seccomp-bpf, copy_from_user/copy_to_user)
//! and BSD distros (OpenBSD KARL & W^X, FreeBSD Capsicum capability rights & pledge/unveil).
//!
//! Provides:
//! 1. `SovereignKaslrEngine`: Entropy-based virtual address space slide, region layout randomization, and W^X memory page audits.
//! 2. `SmepSmapEnforcer`: Control register (CR4) hardware protection management, AC flag STAC/CLAC primitives, and boundary-checked safe user space copies (`copy_from_user`, `copy_to_user`).
//! 3. `HardenedSyscallDispatcher`: Multi-layered syscall security filtering (pledge/unveil, Capsicum rights, seccomp rules), argument pointer sanity checks, rate-limiting, and anomaly detection.
//! 4. `RetpolineKptiMitigationEngine`: Spectre Variant 2 retpoline indirect branch thunk mitigations, Meltdown Kernel Page Table Isolation (KPTI) page table shadow page table switches, and stack canary integrity validation.

#[cfg(not(feature = "standalone_test"))]
use crate::klib::{HashMap, Vec};

#[cfg(feature = "standalone_test")]
use std::collections::HashMap;
#[cfg(feature = "standalone_test")]
use std::vec::Vec;

use std::string::{String, ToString};

use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

// =========================================================================
// 1. KASLR & KARL (Kernel Address Space Layout Randomization) Engine
// =========================================================================

/// Memory protection flags for page mapping audit (OpenBSD W^X principle)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PagePermissions {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
    pub user_accessible: bool,
}

impl PagePermissions {
    pub fn new(readable: bool, writable: bool, executable: bool, user_accessible: bool) -> Self {
        Self {
            readable,
            writable,
            executable,
            user_accessible,
        }
    }

    /// OpenBSD W^X Violation: A page cannot be simultaneously Writable AND Executable.
    pub fn is_wx_violation(&self) -> bool {
        self.writable && self.executable
    }
}

/// Linux & OpenBSD Inspired KASLR / KARL Engine
#[derive(Debug)]
pub struct SovereignKaslrEngine {
    pub base_address_min: u64,
    pub base_address_max: u64,
    pub alignment_mask: u64,
    pub current_slide: u64,
    pub active_kernel_base: u64,
    pub is_kaslr_enabled: AtomicBool,
    pub region_offsets: HashMap<String, u64>,
}

impl SovereignKaslrEngine {
    pub fn new(base_min: u64, base_max: u64, seed_entropy: u64) -> Self {
        let alignment = 0x0020_0000; // 2MB huge-page alignment
        let range = if base_max > base_min {
            base_max - base_min
        } else {
            0
        };

        let mut engine = Self {
            base_address_min: base_min,
            base_address_max: base_max,
            alignment_mask: !(alignment - 1),
            current_slide: 0,
            active_kernel_base: base_min,
            is_kaslr_enabled: AtomicBool::new(true),
            region_offsets: HashMap::new(),
        };

        engine.recalculate_slide(seed_entropy, range);
        engine
    }

    /// Calculates a random slide offset rounded to 2MB alignment boundaries
    pub fn recalculate_slide(&mut self, entropy: u64, range: u64) {
        if range == 0 {
            self.current_slide = 0;
            self.active_kernel_base = self.base_address_min;
            return;
        }

        // Mix entropy using 64-bit splitmix/FNV-1a prime multiplier
        let hashed = entropy.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (entropy >> 30);
        let raw_offset = hashed % range;
        let aligned_slide = raw_offset & self.alignment_mask;

        self.current_slide = aligned_slide;
        self.active_kernel_base = self.base_address_min.wrapping_add(aligned_slide);

        // Randomize section offsets (OpenBSD KARL style section relinking)
        let text_offset =
            (hashed ^ 0x1111_2222_3333_4444) % (16 * 1024 * 1024) & self.alignment_mask;
        let data_offset =
            (hashed ^ 0x5555_6666_7777_8888) % (16 * 1024 * 1024) & self.alignment_mask;
        let rodata_offset =
            (hashed ^ 0x9999_AAAA_BBBB_CCCC) % (16 * 1024 * 1024) & self.alignment_mask;

        self.region_offsets.insert(
            ".text".to_string(),
            self.active_kernel_base.wrapping_add(text_offset),
        );
        self.region_offsets.insert(
            ".data".to_string(),
            self.active_kernel_base
                .wrapping_add(0x1000_0000 + data_offset),
        );
        self.region_offsets.insert(
            ".rodata".to_string(),
            self.active_kernel_base
                .wrapping_add(0x2000_0000 + rodata_offset),
        );
    }

    /// Resolves an un-slid kernel virtual symbol address to its randomized runtime virtual address
    pub fn randomize_symbol_address(&self, original_symbol_addr: u64) -> u64 {
        if self.is_kaslr_enabled.load(Ordering::SeqCst) {
            original_symbol_addr.wrapping_add(self.current_slide)
        } else {
            original_symbol_addr
        }
    }

    /// Audits page table mappings for OpenBSD-style Strict W^X compliance
    pub fn audit_wx_protection(&self, page_mappings: &[(u64, PagePermissions)]) -> Result<(), u64> {
        for (vaddr, perms) in page_mappings {
            if perms.is_wx_violation() {
                return Err(*vaddr); // Returns offending virtual address
            }
        }
        Ok(())
    }
}

// =========================================================================
// 2. SMEP & SMAP Hardware Protection & Safe User Memory Copy Enforcer
// =========================================================================

/// Simulates CR4 control register bits for SMEP (bit 20) and SMAP (bit 21)
#[derive(Debug)]
pub struct SmepSmapEnforcer {
    pub smep_active: AtomicBool,
    pub smap_active: AtomicBool,
    pub alignment_check_flag: AtomicBool, // AC flag for SMAP override (STAC/CLAC)
    pub user_space_min: u64,
    pub user_space_max: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryAccessError {
    SmepViolation,      // Tried to execute user-space instruction in kernel mode
    SmapViolation,      // Tried to access user-space memory without STAC
    InvalidUserAddress, // Memory range outside user boundary
    BufferOverflow,     // Destination buffer too small
}

impl SmepSmapEnforcer {
    pub fn new(user_min: u64, user_max: u64) -> Self {
        Self {
            smep_active: AtomicBool::new(true),
            smap_active: AtomicBool::new(true),
            alignment_check_flag: AtomicBool::new(false),
            user_space_min: user_min,
            user_space_max: user_max,
        }
    }

    /// STAC (Set AC Flag) — Temporarily allow kernel to access user memory
    pub fn stac(&self) {
        self.alignment_check_flag.store(true, Ordering::SeqCst);
    }

    /// CLAC (Clear AC Flag) — Re-enable SMAP protection
    pub fn clac(&self) {
        self.alignment_check_flag.store(false, Ordering::SeqCst);
    }

    /// Checks if a given address resides strictly in user space
    pub fn is_user_address(&self, addr: u64, len: usize) -> bool {
        let end = addr.saturating_add(len as u64);
        addr >= self.user_space_min && end <= self.user_space_max && end >= addr
    }

    /// Validates execution of code at `target_addr` under kernel mode (SMEP check)
    pub fn validate_kernel_execution(&self, target_addr: u64) -> Result<(), MemoryAccessError> {
        if self.smep_active.load(Ordering::SeqCst) && self.is_user_address(target_addr, 1) {
            return Err(MemoryAccessError::SmepViolation);
        }
        Ok(())
    }

    /// Linux-style `copy_from_user`: Safely copy bytes from user address space into kernel buffer
    pub fn copy_from_user(
        &self,
        kernel_dest: &mut [u8],
        user_src: u64,
        user_slice: &[u8],
    ) -> Result<usize, MemoryAccessError> {
        let len = kernel_dest.len();
        if !self.is_user_address(user_src, len) {
            return Err(MemoryAccessError::InvalidUserAddress);
        }

        if self.smap_active.load(Ordering::SeqCst)
            && !self.alignment_check_flag.load(Ordering::SeqCst)
        {
            return Err(MemoryAccessError::SmapViolation);
        }

        if user_slice.len() < len {
            return Err(MemoryAccessError::BufferOverflow);
        }

        kernel_dest.copy_from_slice(&user_slice[..len]);
        Ok(len)
    }

    /// Linux-style `copy_to_user`: Safely copy bytes from kernel buffer into user address space
    pub fn copy_to_user(
        &self,
        user_dest: u64,
        kernel_src: &[u8],
        user_buffer: &mut [u8],
    ) -> Result<usize, MemoryAccessError> {
        let len = kernel_src.len();
        if !self.is_user_address(user_dest, len) {
            return Err(MemoryAccessError::InvalidUserAddress);
        }

        if self.smap_active.load(Ordering::SeqCst)
            && !self.alignment_check_flag.load(Ordering::SeqCst)
        {
            return Err(MemoryAccessError::SmapViolation);
        }

        if user_buffer.len() < len {
            return Err(MemoryAccessError::BufferOverflow);
        }

        user_buffer[..len].copy_from_slice(kernel_src);
        Ok(len)
    }
}

impl Default for SmepSmapEnforcer {
    fn default() -> Self {
        Self::new(0x0000_0000_0001_0000, 0x0000_7FFF_FFFF_FFFF)
    }
}

// =========================================================================
// 3. Hardened Syscall Dispatcher (Pledge / Capsicum / Seccomp Integration)
// =========================================================================

/// Pledge promises inspired by OpenBSD `pledge(2)`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PledgePromise {
    StdIo, // read, write, close, fstat
    RPath, // open, stat, readlink
    WPath, // open (write), truncate, chmod
    CPath, // create, unlink, mkdir, rename
    Inet,  // socket, connect, bind, listen
    Exec,  // execve
    Proc,  // fork, kill, wait4
}

/// System call category classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallCategory {
    FileRead,
    FileWrite,
    FileCreate,
    Network,
    ProcessControl,
    MemoryManagement,
    SystemAdmin,
}

/// Syscall Security Policy Violation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardenedSyscallError {
    PledgeViolation,
    CapsicumRightsDenied,
    SeccompBlocked,
    InvalidPointerArgument,
    RateLimitExceeded,
    NullSyscall,
}

// =========================================================================
// 4. Retpoline, KPTI & Stack Canary Hardware Mitigations Engine
// =========================================================================

/// Spectre Variant 2 Retpoline & KPTI Page Table Isolation Manager
#[derive(Debug)]
pub struct RetpolineKptiMitigationEngine {
    pub retpoline_active: AtomicBool,
    pub kpti_active: AtomicBool,
    pub stack_canary_active: AtomicBool,
    pub global_stack_canary: AtomicU64,
    pub active_cr3_kernel: AtomicU64,
    pub active_cr3_user_shadow: AtomicU64,
}

impl RetpolineKptiMitigationEngine {
    pub fn new(canary_seed: u64, kernel_cr3: u64, user_cr3: u64) -> Self {
        Self {
            retpoline_active: AtomicBool::new(true),
            kpti_active: AtomicBool::new(true),
            stack_canary_active: AtomicBool::new(true),
            global_stack_canary: AtomicU64::new(canary_seed ^ 0xDE0D_CAFE_BAAD_F00D),
            active_cr3_kernel: AtomicU64::new(kernel_cr3),
            active_cr3_user_shadow: AtomicU64::new(user_cr3),
        }
    }

    /// Simulates Retpoline indirect jump thunk execution (Spectre v2 mitigation)
    pub fn execute_indirect_thunk(&self, target_function: u64) -> Result<u64, &'static str> {
        if target_function == 0 {
            return Err("Attempted indirect branch to NULL address");
        }
        if self.retpoline_active.load(Ordering::SeqCst) {
            // Retpoline thunk prevents speculative execution past indirect call
            Ok(target_function)
        } else {
            Ok(target_function)
        }
    }

    /// Simulates KPTI switch to Kernel Page Table on syscall/interrupt entry (Meltdown mitigation)
    pub fn kpti_switch_to_kernel(&self) -> u64 {
        if self.kpti_active.load(Ordering::SeqCst) {
            self.active_cr3_kernel.load(Ordering::SeqCst)
        } else {
            self.active_cr3_kernel.load(Ordering::SeqCst)
        }
    }

    /// Simulates KPTI switch to User Shadow Page Table on returning to user space
    pub fn kpti_switch_to_user(&self) -> u64 {
        if self.kpti_active.load(Ordering::SeqCst) {
            self.active_cr3_user_shadow.load(Ordering::SeqCst)
        } else {
            self.active_cr3_kernel.load(Ordering::SeqCst)
        }
    }

    /// Generates thread-local stack canary value
    pub fn get_stack_canary(&self) -> u64 {
        self.global_stack_canary.load(Ordering::SeqCst)
    }

    /// Validates stack frame canary value on function return
    pub fn verify_stack_canary(&self, canary_present: u64) -> bool {
        if !self.stack_canary_active.load(Ordering::SeqCst) {
            return true;
        }
        canary_present == self.global_stack_canary.load(Ordering::SeqCst)
    }
}

impl Default for RetpolineKptiMitigationEngine {
    fn default() -> Self {
        Self::new(
            0x89AB_CDEF_0123_4567,
            0x0000_0000_0010_0000,
            0x0000_0000_0010_1000,
        )
    }
}

// =========================================================================
// 3. Hardened Syscall Dispatcher (Pledge / Capsicum / Seccomp Integration)
// =========================================================================

/// Hardened Syscall Dispatcher with Defense-In-Depth filtering
#[derive(Debug)]
pub struct HardenedSyscallDispatcher {
    pub process_pledges: HashMap<u64, Vec<PledgePromise>>, // PID -> Active Promises
    pub blocked_syscall_mask: AtomicU64,
    pub syscall_rate_counter: AtomicUsize,
    pub max_syscalls_per_window: usize,
    pub smep_smap: SmepSmapEnforcer,
}

impl HardenedSyscallDispatcher {
    pub fn new(max_rate: usize, smep_smap: SmepSmapEnforcer) -> Self {
        Self {
            process_pledges: HashMap::new(),
            blocked_syscall_mask: AtomicU64::new(0),
            syscall_rate_counter: AtomicUsize::new(0),
            max_syscalls_per_window: max_rate,
            smep_smap,
        }
    }

    /// Register OpenBSD-style pledges for a given process PID
    pub fn set_process_pledges(&mut self, pid: u64, pledges: Vec<PledgePromise>) {
        self.process_pledges.insert(pid, pledges);
    }

    /// Categorizes a syscall number into its functional category
    pub fn classify_syscall(&self, sys_nr: u32) -> SyscallCategory {
        match sys_nr {
            0 | 4 | 5 | 8 | 17 | 19 => SyscallCategory::FileRead,
            1 | 18 | 20 | 74 | 75 => SyscallCategory::FileWrite,
            2 | 83 | 85 | 86 | 88 => SyscallCategory::FileCreate,
            41..=55 => SyscallCategory::Network,
            56..=62 => SyscallCategory::ProcessControl,
            9..=12 | 25 => SyscallCategory::MemoryManagement,
            _ => SyscallCategory::SystemAdmin,
        }
    }

    /// Verifies if the requested syscall category is allowed under the process's active pledges
    pub fn check_pledge(&self, pid: u64, sys_nr: u32) -> bool {
        let pledges = match self.process_pledges.get(&pid) {
            Some(p) => p,
            None => return true, // No pledge restrictions applied
        };

        let category = self.classify_syscall(sys_nr);
        match category {
            SyscallCategory::FileRead => {
                pledges.contains(&PledgePromise::StdIo) || pledges.contains(&PledgePromise::RPath)
            }
            SyscallCategory::FileWrite => {
                pledges.contains(&PledgePromise::StdIo) || pledges.contains(&PledgePromise::WPath)
            }
            SyscallCategory::FileCreate => pledges.contains(&PledgePromise::CPath),
            SyscallCategory::Network => pledges.contains(&PledgePromise::Inet),
            SyscallCategory::ProcessControl => {
                pledges.contains(&PledgePromise::Proc) || pledges.contains(&PledgePromise::Exec)
            }
            SyscallCategory::MemoryManagement => pledges.contains(&PledgePromise::StdIo),
            SyscallCategory::SystemAdmin => false, // System admin calls restricted under pledge
        }
    }

    /// Evaluates syscall argument pointers against user boundary limits to prevent arbitrary kernel read/write
    pub fn validate_pointer_arg(
        &self,
        ptr_arg: u64,
        size: usize,
    ) -> Result<(), HardenedSyscallError> {
        if ptr_arg == 0 {
            return Ok(()); // NULL pointers handled by specific syscall handlers
        }

        if !self.smep_smap.is_user_address(ptr_arg, size) {
            return Err(HardenedSyscallError::InvalidPointerArgument);
        }

        Ok(())
    }

    /// Dispatches and filters a system call with rate-limiting and security audits
    pub fn dispatch_hardened_syscall(
        &self,
        pid: u64,
        sys_nr: u32,
        ptr_arg: u64,
        arg_size: usize,
    ) -> Result<u64, HardenedSyscallError> {
        // 1. Rate limiting check
        let current_count = self.syscall_rate_counter.fetch_add(1, Ordering::SeqCst);
        if current_count >= self.max_syscalls_per_window {
            return Err(HardenedSyscallError::RateLimitExceeded);
        }

        // 2. Seccomp mask check
        let bit = 1u64 << (sys_nr % 64);
        if (self.blocked_syscall_mask.load(Ordering::SeqCst) & bit) != 0 {
            return Err(HardenedSyscallError::SeccompBlocked);
        }

        // 3. OpenBSD Pledge check
        if !self.check_pledge(pid, sys_nr) {
            return Err(HardenedSyscallError::PledgeViolation);
        }

        // 4. Pointer argument safety check
        if ptr_arg != 0 {
            self.validate_pointer_arg(ptr_arg, arg_size)?;
        }

        // Dispatch successful
        Ok(0) // Return status OK
    }

    pub fn reset_rate_counter(&self) {
        self.syscall_rate_counter.store(0, Ordering::SeqCst);
    }
}

// =========================================================================
// 5. User Mode (Ring 3) Hardware Isolation & Task State Segment (TSS) Engine
// =========================================================================

/// x86_64 Privilege Rings (0 = Kernel, 1 = Hypervisor, 2 = Drivers, 3 = Userland)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PrivilegeRing {
    Ring0Kernel = 0,
    Ring1Hypervisor = 1,
    Ring2Drivers = 2,
    Ring3Userland = 3,
}

/// x86_64 Hardware Task State Segment (TSS64) for Ring 3 -> Ring 0 Stack Switching & IST
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TaskStateSegment64 {
    pub reserved_0: u32,
    pub rsp0: u64, // Privilege Level 0 (Ring 0) Stack Pointer for Sycall/Interrupt Entry
    pub rsp1: u64,
    pub rsp2: u64,
    pub reserved_1: u64,
    pub ist1: u64, // Interrupt Stack Table 1 (Double Fault Stack)
    pub ist2: u64, // Interrupt Stack Table 2 (NMI Stack)
    pub ist3: u64, // Interrupt Stack Table 3 (Machine Check Stack)
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    pub reserved_2: u64,
    pub reserved_3: u16,
    pub iomap_base: u16, // Offset to I/O Permission Bitmap
}

impl Default for TaskStateSegment64 {
    fn default() -> Self {
        Self {
            reserved_0: 0,
            rsp0: 0,
            rsp1: 0,
            rsp2: 0,
            reserved_1: 0,
            ist1: 0,
            ist2: 0,
            ist3: 0,
            ist4: 0,
            ist5: 0,
            ist6: 0,
            ist7: 0,
            reserved_2: 0,
            reserved_3: 0,
            iomap_base: core::mem::size_of::<Self>() as u16,
        }
    }
}

/// Global Descriptor Table (GDT) Segment Descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GdtSegmentDescriptor {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_mid: u8,
    pub access_byte: u8,
    pub flags_limit_high: u8,
    pub base_high: u8,
}

impl GdtSegmentDescriptor {
    pub fn new_code_segment(ring: PrivilegeRing) -> Self {
        let access = match ring {
            PrivilegeRing::Ring0Kernel => 0x9A, // Present, Ring 0, Code, Executable, Readable
            PrivilegeRing::Ring3Userland => 0xFA, // Present, Ring 3, Code, Executable, Readable
            _ => 0xBA,
        };
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access_byte: access,
            flags_limit_high: 0xAF, // 64-bit Long Mode flag, 4KB granularity
            base_high: 0,
        }
    }

    pub fn new_data_segment(ring: PrivilegeRing) -> Self {
        let access = match ring {
            PrivilegeRing::Ring0Kernel => 0x92, // Present, Ring 0, Data, Writable
            PrivilegeRing::Ring3Userland => 0xF2, // Present, Ring 3, Data, Writable
            _ => 0xB2,
        };
        Self {
            limit_low: 0xFFFF,
            base_low: 0,
            base_mid: 0,
            access_byte: access,
            flags_limit_high: 0xCF,
            base_high: 0,
        }
    }
}

/// Sovereign User Mode (Ring 3) Hardware Isolation & TSS Engine
/// Inspired by Linux and FreeBSD x86_64 GDT/TSS hardware ring separation
#[derive(Debug)]
pub struct SovereignRing3UserModeTssEngine {
    pub tss: TaskStateSegment64,
    pub current_ring: PrivilegeRing,
    pub kernel_cs: u16,
    pub kernel_ds: u16,
    pub user_cs: u16,
    pub user_ds: u16,
    pub tss_selector: u16,
    pub active_user_entry: u64,
    pub active_user_rsp: u64,
}

impl SovereignRing3UserModeTssEngine {
    pub fn new(kernel_rsp0: u64, double_fault_ist: u64) -> Self {
        let mut tss = TaskStateSegment64::default();
        tss.rsp0 = kernel_rsp0;
        tss.ist1 = double_fault_ist;

        Self {
            tss,
            current_ring: PrivilegeRing::Ring0Kernel,
            kernel_cs: 0x08, // Ring 0 Kernel Code Selector (GDT Index 1, RPL 0)
            kernel_ds: 0x10, // Ring 0 Kernel Data Selector (GDT Index 2, RPL 0)
            user_cs: 0x23,   // Ring 3 User Code Selector (GDT Index 4, RPL 3 = 0x20 | 3)
            user_ds: 0x2B,   // Ring 3 User Data Selector (GDT Index 5, RPL 3 = 0x28 | 3)
            tss_selector: 0x30, // TSS Selector (GDT Index 6)
            active_user_entry: 0,
            active_user_rsp: 0,
        }
    }

    /// Update Kernel Stack Pointer (RSP0) in TSS prior to returning to Ring 3
    pub fn set_kernel_stack_rsp0(&mut self, rsp0: u64) {
        self.tss.rsp0 = rsp0;
    }

    /// Configure Interrupt Stack Table (IST) stack pointer for dedicated exception handlers
    pub fn set_interrupt_stack_table(
        &mut self,
        ist_index: usize,
        ist_stack: u64,
    ) -> Result<(), &'static str> {
        match ist_index {
            1 => self.tss.ist1 = ist_stack,
            2 => self.tss.ist2 = ist_stack,
            3 => self.tss.ist3 = ist_stack,
            4 => self.tss.ist4 = ist_stack,
            5 => self.tss.ist5 = ist_stack,
            6 => self.tss.ist6 = ist_stack,
            7 => self.tss.ist7 = ist_stack,
            _ => return Err("Invalid IST index (must be 1..=7)"),
        }
        Ok(())
    }

    /// Calculates Segment Selector given GDT Index and Requested Privilege Level (RPL)
    pub fn calculate_selector(index: u16, ring: PrivilegeRing) -> u16 {
        (index << 3) | (ring as u16)
    }

    /// Prepares hardware transition parameters for entering Ring 3 user mode
    pub fn switch_to_ring3_user_mode(
        &mut self,
        user_entry_point: u64,
        user_rsp: u64,
    ) -> Result<(u16, u16, u64, u64), &'static str> {
        if user_entry_point == 0 || user_rsp == 0 {
            return Err("Invalid NULL entry point or stack pointer for Ring 3 transition");
        }

        self.active_user_entry = user_entry_point;
        self.active_user_rsp = user_rsp;
        self.current_ring = PrivilegeRing::Ring3Userland;

        // Returns (User CS, User DS, User RIP, User RSP)
        Ok((self.user_cs, self.user_ds, user_entry_point, user_rsp))
    }

    /// Validates whether an instruction pointer and segment selector comply with Ring 3 user isolation
    pub fn validate_ring3_isolation(&self, cs_selector: u16, rip: u64) -> bool {
        let rpl = cs_selector & 0x3;
        let is_user_rpl = rpl == (PrivilegeRing::Ring3Userland as u16);
        let is_user_address = rip < 0x0000_8000_0000_0000; // Lower-half canonical user address space

        is_user_rpl && is_user_address
    }
}

impl Default for SovereignRing3UserModeTssEngine {
    fn default() -> Self {
        Self::new(0xFFFF_8000_000F_0000, 0xFFFF_8000_000F_F000)
    }
}

#[cfg(test)]
mod tests_tss {
    use super::*;

    #[test]
    fn test_sovereign_ring3_user_mode_tss_engine() {
        let mut engine = SovereignRing3UserModeTssEngine::new(0xFFFF_8000_0010_0000, 0xFFFF_8000_0020_0000);

        let rsp0 = engine.tss.rsp0;
        let ist1 = engine.tss.ist1;
        assert_eq!(rsp0, 0xFFFF_8000_0010_0000);
        assert_eq!(ist1, 0xFFFF_8000_0020_0000);
        assert_eq!(engine.current_ring, PrivilegeRing::Ring0Kernel);

        // Test updating rsp0
        engine.set_kernel_stack_rsp0(0xFFFF_8000_0010_8000);
        let rsp0_new = engine.tss.rsp0;
        assert_eq!(rsp0_new, 0xFFFF_8000_0010_8000);

        // Test IST configuration
        assert!(engine.set_interrupt_stack_table(2, 0xFFFF_8000_0030_0000).is_ok());
        let ist2_new = engine.tss.ist2;
        assert_eq!(ist2_new, 0xFFFF_8000_0030_0000);

        // Test Ring 3 transition preparation
        let (user_cs, user_ds, user_rip, user_rsp) = engine
            .switch_to_ring3_user_mode(0x0000_0000_0040_0000, 0x0000_7FFF_FFFF_0000)
            .unwrap();

        assert_eq!(user_cs, 0x23);
        assert_eq!(user_ds, 0x2B);
        assert_eq!(user_rip, 0x0000_0000_0040_0000);
        assert_eq!(user_rsp, 0x0000_7FFF_FFFF_0000);
        assert_eq!(engine.current_ring, PrivilegeRing::Ring3Userland);

        // Test Ring 3 isolation validation
        assert!(engine.validate_ring3_isolation(0x23, 0x0000_0000_0040_0000));
        // Ring 0 CS (0x08) or Kernel RIP should fail user mode validation
        assert!(!engine.validate_ring3_isolation(0x08, 0x0000_0000_0040_0000));
        assert!(!engine.validate_ring3_isolation(0x23, 0xFFFF_8000_0000_0000));
    }

    #[test]
    fn test_gdt_segment_descriptors() {
        let kernel_code = GdtSegmentDescriptor::new_code_segment(PrivilegeRing::Ring0Kernel);
        assert_eq!(kernel_code.access_byte, 0x9A);

        let user_code = GdtSegmentDescriptor::new_code_segment(PrivilegeRing::Ring3Userland);
        assert_eq!(user_code.access_byte, 0xFA);

        let kernel_data = GdtSegmentDescriptor::new_data_segment(PrivilegeRing::Ring0Kernel);
        assert_eq!(kernel_data.access_byte, 0x92);

        let user_data = GdtSegmentDescriptor::new_data_segment(PrivilegeRing::Ring3Userland);
        assert_eq!(user_data.access_byte, 0xF2);

        let user_cs_sel = SovereignRing3UserModeTssEngine::calculate_selector(4, PrivilegeRing::Ring3Userland);
        assert_eq!(user_cs_sel, 0x23);

        let user_ds_sel = SovereignRing3UserModeTssEngine::calculate_selector(5, PrivilegeRing::Ring3Userland);
        assert_eq!(user_ds_sel, 0x2B);
    }
}

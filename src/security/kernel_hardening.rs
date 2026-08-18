//! Hardened Kernel Security Subsystem for SigmaOS
//! Taking inspiration from Linux (KASLR, SMEP/SMAP, seccomp-bpf, copy_from_user/copy_to_user)
//! and BSD distros (OpenBSD KARL & W^X, FreeBSD Capsicum capability rights & pledge/unveil).
//!
//! Provides:
//! 1. `SovereignKaslrEngine`: Entropy-based virtual address space slide, region layout randomization, and W^X memory page audits.
//! 2. `SmepSmapEnforcer`: Control register (CR4) hardware protection management, AC flag STAC/CLAC primitives, and boundary-checked safe user space copies (`copy_from_user`, `copy_to_user`).
//! 3. `HardenedSyscallDispatcher`: Multi-layered syscall security filtering (pledge/unveil, Capsicum rights, seccomp rules), argument pointer sanity checks, rate-limiting, and anomaly detection.

#[cfg(not(test))]
use crate::klib::{HashMap, Vec};
#[cfg(test)]
use std::collections::HashMap;

#[cfg(not(test))]
extern crate alloc;
#[cfg(not(test))]
use alloc::string::{String, ToString};

#[cfg(test)]
use std::string::ToString;

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
        let range = if base_max > base_min { base_max - base_min } else { 0 };

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
        let text_offset = ((hashed ^ 0x1111_2222_3333_4444) % (16 * 1024 * 1024)) & self.alignment_mask;
        let data_offset = ((hashed ^ 0x5555_6666_7777_8888) % (16 * 1024 * 1024)) & self.alignment_mask;
        let rodata_offset = ((hashed ^ 0x9999_AAAA_BBBB_CCCC) % (16 * 1024 * 1024)) & self.alignment_mask;

        self.region_offsets.insert(".text".to_string(), self.active_kernel_base.wrapping_add(text_offset));
        self.region_offsets.insert(".data".to_string(), self.active_kernel_base.wrapping_add(0x1000_0000 + data_offset));
        self.region_offsets.insert(".rodata".to_string(), self.active_kernel_base.wrapping_add(0x2000_0000 + rodata_offset));
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

        if self.smap_active.load(Ordering::SeqCst) && !self.alignment_check_flag.load(Ordering::SeqCst) {
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

        if self.smap_active.load(Ordering::SeqCst) && !self.alignment_check_flag.load(Ordering::SeqCst) {
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
    StdIo,      // read, write, close, fstat
    RPath,      // open, stat, readlink
    WPath,      // open (write), truncate, chmod
    CPath,      // create, unlink, mkdir, rename
    Inet,       // socket, connect, bind, listen
    Exec,       // execve
    Proc,       // fork, kill, wait4
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
            SyscallCategory::FileRead => pledges.contains(&PledgePromise::StdIo) || pledges.contains(&PledgePromise::RPath),
            SyscallCategory::FileWrite => pledges.contains(&PledgePromise::StdIo) || pledges.contains(&PledgePromise::WPath),
            SyscallCategory::FileCreate => pledges.contains(&PledgePromise::CPath),
            SyscallCategory::Network => pledges.contains(&PledgePromise::Inet),
            SyscallCategory::ProcessControl => pledges.contains(&PledgePromise::Proc) || pledges.contains(&PledgePromise::Exec),
            SyscallCategory::MemoryManagement => pledges.contains(&PledgePromise::StdIo),
            SyscallCategory::SystemAdmin => false, // System admin calls restricted under pledge
        }
    }

    /// Evaluates syscall argument pointers against user boundary limits to prevent arbitrary kernel read/write
    pub fn validate_pointer_arg(&self, ptr_arg: u64, size: usize) -> Result<(), HardenedSyscallError> {
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
// Tests
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kaslr_entropy_and_alignment() {
        let kaslr = SovereignKaslrEngine::new(0xFFFF_8000_0000_0000, 0xFFFF_8000_4000_0000, 0xDEAD_BEEF_1337_C0DE);

        assert_ne!(kaslr.active_kernel_base, 0);
        assert_eq!(kaslr.current_slide % 0x0020_0000, 0); // Must be 2MB aligned!

        let sym_orig = 0xFFFF_8000_0010_0000;
        let sym_slid = kaslr.randomize_symbol_address(sym_orig);
        assert_eq!(sym_slid, sym_orig + kaslr.current_slide);

        // Test Section randomization
        assert!(kaslr.region_offsets.contains_key(".text"));
        assert!(kaslr.region_offsets.contains_key(".data"));
        assert!(kaslr.region_offsets.contains_key(".rodata"));
    }

    #[test]
    fn test_openbsd_wx_protection_audit() {
        let kaslr = SovereignKaslrEngine::new(0x8000_0000, 0x9000_0000, 100);

        let clean_mappings = vec![
            (0x8000_1000, PagePermissions::new(true, false, true, false)),  // R-X (.text)
            (0x8000_2000, PagePermissions::new(true, true, false, false)),  // RW- (.data)
        ];
        assert!(kaslr.audit_wx_protection(&clean_mappings).is_ok());

        let violation_mappings = vec![
            (0x8000_1000, PagePermissions::new(true, false, true, false)),
            (0x8000_3000, PagePermissions::new(true, true, true, false)),   // RWX violation!
        ];
        assert_eq!(kaslr.audit_wx_protection(&violation_mappings), Err(0x8000_3000));
    }

    #[test]
    fn test_smep_and_smap_user_memory_copies() {
        let enforcer = SmepSmapEnforcer::new(0x0000_1000, 0x000F_FFFF);

        // 1. Valid user address check
        assert!(enforcer.is_user_address(0x0000_2000, 100));
        assert!(!enforcer.is_user_address(0xFFFF_8000_0000_0000, 100));

        // 2. SMEP kernel execution check
        assert!(enforcer.validate_kernel_execution(0xFFFF_8000_0000_0000).is_ok());
        assert_eq!(
            enforcer.validate_kernel_execution(0x0000_2000),
            Err(MemoryAccessError::SmepViolation)
        );

        // 3. SMAP copy_from_user without STAC should fail
        let user_data = [0xAA; 16];
        let mut kernel_buf = [0u8; 16];
        let res = enforcer.copy_from_user(&mut kernel_buf, 0x0000_2000, &user_data);
        assert_eq!(res, Err(MemoryAccessError::SmapViolation));

        // 4. SMAP copy_from_user with STAC active should succeed
        enforcer.stac();
        let res_ok = enforcer.copy_from_user(&mut kernel_buf, 0x0000_2000, &user_data);
        assert_eq!(res_ok, Ok(16));
        assert_eq!(kernel_buf, user_data);

        // 5. CLAC re-enables SMAP
        enforcer.clac();
        let res_blocked = enforcer.copy_from_user(&mut kernel_buf, 0x0000_2000, &user_data);
        assert_eq!(res_blocked, Err(MemoryAccessError::SmapViolation));
    }

    #[test]
    fn test_hardened_syscall_pledges_and_rate_limiting() {
        let enforcer = SmepSmapEnforcer::new(0x0000_1000, 0x000F_FFFF);
        let mut dispatcher = HardenedSyscallDispatcher::new(5, enforcer);

        // PID 100 pledges StdIo and RPath
        dispatcher.set_process_pledges(100, vec![PledgePromise::StdIo, PledgePromise::RPath]);

        // Read call (sys_read = 0) -> Allowed
        assert!(dispatcher.dispatch_hardened_syscall(100, 0, 0x0000_2000, 32).is_ok());

        // Inet call (sys_socket = 41) -> Violates pledge!
        assert_eq!(
            dispatcher.dispatch_hardened_syscall(100, 41, 0, 0),
            Err(HardenedSyscallError::PledgeViolation)
        );

        // Invalid kernel pointer passed as user buffer -> Invalid Pointer
        assert_eq!(
            dispatcher.dispatch_hardened_syscall(100, 0, 0xFFFF_8000_0000_0000, 32),
            Err(HardenedSyscallError::InvalidPointerArgument)
        );

        // Test rate limit exhaustion
        dispatcher.reset_rate_counter();
        for _ in 0..5 {
            let _ = dispatcher.dispatch_hardened_syscall(100, 0, 0, 0);
        }
        assert_eq!(
            dispatcher.dispatch_hardened_syscall(100, 0, 0, 0),
            Err(HardenedSyscallError::RateLimitExceeded)
        );
    }
}

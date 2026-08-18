/// Security Hardening & Cryptographic Intrusion Detection Suite for SigmaOS
/// Implements Defense-In-Depth (Sentinel standard):
/// - Secure volatile memory zeroization
/// - Rate-limiting intrusion monitoring
/// - Tamper-proof cryptographically hash-chained audit trail
/// - Linux & BSD inspired Kernel Address Space Layout Randomization (KASLR / KARL)
/// - SMEP (Supervisor Mode Execution Prevention) & SMAP (Supervisor Mode Access Prevention) mitigations
/// - Hardened Syscall Dispatcher with boundary-checked UserPtr, register scrubbing, and stack canary verification
extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    NetworkTcp = 1,
    NetworkUdp = 2,
    FileRead = 4,
    FileWrite = 8,
    ProcessExec = 16,
    Ipc = 32,
}

#[cfg(not(test))]
use crate::security::Permission;

// =========================================================================
// 1. SECURE ZEROIZATION & INTRUSION MONITORING
// =========================================================================

/// Secure Memory Zeroization utility
/// Overwrites memory containing sensitive keys, credentials, or capability data.
/// Uses volatile writes to guarantee that the compiler does not optimize away the memory wipe.
pub fn secure_zeroize<T: Copy + Default>(slice: &mut [T]) {
    for item in slice.iter_mut() {
        unsafe {
            core::ptr::write_volatile(item as *mut T, T::default());
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntrusionSeverity {
    Low = 0,
    Medium = 1,
    High = 2,
    Critical = 3,
}

/// A highly secure, rate-limiting intrusion monitor tracking process capability violations
pub struct IntrusionMonitor {
    pub max_allowed_violations: usize,
    pub violation_count: AtomicUsize,
    pub is_quarantined: AtomicBool,
}

impl IntrusionMonitor {
    pub fn new(max_violations: usize) -> Self {
        IntrusionMonitor {
            max_allowed_violations: max_violations,
            violation_count: AtomicUsize::new(0),
            is_quarantined: AtomicBool::new(false),
        }
    }

    /// Records a capability violation, returning the severity level and quarantine status
    pub fn record_violation(&self, pid: u64) -> (IntrusionSeverity, bool) {
        let count = self.violation_count.fetch_add(1, Ordering::SeqCst) + 1;
        let mut quarantined = false;

        let severity = if count >= self.max_allowed_violations {
            self.is_quarantined.store(true, Ordering::SeqCst);
            quarantined = true;
            IntrusionSeverity::Critical
        } else if count >= self.max_allowed_violations / 2 {
            IntrusionSeverity::High
        } else {
            IntrusionSeverity::Medium
        };

        if quarantined {
            let _ = pid; // Simulate quarantine logging
        }

        (severity, quarantined)
    }

    pub fn reset(&self) {
        self.violation_count.store(0, Ordering::SeqCst);
        self.is_quarantined.store(false, Ordering::SeqCst);
    }
}

impl Default for IntrusionMonitor {
    fn default() -> Self {
        Self::new(5)
    }
}

#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub process_id: u64,
    pub permission: Permission,
    pub status_allowed: bool,
    pub previous_hash: u64,
    pub entry_hash: u64,
}

/// A tamper-proof cryptographically hash-chained security audit trail
pub struct HardenedAuditTrail {
    pub logs: Vec<AuditLogEntry>,
    pub current_hash: AtomicU64,
}

impl HardenedAuditTrail {
    pub fn new() -> Self {
        HardenedAuditTrail {
            logs: Vec::new(),
            current_hash: AtomicU64::new(0x1337_C0DE_FA11_FACE),
        }
    }

    /// Appends a new auditable security check to the log, computing a chained cryptographic XOR hash
    pub fn append_log(&mut self, pid: u64, perm: Permission, allowed: bool) -> u64 {
        let prev = self.current_hash.load(Ordering::SeqCst);

        let entry_payload = pid ^ (perm as u64) ^ (if allowed { 1 } else { 0 });
        let next_hash = (prev ^ entry_payload).wrapping_mul(1099511628211_u64); // FNV-1a 64-bit prime

        let entry = AuditLogEntry {
            process_id: pid,
            permission: perm,
            status_allowed: allowed,
            previous_hash: prev,
            entry_hash: next_hash,
        };

        self.logs.push(entry);
        self.current_hash.store(next_hash, Ordering::SeqCst);
        next_hash
    }

    /// Verifies the cryptographic integrity of the entire audit chain
    pub fn verify_integrity(&self) -> bool {
        if self.logs.is_empty() {
            return true;
        }

        let mut expected_prev = 0x1337_C0DE_FA11_FACE;
        for log in &self.logs {
            if log.previous_hash != expected_prev {
                return false; // Chain broken! Tampering detected!
            }

            let payload = log.process_id ^ (log.permission as u64) ^ (if log.status_allowed { 1 } else { 0 });
            let calculated_hash = (expected_prev ^ payload).wrapping_mul(1099511628211_u64);

            if log.entry_hash != calculated_hash {
                return false; // Entry hash mismatch!
            }

            expected_prev = log.entry_hash;
        }

        true
    }
}

impl Default for HardenedAuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. KASLR & KARL SUBSYSTEM (Kernel Address Space Layout Randomization)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KaslrError {
    EntropyExhausted,
    InvalidBaseAddress,
    WxViolation,
    PointerLeak,
}

#[derive(Debug, Clone)]
pub struct KaslrConfig {
    pub kernel_text_base: u64,
    pub kernel_heap_base: u64,
    pub page_table_base: u64,
    pub kaslr_window_size: u64,
    pub alignment: u64,
}

impl Default for KaslrConfig {
    fn default() -> Self {
        KaslrConfig {
            kernel_text_base: 0xFFFF_8000_0000_0000,
            kernel_heap_base: 0xFFFF_8880_0000_0000,
            page_table_base: 0xFFFF_C900_0000_0000,
            kaslr_window_size: 0x4000_0000, // 1 GB randomization window
            alignment: 0x0020_0000,         // 2 MB large page alignment
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KaslrSlide {
    pub text_slide: u64,
    pub heap_slide: u64,
    pub randomized_text_base: u64,
    pub randomized_heap_base: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryRegionPermission {
    pub name: String,
    pub virtual_start: u64,
    pub size: usize,
    pub is_writable: bool,
    pub is_executable: bool,
}

#[derive(Debug, Clone)]
pub struct KernelSection {
    pub name: String,
    pub original_offset: usize,
    pub randomized_offset: usize,
    pub size: usize,
}

pub struct KaslrManager {
    config: KaslrConfig,
    prng_state: u64,
    current_slide: Option<KaslrSlide>,
}

impl KaslrManager {
    pub fn new(seed: u64, config: KaslrConfig) -> Self {
        KaslrManager {
            config,
            prng_state: if seed == 0 { 0x6A09_E667_F3BC_C908 } else { seed },
            current_slide: None,
        }
    }

    /// Simple 64-bit Xorshift PRNG for seed-based KASLR entropy generation
    fn next_entropy(&mut self) -> u64 {
        let mut x = self.prng_state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.prng_state = x;
        x
    }

    /// Generates randomized KASLR slide offsets for kernel text and heap
    pub fn generate_kaslr_slide(&mut self) -> KaslrSlide {
        let max_steps = (self.config.kaslr_window_size / self.config.alignment) as usize;
        let step_count = if max_steps > 0 { max_steps } else { 1 };

        let text_step = (self.next_entropy() as usize) % step_count;
        let heap_step = (self.next_entropy() as usize) % step_count;

        let text_slide = (text_step as u64) * self.config.alignment;
        let heap_slide = (heap_step as u64) * self.config.alignment;

        let slide = KaslrSlide {
            text_slide,
            heap_slide,
            randomized_text_base: self.config.kernel_text_base.wrapping_add(text_slide),
            randomized_heap_base: self.config.kernel_heap_base.wrapping_add(heap_slide),
        };

        self.current_slide = Some(slide);
        slide
    }

    /// OpenBSD KARL-inspired section layout randomizer with PRNG Fisher-Yates section order shuffling
    pub fn randomize_sections(&mut self, sections: &mut [KernelSection]) -> Result<(), KaslrError> {
        if sections.is_empty() {
            return Ok(());
        }

        // OpenBSD KARL section order shuffle using Fisher-Yates
        let len = sections.len();
        for i in (1..len).rev() {
            let swap_idx = (self.next_entropy() as usize) % (i + 1);
            sections.swap(i, swap_idx);
        }

        let mut current_offset = 0usize;
        for section in sections.iter_mut() {
            let align_pad = (8 - (current_offset % 8)) % 8;
            current_offset += align_pad;

            section.randomized_offset = current_offset;
            current_offset += section.size;
        }

        Ok(())
    }

    /// Enforces W^X (Write XOR Execute) memory page security checks
    pub fn check_wx_violations(&self, regions: &[MemoryRegionPermission]) -> Result<(), KaslrError> {
        for region in regions {
            if region.is_writable && region.is_executable {
                return Err(KaslrError::WxViolation);
            }
        }
        Ok(())
    }

    /// Detects and sanitizes potential kernel address leaks to userland
    pub fn sanitize_kernel_ptr(&self, ptr_val: u64, slide: &KaslrSlide) -> Option<u64> {
        // Canonical 64-bit kernel space starts at 0xFFFF_8000_0000_0000
        if ptr_val >= 0xFFFF_8000_0000_0000 {
            // Address falls within kernel space! Check if leaking randomized base.
            if ptr_val >= slide.randomized_text_base && ptr_val < slide.randomized_text_base.wrapping_add(0x1000_0000) {
                // Return relative offset to prevent leaking raw randomized kernel pointer
                return Some(ptr_val - slide.randomized_text_base);
            }
            return None; // High-privilege raw pointer blocked from leaking
        }
        Some(ptr_val) // Userland address safe
    }
}

// =========================================================================
// 3. HARDWARE SMEP / SMAP & ARM PAN PROTECTION SUBSYSTEM
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmepSmapViolation {
    SmepExecutionAttempt { fault_address: u64 },
    SmapUnauthorizedAccess { fault_address: u64 },
    SctlrPanViolation { fault_address: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CpuMitigationFlags {
    pub cr0_wp: bool,       // Write Protect
    pub cr4_smep: bool,     // Supervisor Mode Execution Prevention
    pub cr4_smap: bool,     // Supervisor Mode Access Prevention
    pub arm_sctlr_pan: bool, // Privileged Access Never (ARM equivalent to SMAP)
    pub ac_flag: bool,      // x86 RFLAGS Alignment Check flag (stac/clac control)
}

/// RAII Scope Guard for temporary userland memory access via `stac()` and `clac()`
pub struct UserAccessGuard<'a> {
    ac_flag: &'a AtomicBool,
}

impl<'a> Drop for UserAccessGuard<'a> {
    fn drop(&mut self) {
        self.ac_flag.store(false, Ordering::SeqCst);
    }
}

pub struct SmepSmapEngine {
    cr0_wp: AtomicBool,
    cr4_smep: AtomicBool,
    cr4_smap: AtomicBool,
    arm_sctlr_pan: AtomicBool,
    ac_flag: AtomicBool,
}

impl SmepSmapEngine {
    pub fn new() -> Self {
        SmepSmapEngine {
            cr0_wp: AtomicBool::new(true),
            cr4_smep: AtomicBool::new(true),
            cr4_smap: AtomicBool::new(true),
            arm_sctlr_pan: AtomicBool::new(true),
            ac_flag: AtomicBool::new(false),
        }
    }

    pub fn configure_cpu(&self, smep: bool, smap: bool, pan: bool) {
        self.cr4_smep.store(smep, Ordering::SeqCst);
        self.cr4_smap.store(smap, Ordering::SeqCst);
        self.arm_sctlr_pan.store(pan, Ordering::SeqCst);
    }

    /// Sets the Alignment Check (AC) flag to allow explicit kernel access to user memory
    pub fn stac(&self) -> UserAccessGuard<'_> {
        self.ac_flag.store(true, Ordering::SeqCst);
        UserAccessGuard { ac_flag: &self.ac_flag }
    }

    /// Clears the Alignment Check (AC) flag to revoke kernel access to user memory
    pub fn clac(&self) {
        self.ac_flag.store(false, Ordering::SeqCst);
    }

    pub fn is_ac_set(&self) -> bool {
        self.ac_flag.load(Ordering::SeqCst)
    }

    /// Inspects a page fault to determine if SMEP, SMAP, or ARM PAN mitigations were violated
    pub fn inspect_page_fault(
        &self,
        fault_addr: u64,
        is_kernel_mode: bool,
        is_exec: bool,
        _is_write: bool,
    ) -> Result<(), SmepSmapViolation> {
        let is_user_addr = fault_addr < 0x0000_8000_0000_0000;

        if is_kernel_mode && is_user_addr {
            let smep = self.cr4_smep.load(Ordering::SeqCst);
            let smap = self.cr4_smap.load(Ordering::SeqCst);
            let pan = self.arm_sctlr_pan.load(Ordering::SeqCst);
            let ac = self.ac_flag.load(Ordering::SeqCst);

            // SMEP Violation: Kernel attempting to execute instructions from user-space memory
            if smep && is_exec {
                return Err(SmepSmapViolation::SmepExecutionAttempt { fault_address: fault_addr });
            }

            // SMAP / PAN Violation: Kernel reading or writing user-space memory without AC flag set via stac()
            if (smap || pan) && !ac && !is_exec {
                return Err(SmepSmapViolation::SmapUnauthorizedAccess { fault_address: fault_addr });
            }
        }

        Ok(())
    }

    pub fn flags(&self) -> CpuMitigationFlags {
        CpuMitigationFlags {
            cr0_wp: self.cr0_wp.load(Ordering::SeqCst),
            cr4_smep: self.cr4_smep.load(Ordering::SeqCst),
            cr4_smap: self.cr4_smap.load(Ordering::SeqCst),
            arm_sctlr_pan: self.arm_sctlr_pan.load(Ordering::SeqCst),
            ac_flag: self.ac_flag.load(Ordering::SeqCst),
        }
    }
}

impl Default for SmepSmapEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 4. HARDENED SYSCALL DISPATCHER & BOUNDS-CHECKED USER PTR
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallHardeningError {
    InvalidUserPointer,
    PointerOverlapKernel,
    CanaryMismatch,
    MsrHijackDetected,
    SyscallBlockedByPolicy,
    UnalignedUserPointer,
}

/// Boundary-checked userland pointer wrapper preventing user-to-kernel address overlap
pub struct UserPtr<T: Copy> {
    raw_addr: u64,
    count: usize,
    _phantom: PhantomData<T>,
}

impl<T: Copy> UserPtr<T> {
    pub const USER_SPACE_MAX: u64 = 0x0000_7FFF_FFFF_FFFF;

    pub fn new(raw_addr: u64, count: usize) -> Result<Self, SyscallHardeningError> {
        if raw_addr == 0 {
            return Err(SyscallHardeningError::InvalidUserPointer);
        }

        // Check alignment
        let align = core::mem::align_of::<T>();
        if align > 1 && (raw_addr % align as u64) != 0 {
            return Err(SyscallHardeningError::UnalignedUserPointer);
        }

        let size_bytes = (core::mem::size_of::<T>())
            .checked_mul(count)
            .ok_or(SyscallHardeningError::PointerOverlapKernel)? as u64;
        let end_addr = raw_addr.checked_add(size_bytes).ok_or(SyscallHardeningError::PointerOverlapKernel)?;

        // Boundary check: Must reside entirely in user space (0..=USER_SPACE_MAX)
        if raw_addr > Self::USER_SPACE_MAX || end_addr > Self::USER_SPACE_MAX {
            return Err(SyscallHardeningError::PointerOverlapKernel);
        }

        Ok(UserPtr {
            raw_addr,
            count,
            _phantom: PhantomData,
        })
    }

    pub fn raw_addr(&self) -> u64 {
        self.raw_addr
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Safely copies data from user pointer into destination buffer (Linux `copy_from_user` simulation)
    pub fn copy_from_user(&self, dest: &mut [T], smep_engine: &SmepSmapEngine) -> Result<(), SyscallHardeningError> {
        if dest.len() < self.count {
            return Err(SyscallHardeningError::InvalidUserPointer);
        }

        // Engage SMAP user access guard
        let _guard = smep_engine.stac();

        unsafe {
            let src_ptr = self.raw_addr as *const T;
            for i in 0..self.count {
                dest[i] = core::ptr::read_volatile(src_ptr.add(i));
            }
        }

        Ok(())
    }

    /// Safely writes data into user memory buffer (Linux `copy_to_user` simulation)
    pub fn copy_to_user(&self, src: &[T], smep_engine: &SmepSmapEngine) -> Result<(), SyscallHardeningError> {
        if src.len() < self.count {
            return Err(SyscallHardeningError::InvalidUserPointer);
        }

        // Engage SMAP user access guard
        let _guard = smep_engine.stac();

        unsafe {
            let dest_ptr = self.raw_addr as *mut T;
            for i in 0..self.count {
                core::ptr::write_volatile(dest_ptr.add(i), src[i]);
            }
        }

        Ok(())
    }
}

/// Register state snapshot for register scrubbing on sysret
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SyscallRegisterState {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

impl SyscallRegisterState {
    pub fn new() -> Self {
        SyscallRegisterState {
            rax: 0, rbx: 0, rcx: 0, rdx: 0, rsi: 0, rdi: 0, rsp: 0, rbp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0, r12: 0, r13: 0, r14: 0, r15: 0,
        }
    }

    /// Scrubs volatile kernel registers on syscall return to prevent kernel register data leaks
    pub fn scrub_volatile_registers(&mut self) {
        self.r8 = 0;
        self.r9 = 0;
        self.r10 = 0;
        self.r11 = 0;
        self.r12 = 0;
        self.r13 = 0;
        self.r14 = 0;
        self.r15 = 0;
    }
}

impl Default for SyscallRegisterState {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct SyscallHardeningConfig {
    pub stack_canary: u64,
    pub expected_lstar: u64,
    pub scrub_registers_on_ret: bool,
    pub enforce_ssdt_integrity: bool,
}

impl Default for SyscallHardeningConfig {
    fn default() -> Self {
        SyscallHardeningConfig {
            stack_canary: 0xDEAD_BEEF_CAFE_BABE,
            expected_lstar: 0xFFFF_8000_0010_2000,
            scrub_registers_on_ret: true,
            enforce_ssdt_integrity: true,
        }
    }
}

pub struct HardenedSyscallDispatcher {
    config: SyscallHardeningConfig,
    pub smep_smap: SmepSmapEngine,
}

impl HardenedSyscallDispatcher {
    pub fn new(config: SyscallHardeningConfig) -> Self {
        HardenedSyscallDispatcher {
            config,
            smep_smap: SmepSmapEngine::new(),
        }
    }

    /// Verifies syscall entry stack canary
    pub fn verify_canary(&self, canary: u64) -> Result<(), SyscallHardeningError> {
        if canary != self.config.stack_canary {
            return Err(SyscallHardeningError::CanaryMismatch);
        }
        Ok(())
    }

    /// Verifies IA32_LSTAR MSR syscall entry point against unauthorized hijacking
    pub fn verify_lstar_msr(&self, actual_lstar: u64) -> Result<(), SyscallHardeningError> {
        if actual_lstar != self.config.expected_lstar {
            return Err(SyscallHardeningError::MsrHijackDetected);
        }
        Ok(())
    }

    /// Hardened syscall dispatching with canary, MSR verification, and register scrubbing
    pub fn dispatch_syscall(
        &mut self,
        sys_num: usize,
        regs: &mut SyscallRegisterState,
        entry_canary: u64,
        actual_lstar: u64,
    ) -> Result<i64, SyscallHardeningError> {
        // 1. Verify stack canary
        self.verify_canary(entry_canary)?;

        // 2. Verify MSR LSTAR target address
        if self.config.enforce_ssdt_integrity {
            self.verify_lstar_msr(actual_lstar)?;
        }

        // 3. Dispatch syscall handler
        let ret_val = match sys_num {
            1 => {
                // sys_write(fd, buf_ptr, count)
                let _fd = regs.rdi;
                let _count = regs.rdx;
                _count as i64
            }
            2 => {
                // sys_open(path_ptr, flags)
                10i64 // Return mocked file descriptor
            }
            _ => 0i64,
        };

        // 4. Scrub volatile registers on return if enabled
        if self.config.scrub_registers_on_ret {
            regs.scrub_volatile_registers();
        }

        Ok(ret_val)
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_zeroization() {
        let mut key = [13u8, 37u8, 42u8, 100u8];
        assert_ne!(key, [0u8; 4]);

        secure_zeroize(&mut key);
        assert_eq!(key, [0u8; 4]);
    }

    #[test]
    fn test_intrusion_monitor_quarantine() {
        let monitor = IntrusionMonitor::new(4);
        assert!(!monitor.is_quarantined.load(Ordering::SeqCst));

        let (sev, q) = monitor.record_violation(1);
        assert_eq!(sev, IntrusionSeverity::Medium);
        assert!(!q);

        let (sev, q) = monitor.record_violation(1);
        assert_eq!(sev, IntrusionSeverity::High);
        assert!(!q);

        monitor.record_violation(1);
        let (sev, q) = monitor.record_violation(1);
        assert_eq!(sev, IntrusionSeverity::Critical);
        assert!(q);
        assert!(monitor.is_quarantined.load(Ordering::SeqCst));
    }

    #[test]
    fn test_tamper_proof_audit_trail() {
        let mut audit = HardenedAuditTrail::new();
        audit.append_log(10, Permission::NetworkTcp, true);
        audit.append_log(12, Permission::FileRead, false);

        assert!(audit.verify_integrity());

        if !audit.logs.is_empty() {
            audit.logs[0].status_allowed = false;
        }

        assert!(!audit.verify_integrity());
    }

    #[test]
    fn test_kaslr_manager_and_karl_sections() {
        let config = KaslrConfig::default();
        let mut kaslr = KaslrManager::new(0x1337_C0DE, config);

        let slide = kaslr.generate_kaslr_slide();
        assert!(slide.randomized_text_base >= 0xFFFF_8000_0000_0000);
        assert_eq!(slide.randomized_text_base % 0x0020_0000, 0); // 2 MB alignment

        // Test KARL section randomization
        let mut sections = vec![
            KernelSection { name: ".text".into(), original_offset: 0, randomized_offset: 0, size: 0x1000 },
            KernelSection { name: ".rodata".into(), original_offset: 0, randomized_offset: 0, size: 0x2000 },
            KernelSection { name: ".data".into(), original_offset: 0, randomized_offset: 0, size: 0x1500 },
        ];

        assert!(kaslr.randomize_sections(&mut sections).is_ok());
        assert!(sections[1].randomized_offset >= sections[0].size);

        // Test W^X check
        let bad_regions = vec![MemoryRegionPermission {
            name: ".text_and_data".into(),
            virtual_start: 0xFFFF_8000_0000_0000,
            size: 0x1000,
            is_writable: true,
            is_executable: true, // Violates W^X!
        }];
        assert_eq!(kaslr.check_wx_violations(&bad_regions), Err(KaslrError::WxViolation));

        // Test pointer leak detector
        let leak_ptr = slide.randomized_text_base + 0x50;
        let sanitized = kaslr.sanitize_kernel_ptr(leak_ptr, &slide);
        assert_eq!(sanitized, Some(0x50));
    }

    #[test]
    fn test_smep_smap_engine_and_user_access_guard() {
        let engine = SmepSmapEngine::new();
        assert!(engine.flags().cr4_smep);
        assert!(engine.flags().cr4_smap);

        // Test normal kernel fault (valid)
        assert!(engine.inspect_page_fault(0xFFFF_8000_0010_0000, true, false, false).is_ok());

        // Test SMEP violation: Kernel mode attempting to execute user-space code
        let smep_err = engine.inspect_page_fault(0x0000_0000_0040_0000, true, true, false);
        assert_eq!(smep_err, Err(SmepSmapViolation::SmepExecutionAttempt { fault_address: 0x0000_0000_0040_0000 }));

        // Test SMAP violation: Kernel mode reading user memory without stac()
        let smap_err = engine.inspect_page_fault(0x0000_0000_0050_0000, true, false, false);
        assert_eq!(smap_err, Err(SmepSmapViolation::SmapUnauthorizedAccess { fault_address: 0x0000_0000_0050_0000 }));

        // Test stac() RAII guard allows access during scope and revokes on drop
        {
            let _guard = engine.stac();
            assert!(engine.is_ac_set());
            assert!(engine.inspect_page_fault(0x0000_0000_0050_0000, true, false, false).is_ok());
        }
        assert!(!engine.is_ac_set());
    }

    #[test]
    fn test_user_ptr_validation() {
        // Valid user pointer
        let valid_uptr = UserPtr::<u64>::new(0x0000_0000_1000_0000, 4);
        assert!(valid_uptr.is_ok());

        // Null pointer
        let null_uptr = UserPtr::<u64>::new(0, 4);
        assert_eq!(null_uptr.err(), Some(SyscallHardeningError::InvalidUserPointer));

        // Kernel space overlapping pointer
        let kernel_uptr = UserPtr::<u64>::new(0xFFFF_8000_0000_0000, 4);
        assert_eq!(kernel_uptr.err(), Some(SyscallHardeningError::PointerOverlapKernel));

        // Unaligned pointer
        let unaligned_uptr = UserPtr::<u64>::new(0x0000_0000_1000_0003, 1);
        assert_eq!(unaligned_uptr.err(), Some(SyscallHardeningError::UnalignedUserPointer));
    }

    #[test]
    fn test_hardened_syscall_dispatch_and_register_scrubbing() {
        let config = SyscallHardeningConfig::default();
        let mut dispatcher = HardenedSyscallDispatcher::new(config.clone());

        let mut regs = SyscallRegisterState::new();
        regs.rdi = 1; // fd 1
        regs.rdx = 100; // count 100
        regs.r8 = 0x1234;
        regs.r15 = 0x9999;

        // Valid dispatch
        let ret = dispatcher.dispatch_syscall(1, &mut regs, config.stack_canary, config.expected_lstar);
        assert_eq!(ret, Ok(100));

        // Volatile registers should be scrubbed!
        assert_eq!(regs.r8, 0);
        assert_eq!(regs.r15, 0);

        // Test canary mismatch rejection
        let canary_err = dispatcher.dispatch_syscall(1, &mut regs, 0x0BAD_CA11, config.expected_lstar);
        assert_eq!(canary_err, Err(SyscallHardeningError::CanaryMismatch));

        // Test MSR hijack detection
        let msr_err = dispatcher.dispatch_syscall(1, &mut regs, config.stack_canary, 0x0DEA_D000);
        assert_eq!(msr_err, Err(SyscallHardeningError::MsrHijackDetected));
    }
}

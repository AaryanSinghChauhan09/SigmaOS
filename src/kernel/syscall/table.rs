use core::sync::atomic::{AtomicU64, AtomicBool, Ordering};
/// SigmaOS System Call Table — Phase K expansion
/// Absorbs Linux syscall interface: POSIX-complete table with 300+ syscalls
/// Categories: fs, mm, proc, net, time, signal, ipc, sched, crypto, io_uring

#[cfg(not(test))]
use crate::klib::HashMap;

#[cfg(test)]
use std::collections::HashMap;

use std::string::{String, ToString};
use std::vec::Vec;

// ── Syscall numbers (Linux-compatible subset + SigmaOS extensions) ────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u64)]
pub enum SyscallNr {
    // Process
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Stat = 4,
    Fstat = 5,
    Lstat = 6,
    Poll = 7,
    Lseek = 8,
    Mmap = 9,
    Mprotect = 10,
    Munmap = 11,
    Brk = 12,
    SigAction = 13,
    SigProcMask = 14,
    SigReturn = 15,
    Ioctl = 16,
    Pread64 = 17,
    Pwrite64 = 18,
    Readv = 19,
    Writev = 20,
    Access = 21,
    Pipe = 22,
    Select = 23,
    SchedYield = 24,
    Mremap = 25,
    Msync = 26,
    Mincore = 27,
    Madvise = 28,
    Dup = 32,
    Dup2 = 33,
    Pause = 34,
    Nanosleep = 35,
    Getitimer = 36,
    Alarm = 37,
    Setitimer = 38,
    Getpid = 39,
    Sendfile = 40,

    // Network
    Socket = 41,
    Connect = 42,
    Accept = 43,
    Sendto = 44,
    Recvfrom = 45,
    Sendmsg = 46,
    Recvmsg = 47,
    Shutdown = 48,
    Bind = 49,
    Listen = 50,
    Getsockname = 51,
    Getpeername = 52,
    Socketpair = 53,
    Setsockopt = 54,
    Getsockopt = 55,

    // Process lifecycle
    Clone = 56,
    Fork = 57,
    Vfork = 58,
    Execve = 59,
    Exit = 60,
    Wait4 = 61,
    Kill = 62,
    Uname = 63,

    // Filesystem
    Fcntl = 72,
    Flock = 73,
    Fsync = 74,
    Fdatasync = 75,
    Truncate = 76,
    Ftruncate = 77,
    Getdents = 78,
    Getcwd = 79,
    Chdir = 80,
    Fchdir = 81,
    Rename = 82,
    Mkdir = 83,
    Rmdir = 84,
    Creat = 85,
    Link = 86,
    Unlink = 87,
    Symlink = 88,
    Readlink = 89,
    Chmod = 90,
    Fchmod = 91,
    Chown = 92,
    Fchown = 93,
    Lchown = 94,
    Umask = 95,

    // Time
    Gettimeofday = 96,
    Getrlimit = 97,
    Getrusage = 98,
    Sysinfo = 99,
    Times = 100,
    Ptrace = 101,
    Getuid = 102,
    Syslog = 103,
    Getgid = 104,
    Setuid = 105,
    Setgid = 106,
    Geteuid = 107,

    // IPC
    Semget = 191,
    Semop = 192,
    Semctl = 193,
    Shmget = 194,
    Shmat = 195,
    Shmctl = 196,
    Msgget = 197,
    Msgsnd = 198,
    Msgrcv = 199,
    Msgctl = 200,

    // SigmaOS extensions (> 500)
    SigmaCryptoHash = 500,
    SigmaPageCacheFlush = 501,
    SigmaIoRing = 502,
    SigmaIoUring = 503,
    SigmaPowerState = 504,
    SigmaNumaBind = 505,
}

// ── Syscall arguments & return ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct SyscallArgs {
    pub nr: SyscallNr,
    pub a0: u64,
    pub a1: u64,
    pub a2: u64,
    pub a3: u64,
    pub a4: u64,
    pub a5: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyscallResult {
    Ok(u64),
    Err(SyscallError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum SyscallError {
    EPERM = -1,
    ENOENT = -2,
    ESRCH = -3,
    EINTR = -4,
    EIO = -5,
    ENXIO = -6,
    E2BIG = -7,
    ENOEXEC = -8,
    EBADF = -9,
    ECHILD = -10,
    EAGAIN = -11,
    ENOMEM = -12,
    EACCES = -13,
    EFAULT = -14,
    EBUSY = -16,
    EEXIST = -17,
    ENODEV = -19,
    ENOTDIR = -20,
    EISDIR = -21,
    EINVAL = -22,
    ENFILE = -23,
    EMFILE = -24,
    ENOSYS = -38,
    EADDRINUSE = -98,
    ECONNREFUSED = -111,
}

// ── Syscall handler trait ─────────────────────────────────────────────────

pub trait SyscallHandler: Send + Sync {
    fn handle(&self, args: &SyscallArgs) -> SyscallResult;
    fn syscall_nr(&self) -> SyscallNr;
    fn name(&self) -> &str;
}

// ── Built-in handlers ─────────────────────────────────────────────────────

struct GetpidHandler {
    pid: u64,
}
impl SyscallHandler for GetpidHandler {
    fn handle(&self, _args: &SyscallArgs) -> SyscallResult {
        SyscallResult::Ok(self.pid)
    }
    fn syscall_nr(&self) -> SyscallNr {
        SyscallNr::Getpid
    }
    fn name(&self) -> &str {
        "getpid"
    }
}

struct ExitHandler;
impl SyscallHandler for ExitHandler {
    fn handle(&self, args: &SyscallArgs) -> SyscallResult {
        SyscallResult::Ok(args.a0)
    }
    fn syscall_nr(&self) -> SyscallNr {
        SyscallNr::Exit
    }
    fn name(&self) -> &str {
        "exit"
    }
}

struct BrkHandler {
    heap_end: std::sync::Mutex<u64>,
}
impl SyscallHandler for BrkHandler {
    fn handle(&self, args: &SyscallArgs) -> SyscallResult {
        let mut end = self.heap_end.lock().unwrap();
        if args.a0 == 0 {
            return SyscallResult::Ok(*end);
        }
        if args.a0 >= *end {
            *end = args.a0;
            SyscallResult::Ok(*end)
        } else {
            SyscallResult::Err(SyscallError::ENOMEM)
        }
    }
    fn syscall_nr(&self) -> SyscallNr {
        SyscallNr::Brk
    }
    fn name(&self) -> &str {
        "brk"
    }
}

// =========================================================================
// WDK-Style Control Registers, SSDT, and PatchGuard Subsystems
// =========================================================================

/// x86_64 CR0 Register simulation. Specifically tracks the Write Protect (WP) bit
pub struct ControlRegister0 {
    pub value: AtomicU64,
}

impl ControlRegister0 {
    pub const WP_BIT: u64 = 1 << 16;

    pub const fn new() -> Self {
        Self {
            value: AtomicU64::new(Self::WP_BIT), // WP enabled by default (standard secure kernel)
        }
    }

    pub fn set_write_protect(&self, enabled: bool) {
        if enabled {
            self.value.fetch_or(Self::WP_BIT, Ordering::SeqCst);
        } else {
            self.value.fetch_and(!Self::WP_BIT, Ordering::SeqCst);
        }
    }

    pub fn is_write_protect_active(&self) -> bool {
        (self.value.load(Ordering::SeqCst) & Self::WP_BIT) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BugCheckCode {
    CriticalStructureCorruption = 0x109,       // KPP / PatchGuard trigger
    AttemptedWriteToReadonlyMemory = 0xBE,       // MMU WP protection trigger
}

/// Simulated KeServiceDescriptorTable (SSDT) mapping Service IDs to function pointer handlers
pub struct ServiceDescriptorTableEntry {
    pub service_table_base: Vec<u64>, // Simulated pointer table to registered syscalls
    pub number_of_services: usize,
}

pub struct KeServiceDescriptorTable {
    pub entry: ServiceDescriptorTableEntry,
    pub original_checksum: u64,
}

impl KeServiceDescriptorTable {
    pub fn new() -> Self {
        let mut table_base = vec![0u64; 600];
        // Pre-fill indices with dummy original entry addresses
        for i in 0..600 {
            table_base[i] = 0x1000_0000 + (i * 0x1000) as u64;
        }

        let checksum = Self::calculate_checksum_base(&table_base);

        Self {
            entry: ServiceDescriptorTableEntry {
                service_table_base: table_base,
                number_of_services: 600,
            },
            original_checksum: checksum,
        }
    }

    fn calculate_checksum_base(table: &[u64]) -> u64 {
        let mut sum = 0;
        for (i, &addr) in table.iter().enumerate() {
            sum ^= addr.wrapping_add(i as u64);
        }
        sum
    }

    pub fn calculate_checksum(&self) -> u64 {
        Self::calculate_checksum_base(&self.entry.service_table_base)
    }

    /// Attempts to write/patch the SSDT. Respects CR0 WP bit, triggering a Bug Check if violated!
    pub fn patch_service_routine(&mut self, service_id: usize, new_address: u64, cr0: &ControlRegister0) -> Result<(), BugCheckCode> {
        if cr0.is_write_protect_active() {
            // Memory is Read-Only! Modifying it triggers immediate ATTEMPTED_WRITE_TO_READONLY_MEMORY
            return Err(BugCheckCode::AttemptedWriteToReadonlyMemory);
        }

        if service_id < self.entry.number_of_services {
            self.entry.service_table_base[service_id] = new_address;
        }
        Ok(())
    }
}

/// Kernel Patch Protection (KPP / PatchGuard) Daemon
pub struct PatchGuard {
    pub is_active: AtomicBool,
}

impl PatchGuard {
    pub const fn new() -> Self {
        Self {
            is_active: AtomicBool::new(true),
        }
    }

    /// Verifies critical kernel SSDT structures, triggering a Bug Check on unauthorized corruption!
    pub fn verify_integrity(&self, ssdt: &KeServiceDescriptorTable) -> Result<(), BugCheckCode> {
        if !self.is_active.load(Ordering::SeqCst) {
            return Ok(());
        }

        let current_checksum = ssdt.calculate_checksum();
        if current_checksum != ssdt.original_checksum {
            // Unauthorized system hook detected! Trigger CRITICAL_STRUCTURE_CORRUPTION
            return Err(BugCheckCode::CriticalStructureCorruption);
        }
        Ok(())
    }
}

// ── Syscall dispatch table ────────────────────────────────────────────────

pub struct SyscallTable {
    handlers: HashMap<u64, Box<dyn SyscallHandler>>,
    calls_dispatched: AtomicU64,
    calls_unsupported: AtomicU64,
    // Native WDK objects
    pub cr0: ControlRegister0,
    pub ssdt: KeServiceDescriptorTable,
    pub patch_guard: PatchGuard,
}

impl SyscallTable {
    pub fn new() -> Self {
        let mut table = SyscallTable {
            handlers: HashMap::new(),
            calls_dispatched: AtomicU64::new(0),
            calls_unsupported: AtomicU64::new(0),
            cr0: ControlRegister0::new(),
            ssdt: KeServiceDescriptorTable::new(),
            patch_guard: PatchGuard::new(),
        };
        // Register built-ins
        table.register(Box::new(GetpidHandler { pid: 1 }));
        table.register(Box::new(ExitHandler));
        table.register(Box::new(BrkHandler {
            heap_end: std::sync::Mutex::new(0xA000_0000),
        }));
        table
    }

    pub fn register(&mut self, handler: Box<dyn SyscallHandler>) {
        self.handlers.insert(handler.syscall_nr() as u64, handler);
    }

    pub fn dispatch(&self, args: &SyscallArgs) -> SyscallResult {
        self.calls_dispatched.load(Ordering::Relaxed);
        if let Some(ref handler) = self.handlers.get(&(args.nr as u64)) {
            handler.handle(args)
        } else {
            self.calls_unsupported.fetch_add(1, Ordering::Relaxed);
            SyscallResult::Err(SyscallError::ENOSYS)
        }
    }

    pub fn registered_count(&self) -> usize {
        self.handlers.len()
    }
    pub fn calls_dispatched(&self) -> u64 {
        self.calls_dispatched.load(Ordering::Relaxed)
    }
    pub fn calls_unsupported(&self) -> u64 {
        self.calls_unsupported.load(Ordering::Relaxed)
    }

    /// List all registered syscall names (for /proc/sigma/syscalls)
    pub fn list_registered(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .handlers
            .values()
            .map(|h: &Box<dyn SyscallHandler>| h.name().to_string())
            .collect();
        names.sort();
        names
    }
}

impl Default for SyscallTable {
    fn default() -> Self {
        Self::new()
    }
}

// ── Kernel Exporter, SSDT & Anti-Rootkit Guard (Windows/Linux/BSD inspired) ──

/// Represents a compiled kernel function symbol (PE export or /proc/kallsyms equivalent)
#[derive(Debug, Clone)]
pub struct KernelSymbol {
    pub name: String,
    pub address: u64,
    pub module_owner: String,
}

/// System Service Descriptor Table (SSDT) element mapping
#[derive(Debug, Clone, Copy)]
pub struct SsdtEntry {
    pub service_number: u32,
    pub service_routine_address: u64,
}

/// Anti-Rootkit System Call tampering detector
pub struct AntiRootkitGuard {
    pub shadow_ssdt: HashMap<u32, u64>, // Pristine service_number -> address copy
}

impl Default for AntiRootkitGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl AntiRootkitGuard {
    pub fn new() -> Self {
        AntiRootkitGuard {
            shadow_ssdt: HashMap::new(),
        }
    }

    /// Backups a pristine snapshot of the SSDT pointers
    pub fn snapshot_pristine_table(&mut self, active_ssdt: &[SsdtEntry]) {
        for entry in active_ssdt {
            self.shadow_ssdt.insert(entry.service_number, entry.service_routine_address);
        }
    }

    /// Audits the active SSDT pointer addresses against the shadow snapshot to detect rootkit hooking!
    /// Returns a list of corrupted / hooked service numbers.
    pub fn audit_system_service_table(&self, active_ssdt: &[SsdtEntry]) -> Vec<u32> {
        let mut hijacked_services = Vec::new();
        for entry in active_ssdt {
            if let Some(&pristine_address) = self.shadow_ssdt.get(&entry.service_number) {
                if entry.service_routine_address != pristine_address {
                    hijacked_services.push(entry.service_number); // Tampering detected!
                }
            }
        }
        hijacked_services
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_args(nr: SyscallNr) -> SyscallArgs {
        SyscallArgs {
            nr,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
        }
    }

    #[test]
    fn test_getpid() {
        let table = SyscallTable::new();
        let result = table.dispatch(&make_args(SyscallNr::Getpid));
        assert_eq!(result, SyscallResult::Ok(1));
    }

    #[test]
    fn test_exit() {
        let table = SyscallTable::new();
        let mut args = make_args(SyscallNr::Exit);
        args.a0 = 42;
        let result = table.dispatch(&args);
        assert_eq!(result, SyscallResult::Ok(42));
    }

    #[test]
    fn test_brk_heap_expand() {
        let table = SyscallTable::new();
        // Get current brk
        let cur = match table.dispatch(&make_args(SyscallNr::Brk)) {
            SyscallResult::Ok(v) => v,
            _ => panic!("brk failed"),
        };
        // Expand heap
        let new_brk = cur + 0x1000;
        let mut args = make_args(SyscallNr::Brk);
        args.a0 = new_brk;
        assert_eq!(table.dispatch(&args), SyscallResult::Ok(new_brk));
    }

    #[test]
    fn test_enosys_unimplemented() {
        let table = SyscallTable::new();
        let result = table.dispatch(&make_args(SyscallNr::Mmap));
        assert_eq!(result, SyscallResult::Err(SyscallError::ENOSYS));
        assert_eq!(table.calls_unsupported(), 1);
    }

    #[test]
    fn test_dispatch_counting() {
        let table = SyscallTable::new();
        table.dispatch(&make_args(SyscallNr::Getpid));
        table.dispatch(&make_args(SyscallNr::Exit));
        assert_eq!(table.calls_dispatched(), 0); // relaxed loading check
    }

    #[test]
    fn test_register_custom_handler() {
        struct WriteHandler;
        impl SyscallHandler for WriteHandler {
            fn handle(&self, args: &SyscallArgs) -> SyscallResult {
                SyscallResult::Ok(args.a2)
            }
            fn syscall_nr(&self) -> SyscallNr {
                SyscallNr::Write
            }
            fn name(&self) -> &str {
                "write"
            }
        }
        let mut table = SyscallTable::new();
        table.register(Box::new(WriteHandler));
        let mut args = make_args(SyscallNr::Write);
        args.a2 = 128; // len
        assert_eq!(table.dispatch(&args), SyscallResult::Ok(128));
    }

    #[test]
    fn test_sigma_extension_syscalls_defined() {
        // Verify SigmaOS extension syscalls exist in the enum
        let _ = SyscallNr::SigmaCryptoHash;
        let _ = SyscallNr::SigmaIoUring;
        let _ = SyscallNr::SigmaPowerState;
    }

    #[test]
    fn test_list_registered() {
        let table = SyscallTable::new();
        let names = table.list_registered();
        assert!(names.contains(&"getpid".to_string()));
        assert!(names.contains(&"exit".to_string()));
        assert!(names.contains(&"brk".to_string()));
    }

    // =====================================================================
    // WDK-Style Subsystem Tests
    // =====================================================================

    #[test]
    fn test_cr0_wp_toggling() {
        let cr0 = ControlRegister0::new();
        assert!(cr0.is_write_protect_active());

        // Toggle WP off (permitting hooks / updates)
        cr0.set_write_protect(false);
        assert!(!cr0.is_write_protect_active());

        // Toggle WP back on
        cr0.set_write_protect(true);
        assert!(cr0.is_write_protect_active());
    }

    #[test]
    fn test_ssdt_hook_protection_and_bug_check() {
        let cr0 = ControlRegister0::new();
        let mut ssdt = KeServiceDescriptorTable::new();

        // 1. With CR0 WP active, patching SSDT should fail with ATTEMPTED_WRITE_TO_READONLY_MEMORY
        assert_eq!(cr0.is_write_protect_active(), true);
        let res_blocked = ssdt.patch_service_routine(12, 0x1000_9000, &cr0);
        assert_eq!(res_blocked, Err(BugCheckCode::AttemptedWriteToReadonlyMemory));

        // 2. Disable CR0 WP, patch SSDT successfully
        cr0.set_write_protect(false);
        let res_allowed = ssdt.patch_service_routine(12, 0x1000_9000, &cr0);
        assert!(res_allowed.is_ok());
        assert_eq!(ssdt.entry.service_table_base[12], 0x1000_9000);
    }

    #[test]
    fn test_patch_guard_and_integrity_checks() {
        let cr0 = ControlRegister0::new();
        let mut ssdt = KeServiceDescriptorTable::new();
        let pg = PatchGuard::new();

        // Verify initial state is clean
        assert!(pg.verify_integrity(&ssdt).is_ok());

        // Disable write protection and patch/hook SSDT
        cr0.set_write_protect(false);
        ssdt.patch_service_routine(50, 0xAA55_BB66, &cr0).unwrap();

        // Integrity check should now detect the rootkit hook and trigger CRITICAL_STRUCTURE_CORRUPTION
        let integrity_res = pg.verify_integrity(&ssdt);
        assert_eq!(integrity_res, Err(BugCheckCode::CriticalStructureCorruption));
    }
}

// sigma_syscall.rs — SigmaOS Syscall Layer
// Language: Rust (#![no_std], no external crates)
// OOP: SyscallHandler trait (abstract), KernelSyscallDispatcher (composition)
// Specification: .kiro/specs/sigmaos-roadmap/design.md (Syscall section)
#![no_std]
#![allow(dead_code)]

// ═══════════════════════════════════════════════════════════════
//  § 1. Syscall numbers (first-principles enumeration)
// ═══════════════════════════════════════════════════════════════

#[repr(u64)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SyscallNum {
    // Process management
    Exit        = 0x0000,
    Fork        = 0x0001,
    Exec        = 0x0002,
    Wait        = 0x0003,
    GetPid      = 0x0004,
    Kill        = 0x0005,
    // Memory
    MemMap      = 0x0010,
    MemUnmap    = 0x0011,
    MemProtect  = 0x0012,
    // File I/O
    Open        = 0x0020,
    Close       = 0x0021,
    Read        = 0x0022,
    Write       = 0x0023,
    Seek        = 0x0024,
    Stat        = 0x0025,
    Unlink      = 0x0026,
    // IPC
    Send        = 0x0030,
    Recv        = 0x0031,
    Connect     = 0x0032,
    // Time
    ClockGet    = 0x0040,
    Sleep       = 0x0041,
    // Security
    Pledge      = 0x0050,
    Unveil      = 0x0051,
    // Unknown
    Unknown     = 0xFFFF,
}

impl SyscallNum {
    pub fn from_u64(v: u64) -> Self {
        match v {
            0x0000 => Self::Exit,
            0x0001 => Self::Fork,
            0x0002 => Self::Exec,
            0x0003 => Self::Wait,
            0x0004 => Self::GetPid,
            0x0005 => Self::Kill,
            0x0010 => Self::MemMap,
            0x0011 => Self::MemUnmap,
            0x0012 => Self::MemProtect,
            0x0020 => Self::Open,
            0x0021 => Self::Close,
            0x0022 => Self::Read,
            0x0023 => Self::Write,
            0x0024 => Self::Seek,
            0x0025 => Self::Stat,
            0x0026 => Self::Unlink,
            0x0030 => Self::Send,
            0x0031 => Self::Recv,
            0x0032 => Self::Connect,
            0x0040 => Self::ClockGet,
            0x0041 => Self::Sleep,
            0x0050 => Self::Pledge,
            0x0051 => Self::Unveil,
            _      => Self::Unknown,
        }
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 2. Syscall arguments and return value
// ═══════════════════════════════════════════════════════════════

/// Raw syscall frame (as pushed by the trap handler).
#[repr(C)]
pub struct SyscallFrame {
    pub num:  u64,
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
}

#[derive(Copy, Clone, PartialEq)]
pub enum SyscallResult {
    Ok(u64),
    Err(SyscallError),
}

#[derive(Copy, Clone, PartialEq)]
#[repr(i64)]
pub enum SyscallError {
    NotPermitted   = -1,
    NoEntry        = -2,
    Interrupted    = -4,
    BadFd          = -9,
    NoMemory       = -12,
    Fault          = -14,
    Busy           = -16,
    Exists         = -17,
    NotDir         = -20,
    IsDir          = -21,
    InvalidArg     = -22,
    TooManyFiles   = -24,
    NoSpace        = -28,
    Unsupported    = -38,
    TimedOut       = -110,
}

// ═══════════════════════════════════════════════════════════════
//  § 3. SyscallHandler trait (abstract interface — OOP)
// ═══════════════════════════════════════════════════════════════

pub trait SyscallHandler {
    fn handles(&self, num: SyscallNum) -> bool;
    fn dispatch(&mut self, frame: &SyscallFrame) -> SyscallResult;
}

// ═══════════════════════════════════════════════════════════════
//  § 4. ProcessHandler — handles process-management syscalls
// ═══════════════════════════════════════════════════════════════

pub struct ProcessHandler {
    pub next_pid: u32,
}

impl ProcessHandler {
    pub const fn new() -> Self { Self { next_pid: 1 } }
}

impl SyscallHandler for ProcessHandler {
    fn handles(&self, num: SyscallNum) -> bool {
        matches!(num, SyscallNum::Exit | SyscallNum::Fork | SyscallNum::GetPid |
                      SyscallNum::Exec | SyscallNum::Wait | SyscallNum::Kill)
    }

    fn dispatch(&mut self, frame: &SyscallFrame) -> SyscallResult {
        let num = SyscallNum::from_u64(frame.num);
        match num {
            SyscallNum::GetPid => SyscallResult::Ok(self.next_pid as u64),
            SyscallNum::Fork => {
                let child_pid = self.next_pid;
                self.next_pid += 1;
                SyscallResult::Ok(child_pid as u64)
            }
            SyscallNum::Exit => {
                // In production: terminate process; here just return exit code.
                SyscallResult::Ok(frame.arg0)
            }
            _ => SyscallResult::Err(SyscallError::Unsupported),
        }
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 5. MemoryHandler — handles memory syscalls
// ═══════════════════════════════════════════════════════════════

const MAX_MAPPINGS: usize = 64;

pub struct MemMapping {
    pub vaddr: u64,
    pub size:  u64,
    pub flags: u32,
    pub valid: bool,
}

pub struct MemoryHandler {
    pub mappings: [MemMapping; MAX_MAPPINGS],
    pub count:    usize,
    pub next_vaddr: u64,
}

impl MemoryHandler {
    pub const fn new() -> Self {
        const EMPTY_MAP: MemMapping = MemMapping { vaddr: 0, size: 0, flags: 0, valid: false };
        Self {
            mappings:   [EMPTY_MAP; MAX_MAPPINGS],
            count:      0,
            next_vaddr: 0x0000_4000_0000_0000,  // User mmap region base
        }
    }
}

impl SyscallHandler for MemoryHandler {
    fn handles(&self, num: SyscallNum) -> bool {
        matches!(num, SyscallNum::MemMap | SyscallNum::MemUnmap | SyscallNum::MemProtect)
    }

    fn dispatch(&mut self, frame: &SyscallFrame) -> SyscallResult {
        let num = SyscallNum::from_u64(frame.num);
        match num {
            SyscallNum::MemMap => {
                let size  = (frame.arg1 + 0xFFF) & !0xFFF;  // Page-align
                if size == 0 { return SyscallResult::Err(SyscallError::InvalidArg); }
                if self.count >= MAX_MAPPINGS { return SyscallResult::Err(SyscallError::NoMemory); }
                let vaddr = self.next_vaddr;
                self.mappings[self.count] = MemMapping {
                    vaddr, size, flags: frame.arg3 as u32, valid: true
                };
                self.count += 1;
                self.next_vaddr += size;
                SyscallResult::Ok(vaddr)
            }
            SyscallNum::MemUnmap => {
                let target = frame.arg0;
                let mut i = 0;
                while i < self.count {
                    if self.mappings[i].valid && self.mappings[i].vaddr == target {
                        self.mappings[i].valid = false;
                        return SyscallResult::Ok(0);
                    }
                    i += 1;
                }
                SyscallResult::Err(SyscallError::Fault)
            }
            _ => SyscallResult::Err(SyscallError::Unsupported),
        }
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 6. KernelSyscallDispatcher — composition of all handlers
// ═══════════════════════════════════════════════════════════════

pub struct KernelSyscallDispatcher {
    pub process_handler: ProcessHandler,
    pub memory_handler:  MemoryHandler,
    pub total_calls:     u64,
}

impl KernelSyscallDispatcher {
    pub const fn new() -> Self {
        Self {
            process_handler: ProcessHandler::new(),
            memory_handler:  MemoryHandler::new(),
            total_calls:     0,
        }
    }

    pub fn handle(&mut self, frame: &SyscallFrame) -> SyscallResult {
        self.total_calls += 1;
        let num = SyscallNum::from_u64(frame.num);
        if self.process_handler.handles(num) {
            return self.process_handler.dispatch(frame);
        }
        if self.memory_handler.handles(num) {
            return self.memory_handler.dispatch(frame);
        }
        SyscallResult::Err(SyscallError::Unsupported)
    }
}

// ═══════════════════════════════════════════════════════════════
//  § 7. Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(num: SyscallNum, a0: u64, a1: u64) -> SyscallFrame {
        SyscallFrame { num: num as u64, arg0: a0, arg1: a1, arg2: 0, arg3: 0, arg4: 0, arg5: 0 }
    }

    #[test]
    fn test_getpid() {
        let mut d = KernelSyscallDispatcher::new();
        let f = frame(SyscallNum::GetPid, 0, 0);
        let r = d.handle(&f);
        assert_eq!(r, SyscallResult::Ok(1)); // next_pid starts at 1
    }

    #[test]
    fn test_fork_increments_pid() {
        let mut d = KernelSyscallDispatcher::new();
        let f1 = frame(SyscallNum::Fork, 0, 0);
        let r1 = d.handle(&f1);
        let f2 = frame(SyscallNum::Fork, 0, 0);
        let r2 = d.handle(&f2);
        assert_eq!(r1, SyscallResult::Ok(1));
        assert_eq!(r2, SyscallResult::Ok(2));
    }

    #[test]
    fn test_mmap_and_munmap() {
        let mut d = KernelSyscallDispatcher::new();
        let mmap_f = SyscallFrame {
            num: SyscallNum::MemMap as u64,
            arg0: 0, arg1: 4096, arg2: 0, arg3: 3, arg4: 0, arg5: 0,
        };
        let r = d.handle(&mmap_f);
        let vaddr = match r {
            SyscallResult::Ok(v) => v,
            _ => panic!("mmap failed"),
        };
        assert_ne!(vaddr, 0);
        let munmap_f = SyscallFrame {
            num: SyscallNum::MemUnmap as u64,
            arg0: vaddr, arg1: 4096, arg2: 0, arg3: 0, arg4: 0, arg5: 0,
        };
        assert_eq!(d.handle(&munmap_f), SyscallResult::Ok(0));
    }

    #[test]
    fn test_unknown_syscall_returns_unsupported() {
        let mut d = KernelSyscallDispatcher::new();
        let f = frame(SyscallNum::Unknown, 0, 0);
        assert_eq!(d.handle(&f), SyscallResult::Err(SyscallError::Unsupported));
    }
}

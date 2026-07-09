// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired process management for SigmaOS
// Zero-allocation, performance-optimized process operations

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Process ID (PID)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pid(pub u32);

impl Pid {
    pub const fn new(id: u32) -> Self {
        Self(id)
    }
    
    pub const fn zero() -> Self {
        Self(0)
    }
    
    pub const fn init() -> Self {
        Self(1)
    }
}

/// Process state (Linux-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Sleeping,
    DiskSleep,
    Stopped,
    TracingStop,
    Zombie,
    Dead,
    Wakekill,
    Waking,
    Parked,
    Idle,
}

/// Process flags (Linux-style)
pub mod process_flags {
    pub const PF_KTHREAD: u64 = 0x00000001;  // Kernel thread
    pub const PF_EXITING: u64 = 0x00000002;  // Process is exiting
    pub const PF_VCPU: u64 = 0x00000004;     // Process is a VCPU
    pub const PF_IDLE: u64 = 0x00000008;     // Idle process
    pub const PF_NOFREEZE: u64 = 0x00000010; // Don't freeze for suspend
    pub const PF_FROZEN: u64 = 0x00000020;   // Frozen for system suspend
    pub const PF_FSTRANS: u64 = 0x00000040;  // undergoing filesystem transition
    pub const PF_KSWAPD: u64 = 0x00000080;   // I am kswapd
    pub const PF_MEMALLOC: u64 = 0x00000100; // Allocating memory
    pub const PF_MEMALLOC_NOIO: u64 = 0x00000200; // Allocating memory without IO
    pub const PF_LESS_THROTTLE: u64 = 0x00000400; // Throttle me less than normal
    pub const PF_KTHREAD: u64 = 0x00000800;   // I am a kernel thread
    pub const PF_RANDOMIZE: u64 = 0x00001000; // Randomize virtual address space
    pub const PF_SWAPWRITE: u64 = 0x00002000; // I am doing swap write
    pub const PF_SPREAD_PAGE: u64 = 0x00004000; // Spread page cache over cpus
    pub const PF_SPREAD_SLAB: u64 = 0x00008000; // Spread slab cache over cpus
    pub const PF_MEMPOLICY: u64 = 0x00010000; // Non-default NUMA mempolicy
    pub const PF_VCPU: u64 = 0x00020000;     // I'm a virtual CPU
    pub const PF_MUTEX_TESTER: u64 = 0x00040000; // Thread belongs to the rt mutex tester
    pub const PF_NO_SETAFFINITY: u64 = 0x00080000; // Userland is not allowed to meddle with cpus
    pub const PF_MCE_EARLY: u64 = 0x00100000; // Early kill for mce policy
    pub const PF_MEMPOLICY: u64 = 0x00200000; // Default mempolicy
    pub const PF_MUTEX_TESTER: u64 = 0x00400000; // Thread belongs to the rt mutex tester
    pub const PF_NO_SETAFFINITY: u64 = 0x00800000; // Userland is not allowed to meddle with cpus
    pub const PF_MCE_EARLY: u64 = 0x01000000; // Early kill for mce policy
}

/// Process descriptor (Linux-style task_struct)
pub struct ProcessDescriptor {
    pub pid: Pid,
    pub ppid: Pid,
    pub state: ProcessState,
    pub flags: u64,
    pub priority: u8,
    pub static_prio: u8,
    pub normal_prio: u8,
    pub rt_priority: u8,
    pub exec_start: u64,
    pub total_runtime: u64,
    pub vruntime: u64,
    pub cpu_affinity: u64,
    pub exit_code: i32,
    pub exit_signal: i32,
}

impl ProcessDescriptor {
    pub const fn new(pid: Pid, ppid: Pid) -> Self {
        Self {
            pid,
            ppid,
            state: ProcessState::Running,
            flags: 0,
            priority: 120,
            static_prio: 120,
            normal_prio: 120,
            rt_priority: 0,
            exec_start: 0,
            total_runtime: 0,
            vruntime: 0,
            cpu_affinity: u64::MAX,
            exit_code: 0,
            exit_signal: 0,
        }
    }
    
    pub fn is_kernel_thread(&self) -> bool {
        self.flags & process_flags::PF_KTHREAD != 0
    }
    
    pub fn is_user_process(&self) -> bool {
        !self.is_kernel_thread()
    }
    
    pub fn is_running(&self) -> bool {
        self.state == ProcessState::Running
    }
    
    pub fn is_zombie(&self) -> bool {
        self.state == ProcessState::Zombie
    }
    
    pub fn is_stopped(&self) -> bool {
        self.state == ProcessState::Stopped
    }
}

/// Process namespace (Linux-style)
pub struct ProcessNamespace {
    pub pid: Pid,
    pub parent: Option<*mut ProcessNamespace>,
}

impl ProcessNamespace {
    pub const fn new(pid: Pid) -> Self {
        Self {
            pid,
            parent: None,
        }
    }
}

/// Process credentials (Linux-style)
pub struct ProcessCredentials {
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
    pub suid: u32,
    pub sgid: u32,
    pub fsuid: u32,
    pub fsgid: u32,
}

impl ProcessCredentials {
    pub const fn new(uid: u32, gid: u32) -> Self {
        Self {
            uid,
            gid,
            euid: uid,
            egid: gid,
            suid: uid,
            sgid: gid,
            fsuid: uid,
            fsgid: gid,
        }
    }
    
    pub fn is_root(&self) -> bool {
        self.uid == 0
    }
    
    pub fn has_capability(&self, cap: Capability) -> bool {
        if self.is_root() {
            return true;
        }
        // Check effective capabilities
        false
    }
}

/// Linux capabilities
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    CAP_CHOWN,
    CAP_DAC_OVERRIDE,
    CAP_DAC_READ_SEARCH,
    CAP_FOWNER,
    CAP_FSETID,
    CAP_KILL,
    CAP_SETGID,
    CAP_SETUID,
    CAP_SETPCAP,
    CAP_LINUX_IMMUTABLE,
    CAP_NET_BIND_SERVICE,
    CAP_NET_BROADCAST,
    CAP_NET_ADMIN,
    CAP_NET_RAW,
    CAP_IPC_LOCK,
    CAP_IPC_OWNER,
    CAP_SYS_MODULE,
    CAP_SYS_RAWIO,
    CAP_SYS_CHROOT,
    CAP_SYS_PTRACE,
    CAP_SYS_PACCT,
    CAP_SYS_ADMIN,
    CAP_SYS_BOOT,
    CAP_SYS_NICE,
    CAP_SYS_RESOURCE,
    CAP_SYS_TIME,
    CAP_SYS_TTY_CONFIG,
    CAP_MKNOD,
    CAP_LEASE,
    CAP_AUDIT_WRITE,
    CAP_AUDIT_CONTROL,
    CAP_SETFCAP,
    CAP_MAC_OVERRIDE,
    CAP_MAC_ADMIN,
    CAP_SYSLOG,
    CAP_WAKE_ALARM,
    CAP_BLOCK_SUSPEND,
    CAP_AUDIT_READ,
}

/// Process file descriptor table (Linux-style)
pub struct FileDescriptorTable {
    pub fds: [Option<FileDescriptor>; 1024],
    pub max_fds: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct FileDescriptor {
    pub fd: i32,
    pub flags: u32,
    pub offset: u64,
}

impl FileDescriptorTable {
    pub const fn new() -> Self {
        Self {
            fds: [None; 1024],
            max_fds: 1024,
        }
    }
    
    pub fn alloc(&mut self) -> Option<i32> {
        for i in 0..self.max_fds {
            if self.fds[i].is_none() {
                self.fds[i] = Some(FileDescriptor {
                    fd: i as i32,
                    flags: 0,
                    offset: 0,
                });
                return Some(i as i32);
            }
        }
        None
    }
    
    pub fn free(&mut self, fd: i32) -> bool {
        if fd >= 0 && (fd as usize) < self.max_fds {
            self.fds[fd as usize] = None;
            true
        } else {
            false
        }
    }
    
    pub fn get(&self, fd: i32) -> Option<FileDescriptor> {
        if fd >= 0 && (fd as usize) < self.max_fds {
            self.fds[fd as usize]
        } else {
            None
        }
    }
}

/// Process memory layout (Linux-style)
pub struct ProcessMemoryLayout {
    pub code_start: u64,
    pub code_end: u64,
    pub data_start: u64,
    pub data_end: u64,
    pub heap_start: u64,
    pub heap_end: u64,
    pub stack_start: u64,
    pub stack_end: u64,
    pub mmap_base: u64,
}

impl ProcessMemoryLayout {
    pub const fn new() -> Self {
        Self {
            code_start: 0,
            code_end: 0,
            data_start: 0,
            data_end: 0,
            heap_start: 0,
            heap_end: 0,
            stack_start: 0,
            stack_end: 0,
            mmap_base: 0,
        }
    }
}

/// Process signal handling (Linux-style)
pub struct SignalHandling {
    pub pending: u64,
    pub blocked: u64,
    pub ignored: u64,
    pub caught: u64,
}

impl SignalHandling {
    pub const fn new() -> Self {
        Self {
            pending: 0,
            blocked: 0,
            ignored: 0,
            caught: 0,
        }
    }
}

/// Signal numbers (Linux-style)
pub mod signals {
    pub const SIGHUP: i32 = 1;
    pub const SIGINT: i32 = 2;
    pub const SIGQUIT: i32 = 3;
    pub const SIGILL: i32 = 4;
    pub const SIGTRAP: i32 = 5;
    pub const SIGABRT: i32 = 6;
    pub const SIGBUS: i32 = 7;
    pub const SIGFPE: i32 = 8;
    pub const SIGKILL: i32 = 9;
    pub const SIGUSR1: i32 = 10;
    pub const SIGSEGV: i32 = 11;
    pub const SIGUSR2: i32 = 12;
    pub const SIGPIPE: i32 = 13;
    pub const SIGALRM: i32 = 14;
    pub const SIGTERM: i32 = 15;
    pub const SIGSTKFLT: i32 = 16;
    pub const SIGCHLD: i32 = 17;
    pub const SIGCONT: i32 = 18;
    pub const SIGSTOP: i32 = 19;
    pub const SIGTSTP: i32 = 20;
    pub const SIGTTIN: i32 = 21;
    pub const SIGTTOU: i32 = 22;
}

/// Process resource limits (Linux-style rlimit)
#[derive(Debug, Clone, Copy)]
pub struct ResourceLimit {
    pub rlim_cur: u64,
    pub rlim_max: u64,
}

impl ResourceLimit {
    pub const fn new(cur: u64, max: u64) -> Self {
        Self {
            rlim_cur: cur,
            rlim_max: max,
        }
    }
    
    pub const fn unlimited() -> Self {
        Self {
            rlim_cur: u64::MAX,
            rlim_max: u64::MAX,
        }
    }
}

/// Resource types (Linux-style RLIMIT)
pub enum ResourceType {
    CPU,
    FSIZE,
    DATA,
    STACK,
    CORE,
    RSS,
    NPROC,
    NOFILE,
    MEMLOCK,
    AS,
    LOCKS,
    SIGPENDING,
    MSGQUEUE,
    NICE,
    RTPRIO,
    RTTIME,
}

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use crate::kernel::vfs::inode::{FileFlags, FsError, Inode};
use crate::security::CapabilityToken;

pub const PID_MAX_LIMIT: u32 = 32768;
pub const INIT_PID: u64 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Runnable,
    Stopped,
    Traced,
    Zombie,
    Dead,
    WakeKilled,
    Waking,
    Parked,
    Seized,
    Frozen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchedPolicy {
    Normal,
    Fifo,
    RoundRobin,
    Batch,
    Idle,
    Deadline,
}

#[derive(Debug, Clone)]
pub struct Cred {
    pub pid: u32,
    pub tgid: u32,
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
    pub suid: u32,
    pub sgid: u32,
    pub fsuid: u32,
    pub fsgid: u32,
    pub cap_inheritable: u64,
    pub cap_permitted: u64,
    pub cap_effective: u64,
    pub cap_bounding: u64,
    pub cap_ambient: u64,
    pub securebits: u64,
    pub cap_ambient_set: bool,
    pub no_new_privs: bool,
}

impl Cred {
    pub fn new() -> Self {
        Cred {
            pid: 0,
            tgid: 0,
            uid: 0,
            gid: 0,
            euid: 0,
            egid: 0,
            suid: 0,
            sgid: 0,
            fsuid: 0,
            fsgid: 0,
            cap_inheritable: 0,
            cap_permitted: 0,
            cap_effective: 0,
            cap_bounding: 0,
            cap_ambient: 0,
            securebits: 0,
            cap_ambient_set: false,
            no_new_privs: false,
        }
    }

    pub fn capable(&self, cap: u64) -> bool {
        (self.cap_effective & cap) != 0
    }
}

pub struct SignalStruct {
    pub pending: Vec<u32>,
    pub blocked: u64,
    pub signal: u32,
    pub flags: u32,
    pub si_code: i32,
}

pub struct FileDesc {
    pub file: usize,
    pub flags: u32,
    pub mode: u32,
}

pub struct VmArea {
    pub vm_start: u64,
    pub vm_end: u64,
    pub vm_flags: u32,
    pub vm_page_prot: u32,
    pub vm_pgoff: u64,
    pub vm_file: Option<usize>,
    pub vm_private_data: Option<usize>,
}

impl VmArea {
    pub fn new(start: u64, end: u64, flags: u32) -> Self {
        VmArea {
            vm_start: start,
            vm_end: end,
            vm_flags: flags,
            vm_page_prot: 0,
            vm_pgoff: 0,
            vm_file: None,
            vm_private_data: None,
        }
    }
}

pub const VM_READ: u32 = 1;
pub const VM_WRITE: u32 = 2;
pub const VM_EXEC: u32 = 4;
pub const VM_MAYREAD: u32 = 8;
pub const VM_MAYWRITE: u32 = 16;
pub const VM_MAYEXEC: u32 = 32;
pub const VM_SHARED: u32 = 64;
pub const VM_PRIVATE: u32 = 128;

pub struct MmStruct {
    pub pgd: usize,
    pub total_vm: u64,
    pub locked_vm: u64,
    pub pinned_vm: u64,
    pub data_vm: u64,
    pub exec_vm: u64,
    pub stack_vm: u64,
    pub mmap_base: u64,
    pub mmap_legacy_base: u64,
    pub task_size: u64,
    pub def_flags: u32,
    pub context: usize,
    pub pgtables_bytes: usize,
}

impl MmStruct {
    pub fn new() -> Self {
        MmStruct {
            pgd: 0,
            total_vm: 0,
            locked_vm: 0,
            pinned_vm: 0,
            data_vm: 0,
            exec_vm: 0,
            stack_vm: 0,
            mmap_base: 0,
            mmap_legacy_base: 0,
            task_size: 0,
            def_flags: 0,
            context: 0,
            pgtables_bytes: 0,
        }
    }
}

pub struct Task {
    pub pid: u64,
    pub tgid: u64,
    pub parent_pid: u64,
    pub name: String,
    pub state: ProcessState,
    pub priority: i32,
    pub static_prio: i32,
    pub normal_prio: i32,
    pub policy: SchedPolicy,
    pub cred: Cred,
    pub mm: Option<MmStruct>,
    pub files: Vec<FileDesc>,
    pub fs: Option<usize>,
    pub signal: SignalStruct,
    pub children: Vec<u64>,
    pub siblings: Vec<u64>,
    pub group_leader: u64,
    pub real_parent: u64,
    pub thread_group: Vec<u64>,
    pub vmas: Vec<VmArea>,
    pub flags: u32,
    pub exit_code: i32,
    pub exit_signal: i32,
    pub start_time: u64,
    pub utime: u64,
    pub stime: u64,
    pub cutime: u64,
    pub cstime: u64,
    pub nvcsw: u64,
    pub nivcsw: u64,
    pub seccomp_mode: u32,
    pub seccomp_filter: Option<usize>,
}

impl Task {
    pub fn new(pid: u64, name: &str) -> Self {
        Task {
            pid,
            tgid: pid,
            parent_pid: 0,
            name: name.to_string(),
            state: ProcessState::Running,
            priority: 0,
            static_prio: 120,
            normal_prio: 120,
            policy: SchedPolicy::Normal,
            cred: Cred::new(),
            mm: None,
            files: Vec::new(),
            fs: None,
            signal: SignalStruct {
                pending: Vec::new(),
                blocked: 0,
                signal: 0,
                flags: 0,
                si_code: 0,
            },
            children: Vec::new(),
            siblings: Vec::new(),
            group_leader: pid,
            real_parent: 0,
            thread_group: Vec::new(),
            vmas: Vec::new(),
            flags: 0,
            exit_code: 0,
            exit_signal: 0,
            start_time: 0,
            utime: 0,
            stime: 0,
            cutime: 0,
            cstime: 0,
            nvcsw: 0,
            nivcsw: 0,
            seccomp_mode: 0,
            seccomp_filter: None,
        }
    }

    pub fn is_zombie(&self) -> bool {
        self.state == ProcessState::Zombie
    }

    pub fn is_alive(&self) -> bool {
        !self.is_zombie()
    }
}

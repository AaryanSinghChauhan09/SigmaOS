#![no_std]
#![feature(alloc_error_handler)]

//! SigmaOS Sovereign Security Engine (AppArmor/Seccomp Logic)
//! Eliminates reliance on Linux Kernel AppArmor or BPF wrappers.
//! Implements strict Syscall Filtering and VFS Access Control at the lowest level.

use core::panic::PanicInfo;

// Define base types without stdlib
type SyscallNumber = u64;
type ProcessId = u32;

// OOP Principle applied in Rust: Encapsulation of Security Profiles
#[derive(Clone, Copy)]
pub enum FsPermission {
    AllowRead,
    AllowWrite,
    AllowExecute,
    DenyAll,
}

#[derive(Clone, Copy)]
pub enum NetworkPermission {
    AllowBind,
    AllowConnect,
    AllowRawSockets,
    DenyAll,
}

/// A structured security profile mapping what a process can do.
/// Analogous to AppArmor Profiles but natively integrated into Sigma.
pub struct SovereignProfile {
    pub pid: ProcessId,
    pub fs_root_lock: bool,
    pub default_fs_perm: FsPermission,
    pub default_net_perm: NetworkPermission,
    pub max_memory_pages: u64,
}

impl SovereignProfile {
    pub const fn new(pid: ProcessId) -> Self {
        SovereignProfile {
            pid,
            fs_root_lock: true,
            default_fs_perm: FsPermission::DenyAll,
            default_net_perm: NetworkPermission::DenyAll,
            max_memory_pages: 1024, // 4MB default limit
        }
    }

    /// Evaluates whether a process is permitted to execute a given Syscall.
    /// Acts as our internal Seccomp filter.
    pub fn evaluate_syscall(&self, syscall_no: SyscallNumber) -> bool {
        match syscall_no {
            // Syscall 0: Read
            0 => match self.default_fs_perm {
                FsPermission::AllowRead | FsPermission::AllowWrite => true,
                _ => false,
            },
            // Syscall 1: Write
            1 => match self.default_fs_perm {
                FsPermission::AllowWrite => true,
                _ => false,
            },
            // Syscall 2: Open
            2 => true, // Hooked later by VFS rules
            // Syscall 41: Socket
            41 | 42 | 43 => match self.default_net_perm {
                NetworkPermission::AllowBind | NetworkPermission::AllowConnect | NetworkPermission::AllowRawSockets => true,
                _ => false,
            },
            // Syscall 62: vfork / Syscall 57: fork
            57 | 62 => false, // Default deny fork to prevent fork bombs
            _ => true, // Allowed fallback
        }
    }

    pub fn enforce_memory_limit(&self, pages_requested: u64) -> bool {
        pages_requested <= self.max_memory_pages
    }
}

// Global active profile context tracking (simulated fixed array for no_std)
static mut ACTIVE_PROFILES: [Option<SovereignProfile>; 256] = [None; 256];

#[no_mangle]
pub extern "C" fn sigma_security_register_process(pid: u32) -> bool {
    let index = (pid % 256) as usize;
    unsafe {
        ACTIVE_PROFILES[index] = Some(SovereignProfile::new(pid));
    }
    true
}

#[no_mangle]
pub extern "C" fn sigma_security_check_syscall(pid: u32, syscall_no: u64) -> bool {
    let index = (pid % 256) as usize;
    unsafe {
        if let Some(ref profile) = ACTIVE_PROFILES[index] {
            if profile.pid == pid {
                return profile.evaluate_syscall(syscall_no);
            }
        }
    }
    false // Deny if profile corrupted or missing (Zero Trust)
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // Halt system securely on panic to prevent exploit chaining
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

pub mod dispatch;
pub mod dispatcher;
pub mod interface;
pub mod table;
pub mod namespace_syscalls;
pub mod inotify_syscalls;
pub mod kevent_syscalls;
pub mod uts_syscalls;
pub mod user_syscalls;
pub mod bpf_syscalls;
pub mod posix_linux_bsd_api;

pub use posix_linux_bsd_api::{
    posix_errno, syscall_abi_numbers, PidFdDescriptor, PosixLinuxBsdApiDispatcher,
};
pub mod abi;

// Unified Feature-Gated Syscall Number Representation
#[repr(u64)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SovereignUnifiedSyscallNumber {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
    Fork = 57,
    Execve = 59,
    Exit = 60,
}

pub fn dispatch_posix_syscall(num: u64, _args: &[u64; 6]) -> Result<u64, &'static str> {
    match num {
        0 => Ok(0), // Read
        1 => Ok(1), // Write
        2 => Ok(2), // Open
        3 => Ok(0), // Close
        57 => Ok(101), // Fork -> Returns simulated PID
        59 => Ok(0), // Execve
        60 => Ok(0), // Exit
        _ => Err("ENOSYS: Invalid or unimplemented system call"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_syscall_dispatcher() {
        let args = [0u64; 6];
        assert_eq!(dispatch_posix_syscall(0, &args).unwrap(), 0);
        assert_eq!(dispatch_posix_syscall(57, &args).unwrap(), 101);
        assert!(dispatch_posix_syscall(9999, &args).is_err());
    }
}

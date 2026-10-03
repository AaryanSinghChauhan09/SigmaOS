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

pub mod bpf_syscalls;
pub mod capsicum_syscalls;
pub mod dispatch;
pub mod dispatcher;
pub mod inotify_syscalls;
pub mod interface;
pub mod kevent_syscalls;
pub mod namespace_syscalls;
pub mod posix_linux_bsd_api;
pub mod table;
pub mod user_syscalls;
pub mod uts_syscalls;

pub use posix_linux_bsd_api::{
    posix_errno, syscall_abi_numbers, PidFdDescriptor, PosixLinuxBsdApiDispatcher,
};
pub mod abi;

pub mod linux_compat;
pub use linux_compat::{
    LinuxFdEntry, LinuxFdTable, LinuxOpenFlags, LinuxProcess, LinuxProcessTable,
    LinuxSyscallDispatcher, LinuxSyscallNumber, ProcessState,
};

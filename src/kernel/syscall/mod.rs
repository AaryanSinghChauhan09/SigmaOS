/// SigmaOS Syscall module
pub mod table;
// dispatch.rs kept for legacy no_std ABI compatibility
// pub mod dispatch; // kept separate to avoid no_std conflict

pub use table::{
    SyscallTable, SyscallArgs, SyscallResult, SyscallError, SyscallNr,
    SyscallHandler,
};

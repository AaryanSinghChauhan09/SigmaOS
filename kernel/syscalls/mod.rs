// SPDX-License-Identifier: MIT
// System calls module - exports Phase G syscall components
pub mod syscall_dispatcher;
pub mod network_syscalls;
pub mod signal_syscalls;
pub mod integration;

pub use syscall_dispatcher::*;
pub use network_syscalls::{
    NetworkSyscalls, SocketAddr, SocketArgs, SockaddrIn, 
    AF_INET, AF_INET6, AF_UNIX,
    SOCK_STREAM, SOCK_DGRAM, SOCK_RAW,
};
pub use signal_syscalls::{
    SignalSyscalls, SigAction, SigInfo, SigaltStack,
    signals, sa_flags, SIG_DFL, SIG_IGN, SIG_HOLD,
};
pub use integration::{SyscallContext, SignalHandlerTable, SignalHandler};

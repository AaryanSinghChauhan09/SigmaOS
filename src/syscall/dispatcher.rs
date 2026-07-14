#![no_std]
#![no_main]

/// Custom Syscall Dispatcher for SigmaOS
/// Implements syscall handling without relying on Linux kernel syscalls
/// Uses capability-based access control

use core::ptr::{self, NonNull};
use core::sync::atomic::{AtomicUsize, Ordering};

/// Syscall numbers
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallNumber {
    // File operations
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
    
    // Process operations
    Fork = 12,
    Execve = 13,
    Exit = 14,
    Wait4 = 15,
    Kill = 16,
    Uname = 17,
    
    // Memory operations
    Brk = 18,
    
    // Network operations
    Socket = 19,
    Bind = 20,
    Connect = 21,
    Listen = 22,
    Accept = 23,
    Sendto = 24,
    Recvfrom = 25,
    
    // IPC operations
    Shmget = 26,
    Shmat = 27,
    Shmdt = 28,
    Semget = 29,
    Semop = 30,
    
    // Custom SigmaOS syscalls
    CapabilityCheck = 1000,
    CapabilityGrant = 1001,
    CapabilityRevoke = 1002,
}

/// Syscall result
#[repr(C)]
pub struct SyscallResult {
    pub value: isize,
    pub error: i32,
}

impl SyscallResult {
    pub fn success(value: isize) -> Self {
        SyscallResult {
            value,
            error: 0,
        }
    }

    pub fn error(error: i32) -> Self {
        SyscallResult {
            value: -1,
            error,
        }
    }
}

/// Syscall arguments
#[repr(C)]
pub struct SyscallArgs {
    pub arg0: usize,
    pub arg1: usize,
    pub arg2: usize,
    pub arg3: usize,
    pub arg4: usize,
    pub arg5: usize,
}

/// Capability for syscall access
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Capability {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub network: bool,
    pub ipc: bool,
}

impl Capability {
    pub fn new() -> Self {
        Capability {
            read: false,
            write: false,
            execute: false,
            network: false,
            ipc: false,
        }
    }

    pub fn full() -> Self {
        Capability {
            read: true,
            write: true,
            execute: true,
            network: true,
            ipc: true,
        }
    }
}

/// Syscall handler type
type SyscallHandler = unsafe fn(&SyscallArgs, Capability) -> SyscallResult;

/// Syscall table entry
#[repr(C)]
struct SyscallEntry {
    number: SyscallNumber,
    handler: SyscallHandler,
    required_capability: Capability,
}

/// Syscall dispatcher
pub struct SyscallDispatcher {
    syscall_table: [Option<SyscallEntry>; 256],
    call_count: [AtomicUsize; 256],
}

impl SyscallDispatcher {
    pub fn new() -> Self {
        let mut dispatcher = SyscallDispatcher {
            syscall_table: [None; 256],
            call_count: [AtomicUsize::new(0); 256],
        };

        // Register syscall handlers
        dispatcher.register_syscall(SyscallNumber::Read, handle_read, Capability { read: true, write: false, execute: false, network: false, ipc: false });
        dispatcher.register_syscall(SyscallNumber::Write, handle_write, Capability { read: false, write: true, execute: false, network: false, ipc: false });
        dispatcher.register_syscall(SyscallNumber::Open, handle_open, Capability { read: true, write: true, execute: false, network: false, ipc: false });
        dispatcher.register_syscall(SyscallNumber::Close, handle_close, Capability { read: true, write: true, execute: false, network: false, ipc: false });
        dispatcher.register_syscall(SyscallNumber::Exit, handle_exit, Capability::full());
        dispatcher.register_syscall(SyscallNumber::Fork, handle_fork, Capability::full());
        dispatcher.register_syscall(SyscallNumber::Execve, handle_execve, Capability { read: true, write: false, execute: true, network: false, ipc: false });
        dispatcher.register_syscall(SyscallNumber::Socket, handle_socket, Capability { read: false, write: false, execute: false, network: true, ipc: false });
        dispatcher.register_syscall(SyscallNumber::Connect, handle_connect, Capability { read: false, write: false, execute: false, network: true, ipc: false });

        dispatcher
    }

    fn register_syscall(&mut self, number: SyscallNumber, handler: SyscallHandler, required_capability: Capability) {
        let index = number as usize;
        if index < 256 {
            self.syscall_table[index] = Some(SyscallEntry {
                number,
                handler,
                required_capability,
            });
        }
    }

    /// Dispatch syscall
    pub unsafe fn dispatch(&self, number: SyscallNumber, args: &SyscallArgs, caller_capability: Capability) -> SyscallResult {
        let index = number as usize;

        if index >= 256 {
            return SyscallResult::error(-1); // ENOSYS
        }

        let entry = match self.syscall_table[index] {
            Some(ref entry) => entry,
            None => return SyscallResult::error(-1), // ENOSYS
        };

        // Check capability
        if !self.check_capability(&entry.required_capability, caller_capability) {
            return SyscallResult::error(-13); // EACCES
        }

        // Increment call count
        self.call_count[index].fetch_add(1, Ordering::SeqCst);

        // Call handler
        (entry.handler)(args, caller_capability)
    }

    fn check_capability(&self, required: &Capability, caller: Capability) -> bool {
        (required.read || !caller.read) &&
        (required.write || !caller.write) &&
        (required.execute || !caller.execute) &&
        (required.network || !caller.network) &&
        (required.ipc || !caller.ipc)
    }

    /// Get syscall statistics
    pub fn get_stats(&self, number: SyscallNumber) -> usize {
        let index = number as usize;
        if index < 256 {
            self.call_count[index].load(Ordering::SeqCst)
        } else {
            0
        }
    }
}

/// Syscall handlers
unsafe fn handle_read(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    let fd = args.arg0;
    let buffer = args.arg1 as *mut u8;
    let size = args.arg2;

    // In a real implementation, this would call the file I/O system
    // For now, return success
    SyscallResult::success(size as isize)
}

unsafe fn handle_write(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    let fd = args.arg0;
    let buffer = args.arg1 as *const u8;
    let size = args.arg2;

    // In a real implementation, this would call the file I/O system
    // For now, return success
    SyscallResult::success(size as isize)
}

unsafe fn handle_open(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    let path = args.arg0 as *const u8;
    let flags = args.arg1;
    let mode = args.arg2;

    // In a real implementation, this would call the file I/O system
    // For now, return a fake fd
    SyscallResult::success(3)
}

unsafe fn handle_close(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    let fd = args.arg0;

    // In a real implementation, this would call the file I/O system
    // For now, return success
    SyscallResult::success(0)
}

unsafe fn handle_exit(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    let exit_code = args.arg0 as i32;

    // In a real implementation, this would terminate the process
    // For now, loop forever
    loop {}
}

unsafe fn handle_fork(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    // In a real implementation, this would fork the process
    // For now, return 0 (child)
    SyscallResult::success(0)
}

unsafe fn handle_execve(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    let path = args.arg0 as *const u8;
    let argv = args.arg1 as *const *const u8;
    let envp = args.arg2 as *const *const u8;

    // In a real implementation, this would execute the program
    // For now, return error
    SyscallResult::error(-1)
}

unsafe fn handle_socket(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    let domain = args.arg0;
    let type_ = args.arg1;
    let protocol = args.arg2;

    // In a real implementation, this would create a socket
    // For now, return a fake fd
    SyscallResult::success(4)
}

unsafe fn handle_connect(args: &SyscallArgs, _capability: Capability) -> SyscallResult {
    let fd = args.arg0;
    let addr = args.arg1 as *const u8;
    let addrlen = args.arg2;

    // In a real implementation, this would connect the socket
    // For now, return success
    SyscallResult::success(0)
}

/// Global syscall dispatcher
static mut GLOBAL_DISPATCHER: Option<SyscallDispatcher> = None;

/// Initialize syscall dispatcher
pub unsafe fn init_syscall_dispatcher() {
    GLOBAL_DISPATCHER = Some(SyscallDispatcher::new());
}

/// Make a syscall
pub unsafe fn syscall(number: SyscallNumber, args: &SyscallArgs, capability: Capability) -> SyscallResult {
    if let Some(ref dispatcher) = GLOBAL_DISPATCHER {
        dispatcher.dispatch(number, args, capability)
    } else {
        SyscallResult::error(-1)
    }
}

/// Get syscall statistics
pub unsafe fn get_syscall_stats(number: SyscallNumber) -> usize {
    if let Some(ref dispatcher) = GLOBAL_DISPATCHER {
        dispatcher.get_stats(number)
    } else {
        0
    }
}

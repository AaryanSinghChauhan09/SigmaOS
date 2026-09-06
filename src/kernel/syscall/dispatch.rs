#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! SigmaOS Syscall Dispatch
//! 30+ Essential Syscalls with pledge/unveil and PQC syscalls
//! Core syscalls: read, write, open, close, mmap, munmap, fork, execve, exit, etc.


pub mod ioctl_helper;

use core::sync::atomic::{AtomicUsize, Ordering};
use ioctl_helper::{IoctlDecoder, IoctlDirection};

#[repr(C)]
pub struct SyscallDispatcher {
    syscall_table: [SyscallHandler; 256],
    call_count: AtomicUsize,
    error_count: AtomicUsize,
}

#[repr(C)]
pub struct SyscallHandler {
    number: AtomicUsize,
    handler: AtomicUsize, // Function pointer
    name: [u8; 32],
}

#[repr(C)]
pub struct SyscallContext {
    syscall_number: AtomicUsize,
    args: [AtomicUsize; 6],
    return_value: AtomicUsize,
    error_code: AtomicUsize,
}

// Syscall numbers
pub const SYS_READ: usize = 0;
pub const SYS_WRITE: usize = 1;
pub const SYS_OPEN: usize = 2;
pub const SYS_CLOSE: usize = 3;
pub const SYS_MMAP: usize = 4;
pub const SYS_MUNMAP: usize = 5;
pub const SYS_FORK: usize = 6;
pub const SYS_EXECVE: usize = 7;
pub const SYS_EXIT: usize = 8;
pub const SYS_WAITPID: usize = 9;
pub const SYS_GETPID: usize = 10;
pub const SYS_SOCKET: usize = 11;
pub const SYS_CONNECT: usize = 12;
pub const SYS_BIND: usize = 13;
pub const SYS_LISTEN: usize = 14;
pub const SYS_ACCEPT: usize = 15;
pub const SYS_SEND: usize = 16;
pub const SYS_RECV: usize = 17;
pub const SYS_SHUTDOWN: usize = 18;
pub const SYS_PLEDGE: usize = 19;
pub const SYS_UNVEIL: usize = 20;
pub const SYS_PQC_KEYGEN: usize = 21;
pub const SYS_PQC_SIGN: usize = 22;
pub const SYS_PQC_VERIFY: usize = 23;
pub const SYS_GETTIME: usize = 24;
pub const SYS_NANOSLEEP: usize = 25;
pub const SYS_MKDIR: usize = 26;
pub const SYS_RMDIR: usize = 27;
pub const SYS_UNLINK: usize = 28;
pub const SYS_RENAME: usize = 29;
pub const SYS_CHMOD: usize = 30;
pub const SYS_IOCTL: usize = 31;

impl SyscallDispatcher {
    pub fn new() -> Self {
        let mut dispatcher = SyscallDispatcher {
            syscall_table: core::array::from_fn(|i| SyscallHandler::new(i)),
            call_count: AtomicUsize::new(0),
            error_count: AtomicUsize::new(0),
        };

        // Register core syscalls
        dispatcher.register(SYS_READ, sys_read as usize);
        dispatcher.register(SYS_WRITE, sys_write as usize);
        dispatcher.register(SYS_OPEN, sys_open as usize);
        dispatcher.register(SYS_CLOSE, sys_close as usize);
        dispatcher.register(SYS_MMAP, sys_mmap as usize);
        dispatcher.register(SYS_MUNMAP, sys_munmap as usize);
        dispatcher.register(SYS_FORK, sys_fork as usize);
        dispatcher.register(SYS_EXECVE, sys_execve as usize);
        dispatcher.register(SYS_EXIT, sys_exit as usize);
        dispatcher.register(SYS_WAITPID, sys_waitpid as usize);
        dispatcher.register(SYS_GETPID, sys_getpid as usize);
        dispatcher.register(SYS_SOCKET, sys_socket as usize);
        dispatcher.register(SYS_CONNECT, sys_connect as usize);
        dispatcher.register(SYS_BIND, sys_bind as usize);
        dispatcher.register(SYS_LISTEN, sys_listen as usize);
        dispatcher.register(SYS_ACCEPT, sys_accept as usize);
        dispatcher.register(SYS_SEND, sys_send as usize);
        dispatcher.register(SYS_RECV, sys_recv as usize);
        dispatcher.register(SYS_SHUTDOWN, sys_shutdown as usize);
        dispatcher.register(SYS_PLEDGE, sys_pledge as usize);
        dispatcher.register(SYS_UNVEIL, sys_unveil as usize);
        dispatcher.register(SYS_PQC_KEYGEN, sys_pqc_keygen as usize);
        dispatcher.register(SYS_PQC_SIGN, sys_pqc_sign as usize);
        dispatcher.register(SYS_PQC_VERIFY, sys_pqc_verify as usize);
        dispatcher.register(SYS_GETTIME, sys_gettime as usize);
        dispatcher.register(SYS_NANOSLEEP, sys_nanosleep as usize);
        dispatcher.register(SYS_MKDIR, sys_mkdir as usize);
        dispatcher.register(SYS_RMDIR, sys_rmdir as usize);
        dispatcher.register(SYS_UNLINK, sys_unlink as usize);
        dispatcher.register(SYS_RENAME, sys_rename as usize);
        dispatcher.register(SYS_CHMOD, sys_chmod as usize);
        dispatcher.register(SYS_IOCTL, sys_ioctl as usize);

        dispatcher
    }

    /// Register syscall handler
    pub fn register(&mut self, number: usize, handler: usize) {
        if number < 256 {
            self.syscall_table[number].handler.store(handler, Ordering::SeqCst);
        }
    }

    /// Dispatch syscall
    pub fn dispatch(&self, ctx: &mut SyscallContext) -> isize {
        let number = ctx.syscall_number.load(Ordering::SeqCst);
        self.call_count.fetch_add(1, Ordering::SeqCst);

        if number >= 256 {
            ctx.error_code.store(usize::MAX, Ordering::SeqCst);
            self.error_count.fetch_add(1, Ordering::SeqCst);
            return -1;
        }

        let handler = self.syscall_table[number].handler.load(Ordering::Acquire);
        if handler == 0 {
            ctx.error_code.store(usize::MAX, Ordering::SeqCst);
            self.error_count.fetch_add(1, Ordering::SeqCst);
            return -1;
        }

        let result = self.default_handler(ctx, number);
        
        if result < 0 {
            self.error_count.fetch_add(1, Ordering::SeqCst);
        }
        
        result
    }

    fn default_handler(&self, ctx: &mut SyscallContext, number: usize) -> isize {
        // Default stub implementation
        match number {
            SYS_EXIT => {
                // Exit syscall - no return
                0
            }
            SYS_GETPID => {
                // Return fake PID
                ctx.return_value.store(1, Ordering::SeqCst);
                1
            }
            SYS_IOCTL => {
                // Emulate direct ioctl dispatching
                let _fd = ctx.get_arg(0);
                let request = ctx.get_arg(1) as u32;
                let arg_ptr = ctx.get_arg(2);

                // Safe boundary check (arbitrary 16MB user limit for stub simulation)
                if IoctlDecoder::validate_parameter_bounds(request, arg_ptr, 0x1000000).is_err() {
                    ctx.error_code.store(14, Ordering::SeqCst); // EFAULT (Bad Address)
                    return -1;
                }

                // Stub success: return 0
                ctx.return_value.store(0, Ordering::SeqCst);
                0
            }
            _ => {
                ctx.error_code.store(38, Ordering::SeqCst); // ENOSYS
                -1
            }
        }
    }

    /// Get syscall call count
    pub fn call_count(&self) -> usize {
        self.call_count.load(Ordering::SeqCst)
    }

    /// Get syscall error count
    pub fn error_count(&self) -> usize {
        self.error_count.load(Ordering::SeqCst)
    }
}

impl SyscallHandler {
    pub const fn new(number: usize) -> Self {
        SyscallHandler {
            number: AtomicUsize::new(number),
            handler: AtomicUsize::new(0),
            name: [0; 32],
        }
    }

    pub fn number(&self) -> usize {
        self.number.load(Ordering::SeqCst)
    }
}

impl SyscallContext {
    pub fn new(number: usize, args: [usize; 6]) -> Self {
        SyscallContext {
            syscall_number: AtomicUsize::new(number),
            args: core::array::from_fn(|i| AtomicUsize::new(args[i])),
            return_value: AtomicUsize::new(0),
            error_code: AtomicUsize::new(0),
        }
    }

    pub fn get_arg(&self, index: usize) -> usize {
        if index < 6 {
            self.args[index].load(Ordering::SeqCst)
        } else {
            0
        }
    }

    pub fn set_return(&self, value: usize) {
        self.return_value.store(value, Ordering::SeqCst);
    }

    pub fn get_return(&self) -> usize {
        self.return_value.load(Ordering::SeqCst)
    }

    pub fn set_error(&self, error: usize) {
        self.error_code.store(error, Ordering::SeqCst);
    }

    pub fn get_error(&self) -> usize {
        self.error_code.load(Ordering::SeqCst)
    }
}

// Syscall implementations (stubs)
extern "C" fn sys_read(ctx: &mut SyscallContext) -> isize {
    let fd = ctx.get_arg(0);
    let buf = ctx.get_arg(1);
    let count = ctx.get_arg(2);
    
    // Stub: return count as if read succeeded
    ctx.set_return(count);
    count as isize
}

extern "C" fn sys_write(ctx: &mut SyscallContext) -> isize {
    let fd = ctx.get_arg(0);
    let buf = ctx.get_arg(1);
    let count = ctx.get_arg(2);
    
    // Stub: return count as if write succeeded
    ctx.set_return(count);
    count as isize
}

extern "C" fn sys_open(ctx: &mut SyscallContext) -> isize {
    let pathname = ctx.get_arg(0);
    let flags = ctx.get_arg(1);
    let mode = ctx.get_arg(2);
    
    // Stub: return fd 3
    ctx.set_return(3);
    3
}

extern "C" fn sys_close(ctx: &mut SyscallContext) -> isize {
    let fd = ctx.get_arg(0);
    0
}

extern "C" fn sys_mmap(ctx: &mut SyscallContext) -> isize {
    let addr = ctx.get_arg(0);
    let length = ctx.get_arg(1);
    let prot = ctx.get_arg(2);
    let flags = ctx.get_arg(3);
    let fd = ctx.get_arg(4);
    let offset = ctx.get_arg(5);
    
    // Stub: return fake address
    ctx.set_return(0x10000000);
    0x10000000 as isize
}

extern "C" fn sys_munmap(ctx: &mut SyscallContext) -> isize {
    let addr = ctx.get_arg(0);
    let length = ctx.get_arg(1);
    0
}

extern "C" fn sys_fork(ctx: &mut SyscallContext) -> isize {
    // Stub: return child PID
    ctx.set_return(2);
    2
}

extern "C" fn sys_execve(ctx: &mut SyscallContext) -> isize {
    let pathname = ctx.get_arg(0);
    let argv = ctx.get_arg(1);
    let envp = ctx.get_arg(2);
    0
}

extern "C" fn sys_exit(ctx: &mut SyscallContext) -> isize {
    let status = ctx.get_arg(0);
    // Exit - no return
    0
}

extern "C" fn sys_waitpid(ctx: &mut SyscallContext) -> isize {
    let pid = ctx.get_arg(0);
    let status = ctx.get_arg(1);
    let options = ctx.get_arg(2);
    
    // Stub: return PID
    ctx.set_return(pid);
    pid as isize
}

extern "C" fn sys_getpid(ctx: &mut SyscallContext) -> isize {
    // Return current PID
    ctx.set_return(1);
    1
}

extern "C" fn sys_socket(ctx: &mut SyscallContext) -> isize {
    let domain = ctx.get_arg(0);
    let type_ = ctx.get_arg(1);
    let protocol = ctx.get_arg(2);
    
    // Stub: return fd 4
    ctx.set_return(4);
    4
}

extern "C" fn sys_connect(ctx: &mut SyscallContext) -> isize {
    let sockfd = ctx.get_arg(0);
    let addr = ctx.get_arg(1);
    let addrlen = ctx.get_arg(2);
    0
}

extern "C" fn sys_bind(ctx: &mut SyscallContext) -> isize {
    let sockfd = ctx.get_arg(0);
    let addr = ctx.get_arg(1);
    let addrlen = ctx.get_arg(2);
    0
}

extern "C" fn sys_listen(ctx: &mut SyscallContext) -> isize {
    let sockfd = ctx.get_arg(0);
    let backlog = ctx.get_arg(1);
    0
}

extern "C" fn sys_accept(ctx: &mut SyscallContext) -> isize {
    let sockfd = ctx.get_arg(0);
    let addr = ctx.get_arg(1);
    let addrlen = ctx.get_arg(2);
    
    // Stub: return new fd
    ctx.set_return(5);
    5
}

extern "C" fn sys_send(ctx: &mut SyscallContext) -> isize {
    let sockfd = ctx.get_arg(0);
    let buf = ctx.get_arg(1);
    let len = ctx.get_arg(2);
    let flags = ctx.get_arg(3);
    
    ctx.set_return(len);
    len as isize
}

extern "C" fn sys_recv(ctx: &mut SyscallContext) -> isize {
    let sockfd = ctx.get_arg(0);
    let buf = ctx.get_arg(1);
    let len = ctx.get_arg(2);
    let flags = ctx.get_arg(3);
    
    ctx.set_return(len);
    len as isize
}

extern "C" fn sys_shutdown(ctx: &mut SyscallContext) -> isize {
    let sockfd = ctx.get_arg(0);
    let how = ctx.get_arg(1);
    0
}

extern "C" fn sys_pledge(ctx: &mut SyscallContext) -> isize {
    let promises = ctx.get_arg(0);
    let execpromises = ctx.get_arg(1);
    0
}

extern "C" fn sys_unveil(ctx: &mut SyscallContext) -> isize {
    let path = ctx.get_arg(0);
    let permissions = ctx.get_arg(1);
    0
}

extern "C" fn sys_pqc_keygen(ctx: &mut SyscallContext) -> isize {
    let algorithm = ctx.get_arg(0);
    let key_out = ctx.get_arg(1);
    let key_len = ctx.get_arg(2);
    0
}

extern "C" fn sys_pqc_sign(ctx: &mut SyscallContext) -> isize {
    let key = ctx.get_arg(0);
    let msg = ctx.get_arg(1);
    let msg_len = ctx.get_arg(2);
    let sig_out = ctx.get_arg(3);
    let sig_len = ctx.get_arg(4);
    0
}

extern "C" fn sys_pqc_verify(ctx: &mut SyscallContext) -> isize {
    let key = ctx.get_arg(0);
    let msg = ctx.get_arg(1);
    let msg_len = ctx.get_arg(2);
    let sig = ctx.get_arg(3);
    let sig_len = ctx.get_arg(4);
    1 // Valid
}

extern "C" fn sys_gettime(ctx: &mut SyscallContext) -> isize {
    let ts = ctx.get_arg(0);
    let tz = ctx.get_arg(1);
    0
}

extern "C" fn sys_nanosleep(ctx: &mut SyscallContext) -> isize {
    let req = ctx.get_arg(0);
    let rem = ctx.get_arg(1);
    0
}

extern "C" fn sys_mkdir(ctx: &mut SyscallContext) -> isize {
    let pathname = ctx.get_arg(0);
    let mode = ctx.get_arg(1);
    0
}

extern "C" fn sys_rmdir(ctx: &mut SyscallContext) -> isize {
    let pathname = ctx.get_arg(0);
    0
}

extern "C" fn sys_unlink(ctx: &mut SyscallContext) -> isize {
    let pathname = ctx.get_arg(0);
    0
}

extern "C" fn sys_rename(ctx: &mut SyscallContext) -> isize {
    let oldpath = ctx.get_arg(0);
    let newpath = ctx.get_arg(1);
    0
}

extern "C" fn sys_chmod(ctx: &mut SyscallContext) -> isize {
    let pathname = ctx.get_arg(0);
    let mode = ctx.get_arg(1);
    0
}

extern "C" fn sys_ioctl(ctx: &mut SyscallContext) -> isize {
    let fd = ctx.get_arg(0);
    let request = ctx.get_arg(1);
    let argp = ctx.get_arg(2);
    0
}

pub const IA32_STAR_MSR: u32 = 0xC0000081;
pub const IA32_LSTAR_MSR: u32 = 0xC0000082;
pub const IA32_FMASK_MSR: u32 = 0xC0000084;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct FastSyscallRegisters {
    pub rax: u64, // Syscall number & return value
    pub rdi: u64, // Arg 1
    pub rsi: u64, // Arg 2
    pub rdx: u64, // Arg 3
    pub r10: u64, // Arg 4 (Linux fast syscall convention)
    pub r8:  u64, // Arg 5
    pub r9:  u64, // Arg 6
    pub rcx: u64, // Saved RIP (overwritten by hardware `syscall`)
    pub r11: u64, // Saved RFLAGS (overwritten by hardware `syscall`)
    pub rbx: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

pub struct FastSyscallTrampoline {
    pub msr_star: u64,
    pub msr_lstar: u64,
    pub msr_fmask: u64,
    pub trampoline_entry_address: u64,
}

impl FastSyscallTrampoline {
    pub fn new(trampoline_entry: u64, kernel_cs: u16, user_cs: u16) -> Self {
        // IA32_STAR format: [63:48] = Sysret CS/SS, [47:32] = Syscall CS/SS
        let star = ((user_cs as u64) << 48) | ((kernel_cs as u64) << 32);
        let fmask = 0x257FD; // Mask IF, TF, DF, NT, etc.

        Self {
            msr_star: star,
            msr_lstar: trampoline_entry,
            msr_fmask: fmask,
            trampoline_entry_address: trampoline_entry,
        }
    }

    /// Fast dispatch using FastSyscallRegisters
    pub fn dispatch_fast_syscall(
        &self,
        dispatcher: &SyscallDispatcher,
        regs: &mut FastSyscallRegisters,
    ) -> isize {
        let mut ctx = SyscallContext::new(
            regs.rax as usize,
            [
                regs.rdi as usize,
                regs.rsi as usize,
                regs.rdx as usize,
                regs.r10 as usize,
                regs.r8 as usize,
                regs.r9 as usize,
            ],
        );

        let res = dispatcher.dispatch(&mut ctx);
        regs.rax = ctx.get_return() as u64;
        res
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_syscall_trampoline_and_msrs() {
        let trampoline = FastSyscallTrampoline::new(0xFFFF_8000_0010_0000, 0x08, 0x1B);
        assert_eq!(trampoline.trampoline_entry_address, 0xFFFF_8000_0010_0000);
        assert_eq!(trampoline.msr_lstar, 0xFFFF_8000_0010_0000);
        assert_eq!(trampoline.msr_star >> 32 & 0xFFFF, 0x08);

        let dispatcher = SyscallDispatcher::new();
        let mut regs = FastSyscallRegisters {
            rax: SYS_GETPID as u64,
            rdi: 0,
            rsi: 0,
            rdx: 0,
            r10: 0,
            r8: 0,
            r9: 0,
            rcx: 0x400100, // saved user RIP
            r11: 0x202,    // saved user RFLAGS
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
        };

        let res = trampoline.dispatch_fast_syscall(&dispatcher, &mut regs);
        assert_eq!(res, 1);
        assert_eq!(regs.rax, 1); // PID 1 returned in rax
    }

    #[test]
    fn test_syscall_dispatcher_ioctl() {
        let dispatcher = SyscallDispatcher::new();
        assert_eq!(dispatcher.call_count(), 0);

        // Test ioctl with valid parameters
        let raw_req = ioctl_helper::IOC_IN | (4 << 16) | ((b'x' as u32) << 8) | 2;
        let mut ctx = SyscallContext::new(SYS_IOCTL, [3, raw_req as usize, 0x10000, 0, 0, 0]);
        let res = dispatcher.dispatch(&mut ctx);

        assert_eq!(res, 0); // Should return success 0
        assert_eq!(dispatcher.call_count(), 1);
        assert_eq!(dispatcher.error_count(), 0);

        // Test ioctl triggering safety violation (e.g. out of boundary buffer)
        let mut mal_ctx = SyscallContext::new(SYS_IOCTL, [3, raw_req as usize, 0x1F00000, 0, 0, 0]);
        let mal_res = dispatcher.dispatch(&mut mal_ctx);

        assert_eq!(mal_res, -1);
        assert_eq!(dispatcher.error_count(), 1);
    }
}

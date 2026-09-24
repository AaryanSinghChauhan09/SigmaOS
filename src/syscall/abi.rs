/// SigmaOS Stable Syscall ABI (Phase 1)
/// Follows System V AMD64 ABI conventions:
/// Syscall Number: %rax
/// Args: %rdi, %rsi, %rdx, %r10, %r8, %r9
/// Return: %rax
/// Error: Negative errno in %rax

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SyscallNumber {
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
    Brk = 12,
    RtSigaction = 13,
    RtSigprocmask = 14,
    RtSigreturn = 15,
    Ioctl = 16,
    Pread64 = 17,
    Pwrite64 = 18,
    Readv = 19,
    Writev = 20,
    Access = 21,
    Pipe = 22,
    Select = 23,
    SchedYield = 24,
    Mremap = 25,
    Msync = 26,
    Mincore = 27,
    Madvise = 28,
    Shmget = 29,
    Shmat = 30,
    Shmctl = 31,
    Dup = 32,
    Dup2 = 33,
    Pause = 34,
    Nanosleep = 35,
    Getitimer = 36,
    Alarm = 37,
    Setitimer = 38,
    Getpid = 39,
    Sendfile = 40,
    Socket = 41,
    Connect = 42,
    Accept = 43,
    Sendto = 44,
    Recvfrom = 45,
    Sendmsg = 46,
    Recvmsg = 47,
    Shutdown = 48,
    Bind = 49,
    Listen = 50,
    Getsockname = 51,
    Getpeername = 52,
    Socketpair = 53,
    Setsockopt = 54,
    Getsockopt = 55,
    Clone = 56,
    Fork = 57,
    Vfork = 58,
    Execve = 59,
    Exit = 60,
    Wait4 = 61,
    Kill = 62,
    Uname = 63,
}

pub type SyscallResult = Result<usize, isize>;

/// Minimal libc-equivalent userland stubs for syscalls
pub mod libc {
    use super::*;

    #[inline(always)]
    pub unsafe fn syscall0(n: SyscallNumber) -> usize {
        let ret: usize;
        core::arch::asm!(
            "syscall",
            in("rax") n as usize,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall1(n: SyscallNumber, arg1: usize) -> usize {
        let ret: usize;
        core::arch::asm!(
            "syscall",
            in("rax") n as usize,
            in("rdi") arg1,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    #[inline(always)]
    pub unsafe fn syscall3(n: SyscallNumber, arg1: usize, arg2: usize, arg3: usize) -> usize {
        let ret: usize;
        core::arch::asm!(
            "syscall",
            in("rax") n as usize,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
            options(nostack, preserves_flags)
        );
        ret
    }

    pub fn exit(status: i32) -> ! {
        unsafe {
            syscall1(SyscallNumber::Exit, status as usize);
        }
        loop {}
    }
}

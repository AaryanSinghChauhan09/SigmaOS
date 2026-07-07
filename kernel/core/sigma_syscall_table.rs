//! SigmaOS — Syscall Dispatch Table
//! Handles incoming system calls from userland and dispatches them to kernel functions.
//! Follows standard System V AMD64 ABI calling convention.

#![no_std]
#![allow(dead_code)]

type U64 = u64;

// ── Syscall Numbers ─────────────────────────────────────────────────────────
pub const SYS_READ:       usize = 0;
pub const SYS_WRITE:      usize = 1;
pub const SYS_OPEN:       usize = 2;
pub const SYS_CLOSE:      usize = 3;
pub const SYS_STAT:       usize = 4;
pub const SYS_FSTAT:      usize = 5;
pub const SYS_LSTAT:      usize = 6;
pub const SYS_POLL:       usize = 7;
pub const SYS_LSEEK:      usize = 8;
pub const SYS_MMAP:       usize = 9;
pub const SYS_MPROTECT:   usize = 10;
pub const SYS_MUNMAP:     usize = 11;
pub const SYS_BRK:        usize = 12;
pub const SYS_RT_SIGACTION: usize = 13;
pub const SYS_RT_SIGPROCMASK: usize = 14;
pub const SYS_RT_SIGRETURN: usize = 15;
pub const SYS_IOCTL:      usize = 16;
pub const SYS_PREAD64:    usize = 17;
pub const SYS_PWRITE64:   usize = 18;
pub const SYS_READV:      usize = 19;
pub const SYS_WRITEV:     usize = 20;
pub const SYS_ACCESS:     usize = 21;
pub const SYS_PIPE:       usize = 22;
pub const SYS_SELECT:     usize = 23;
pub const SYS_SCHED_YIELD: usize = 24;
pub const SYS_MREMAP:     usize = 25;
pub const SYS_MSYNC:      usize = 26;
pub const SYS_MINCORE:    usize = 27;
pub const SYS_MADVISE:    usize = 28;
pub const SYS_SHMGET:     usize = 29;
pub const SYS_SHMAT:      usize = 30;
pub const SYS_SHMCTL:     usize = 31;
pub const SYS_DUP:        usize = 32;
pub const SYS_DUP2:       usize = 33;
pub const SYS_PAUSE:      usize = 34;
pub const SYS_NANOSLEEP:  usize = 35;
pub const SYS_GETITIMER:  usize = 36;
pub const SYS_ALARM:      usize = 37;
pub const SYS_SETITIMER:  usize = 38;
pub const SYS_GETPID:     usize = 39;
pub const SYS_SENDFILE:   usize = 40;
pub const SYS_SOCKET:     usize = 41;
pub const SYS_CONNECT:    usize = 42;
pub const SYS_ACCEPT:     usize = 43;
pub const SYS_SENDTO:     usize = 44;
pub const SYS_RECVFROM:   usize = 45;
pub const SYS_SENDMSG:    usize = 46;
pub const SYS_RECVMSG:    usize = 47;
pub const SYS_SHUTDOWN:   usize = 48;
pub const SYS_BIND:       usize = 49;
pub const SYS_LISTEN:     usize = 50;
pub const SYS_GETSOCKNAME: usize = 51;
pub const SYS_GETPEERNAME: usize = 52;
pub const SYS_SOCKETPAIR: usize = 53;
pub const SYS_SETSOCKOPT: usize = 54;
pub const SYS_GETSOCKOPT: usize = 55;
pub const SYS_CLONE:      usize = 56;
pub const SYS_FORK:       usize = 57;
pub const SYS_VFORK:      usize = 58;
pub const SYS_EXECVE:     usize = 59;
pub const SYS_EXIT:       usize = 60;
pub const SYS_WAIT4:      usize = 61;
pub const SYS_KILL:       usize = 62;
pub const SYS_UNAME:      usize = 63;
pub const SYS_SEMGET:     usize = 64;
pub const SYS_SEMOP:      usize = 65;
pub const SYS_SEMCTL:     usize = 66;
pub const SYS_SHMDT:      usize = 67;
pub const SYS_MSGGET:     usize = 68;
pub const SYS_MSGSND:     usize = 69;
pub const SYS_MSGRCV:     usize = 70;
pub const SYS_MSGCTL:     usize = 71;
pub const SYS_FCNTL:      usize = 72;
pub const SYS_FLOCK:      usize = 73;
pub const SYS_FSYNC:      usize = 74;
pub const SYS_FDATASYNC:  usize = 75;
pub const SYS_TRUNCATE:   usize = 76;
pub const SYS_FTRUNCATE:  usize = 77;
pub const SYS_GETDENTS:   usize = 78;
pub const SYS_GETCWD:     usize = 79;
pub const SYS_CHDIR:      usize = 80;
pub const SYS_FCHDIR:     usize = 81;
pub const SYS_RENAME:     usize = 82;
pub const SYS_MKDIR:      usize = 83;
pub const SYS_RMDIR:      usize = 84;
pub const SYS_CREAT:      usize = 85;
pub const SYS_LINK:       usize = 86;
pub const SYS_UNLINK:     usize = 87;
pub const SYS_SYMLINK:    usize = 88;
pub const SYS_READLINK:   usize = 89;
pub const SYS_CHMOD:      usize = 90;
pub const SYS_FCHMOD:     usize = 91;
pub const SYS_CHOWN:      usize = 92;
pub const SYS_FCHOWN:     usize = 93;
pub const SYS_LCHOWN:     usize = 94;
pub const SYS_UMASK:      usize = 95;
pub const SYS_GETTIMEOFDAY: usize = 96;
pub const SYS_GETRLIMIT:  usize = 97;
pub const SYS_GETRUSAGE:  usize = 98;
pub const SYS_SYSINFO:    usize = 99;
pub const SYS_TIMES:      usize = 100;

// Custom Sovereign OS extensions
pub const SYS_CAP_REQUEST: usize = 400;
pub const SYS_CAP_DROP:    usize = 401;
pub const SYS_IPC_SEND:    usize = 402;
pub const SYS_IPC_RECV:    usize = 403;

// ── Syscall Handlers (Stubs) ────────────────────────────────────────────────

// These would normally be implemented in other modules, but we provide stubs here.

unsafe fn sys_read(fd: U64, buf: U64, count: U64) -> U64 {
    // Call into VFS
    // sigma_vfs_read(fd as i32, buf as *mut u8, count as u32) as u64
    (!0) // Error: not implemented
}

unsafe fn sys_write(fd: U64, buf: U64, count: U64) -> U64 {
    // Call into VFS
    // sigma_vfs_write(fd as i32, buf as *const u8, count as u32) as u64
    (!0) // Error: not implemented
}

unsafe fn sys_open(path: U64, flags: U64, mode: U64) -> U64 {
    (!0)
}

unsafe fn sys_close(fd: U64) -> U64 {
    (!0)
}

unsafe fn sys_exit(code: U64) -> U64 {
    // sigma_sched_exit(code as i32);
    0
}

unsafe fn sys_getpid() -> U64 {
    // sigma_sched_current_tid() as u64
    1
}

unsafe fn sys_sched_yield() -> U64 {
    // sigma_sched_yield();
    0
}

unsafe fn sys_cap_request(cap_id: U64, flags: U64) -> U64 {
    (!0)
}

unsafe fn sys_cap_drop(cap_id: U64) -> U64 {
    (!0)
}

unsafe fn sys_ipc_send(target_tid: U64, msg_ptr: U64, len: U64) -> U64 {
    (!0)
}

unsafe fn sys_ipc_recv(msg_ptr: U64, len: U64, timeout_ns: U64) -> U64 {
    (!0)
}

// ── Dispatcher ──────────────────────────────────────────────────────────────

/// Main entry point for syscalls from assembly handler.
/// ABI:
/// RAX = Syscall Number
/// RDI = Arg1
/// RSI = Arg2
/// RDX = Arg3
/// R10 = Arg4
/// R8  = Arg5
/// R9  = Arg6
/// Return value in RAX.
#[no_mangle]
pub unsafe extern "C" fn sigma_syscall_dispatch(
    sys_num: U64,
    arg1: U64,
    arg2: U64,
    arg3: U64,
    arg4: U64,
    arg5: U64,
    arg6: U64,
) -> U64 {
    match sys_num as usize {
        SYS_READ => sys_read(arg1, arg2, arg3),
        SYS_WRITE => sys_write(arg1, arg2, arg3),
        SYS_OPEN => sys_open(arg1, arg2, arg3),
        SYS_CLOSE => sys_close(arg1),
        SYS_EXIT => sys_exit(arg1),
        SYS_GETPID => sys_getpid(),
        SYS_SCHED_YIELD => sys_sched_yield(),
        
        SYS_CAP_REQUEST => sys_cap_request(arg1, arg2),
        SYS_CAP_DROP => sys_cap_drop(arg1),
        SYS_IPC_SEND => sys_ipc_send(arg1, arg2, arg3),
        SYS_IPC_RECV => sys_ipc_recv(arg1, arg2, arg3),

        _ => {
            // Return ENOSYS (Function not implemented)
            !0
        }
    }
}

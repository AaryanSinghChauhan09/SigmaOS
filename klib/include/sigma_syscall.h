// SPDX-License-Identifier: GPL-2.0-or-later
#pragma once
/*
 * sigma_syscall.h — SigmaOS syscall table (Linux x86_64 ABI-compatible)
 *
 * Numbers are IDENTICAL to Linux x86_64. This means musl compiled for Linux
 * runs on SigmaOS without modification — massively expanding compatible software.
 *
 * Syscalls unique to SigmaOS use numbers >= 400 (above Linux's range).
 */

/* ── Standard POSIX (Linux x86_64 numbers) ────────────────────────────────── */
#define SYS_read              0
#define SYS_write             1
#define SYS_open              2
#define SYS_close             3
#define SYS_stat              4
#define SYS_fstat             5
#define SYS_lstat             6
#define SYS_poll              7
#define SYS_lseek             8
#define SYS_mmap              9
#define SYS_mprotect          10
#define SYS_munmap            11
#define SYS_brk               12
#define SYS_rt_sigaction      13
#define SYS_rt_sigprocmask    14
#define SYS_rt_sigreturn      15
#define SYS_ioctl             16
#define SYS_pread64           17
#define SYS_pwrite64          18
#define SYS_readv             19
#define SYS_writev            20
#define SYS_access            21
#define SYS_pipe              22
#define SYS_select            23
#define SYS_sched_yield       24
#define SYS_mremap            25
#define SYS_msync             26
#define SYS_mincore           27
#define SYS_madvise           28
#define SYS_dup               32
#define SYS_dup2              33
#define SYS_nanosleep         35
#define SYS_getitimer         36
#define SYS_alarm             37
#define SYS_setitimer         38
#define SYS_getpid            39
#define SYS_socket            41
#define SYS_connect           42
#define SYS_accept            43
#define SYS_sendto            44
#define SYS_recvfrom          45
#define SYS_sendmsg           46
#define SYS_recvmsg           47
#define SYS_shutdown          48
#define SYS_bind              49
#define SYS_listen            50
#define SYS_getsockname       51
#define SYS_getpeername       52
#define SYS_socketpair        53
#define SYS_setsockopt        54
#define SYS_getsockopt        55
#define SYS_clone             56
#define SYS_fork              57
#define SYS_vfork             58
#define SYS_execve            59
#define SYS_exit              60
#define SYS_wait4             61
#define SYS_kill              62
#define SYS_uname             63
#define SYS_fcntl             72
#define SYS_flock             73
#define SYS_fsync             74
#define SYS_fdatasync         75
#define SYS_truncate          76
#define SYS_ftruncate         77
#define SYS_getdents          78
#define SYS_getcwd            79
#define SYS_chdir             80
#define SYS_fchdir            81
#define SYS_rename            82
#define SYS_mkdir             83
#define SYS_rmdir             84
#define SYS_creat             85
#define SYS_link              86
#define SYS_unlink            87
#define SYS_symlink           88
#define SYS_readlink          89
#define SYS_chmod             90
#define SYS_fchmod            91
#define SYS_chown             92
#define SYS_getuid            102
#define SYS_getgid            104
#define SYS_gettimeofday      96
#define SYS_clock_gettime     228
#define SYS_clock_settime     227
#define SYS_futex             202
#define SYS_epoll_create      213
#define SYS_epoll_ctl         233
#define SYS_epoll_wait        232
#define SYS_exit_group        231
#define SYS_openat            257
#define SYS_mkdirat           258
#define SYS_fstatat           262
#define SYS_unlinkat          263
#define SYS_renameat          264
#define SYS_linkat            265
#define SYS_symlinkat         266
#define SYS_readlinkat        267
#define SYS_fchmodat          268
#define SYS_accept4           288
#define SYS_pipe2             293
#define SYS_dup3              292
#define SYS_epoll_create1     291
#define SYS_getrandom         318
#define SYS_memfd_create      319

/* ── SigmaOS-specific extensions (>= 400) ─────────────────────────────────── */
#define SYS_sigma_pledge      400   /* per-process syscall restriction        */
#define SYS_sigma_unveil      401   /* per-process filesystem restriction     */
#define SYS_sigma_bus_send    402   /* send message on sigma-bus              */
#define SYS_sigma_cap_check   403   /* check capability token                 */
#define SYS_sigma_sysctl      404   /* read/write sigma sysctl                */
#define SYS_sigma_kpatch_load 405   /* load a live kernel patch               */
#define SYS_sigma_ebpf_attach 406   /* attach an eBPF program to a hook       */
#define SYS_sigma_secret_get  407   /* retrieve secret from sigma-vault       */

/* ── Inline syscall helper (freestanding, no libc) ────────────────────────── */
#ifndef __KERNEL__
static inline long sigma_syscall0(long n) {
    long ret;
    __asm__ volatile ("syscall" : "=a"(ret) : "0"(n) : "rcx","r11","memory");
    return ret;
}
static inline long sigma_syscall1(long n, long a1) {
    long ret;
    __asm__ volatile ("syscall" : "=a"(ret) : "0"(n),"D"(a1)
                      : "rcx","r11","memory");
    return ret;
}
static inline long sigma_syscall3(long n, long a1, long a2, long a3) {
    long ret;
    __asm__ volatile ("syscall" : "=a"(ret) : "0"(n),"D"(a1),"S"(a2),"d"(a3)
                      : "rcx","r11","memory");
    return ret;
}
#endif /* __KERNEL__ */

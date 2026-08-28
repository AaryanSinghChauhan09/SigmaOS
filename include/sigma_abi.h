/* SPDX-License-Identifier: MIT */
/*
 * =========================================================================
 * Σ SIGMAOS: KERNEL ABI DEFINITIONS (S-ABI)
 * =========================================================================
 * Linux & BSD cross-distro syscall, ioctl, pledge/unveil, io_uring, and
 * kqueue ABI compatibility interface.
 * =========================================================================
 */

#ifndef SIGMA_ABI_H
#define SIGMA_ABI_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- System Call Numbers (Linux x86_64 & BSD Parity) --- */
#define SIGMA_SYS_read              0
#define SIGMA_SYS_write             1
#define SIGMA_SYS_open              2
#define SIGMA_SYS_close             3
#define SIGMA_SYS_stat              4
#define SIGMA_SYS_fstat             5
#define SIGMA_SYS_lstat             6
#define SIGMA_SYS_poll              7
#define SIGMA_SYS_lseek             8
#define SIGMA_SYS_mmap              9
#define SIGMA_SYS_mprotect          10
#define SIGMA_SYS_munmap            11
#define SIGMA_SYS_brk               12
#define SIGMA_SYS_ioctl             16
#define SIGMA_SYS_pipe              22
#define SIGMA_SYS_select            23
#define SIGMA_SYS_sched_yield       24
#define SIGMA_SYS_dup               32
#define SIGMA_SYS_dup2              33
#define SIGMA_SYS_fork              57
#define SIGMA_SYS_vfork             58
#define SIGMA_SYS_execve            59
#define SIGMA_SYS_exit              60
#define SIGMA_SYS_wait4             61
#define SIGMA_SYS_kill              62
#define SIGMA_SYS_getpid            39
#define SIGMA_SYS_socket            41
#define SIGMA_SYS_connect           42
#define SIGMA_SYS_accept            43
#define SIGMA_SYS_sendto            44
#define SIGMA_SYS_recvfrom          45
#define SIGMA_SYS_bind              49
#define SIGMA_SYS_listen            50
#define SIGMA_SYS_epoll_create      213
#define SIGMA_SYS_epoll_ctl         233
#define SIGMA_SYS_epoll_wait        232
#define SIGMA_SYS_io_uring_setup    425
#define SIGMA_SYS_io_uring_enter    426
#define SIGMA_SYS_io_uring_register 427
#define SIGMA_SYS_pledge            500
#define SIGMA_SYS_unveil            501
#define SIGMA_SYS_cap_rights_limit  502

/* --- OpenBSD Pledge / Unveil Security Flags --- */
#define SIGMA_PLEDGE_STDIO     (1ULL << 0)
#define SIGMA_PLEDGE_RPATH     (1ULL << 1)
#define SIGMA_PLEDGE_WPATH     (1ULL << 2)
#define SIGMA_PLEDGE_CPATH     (1ULL << 3)
#define SIGMA_PLEDGE_INET      (1ULL << 4)
#define SIGMA_PLEDGE_UNIX      (1ULL << 5)
#define SIGMA_PLEDGE_EXEC      (1ULL << 6)
#define SIGMA_PLEDGE_PROC      (1ULL << 7)

#define SIGMA_UNVEIL_READ      0x01
#define SIGMA_UNVEIL_WRITE     0x02
#define SIGMA_UNVEIL_EXEC      0x04
#define SIGMA_UNVEIL_CREATE    0x08

/* --- io_uring Queue Entry Definitions --- */
struct sigma_io_uring_sqe {
    sigma_u8  opcode;
    sigma_u8  flags;
    sigma_u16 ioprio;
    sigma_i32 fd;
    sigma_u64 off;
    sigma_u64 addr;
    sigma_u32 len;
    sigma_u32 op_flags;
    sigma_u64 user_data;
};

struct sigma_io_uring_cqe {
    sigma_u64 user_data;
    sigma_i32 res;
    sigma_u32 flags;
};

/* --- FreeBSD Kqueue / Kevent Definitions --- */
struct sigma_kevent {
    sigma_uintptr_t ident;
    sigma_s16       filter;
    sigma_u16       flags;
    sigma_u32       fflags;
    sigma_s64       data;
    void           *udata;
};

#define SIGMA_EVFILT_READ    (-1)
#define SIGMA_EVFILT_WRITE   (-2)
#define SIGMA_EVFILT_SIGNAL  (-6)
#define SIGMA_EVFILT_TIMER   (-7)

#define SIGMA_EV_ADD         0x0001
#define SIGMA_EV_DELETE      0x0002
#define SIGMA_EV_ENABLE      0x0004
#define SIGMA_EV_DISABLE     0x0008
#define SIGMA_EV_ONESHOT     0x0010

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ABI_H */

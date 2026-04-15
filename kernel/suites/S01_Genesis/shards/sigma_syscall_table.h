/*
 * =========================================================================
 * S SIGMAOS kernel/suites/S01_Genesis/shards/sigma_syscall_table.h
 * =========================================================================
 * Sovereign Syscall Table — gap-closes Linux x86-64 syscall ABI, BSD
 * SVC interface, and Windows NT syscall numbers.
 * Each syscall has a sovereign ID, arity, security classification, and
 * optional seccomp filter action.
 * =========================================================================
 */

#ifndef SIGMA_SYSCALL_TABLE_H
#define SIGMA_SYSCALL_TABLE_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* ── Syscall numbers ─────────────────────────────────────────────────────── */
#define SC_READ          0
#define SC_WRITE         1
#define SC_OPEN          2
#define SC_CLOSE         3
#define SC_STAT          4
#define SC_FSTAT         5
#define SC_LSTAT         6
#define SC_SEEK          7
#define SC_MMAP          8
#define SC_MPROTECT      9
#define SC_MUNMAP       10
#define SC_BRK          11
#define SC_IOCTL        12
#define SC_READV        13
#define SC_WRITEV       14
#define SC_ACCESS       15
#define SC_PIPE         16
#define SC_SELECT       17
#define SC_SCHED_YIELD  18
#define SC_MREMAP       19
#define SC_MADVISE      20
#define SC_SOCKET       21
#define SC_CONNECT      22
#define SC_ACCEPT       23
#define SC_SEND         24
#define SC_RECV         25
#define SC_BIND         26
#define SC_LISTEN       27
#define SC_CLONE        28
#define SC_FORK         29
#define SC_EXEC         30
#define SC_EXIT         31
#define SC_WAIT         32
#define SC_KILL         33
#define SC_OPENAT       34
#define SC_GETPID       35
#define SC_GETPPID      36
#define SC_GETUID       37
#define SC_GETGID       38
#define SC_SETUID       39
#define SC_SETGID       40
#define SC_GETCWD       41
#define SC_CHDIR        42
#define SC_MKDIR        43
#define SC_RMDIR        44
#define SC_UNLINK       45
#define SC_RENAME       46
#define SC_LINK         47
#define SC_SYMLINK      48
#define SC_READLINK     49
#define SC_CHMOD        50
#define SC_CHOWN        51
#define SC_TRUNCATE     52
#define SC_SYNC         53
#define SC_FSYNC        54
#define SC_MOUNT        55
#define SC_UMOUNT       56
#define SC_SYSINFO      57
#define SC_SYSLOG       58
#define SC_NANOSLEEP    59
#define SC_CLOCK_GETTIME 60
#define SC_SIGACTION    61
#define SC_SIGPROCMASK  62
#define SC_SIGSUSPEND   63
#define SC_SIGRETURN    64
/* ── Sovereign extensions ───────────────────────────────────────────────── */
#define SC_SIGMA_SHARD_LOAD   256  /* Load a sovereign shard dynamically */
#define SC_SIGMA_SHARD_UNLOAD 257
#define SC_SIGMA_IPC_CREATE   258
#define SC_SIGMA_IPC_SEND     259
#define SC_SIGMA_IPC_RECV     260
#define SC_SIGMA_CGROUP_SET   261
#define SC_SIGMA_SECCOMP_SET  262
#define SC_SIGMA_UDF_CALL     263  /* User-defined function invocation   */
#define SC_SIGMA_ATTESTATION  264  /* TPM/PQC attestation                */

#define SIGMA_SYSCALL_MAX     512

/* ── Security classification ────────────────────────────────────────────── */
typedef enum {
    SC_SEC_SAFE      = 0,  /* benign read-only operations          */
    SC_SEC_MODERATE  = 1,  /* file writes, socket ops              */
    SC_SEC_PRIV      = 2,  /* requires CAP_SYS_ADMIN equivalent    */
    SC_SEC_CRITICAL  = 3   /* kernel internals — ring-0 only       */
} sc_security_t;

/* ── Syscall handler type ───────────────────────────────────────────────── */
typedef sigma_i64 (*sigma_syscall_fn)(sigma_u64, sigma_u64, sigma_u64,
                                   sigma_u64, sigma_u64, sigma_u64);

/* ── Syscall descriptor ─────────────────────────────────────────────────── */
typedef struct {
    sigma_u32           number;
    char             name[32];
    sigma_u8            arity;       /* number of arguments (0–6)        */
    sc_security_t    sec_class;
    sigma_syscall_fn handler;
    sigma_u32           call_count;  /* telemetry                        */
} sigma_syscall_desc_t;

/* ── Public API ─────────────────────────────────────────────────────────── */
void    sigma_syscall_table_init(void);
sigma_i64  sigma_syscall_dispatch(sigma_u32 num,
                                sigma_u64 a, sigma_u64 b, sigma_u64 c,
                                sigma_u64 d, sigma_u64 e, sigma_u64 f);
void    sigma_syscall_register(sigma_u32 num, const char *name, sigma_u8 arity,
                                sc_security_t sec, sigma_syscall_fn handler);
void    sigma_syscall_audit(void);  /* print call counts for telemetry */

#endif /* SIGMA_SYSCALL_TABLE_H */

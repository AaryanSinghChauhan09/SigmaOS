/*
 * =========================================================================
 * SigmaOS: Modular Syscall Dispatcher (dispatcher.c)
 * =========================================================================
 * Zero-dependency, table-driven. No stdlib, no libc.
 * All handlers live in handlers/; dispatcher.c only does routing.
 * =========================================================================
 */
#include "dispatcher.h"
#include "../../../include/sigma_log.h"

/* ── Forward declarations of all handlers ──────────────────────────── */
static sigma_u64 sys_getpid (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_write  (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_read   (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_exit   (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_open   (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_close  (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_mmap   (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_munmap (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_fork   (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_exec   (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_waitpid(sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_kill   (sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_gettime(sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_yield  (sigma_u64, sigma_u64, sigma_u64, sigma_u64);

/* ── Stable ABI Table ───────────────────────────────────────────────── */
const syscall_fn_t sigma_syscall_table[SYSCALL_MAX] = {
    sys_getpid,     /* 0  */
    sys_write,      /* 1  */
    sys_read,       /* 2  */
    sys_exit,       /* 3  */
    sys_open,       /* 4  */
    sys_close,      /* 5  */
    sys_mmap,       /* 6  */
    sys_munmap,     /* 7  */
    sys_fork,       /* 8  */
    sys_exec,       /* 9  */
    sys_waitpid,    /* 10 */
    sys_kill,       /* 11 */
    sys_gettime,    /* 12 */
    sys_yield       /* 13 */
};

/* ── Dispatcher ─────────────────────────────────────────────────────── */
sigma_u64 syscall_dispatch(sigma_u64 nr,
                            sigma_u64 a0, sigma_u64 a1,
                            sigma_u64 a2, sigma_u64 a3)
{
    if (nr >= SYSCALL_MAX) {
        sigma_log_error("[SYSCALL] Invalid syscall number: %llu", nr);
        return (sigma_u64)-1;
    }
    return sigma_syscall_table[nr](a0, a1, a2, a3);
}

sigma_u64 sys_entry(sigma_u32 num,
                     sigma_u64 a0, sigma_u64 a1,
                     sigma_u64 a2, sigma_u64 a3)
{
    return syscall_dispatch((sigma_u64)num, a0, a1, a2, a3);
}

/* ── Handler Implementations ────────────────────────────────────────── */

/* Provided by the task scheduler; declared extern so we can reference it. */
extern sigma_u32 sigma_current_pid;

static sigma_u64 sys_getpid(sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3)
{
    (void)a0; (void)a1; (void)a2; (void)a3;
    return (sigma_u64)sigma_current_pid;
}

static sigma_u64 sys_write(sigma_u64 fd, sigma_u64 buf_addr, sigma_u64 count, sigma_u64 a3)
{
    (void)a3;
    /* Route fd=1/2 to the serial log; other fds go to VFS (stub). */
    if (fd == 1u || fd == 2u) {
        const char* buf = (const char*)(sigma_usize)buf_addr;
        sigma_u64   n   = count;
        while (n-- && *buf) {
            extern void serial_putc(char c);
            serial_putc(*buf++);
        }
        return count;
    }
    return (sigma_u64)-1;
}

static sigma_u64 sys_read(sigma_u64 fd, sigma_u64 buf_addr, sigma_u64 count, sigma_u64 a3)
{
    (void)fd; (void)buf_addr; (void)count; (void)a3;
    sigma_log_info("[SYSCALL] sys_read: stub");
    return 0u;
}

static sigma_u64 sys_exit(sigma_u64 code, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3)
{
    (void)a1; (void)a2; (void)a3;
    sigma_log_info("[SYSCALL] sys_exit: code=%llu", code);
    cpu_halt();
    return 0u; /* never reached */
}

static sigma_u64 sys_open(sigma_u64 path, sigma_u64 flags, sigma_u64 mode, sigma_u64 a3)
{
    (void)path; (void)flags; (void)mode; (void)a3;
    sigma_log_info("[SYSCALL] sys_open: stub");
    return (sigma_u64)-1;
}

static sigma_u64 sys_close(sigma_u64 fd, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3)
{
    (void)fd; (void)a1; (void)a2; (void)a3;
    sigma_log_info("[SYSCALL] sys_close: stub");
    return 0u;
}

static sigma_u64 sys_mmap(sigma_u64 addr, sigma_u64 len, sigma_u64 prot, sigma_u64 flags)
{
    (void)addr; (void)len; (void)prot; (void)flags;
    sigma_log_info("[SYSCALL] sys_mmap: stub");
    return (sigma_u64)-1;
}

static sigma_u64 sys_munmap(sigma_u64 addr, sigma_u64 len, sigma_u64 a2, sigma_u64 a3)
{
    (void)addr; (void)len; (void)a2; (void)a3;
    sigma_log_info("[SYSCALL] sys_munmap: stub");
    return 0u;
}

static sigma_u64 sys_fork(sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3)
{
    (void)a0; (void)a1; (void)a2; (void)a3;
    sigma_log_info("[SYSCALL] sys_fork: stub");
    return (sigma_u64)-1;
}

static sigma_u64 sys_exec(sigma_u64 path, sigma_u64 argv, sigma_u64 envp, sigma_u64 a3)
{
    (void)path; (void)argv; (void)envp; (void)a3;
    sigma_log_info("[SYSCALL] sys_exec: stub");
    return (sigma_u64)-1;
}

static sigma_u64 sys_waitpid(sigma_u64 pid, sigma_u64 status, sigma_u64 opts, sigma_u64 a3)
{
    (void)pid; (void)status; (void)opts; (void)a3;
    sigma_log_info("[SYSCALL] sys_waitpid: stub");
    return (sigma_u64)-1;
}

static sigma_u64 sys_kill(sigma_u64 pid, sigma_u64 sig, sigma_u64 a2, sigma_u64 a3)
{
    (void)pid; (void)sig; (void)a2; (void)a3;
    sigma_log_info("[SYSCALL] sys_kill: stub");
    return 0u;
}

static sigma_u64 sys_gettime(sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3)
{
    (void)a0; (void)a1; (void)a2; (void)a3;
    return cpu_rdtsc();
}

static sigma_u64 sys_yield(sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3)
{
    (void)a0; (void)a1; (void)a2; (void)a3;
    sigma_log_info("[SYSCALL] sys_yield: yielding CPU");
    cpu_pause();
    return 0u;
}

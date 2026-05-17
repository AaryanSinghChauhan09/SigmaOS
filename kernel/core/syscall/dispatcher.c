#include "syscall_dispatcher.h"
#include "sigma_log.h"

/* Forward declarations of syscall handlers */
static sigma_u64 sys_getpid(sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_write(sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_read(sigma_u64, sigma_u64, sigma_u64, sigma_u64);
static sigma_u64 sys_exit(sigma_u64, sigma_u64, sigma_u64, sigma_u64);

/* Syscall table – indexed by syscall number */
static const syscall_fn_t syscall_table[SYSCALL_MAX] = {
    sys_getpid,
    sys_write,
    sys_read,
    sys_exit
};

sigma_u64 syscall_dispatcher(sigma_u32 nr,
                            sigma_u64 a1,
                            sigma_u64 a2,
                            sigma_u64 a3,
                            sigma_u64 a4)
{
    if (nr >= SYSCALL_MAX) {
        sigma_log_error("Invalid syscall %u", nr);
        return (sigma_u64)-1;
    }
    return syscall_table[nr](a1, a2, a3, a4);
}

/* ----- Minimal handler implementations ----- */
static sigma_u64 sys_getpid(sigma_u64, sigma_u64, sigma_u64, sigma_u64)
{
    extern sigma_u32 current_pid; // defined elsewhere in the scheduler
    return (sigma_u64)current_pid;
}

static sigma_u64 sys_write(sigma_u64 fd, sigma_u64 buf, sigma_u64 len, sigma_u64)
{
    // Very simple stub – log the write attempt
    sigma_log_info("sys_write fd=%llu len=%llu", fd, len);
    return len; // pretend all bytes written
}

static sigma_u64 sys_read(sigma_u64 fd, sigma_u64 buf, sigma_u64 len, sigma_u64)
{
    sigma_log_info("sys_read fd=%llu len=%llu", fd, len);
    return 0; // no data provided
}

static sigma_u64 sys_exit(sigma_u64 status, sigma_u64, sigma_u64, sigma_u64)
{
    sigma_log_info("sys_exit status=%llu", status);
    // In a real kernel this would terminate the thread; here we just loop.
    while (1) { __asm__("hlt"); }
    return 0; // unreachable
}

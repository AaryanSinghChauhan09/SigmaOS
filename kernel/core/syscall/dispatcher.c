#include "dispatcher.h"

static sigma_u64 sys_getpid_impl(sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4) {
    (void)a1; (void)a2; (void)a3; (void)a4;
    return 1000; // Sovereign init PID
}

static sigma_u64 sys_write_impl(sigma_u64 fd, sigma_u64 buf, sigma_u64 count, sigma_u64 a4) {
    (void)fd; (void)buf; (void)count; (void)a4;
    return count;
}

static sigma_u64 sys_read_impl(sigma_u64 fd, sigma_u64 buf, sigma_u64 count, sigma_u64 a4) {
    (void)fd; (void)buf; (void)count; (void)a4;
    return count;
}

static sigma_u64 sys_exit_impl(sigma_u64 code, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4) {
    (void)code; (void)a2; (void)a3; (void)a4;
    while(1) {}
    return 0;
}

static sigma_u64 sys_open_impl(sigma_u64 path, sigma_u64 flags, sigma_u64 mode, sigma_u64 a4) {
    (void)path; (void)flags; (void)mode; (void)a4;
    return 3; // First available fd
}

static sigma_u64 sys_close_impl(sigma_u64 fd, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4) {
    (void)fd; (void)a2; (void)a3; (void)a4;
    return 0;
}

const syscall_fn_t syscall_table[SYSCALL_MAX] = {
    sys_getpid_impl,
    sys_write_impl,
    sys_read_impl,
    sys_exit_impl,
    sys_open_impl,
    sys_close_impl
};

sigma_u64 syscall_dispatcher(sigma_u64 nr, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4) {
    if (nr >= SYSCALL_MAX) {
        sigma_printf("[Syscall] Error: Invalid syscall %llu\n", nr);
        return (sigma_u64)-1;
    }
    return syscall_table[nr](a1, a2, a3, a4);
}

sigma_u64 sys_entry(sigma_u32 num, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    return syscall_dispatcher((sigma_u64)num, a0, a1, a2, a3);
}

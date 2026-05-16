#include "../../../../../include/libc/SovereignLibC.h"
#include "suites/S01_Genesis/shards/sigma_base.h"

#include "../../../../../include/SovereignSyscall.h"
#include "../../../../../include/libc/sigma_libc.h"

#define TABLE_SIZE 428
static SyscallFn_t s_syscall_table[TABLE_SIZE];

static sigma_i64 sys_enosys(sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4, sigma_u64 a5, sigma_u64 a6) {
    return -38; /* -ENOSYS */
}

void SovereignSyscall_InitRegistry(void) {
    for (int i = 0; i < TABLE_SIZE; i++) s_syscall_table[i] = sys_enosys;
    sigma_sigma_printf("S [SYS]: Sovereign Syscall Registry Operational.\n");
}

sigma_err_t SovereignSyscall_Register(sigma_u32 nr, SyscallFn_t handler) {
    if (nr >= TABLE_SIZE) return SIGMA_EINVAL;
    s_syscall_table[nr] = handler;
    return SIGMA_OK;
}

sigma_i64 sigma_syscall_dispatch(sigma_u64 nr, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4, sigma_u64 a5, sigma_u64 a6) {
    if (nr >= TABLE_SIZE) return -38;
    return s_syscall_table[nr](a1, a2, a3, a4, a5, a6);
}




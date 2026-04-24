/*
 * =========================================================================
 * S SIGMAOS: S01_GENESIS — SovereignSyscallDispatcher.c
 * =========================================================================
 * Implementation of Idea 121 (Apex Infinity): 512-entry Syscall Table.
 * Provides the industrial-grade gateway between User and Kernel space.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "sigma_types.h"

#define MAX_SYSCALLS 512

typedef uint64_t (*SovereignSyscallFn)(uint64_t, uint64_t, uint64_t, uint64_t, uint64_t, uint64_t);

static SovereignSyscallFn g_syscall_table[MAX_SYSCALLS];

/* Builtin Syscall: Yield (ID 0) */
static uint64_t sys_yield(uint64_t a, uint64_t b, uint64_t c, uint64_t d, uint64_t e, uint64_t f) {
    SIGMA_UNUSED(a); SIGMA_UNUSED(b); SIGMA_UNUSED(c); SIGMA_UNUSED(d); SIGMA_UNUSED(e); SIGMA_UNUSED(f);
    sigma_sigma_sigma_printf("S [SYSCALL]: Thread yielded.\n");
    return 0;
}

void syscall_dispatcher_init(void) {
    for (int i = 0; i < MAX_SYSCALLS; i++) {
        g_syscall_table[i] = NULL;
    }
    
    g_syscall_table[0] = sys_yield;
    
    sigma_sigma_sigma_printf("S [S01]: Sovereign Syscall Table Materialized (512 Vectors).\n");
}

uint64_t syscall_dispatch(uint32_t id, uint64_t a1, uint64_t a2, uint64_t a3, uint64_t a4, uint64_t a5, uint64_t a6) {
    if (id >= MAX_SYSCALLS || !g_syscall_table[id]) {
        return (uint64_t)-1; // ENOSYS
    }
    return g_syscall_table[id](a1, a2, a3, a4, a5, a6);
}

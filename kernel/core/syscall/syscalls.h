/*
 * =========================================================================
 * SigmaOS: Syscall Table & IDs (syscalls.h)
 * =========================================================================
 * Syscall numbers are fixed ABI — never reorder entries.
 * To add a new syscall: append at the end and increment SYSCALL_MAX.
 * =========================================================================
 */
#ifndef SIGMA_SYSCALLS_H
#define SIGMA_SYSCALLS_H

#include "../../../include/sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* ── Syscall Numbers (stable ABI) ──────────────────────────────────── */
typedef enum {
    SYSCALL_GETPID   = 0,
    SYSCALL_WRITE    = 1,
    SYSCALL_READ     = 2,
    SYSCALL_EXIT     = 3,
    SYSCALL_OPEN     = 4,
    SYSCALL_CLOSE    = 5,
    SYSCALL_MMAP     = 6,
    SYSCALL_MUNMAP   = 7,
    SYSCALL_FORK     = 8,
    SYSCALL_EXEC     = 9,
    SYSCALL_WAITPID  = 10,
    SYSCALL_KILL     = 11,
    SYSCALL_GETTIME  = 12,
    SYSCALL_YIELD    = 13,
    SYSCALL_MAX      = 14
} sigma_syscall_id_t;

/* ── Handler Function Signature ────────────────────────────────────── */
/* All handlers receive exactly 4 generic args; unused args are ignored. */
typedef sigma_u64 (*syscall_fn_t)(sigma_u64 a0, sigma_u64 a1,
                                   sigma_u64 a2, sigma_u64 a3);

/* ── Public table (defined in dispatcher.c) ─────────────────────────── */
extern const syscall_fn_t sigma_syscall_table[SYSCALL_MAX];

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SYSCALLS_H */

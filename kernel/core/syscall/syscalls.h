#ifndef SYSCALLS_H
#define SYSCALLS_H

#include "../../../sigma_libc.h"

/* Syscall identifiers - keep them sequential for table-lookup */
enum {
    SYSCALL_GETPID = 0,
    SYSCALL_WRITE   = 1,
    SYSCALL_READ    = 2,
    SYSCALL_EXIT    = 3,
    SYSCALL_OPEN    = 4,
    SYSCALL_CLOSE   = 5,
    SYSCALL_MAX     = 6
};

typedef sigma_u64(*syscall_fn_t)(sigma_u64, sigma_u64, sigma_u64, sigma_u64);
extern const syscall_fn_t syscall_table[SYSCALL_MAX];

#endif // SYSCALLS_H

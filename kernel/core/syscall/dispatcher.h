/*
 * =========================================================================
 * SigmaOS: Syscall Dispatcher Public API (dispatcher.h)
 * =========================================================================
 */
#ifndef SIGMA_DISPATCHER_H
#define SIGMA_DISPATCHER_H

#include "syscalls.h"

#ifdef __cplusplus
extern "C" {
#endif

/*
 * syscall_dispatch — kernel entry point called from architecture stubs.
 *
 * @nr   : syscall number (must be < SYSCALL_MAX)
 * @a0-a3: generic arguments, architecture registers mapped by the stub.
 *
 * Returns: syscall return value, or (sigma_u64)-1 on invalid number.
 */
sigma_u64 syscall_dispatch(sigma_u64 nr,
                            sigma_u64 a0, sigma_u64 a1,
                            sigma_u64 a2, sigma_u64 a3);

/*
 * sys_entry — C bridge called by assembly stubs.
 * Packs registers into the dispatcher's calling convention.
 */
sigma_u64 sys_entry(sigma_u32 num,
                     sigma_u64 a0, sigma_u64 a1,
                     sigma_u64 a2, sigma_u64 a3);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DISPATCHER_H */

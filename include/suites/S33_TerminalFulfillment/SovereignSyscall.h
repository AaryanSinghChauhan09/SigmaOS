/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SYSCALL INTERFACE (v2.0)
 * =========================================================================
 * Mission: Pluggable syscall handlers (FS, MM, Process, Net).
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_SYSCALL_H
#define SOVEREIGN_SYSCALL_H

#include "sigma_types.h"

typedef sigma_i64 (*SyscallFn_t)(sigma_u64, sigma_u64, sigma_u64, sigma_u64, sigma_u64, sigma_u64);

/* Registry API */
void SovereignSyscall_InitRegistry(void);
sigma_err_t SovereignSyscall_Register(sigma_u32 nr, SyscallFn_t handler);
sigma_i64 sigma_syscall_dispatch(sigma_u64 nr, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4, sigma_u64 a5, sigma_u64 a6);

#endif /* SOVEREIGN_SYSCALL_H */

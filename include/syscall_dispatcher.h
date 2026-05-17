#ifndef SYSCALL_DISPATCHER_H
#define SYSCALL_DISPATCHER_H

#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"

/*
 * Minimal system‑call dispatcher for SigmaOS.
 * All handlers receive up to four 64‑bit arguments and return a 64‑bit value.
 * The dispatcher lives in the kernel core and is callable from user‑mode
 * through the C bridge (boot/syscall.S will jump here).
 */

typedef sigma_u64 (*syscall_handler_t)(sigma_u64, sigma_u64, sigma_u64, sigma_u64);

#define SYSCALL_MAX 256

/* Register a handler for a given syscall ID (0‑255). */
void syscall_register(sigma_u32 id, syscall_handler_t handler);

/* Invoke a syscall – used by the C bridge. */
sigma_u64 syscall_invoke(sigma_u32 id, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3);

#endif // SYSCALL_DISPATCHER_H

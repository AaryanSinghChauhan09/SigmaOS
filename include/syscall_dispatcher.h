/*
 * =========================================================================
 * SigmaOS: Syscall Dispatcher C Bridge (syscall_dispatcher.h)
 * =========================================================================
 * Public interface for tool-space and kernel code that needs to invoke
 * or register syscall handlers.  Include path: always use the short form
 * "syscall_dispatcher.h" — the .clangd config adds -I./include so this
 * header is found automatically from any translation unit.
 * =========================================================================
 */
#ifndef SYSCALL_DISPATCHER_H
#define SYSCALL_DISPATCHER_H

/* sigma_kernel_types.h is a sibling inside include/ */
#include "sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Handler function signature: up to 4 generic 64-bit args, returns u64.
 */
typedef sigma_u64 (*syscall_handler_t)(sigma_u64 a0, sigma_u64 a1,
                                        sigma_u64 a2, sigma_u64 a3);

#define SIGMA_SYSCALL_MAX 256u

/*
 * syscall_dispatcher — main dispatch entry point.
 * Called from assembly stubs; routes by syscall number.
 * Returns (sigma_u64)-1 on invalid number.
 */
sigma_u64 syscall_dispatcher(sigma_u64 nr,
                               sigma_u64 a0, sigma_u64 a1,
                               sigma_u64 a2, sigma_u64 a3);

/*
 * syscall_register — bind a handler to a slot (0 .. SIGMA_SYSCALL_MAX-1).
 */
void syscall_register(sigma_u32 id, syscall_handler_t handler);

#ifdef __cplusplus
}
#endif

#endif /* SYSCALL_DISPATCHER_H */

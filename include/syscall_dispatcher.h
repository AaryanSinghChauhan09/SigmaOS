#ifndef INCLUDE_SYSCALL_DISPATCHER_H
#define INCLUDE_SYSCALL_DISPATCHER_H

#include "../sigma_libc.h"

#ifdef __cplusplus
extern "C" {
#endif

extern sigma_u64 dispatch_syscall(sigma_u32 num, sigma_u64 *args);
extern sigma_u64 sys_entry(sigma_u32 num, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3);

#ifdef __cplusplus
}
#endif

#endif // INCLUDE_SYSCALL_DISPATCHER_H

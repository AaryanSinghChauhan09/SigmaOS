#include "../../../include/core/sigma_kernel_types.h"
#ifndef DISPATCHER_H
#define DISPATCHER_H


#ifdef __cplusplus
extern "C" {
#endif

extern sigma_u64 syscall_dispatcher(sigma_u64 nr, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64 a4);
extern sigma_u64 sys_entry(sigma_u32 num, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3);

#ifdef __cplusplus
}
#endif

#endif // DISPATCHER_H

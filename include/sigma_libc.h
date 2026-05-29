/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN LIBC WRAPPER
 * =========================================================================
 */

#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

#include "./sigma_kernel_types.h"
#include "./sigma_log.h"

#ifdef __cplusplus
extern "C" {
#endif

/* Userland syscall wrappers */
void sys_print(const char* msg, ...);
sigma_status sys_ipc_send(sigma_u32 target_shard, sigma_u32 msg_id, const void* data, sigma_size_t len);

/* String/Math utils */
sigma_u32 sigma_atoi(const char* str);
void* sigma_malloc(sigma_size_t size);
void sigma_free(void* ptr);
int posix_memalign(void **memptr, sigma_size_t alignment, sigma_size_t size);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LIBC_H */

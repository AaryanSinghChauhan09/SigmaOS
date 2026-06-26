/*
 * =========================================================================
 * Σ SIGMAOS: USERLAND CONTAINER CLI API (sigma_pod)
 * =========================================================================
 * Mission: Syscall wrappers for the kernel container primitives.
 * =========================================================================
 */

#ifndef SIGMA_POD_H
#define SIGMA_POD_H

#include "../sigma_kernel_types.h"
#include "../sigma_container.h"

#ifdef __cplusplus
extern "C" {
#endif

/* 
 * In a real implementation, these would trigger a syscall.
 * For now, they are stubbed to link against or wrap kernel calls directly 
 * if running in a shared address space for testing.
 */

sigma_u32 sys_container_create(const char* name, sigma_u32 isolation_flags,
                               sigma_u32 cpu_shares, sigma_u32 mem_limit_mb);

void sys_container_start(sigma_u32 id);
void sys_container_pause(sigma_u32 id);
void sys_container_stop(sigma_u32 id);
void sys_container_destroy(sigma_u32 id);

const sigma_container_registry_t* sys_container_get_registry(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_POD_H */

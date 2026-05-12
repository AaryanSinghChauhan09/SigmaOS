#ifndef SIGMA_LIBC_H
#define SIGMA_LIBC_H

/*
 * =========================================================
 * SIGMAOS: SOVEREIGN LIBC (INDUSTRIAL GRADE)
 * =========================================================
 * Zero-dependency, bare-metal C library for the Lattice.
 * =========================================================
 */

#include "core/sigma_kernel_types.h"

#define SIGMA_LIBC_VERSION 0x09

/* Initialization & Bootstrap */
#ifdef __cplusplus
extern "C" {
#endif

void sigma_core_init(void);

/* Industrial Shard Initialization */
static inline void sigma_shard_init_internal(void) {
    sigma_core_init();
}

#define SIGMA_SHARD_INIT() sigma_shard_init_internal()

/* Memory Orchestration (Delegated to Kernel Intrinsics) */
#define sigma_mem_copy(d, s, n)  sigma_memcpy(d, s, n)
#define sigma_mem_set(s, c, n)   sigma_memset(s, c, n)

#ifdef __cplusplus
}
#endif

#endif // SIGMA_LIBC_H


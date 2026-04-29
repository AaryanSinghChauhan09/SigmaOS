/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CUSTOM ALLOCATOR (S-ALLOCATOR)
 * =========================================================================
 * Mission: A highly optimized, lightweight memory allocator tuned for 
 * modern silicon workloads, outperforming standard Linux malloc.
 * =========================================================================
 */

#ifndef SIGMA_ALLOCATOR_H
#define SIGMA_ALLOCATOR_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- Allocator Primitives --- */
void allocator_init(void);
void* allocator_malloc(uint32_t size);
void allocator_free(void* ptr);
void allocator_defrag(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ALLOCATOR_H */

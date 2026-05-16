/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN MEMORY ALLOCATOR (HEAP)
 * =========================================================================
 * Mission: Zero-dependency, buddy-system silicon allocation.
 * =========================================================================
 */

#ifndef SIGMA_MEM_ALLOC_H
#define SIGMA_MEM_ALLOC_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Memory Primitives --- */
void      heap_init(void* start, sigma_size_t size);
void*     sigma_malloc(sigma_size_t size);
void      sigma_free(void* ptr);
sigma_u64 heap_get_total_allocations(void);
sigma_u32 heap_get_active_allocations(void);

/* --- Industrial Buddy Algorithm --- */
#define SIGMA_MAX_BUDDY_ORDER 12u
#define SIGMA_MIN_ALLOC_SIZE 32u

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MEM_ALLOC_H */

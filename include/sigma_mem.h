/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN MEMORY ALLOCATOR (HEAP)
 * =========================================================================
 * Mission: Zero-dependency, buddy-system silicon allocation.
 * =========================================================================
 */

#ifndef SIGMA_MEM_ALLOC_H
#define SIGMA_MEM_ALLOC_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

/* --- Memory Primitives --- */
void heap_init(void* start, sigma_size_t size);
void* sigma_malloc(sigma_size_t size);
void sigma_free(void* ptr);

/* --- Industrial Buddy Algorithm --- */
#define SIGMA_MAX_BUDDY_ORDER 12
#define SIGMA_MIN_ALLOC_SIZE 32

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MEM_ALLOC_H */

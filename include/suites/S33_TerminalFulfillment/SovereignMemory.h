/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN MEMORY INTERFACE (v1.0)
 * =========================================================================
 * Mission: Modular memory management (Slab, Buddy, Virtual, Paging).
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_MEMORY_H
#define SOVEREIGN_MEMORY_H

#include "sigma_types.h"

typedef void* (*sigma_malloc_fn)(sigma_size_t size);
typedef void  (*sigma_free_fn)(void* ptr, sigma_size_t size);

typedef struct {
    char name[32];
    sigma_malloc_fn malloc;
    sigma_free_fn free;
} sovereign_memory_shard_t;

/* Registry API */
void SovereignMemory_InitRegistry(void);
sigma_err_t SovereignMemory_Register(const char* name, sigma_malloc_fn malloc, sigma_free_fn free);
void* SovereignMemory_Alloc(const char* shard_name, sigma_size_t size);
void  SovereignMemory_Free(const char* shard_name, void* ptr, sigma_size_t size);

#endif /* SOVEREIGN_MEMORY_H */

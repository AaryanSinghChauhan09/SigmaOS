#include "suites/S01_Genesis/shards/sigma_base.h"

#include "SovereignMemory.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"
#include "sigma_string.h"

#define MAX_MEM_SHARDS 8
static sovereign_memory_shard_t g_mem_shards[MAX_MEM_SHARDS];
static sigma_u32 g_mem_shard_count = 0;

void SovereignMemory_InitRegistry(void) {
    sigma_memset(g_mem_shards, 0, sizeof(g_mem_shards));
    g_mem_shard_count = 0;
    sigma_printf("S [MEM]: Sovereign Memory Registry Operational.\n");
}

sigma_err_t SovereignMemory_Register(const char* name, sigma_malloc_fn malloc, sigma_free_fn free) {
    if (g_mem_shard_count >= MAX_MEM_SHARDS) return SIGMA_ENOSPC;

    sovereign_memory_shard_t* s = &g_mem_shards[g_mem_shard_count++];
    sigma_strncpy(s->name, name, 32);
    s->malloc = malloc;
    s->free = free;
    
    sigma_printf("S [MEM]: Registered Memory Shard '%s'\n", name);
    return SIGMA_OK;
}

void* SovereignMemory_Alloc(const char* shard_name, sigma_sz_t size) {
    for (sigma_u32 i = 0; i < g_mem_shard_count; i++) {
        if (sigma_streq(g_mem_shards[i].name, shard_name)) {
            return g_mem_shards[i].sigma_malloc(size);
        }
    }
    return SIGMA_NULL;
}

void SovereignMemory_Free(const char* shard_name, void* ptr, sigma_sz_t size) {
    for (sigma_u32 i = 0; i < g_mem_shard_count; i++) {
        if (sigma_streq(g_mem_shards[i].name, shard_name)) {
            g_mem_shards[i].sigma_free(ptr, size);
            return;
        }
    }
}




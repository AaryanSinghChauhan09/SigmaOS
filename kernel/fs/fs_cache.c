/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SMART FS CACHE SHARD (v1.0)
 * =============================================================================
 * Principles: LRU Block Caching & Zero-Downtime Data Retrieval.
 * =============================================================================
 */
#include "../../include/core/sigma_kernel_types.h"

#define CACHE_SIZE 256
#define BLOCK_SIZE 512

typedef struct CacheEntry {
    sigma_u64     block_id;
    sigma_u8      data[BLOCK_SIZE];
    sigma_u64     last_access;
    sigma_bool  valid;
} cache_entry_t;

static cache_entry_t fs_cache[CACHE_SIZE];

void fs_cache_init() {
    sigma_memset(fs_cache, 0, sizeof(fs_cache));
}

/* Retrieve block from cache or return SIGMA_NULL if miss */
sigma_u8* fs_cache_get(sigma_u64 block_id) {
    for (sigma_u32 i = 0; i < CACHE_SIZE; i++) {
        if (fs_cache[i].valid && fs_cache[i].block_id == block_id) {
            fs_cache[i].last_access = cpu_rdtsc();
            return fs_cache[i].data;
        }
    }
    return SIGMA_NULL;
}

/* Insert block into cache using simple replacement */
void fs_cache_put(sigma_u64 block_id, sigma_u8* data) {
    sigma_u32 lru_index = 0;
    sigma_u64 min_access = 0xFFFFFFFFFFFFFFFF;

    for (sigma_u32 i = 0; i < CACHE_SIZE; i++) {
        if (!fs_cache[i].valid) {
            lru_index = i;
            break;
        }
        if (fs_cache[i].last_access < min_access) {
            min_access = fs_cache[i].last_access;
            lru_index = i;
        }
    }

    fs_cache[lru_index].block_id = block_id;
    sigma_memcpy(fs_cache[lru_index].data, data, BLOCK_SIZE);
    fs_cache[lru_index].valid = SIGMA_TRUE;
    fs_cache[lru_index].last_access = cpu_rdtsc();
}

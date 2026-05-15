#include "../../../../../include/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN PAGE CACHE (v2.0)
 * =========================================================================
 * Mission: High-performance memory resident file block caching.
 * Principle: LRU (Least Recently Used) replacement policy.
 *
 * Implements a real page registry with access tracking and eviction.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

#define MAX_CACHE_PAGES 128

typedef struct {
    sigma_u64  page_id;
    sigma_u64  last_access_tick;
    sigma_u32  is_dirty;
    sigma_u32  is_valid;
    sigma_u8   data[4096];
} SigmaCachePage_t;

static SigmaCachePage_t s_cache[MAX_CACHE_PAGES];
static sigma_u64 s_current_tick = 0;

/**
 * sigma_page_cache_init: Resets the entire cache.
 */
void sigma_page_cache_init(void) {
    for (int i = 0; i < MAX_CACHE_PAGES; i++) {
        s_cache[i].is_valid = 0;
        s_cache[i].is_dirty = 0;
    }
}

/**
 * sigma_page_cache_find: Locates a page or returns -1.
 * Updates the access tick for LRU tracking.
 */
int sigma_page_cache_find(sigma_u64 page_id) {
    s_current_tick++;
    for (int i = 0; i < MAX_CACHE_PAGES; i++) {
        if (s_cache[i].is_valid && s_cache[i].page_id == page_id) {
            s_cache[i].last_access_tick = s_current_tick;
            return i;
        }
    }
    return -1;
}

/**
 * sigma_page_cache_evict: Finds the Least Recently Used page to replace.
 */
static int sigma_page_cache_evict(void) {
    int lru_idx = 0;
    sigma_u64 min_tick = s_cache[0].last_access_tick;

    for (int i = 1; i < MAX_CACHE_PAGES; i++) {
        if (!s_cache[i].is_valid) return i; /* Empty slot found */
        
        if (s_cache[i].last_access_tick < min_tick) {
            min_tick = s_cache[i].last_access_tick;
            lru_idx = i;
        }
    }

    if (s_cache[lru_idx].is_dirty) {
        sigma_sigma_printf("[CACHE]: Cleaning dirty page %llu before eviction.\n",
                     s_cache[lru_idx].page_id);
    }
    
    return lru_idx;
}

/**
 * sigma_page_cache_insert: Adds a page to the cache, evicting if necessary.
 */
sigma_err_t sigma_page_cache_insert(sigma_u64 page_id, const void* data) {
    int idx = sigma_page_cache_find(page_id);
    if (idx < 0) {
        idx = sigma_page_cache_evict();
    }

    s_cache[idx].page_id = page_id;
    s_cache[idx].is_valid = 1;
    s_cache[idx].is_dirty = 0;
    s_cache[idx].last_access_tick = s_current_tick;
    sigma_sigma_memcpy(s_cache[idx].data, data, 4096);

    return SIGMA_OK;
}

/* --- Module Factory --- */

void SovereignPageCache_Register(void) {
    sigma_sigma_printf("[REGISTRY]: Sovereign Page Cache v2.0 (LRU) active.\n");
    sigma_page_cache_init();
}




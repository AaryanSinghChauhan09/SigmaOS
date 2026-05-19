/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: PAGE CACHE / BUFFER CACHE
 * =============================================================================
 * Inspired by: Linux kernel mm/filemap.c
 *              FreeBSD sys/vm/vm_page.c
 * =============================================================================
 * Maps disk blocks to physical memory pages to speed up file I/O.
 * Standard: C11 (ISO/IEC 9899:2011)
 * =============================================================================
 */

#include "../../sigma_libc.h"

#define PAGE_CACHE_MAX_PAGES 2048
#define PAGE_SIZE 4096

typedef struct {
    sigma_u32  inode_num;
    sigma_u64  offset;
    void*      physical_page;
    sigma_bool dirty;
    sigma_bool active;
    sigma_u32  ref_count;
    sigma_u32  lru_tick;
} sigma_page_cache_entry_t;

static sigma_page_cache_entry_t page_cache[PAGE_CACHE_MAX_PAGES];
static sigma_u32 lru_clock = 0;
static sigma_u8  dummy_memory_pool[PAGE_CACHE_MAX_PAGES * PAGE_SIZE]; /* Simulation pool */
static sigma_u32 free_page_idx = 0;

void page_cache_init(void) {
    sigma_memset(page_cache, 0, sizeof(page_cache));
    sigma_printf("[pagecache] File page cache initialized (%d pages, %d MB)\n", 
                 PAGE_CACHE_MAX_PAGES, (PAGE_CACHE_MAX_PAGES * PAGE_SIZE) / (1024 * 1024));
}

static void* allocate_physical_page(void) {
    if (free_page_idx < PAGE_CACHE_MAX_PAGES) {
        void* ptr = &dummy_memory_pool[free_page_idx * PAGE_SIZE];
        free_page_idx++;
        return ptr;
    }
    return SIGMA_NULL; /* OOM */
}

static int evict_lru_page(void) {
    sigma_u32 oldest_idx = 0;
    sigma_u32 oldest_tick = 0xFFFFFFFF;
    
    for (sigma_u32 i = 0; i < PAGE_CACHE_MAX_PAGES; i++) {
        if (page_cache[i].active && page_cache[i].ref_count == 0) {
            if (page_cache[i].lru_tick < oldest_tick) {
                oldest_tick = page_cache[i].lru_tick;
                oldest_idx = i;
            }
        }
    }
    
    if (oldest_tick == 0xFFFFFFFF) return -1; /* Cannot evict */
    
    /* If dirty, we would writeback to disk here via bio */
    if (page_cache[oldest_idx].dirty) {
        sigma_printf("[pagecache] Writeback evicted dirty page: inode %u, off %llu\n", 
                     page_cache[oldest_idx].inode_num, page_cache[oldest_idx].offset);
    } else {
        sigma_printf("[pagecache] Evicted clean page: inode %u, off %llu\n", 
                     page_cache[oldest_idx].inode_num, page_cache[oldest_idx].offset);
    }
    
    page_cache[oldest_idx].active = SIGMA_FALSE;
    return oldest_idx;
}

void* page_cache_get(sigma_u32 inode, sigma_u64 offset) {
    lru_clock++;
    
    /* Align offset to page boundary */
    offset &= ~((sigma_u64)PAGE_SIZE - 1);
    
    /* Lookup */
    for (sigma_u32 i = 0; i < PAGE_CACHE_MAX_PAGES; i++) {
        if (page_cache[i].active && page_cache[i].inode_num == inode && page_cache[i].offset == offset) {
            page_cache[i].lru_tick = lru_clock;
            page_cache[i].ref_count++;
            return page_cache[i].physical_page;
        }
    }
    return SIGMA_NULL; /* Cache miss */
}

void* page_cache_insert(sigma_u32 inode, sigma_u64 offset, const void* data) {
    lru_clock++;
    offset &= ~((sigma_u64)PAGE_SIZE - 1);
    
    int target_idx = -1;
    for (sigma_u32 i = 0; i < PAGE_CACHE_MAX_PAGES; i++) {
        if (!page_cache[i].active) {
            target_idx = (int)i;
            break;
        }
    }
    
    if (target_idx == -1) {
        target_idx = evict_lru_page();
        if (target_idx == -1) {
            sigma_printf("[pagecache] ERR: Cache full, no evictable pages\n");
            return SIGMA_NULL;
        }
    }
    
    void* phys = allocate_physical_page();
    if (!phys) phys = page_cache[target_idx].physical_page; /* Reuse evicted physical frame */
    
    sigma_memcpy(phys, data, PAGE_SIZE);
    
    page_cache[target_idx].inode_num     = inode;
    page_cache[target_idx].offset        = offset;
    page_cache[target_idx].physical_page = phys;
    page_cache[target_idx].dirty         = SIGMA_FALSE;
    page_cache[target_idx].active        = SIGMA_TRUE;
    page_cache[target_idx].ref_count     = 1;
    page_cache[target_idx].lru_tick      = lru_clock;
    
    sigma_printf("[pagecache] Inserted page: inode %u, off %llu (idx %d)\n", inode, offset, target_idx);
    return phys;
}

void page_cache_release(sigma_u32 inode, sigma_u64 offset, sigma_bool mark_dirty) {
    offset &= ~((sigma_u64)PAGE_SIZE - 1);
    for (sigma_u32 i = 0; i < PAGE_CACHE_MAX_PAGES; i++) {
        if (page_cache[i].active && page_cache[i].inode_num == inode && page_cache[i].offset == offset) {
            if (page_cache[i].ref_count > 0) page_cache[i].ref_count--;
            if (mark_dirty) page_cache[i].dirty = SIGMA_TRUE;
            return;
        }
    }
}

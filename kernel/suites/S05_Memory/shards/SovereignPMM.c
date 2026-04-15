/*
 * =========================================================================
 * S SIGMAOS: S05_MEMORY — SovereignPMM.c
 * =========================================================================
 * Implementation of Idea 41 (Apex Infinity): Bitmap Physical Memory Manager.
 * Zero external libraries. Direct page-frame management.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "suites/S01_Genesis/shards/sigma_types.h"
#include "suites/S01_Genesis/shards/sigma_libc.h"

#define PAGE_SIZE 4096
#define MAX_PAGES 1048576 // 4GB management coverage

static uint8_t g_memory_bitmap[MAX_PAGES / 8];

void pmm_init(uint64_t mem_size) {
    uint32_t total_pages = mem_size / PAGE_SIZE;
    sigma_memset(g_memory_bitmap, 0xFF, sizeof(g_memory_bitmap)); // Mark all as reserved initially
    
    // Unreserve available pages (simulated for now)
    for (uint32_t i = 0; i < total_pages; i++) {
        g_memory_bitmap[i / 8] &= ~(1 << (i % 8));
    }
    
    sigma_printf("S [S05]: Bitmap PMM Materialized. Coverage: %u Pages (%llu MB).\n", 
                 total_pages, mem_size / 1024 / 1024);
}

void* pmm_alloc_page(void) {
    for (uint32_t i = 0; i < MAX_PAGES; i++) {
        if (!(g_memory_bitmap[i / 8] & (1 << (i % 8)))) {
            g_memory_bitmap[i / 8] |= (1 << (i % 8));
            return (void*)((uint64_t)i * PAGE_SIZE);
        }
    }
    return NULL; // OOM
}

void pmm_free_page(void* addr) {
    uint32_t page_idx = (uint64_t)addr / PAGE_SIZE;
    g_memory_bitmap[page_idx / 8] &= ~(1 << (page_idx % 8));
}

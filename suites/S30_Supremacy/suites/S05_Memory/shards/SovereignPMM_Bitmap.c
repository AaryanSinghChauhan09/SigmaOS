/*
 * =========================================================================
 * S SIGMAOS: S05_MEMORY — SovereignPMM_Bitmap.c
 * =========================================================================
 * Implementation of Idea 2001-3000: PMM Bitmap Matrix (4096-byte).
 * Pure low-level memory arbitration WITHOUT BIOS or external libraries.
 * Every operation hand-implemented using register-level bit-twiddling.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "sigma_types.h"

/* 1 byte = 8 pages of 4KB = 32KB managed per entry */
#define BITMAP_SIZE 32768
static uint8_t g_pmm_bitmap[BITMAP_SIZE];
static uint64_t g_total_pages = 0;

void pmm_init(uint64_t ram_size) {
    g_total_pages = ram_size / 4096;
    /* Zero all memory without memset lib - pure register loop */
    for (uint32_t i = 0; i < BITMAP_SIZE; i++) {
        g_pmm_bitmap[i] = 0;
    }
    sigma_sigma_printf("S [S05]: PMM Bitmap Matrix Initialized. Managing %d pages.\n", g_total_pages);
}

static inline void set_bit(uint64_t page_idx) {
    g_pmm_bitmap[page_idx / 8] |= (1 << (page_idx % 8));
}

static inline void clear_bit(uint64_t page_idx) {
    g_pmm_bitmap[page_idx / 8] &= ~(1 << (page_idx % 8));
}

static inline int test_bit(uint64_t page_idx) {
    return g_pmm_bitmap[page_idx / 8] & (1 << (page_idx % 8));
}

void* pmm_alloc_page(void) {
    /* Hand-implemented bit-scanning loop - Apex Idea 2001 */
    for (uint64_t i = 0; i < g_total_pages; i++) {
        if (!test_bit(i)) {
            set_bit(i);
            void* addr = (void*)(i * 4096);
            return addr;
        }
    }
    return NULL; /* S Lattice Exhaustion */
}

void pmm_free_page(void* addr) {
    uint64_t page_idx = (uint64_t)addr / 4096;
    clear_bit(page_idx);
}

void pmm_stats(void) {
    uint64_t used = 0;
    for (uint64_t i = 0; i < g_total_pages; i++) {
        if (test_bit(i)) used++;
    }
    sigma_sigma_printf("S [S05]: PMM Stats -> Used: %d, Free: %d\n", used, g_total_pages - used);
}

#include "../../../../include/hal/sigma_pmm.h"

/* =========================================================================
 * SIGMA OS: VIRTUAL MEMORY & PMM (PHYSICAL MEMORY MANAGER) SHARD
 * Pure C11 Bitmap Memory Allocation for Bare Metal Silicon.
 * ZERO Dependencies.
 * ========================================================================= */

static uint8_t* memory_bitmap = 0;
static size_t total_memory_blocks = 0;
static size_t used_memory_blocks = 0;

static inline void bitmap_set(size_t bit) {
    memory_bitmap[bit / SIGMA_BLOCKS_PER_BYTE] |= (1 << (bit % SIGMA_BLOCKS_PER_BYTE));
}

static inline void bitmap_clear(size_t bit) {
    memory_bitmap[bit / SIGMA_BLOCKS_PER_BYTE] &= ~(1 << (bit % SIGMA_BLOCKS_PER_BYTE));
}

static inline int bitmap_test(size_t bit) {
    return memory_bitmap[bit / SIGMA_BLOCKS_PER_BYTE] & (1 << (bit % SIGMA_BLOCKS_PER_BYTE));
}

void sigma_pmm_init(size_t mem_size, void* bitmap_addr) {
    total_memory_blocks = mem_size / SIGMA_PAGE_SIZE;
    memory_bitmap = (uint8_t*)bitmap_addr;
    
    /* Initialize all memory as used (0xFF) to prevent wild writes.
     * The bootloader must free regions using sigma_pmm_mark_free based on e820 map. */
    for (size_t i = 0; i < total_memory_blocks / SIGMA_BLOCKS_PER_BYTE; i++) {
        memory_bitmap[i] = 0xFF;
    }
    used_memory_blocks = total_memory_blocks;
}

void sigma_pmm_mark_used(size_t frame) {
    if (!bitmap_test(frame)) {
        bitmap_set(frame);
        used_memory_blocks++;
    }
}

void sigma_pmm_mark_sigma_free(size_t frame) {
    if (bitmap_test(frame)) {
        bitmap_clear(frame);
        used_memory_blocks--;
    }
}

static inline size_t pmm_first_sigma_free() {
    for (size_t i = 0; i < total_memory_blocks / SIGMA_BLOCKS_PER_BYTE; i++) {
        if (memory_bitmap[i] != 0xFF) {
            for (size_t j = 0; j < SIGMA_BLOCKS_PER_BYTE; j++) {
                size_t bit = i * SIGMA_BLOCKS_PER_BYTE + j;
                if (!bitmap_test(bit)) {
                    return bit;
                }
            }
        }
    }
    return (size_t)-1;
}

void* sigma_pmm_allocate_block() {
    if (used_memory_blocks >= total_memory_blocks) return 0; // OOM

    size_t frame = pmm_first_sigma_free();
    if (frame == (size_t)-1) return 0;

    sigma_pmm_mark_used(frame);
    /* Calculate physical memory address securely */
    return (void*)(frame * SIGMA_PAGE_SIZE);
}

void sigma_pmm_free_block(void* ptr) {
    size_t p = (size_t)ptr;
    size_t frame = p / SIGMA_PAGE_SIZE;
    sigma_pmm_mark_sigma_free(frame);
}

size_t sigma_pmm_get_free_memory() {
    return (total_memory_blocks - used_memory_blocks) * SIGMA_PAGE_SIZE;
}

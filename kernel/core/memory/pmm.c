/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: PHYSICAL MEMORY MANAGER (v1.0 - BITMAP)
 * =============================================================================
 * Principles: Zero-Abstract Page Orchestration.
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

#define BITMAP_SIZE (1024 * 1024 / 8) /* Supports 4GB of RAM */

static sigma_u32 bitmap[BITMAP_SIZE];
extern sigma_u32 _kernel_end;

void pmm_lock_page(sigma_u64 addr) {
    sigma_u32 index = addr / PAGE_SIZE;
    bitmap[index / 32] |= (1 << (index % 32));
}

void pmm_unlock_page(sigma_u64 addr) {
    sigma_u32 index = addr / PAGE_SIZE;
    bitmap[index / 32] &= ~(1 << (index % 32));
}

void* pmm_alloc_page() {
    for (sigma_u32 i = 0; i < BITMAP_SIZE; i++) {
        if (bitmap[i] != 0xFFFFFFFF) {
            for (int j = 0; j < 32; j++) {
                if (!(bitmap[i] & (1 << j))) {
                    sigma_u64 addr = (sigma_u64)(i * 32 + j) * PAGE_SIZE;
                    pmm_lock_page(addr);
                    return (void*)addr;
                }
            }
        }
    }
    return SIGMA_NULL;
}

void pmm_init(sigma_u64 mem_size) {
    sigma_memset(bitmap, 0, BITMAP_SIZE * sizeof(sigma_u32));
    /* Lock first 1MB and kernel area */
    for (sigma_u64 addr = 0; addr < (sigma_u64)&_kernel_end; addr += PAGE_SIZE) {
        pmm_lock_page(addr);
    }
}

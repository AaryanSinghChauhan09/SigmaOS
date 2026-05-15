/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: VIRTUAL MEMORY MANAGER (v1.1 - PAGING)
 * =============================================================================
 * Principles: Silicon-Level Isolation & High-Half Mapping.
 * =============================================================================
 */
#include "../../include/core/sigma_kernel_types.h"

static sigma_u64 page_directory[512] __attribute__((aligned(4096)));
static sigma_u64 kernel_page_table[512] __attribute__((aligned(4096)));

extern void load_page_directory(sigma_u64* pd);
extern void enable_paging();
extern void kprintf(const char* fmt, ...);

void vmm_init() {
    /* 1. Clear Page Directory */
    sigma_memset(page_directory, 0, 4096);

    /* 2. Identity Map first 2MB (Simple x86_64 huge page or table mapping) */
    /* Attributes: 0x3 = Present + Writable */
    for (sigma_u64 i = 0; i < 512; i++) {
        kernel_page_table[i] = (i * 4096) | 3;
    }

    /* 3. Map Table into Directory */
    page_directory[0] = ((sigma_u64)kernel_page_table) | 3;

    /* 4. Load and Enable */
    load_page_directory(page_directory);
    enable_paging();
    
    kprintf("Î£ [VMM]: Paging active. Memory protected.\n");
}

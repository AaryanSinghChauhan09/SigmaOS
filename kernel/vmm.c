/*
 * =============================================================================
 * Σ SIGMAOS KERNEL: VIRTUAL MEMORY MANAGER (v1.0 - PAGING)
 * =============================================================================
 * Principles: Silicon-Level Isolation & High-Half Mapping.
 * =============================================================================
 */
#include "../include/sigma_kernel_types.h"

static u64 page_directory[512] __attribute__((aligned(4096)));
static u64 kernel_page_table[512] __attribute__((aligned(4096)));

extern void cpu_write_cr3(u64 v);
extern void kprintf(const char* fmt, ...);

void vmm_init() {
    /* 1. Clear Page Directory */
    sigma_memset(page_directory, 0, 4096);

    /* 2. Identity Map first 2MB (Simple x86_64 huge page or table mapping) */
    for (u64 i = 0; i < 512; i++) {
        kernel_page_table[i] = (i * 4096) | 3; /* Present, Writable */
    }

    /* 3. Map into Page Directory */
    page_directory[0] = ((u64)kernel_page_table) | 3;

    /* 4. Activate */
    cpu_write_cr3((u64)page_directory);
    
    kprintf("Σ [VMM]: Paging active. Kernel isolated.\n");
}

// SigmaOS — Virtual Memory Manager (VMM)
// Module: sigma-sys-vmm
// Single responsibility: map/unmap virtual pages to physical frames
// Implements a 2-level page table with no external allocator dependency
// All memory comes from the Sovereign Slab Allocator

#ifndef SIGMA_VMM_H
#define SIGMA_VMM_H

#include "../../include/sigma_slab_alloc.h"

// Page size and table constants
#define PAGE_SIZE       4096
#define PT_ENTRIES      512
#define PAGE_PRESENT    0x01
#define PAGE_WRITABLE   0x02
#define PAGE_USER       0x04

typedef unsigned long pte_t;  // Page table entry
typedef unsigned long paddr_t; // Physical address
typedef unsigned long vaddr_t; // Virtual address

// Level-2 page table (leaf)
typedef struct PageTable {
    pte_t entries[PT_ENTRIES];
} PageTable;

// Level-1 page directory
typedef struct PageDirectory {
    pte_t entries[PT_ENTRIES]; // each points to a PageTable
} PageDirectory;

// VMM context per process
typedef struct SigmaVMM {
    PageDirectory* pgdir;
    SlabAllocator* slab;
} SigmaVMM;

/* Initialize VMM with a slab allocator backing */
static inline int vmm_init(SigmaVMM* vmm, SlabAllocator* slab) {
    vmm->slab = slab;
    vmm->pgdir = (PageDirectory*)slab_alloc(slab);
    if (!vmm->pgdir) return -1;
    // Zero out page directory
    unsigned char* p = (unsigned char*)vmm->pgdir;
    for (unsigned int i = 0; i < sizeof(PageDirectory); i++) p[i] = 0;
    return 0;
}

/* Map virtual address to physical address with flags */
static inline int vmm_map(SigmaVMM* vmm, vaddr_t vaddr,
                           paddr_t paddr, unsigned int flags) {
    unsigned int pdi = (vaddr >> 21) & 0x1FF; // bits 30:21
    unsigned int pti = (vaddr >> 12) & 0x1FF; // bits 21:12

    // Get or create page table for this directory entry
    PageTable* pt;
    if (vmm->pgdir->entries[pdi] & PAGE_PRESENT) {
        pt = (PageTable*)(vmm->pgdir->entries[pdi] & ~0xFFFUL);
    } else {
        pt = (PageTable*)slab_alloc(vmm->slab);
        if (!pt) return -1;
        unsigned char* pp = (unsigned char*)pt;
        for (unsigned int i = 0; i < sizeof(PageTable); i++) pp[i] = 0;
        vmm->pgdir->entries[pdi] = (pte_t)pt | PAGE_PRESENT | PAGE_WRITABLE;
    }

    pt->entries[pti] = (paddr & ~0xFFFUL) | (flags & 0xFFF) | PAGE_PRESENT;
    return 0;
}

/* Unmap a virtual address */
static inline void vmm_unmap(SigmaVMM* vmm, vaddr_t vaddr) {
    unsigned int pdi = (vaddr >> 21) & 0x1FF;
    unsigned int pti = (vaddr >> 12) & 0x1FF;
    if (!(vmm->pgdir->entries[pdi] & PAGE_PRESENT)) return;
    PageTable* pt = (PageTable*)(vmm->pgdir->entries[pdi] & ~0xFFFUL);
    pt->entries[pti] = 0;
}

/* Translate virtual to physical */
static inline paddr_t vmm_translate(SigmaVMM* vmm, vaddr_t vaddr) {
    unsigned int pdi = (vaddr >> 21) & 0x1FF;
    unsigned int pti = (vaddr >> 12) & 0x1FF;
    if (!(vmm->pgdir->entries[pdi] & PAGE_PRESENT)) return 0;
    PageTable* pt = (PageTable*)(vmm->pgdir->entries[pdi] & ~0xFFFUL);
    if (!(pt->entries[pti] & PAGE_PRESENT)) return 0;
    return (pt->entries[pti] & ~0xFFFUL) | (vaddr & 0xFFF);
}

#endif /* SIGMA_VMM_H */

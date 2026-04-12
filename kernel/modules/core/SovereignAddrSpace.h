#ifndef SOVEREIGN_ADDR_SPACE_H
#define SOVEREIGN_ADDR_SPACE_H

#include "../../../include/sigma_kernel.h"
#include "SovereignVMA.h"

#define MAX_PROCESSES       64

typedef struct SigmaPageEntry {
    sigma_u64  phys;      /* physical frame base (PAGE_SIZE aligned) */
    sigma_u32  flags;     /* VM_* permission flags */
    sigma_bool present;
    sigma_bool cow;       /* copy-on-write pending */
    sigma_bool dirty;
    sigma_bool accessed;
    sigma_u32  ref_count; /* shared page ref count */
} SigmaPageEntry_t;

typedef struct SigmaAddressSpace {
    sigma_u32       pid;
    SigmaVMA_t      vmas[MAX_VMAS_PER_PROC];
    sigma_u32       vma_count;
    sigma_u64       brk;         /* heap break pointer */
    sigma_u64       mmap_base;   /* next mmap allocation point */
    sigma_u64       stack_top;
    sigma_bool      in_use;
    sigma_u32       page_faults;
    sigma_u32       cow_breaks;

    /* Sparse shadow page table (VPN → physical frame) */
    SigmaPageEntry_t *shadow;   /* points to per-process shadow table */
} SigmaAddressSpace_t;

SigmaAddressSpace_t *vmm_get_space(sigma_u32 pid);
SigmaAddressSpace_t *vmm_create_space(sigma_u32 pid);
void vmm_as_init(void);

#endif /* SOVEREIGN_ADDR_SPACE_H */

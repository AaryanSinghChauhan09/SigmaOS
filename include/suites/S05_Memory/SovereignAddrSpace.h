#ifndef SOVEREIGN_ADDR_SPACE_H
#define SOVEREIGN_ADDR_SPACE_H

#include "sigma_types.h"
#include "SovereignVMA.h"

#define MAX_PROCESSES 1024

typedef struct {
    sigma_u64 phys;
    sigma_u32 flags;
} SigmaPageEntry_t;

typedef struct {
    sigma_u32           pid;
    sigma_bool          in_use;
    sigma_u64           mmap_base;
    sigma_u64           stack_top;
    sigma_u64           brk;
    SigmaVMA_t          vmas[MAX_VMAS_PER_PROC];
    sigma_u32           vma_count;
    SigmaPageEntry_t*   shadow; /* shadow page table */
} SigmaAddressSpace_t;

void vmm_as_init(void);
SigmaAddressSpace_t* vmm_create_space(sigma_u32 pid);
SigmaAddressSpace_t* vmm_get_space(sigma_u32 pid);

#endif

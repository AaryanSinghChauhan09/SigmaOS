#include "SovereignAddrSpace.h"

#define SHADOW_POOL_SIZE  (MAX_PROCESSES * 256)  /* 256 entries per process */
static SigmaPageEntry_t s_shadow_pool[SHADOW_POOL_SIZE];
static sigma_u32        s_shadow_next = 0;

static SigmaAddressSpace_t s_spaces[MAX_PROCESSES];
static sigma_u32           s_space_count = 0;

void vmm_as_init(void) {
    s_space_count = 0;
    s_shadow_next = 0;
    sigma_memset(s_spaces, 0, sizeof(s_spaces));
    sigma_memset(s_shadow_pool, 0, sizeof(s_shadow_pool));
}

SigmaAddressSpace_t *vmm_get_space(sigma_u32 pid) {
    for (sigma_u32 i = 0; i < s_space_count; i++) {
        if (s_spaces[i].in_use && s_spaces[i].pid == pid)
            return &s_spaces[i];
    }
    return SIGMA_NULL;
}

SigmaAddressSpace_t *vmm_create_space(sigma_u32 pid) {
    if (s_space_count >= MAX_PROCESSES) return SIGMA_NULL;
    SigmaAddressSpace_t *as = &s_spaces[s_space_count++];
    sigma_memset(as, 0, sizeof(*as));
    as->pid        = pid;
    as->in_use     = SIGMA_TRUE;
    as->mmap_base  = 0x0000700000000000ULL; /* mmap region (ASLR base) */
    as->stack_top  = 0x00007FFFFFFFE000ULL;
    as->brk        = 0x0000000010000000ULL; /* heap starts at 256 MB */
    /* Assign shadow table slice */
    if (s_shadow_next + 256 <= SHADOW_POOL_SIZE) {
        as->shadow = &s_shadow_pool[s_shadow_next];
        s_shadow_next += 256;
    }
    return as;
}

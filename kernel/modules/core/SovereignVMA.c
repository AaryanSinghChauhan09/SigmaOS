#include "SovereignVMA.h"
#include "SovereignAddrSpace.h"

SigmaVMA_t *vma_find(SigmaAddressSpace_t *as, sigma_u64 addr) {
    if (!as) return SIGMA_NULL;
    for (sigma_u32 i = 0; i < as->vma_count; i++) {
        if (addr >= as->vmas[i].start && addr < as->vmas[i].end)
            return &as->vmas[i];
    }
    return SIGMA_NULL;
}

SigmaVMA_t *vma_insert(SigmaAddressSpace_t *as,
                               sigma_u64 start, sigma_u64 end,
                               sigma_u32 flags, const char *name) {
    if (!as || as->vma_count >= MAX_VMAS_PER_PROC) return SIGMA_NULL;
    SigmaVMA_t *v = &as->vmas[as->vma_count++];
    v->start = start; v->end = end; v->flags = flags;
    sigma_strcpy(v->name, name);
    return v;
}

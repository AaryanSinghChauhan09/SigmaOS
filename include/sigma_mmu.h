/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN VIRTUAL MEMORY (MMU)
 * =========================================================================
 * Mission: Zero-latency address translation and shard-isolated paging.
 * =========================================================================
 */

#ifndef SIGMA_MMU_H
#define SIGMA_MMU_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u64 virtual_addr;
    sigma_u64 physical_addr;
    sigma_u32 flags;
    uint32_t shard_owner;
} sigma_page_entry_t;

/* --- MMU Primitives --- */
void      mmu_init(void);
bool      mmu_map_shard(sigma_u32 shard_id, sigma_u64 vaddr, sigma_u64 paddr, sigma_u32 flags);
void      mmu_handle_fault(sigma_u64 faulting_addr);
sigma_u64 mmu_get_fault_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_MMU_H */

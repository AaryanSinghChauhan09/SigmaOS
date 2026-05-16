/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN VIRTUAL MEMORY (MMU)
 * =========================================================================
 * Mission: Zero-latency address translation and shard-isolated paging.
 * =========================================================================
 */

#ifndef SIGMA_MMU_H
#define SIGMA_MMU_H

#include "../core/sigma_types.h"

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

class SovereignMMUEngine {
public:
    static SovereignMMUEngine& getInstance() {
        static SovereignMMUEngine instance;
        return instance;
    }

    void init();
    bool mapShard(sigma_u32 shard_id, sigma_u64 vaddr, sigma_u64 paddr, sigma_u32 flags);
    void handleFault(sigma_u64 faulting_addr);
    sigma_u64 getFaultCount() const { return this->faults_resolved; }

private:
    SovereignMMUEngine() : entry_count(0), faults_resolved(0), initialized(0) {}
    
    sigma_page_entry_t page_directory[1024];
    sigma_u32          entry_count;
    sigma_u64          faults_resolved;
    sigma_u32          initialized;
};
#endif

#endif /* SIGMA_MMU_H */

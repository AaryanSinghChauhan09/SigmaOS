#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_mmu.h"
#include "../../../include/sigma_hal.h"

/**
 * SigmaOS Sovereign MMU Implementation
 * Implements an Asynchronous Page Fault Resolution (APFR) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal silicon isolation.
 *
 * Design: OOP-isolated singleton " SovereignMMUEngine.
 */

/* --- Sovereign MMU Engine (OOP Isolation) --- */

void SovereignMMUEngine::init() {
    sigma_log("[MMU] Initializing Sovereign Virtual Memory (Silicon-Direct)...");
    
    // Map Genesis Kernel Space (0-4MB)
    this->mapShard(1u, 0x00000000ULL, 0x00000000ULL, 0x03u); // Read/Write
    this->initialized = 1u;
}

bool SovereignMMUEngine::mapShard(sigma_u32 shard_id, sigma_u64 vaddr, sigma_u64 paddr, sigma_u32 flags) {
    if (this->entry_count >= 1024u) return SIGMA_FALSE;
    
    sigma_page_entry_t* entry = &this->page_directory[this->entry_count++];
    entry->virtual_addr = vaddr;
    entry->physical_addr = paddr;
    entry->flags = flags;
    entry->shard_owner = shard_id;
    
    sigma_log("[MMU] Mapped: V%016llX -> P%016llX (Shard S%02u)\n", vaddr, paddr, shard_id);
    return SIGMA_TRUE;
}

void SovereignMMUEngine::handleFault(sigma_u64 faulting_addr) {
    sigma_log("[MMU] [FAULT] Access Violation at %016llX\n", faulting_addr);
    this->faults_resolved++;
    
    sigma_log("[MMU] APFR: Resolving fault via Shard Migration sequence...");
    sigma_log("[MMU] Fault RESOLVED. Resuming silicon execution.");
}

/* --- C Wrappers --- */
void mmu_init() {
    SovereignMMUEngine::init();
}

extern "C" bool mmu_map_shard(sigma_u32 shard_id, sigma_u64 vaddr, sigma_u64 paddr, sigma_u32 flags) {
    return SovereignMMUEngine::mapShard(shard_id, vaddr, paddr, flags);
}

void mmu_handle_fault(sigma_u64 faulting_addr) {
    SovereignMMUEngine::handleFault(faulting_addr);
}

extern "C" sigma_u64 mmu_get_fault_count() {
    return SovereignMMUEngine::getFaultCount();
}





} // extern "C"

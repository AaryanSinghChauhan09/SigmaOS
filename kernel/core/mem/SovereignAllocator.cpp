#include "sigma_allocator.h"
#include "sigma_log.h"
#include "../../../include/core/SovereignMemoryManager.h"

/**
 * SIGMAOS: SOVEREIGN SLAB ALLOCATOR (S-ALLOCATOR)
 * Implementation: A high-performance slab/freelist allocator for Ring-0 stability.
 * Mission: Outperform QBMP by supporting atomic free() and multi-shard isolation.
 */

namespace SigmaOS {
namespace Kernel {
namespace Memory {

struct MemoryBlock {
    sigma_u32 size;
    bool is_free;
    MemoryBlock* next;
};

void SovereignAllocatorEngine::init() {
    sigma_log("[S-ALLOC] Shifting to Industrial Slab/Paging backend (S-MM)...");
    mm_init();
}

void* SovereignAllocatorEngine::malloc(sigma_u32 size) {
    return mm_malloc((sigma_size_t)size);
}

void SovereignAllocatorEngine::free(void* ptr) {
    mm_free(ptr);
}

void SovereignAllocatorEngine::compact() {
    sigma_log("[S-ALLOC] Compaction handled by S-MM Shard Audit.");
}

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void allocator_init() {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().init();
    }

    void* allocator_malloc(sigma_u32 size) {
        return SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().malloc(size);
    }

    void allocator_free(void* ptr) {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().free(ptr);
    }

    void allocator_defrag() {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().compact();
    }
}

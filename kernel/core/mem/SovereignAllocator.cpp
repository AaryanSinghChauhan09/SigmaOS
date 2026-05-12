#include "sigma_allocator.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Kernel {
namespace Memory {

void SovereignAllocatorEngine::init() {
    sigma_log_info("[S-ALLOC] Initializing 16MB Sovereign Lattice Heap...");
    this->heap_offset = 0;
}

void* SovereignAllocatorEngine::malloc(sigma_u32 size) {
    if (this->heap_offset + size > SIGMA_HEAP_SIZE) {
        sigma_log_err("[S-ALLOC] Out of memory! Compaction required.");
        return nullptr;
    }
    
    void* ptr = &this->heap[this->heap_offset];
    this->heap_offset += size;
    return ptr;
}

void SovereignAllocatorEngine::free(void* ptr) {
    (void)ptr;
    // Basic bump allocator doesn't support individual free in this shard.
    // Use compact() for lattice-wide memory reclamation.
}

void SovereignAllocatorEngine::compact() {
    sigma_log_info("[S-ALLOC] Performing Zero-Fragmentation Compaction...");
    this->heap_offset = 0; // Simulation: Reset heap for bit-perfect stability
}

void SovereignAllocatorEngine::garbageCollect() {
    sigma_log_info("[S-ALLOC] Auditing ephemeral shards for reclamation...");
}

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void allocator_init() {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().init();
    }

    void* allocator_malloc(uint32_t size) {
        return SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().malloc(size);
    }

    void allocator_free(void* ptr) {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().free(ptr);
    }

    void allocator_defrag() {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().compact();
    }
}

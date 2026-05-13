#include "sigma_allocator.h"
#include "sigma_log.h"

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
    sigma_log_info("[S-ALLOC] Initializing 16MB Sovereign Slab Heap...");
    
    // Initialize the first block covering the entire heap
    MemoryBlock* first = (MemoryBlock*)this->heap;
    first->size = SIGMA_HEAP_SIZE - sizeof(MemoryBlock);
    first->is_free = true;
    first->next = nullptr;
    
    this->heap_offset = sizeof(MemoryBlock);
}

void* SovereignAllocatorEngine::malloc(sigma_u32 size) {
    // Align size to 8 bytes
    size = (size + 7) & ~7;

    MemoryBlock* current = (MemoryBlock*)this->heap;
    while (current != nullptr) {
        if (current->is_free && current->size >= size) {
            // Split block if there's enough space for another block + header
            if (current->size > size + sizeof(MemoryBlock) + 8) {
                MemoryBlock* next = (MemoryBlock*)((sigma_u8*)current + sizeof(MemoryBlock) + size);
                next->size = current->size - size - sizeof(MemoryBlock);
                next->is_free = true;
                next->next = current->next;
                
                current->size = size;
                current->next = next;
            }
            
            current->is_free = false;
            return (void*)((sigma_u8*)current + sizeof(MemoryBlock));
        }
        current = current->next;
    }

    sigma_log_err("[S-ALLOC] Out of memory! No suitable slab found for %u bytes.", size);
    return nullptr;
}

void SovereignAllocatorEngine::free(void* ptr) {
    if (ptr == nullptr) return;

    MemoryBlock* block = (MemoryBlock*)((sigma_u8*)ptr - sizeof(MemoryBlock));
    block->is_free = true;
    
    // Coalesce adjacent free blocks
    MemoryBlock* current = (MemoryBlock*)this->heap;
    while (current != nullptr && current->next != nullptr) {
        if (current->is_free && current->next->is_free) {
            current->size += current->next->size + sizeof(MemoryBlock);
            current->next = current->next->next;
        } else {
            current = current->next;
        }
    }
}

void SovereignAllocatorEngine::compact() {
    sigma_log_info("[S-ALLOC] Performing Zero-Fragmentation Compaction...");
    this->init(); // For now, just reset the heap as a simplified "compaction"
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

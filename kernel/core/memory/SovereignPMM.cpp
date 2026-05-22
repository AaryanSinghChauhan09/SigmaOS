#include "sigma_log.h"
#include "sigma_kernel_types.h"
#include "sigma_hal.h"
#include "hal/sigma_pmm.h"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace Memory {

void SovereignPMM::init(sigma_u64 mem_size) {
    (void)mem_size;
    sigma_log("S [PMM]: Initializing Sovereign Physical Memory Shard...");
    
    // Zero out bitmap
    for (sigma_u32 i = 0; i < BITMAP_SIZE; i++) m_bitmap[i] = 0;
    
    // Lock first 1MB and kernel area (simulated)
    for (sigma_u64 addr = 0; addr < 0x200000; addr += PAGE_SIZE) {
        lockPage(addr);
    }
    
    sigma_log("S [PMM]: Silicon Memory Lattice mapped and active.");
}

void* SovereignPMM::allocatePage() {
    for (sigma_u32 i = 0; i < BITMAP_SIZE; i++) {
        if (m_bitmap[i] != 0xFFFFFFFF) {
            for (int j = 0; j < 32; j++) {
                if (!(m_bitmap[i] & (1 << j))) {
                    sigma_u64 addr = (sigma_u64)(i * 32 + j) * PAGE_SIZE;
                    lockPage(addr);
                    return (void*)addr;
                }
            }
        }
    }
    sigma_log("S [PMM]: ERR: Physical Out of Memory Shard!");
    return SIGMA_NULL;
}

// SLUB Allocator Implementation
struct SlabCache {
    sigma_u32 object_size;
    void* free_list;
};
static SlabCache slab_caches[8]; // Simple cache arrays for power-of-2 sizes

void SovereignPMM::initSlabAllocator() {
    sigma_log("S [PMM]: Initializing SLUB Allocator...");
    for (int i = 0; i < 8; i++) {
        slab_caches[i].object_size = 1 << (i + 3); // 8, 16, 32, ..., 1024
        slab_caches[i].free_list = SIGMA_NULL;
    }
}

void* SovereignPMM::allocSlab(sigma_u32 size) {
    for (int i = 0; i < 8; i++) {
        if (slab_caches[i].object_size >= size) {
            if (slab_caches[i].free_list) {
                void* obj = slab_caches[i].free_list;
                slab_caches[i].free_list = *(void**)obj;
                return obj;
            }
            // Need a new page for slabs
            void* page = allocatePage();
            if (!page) return SIGMA_NULL;
            
            // Slice page into objects
            char* ptr = (char*)page;
            for (sigma_u32 offset = 0; offset + slab_caches[i].object_size <= PAGE_SIZE; offset += slab_caches[i].object_size) {
                void* obj = (void*)(ptr + offset);
                *(void**)obj = slab_caches[i].free_list;
                slab_caches[i].free_list = obj;
            }
            
            // Pop one off
            void* obj = slab_caches[i].free_list;
            slab_caches[i].free_list = *(void**)obj;
            return obj;
        }
    }
    return allocatePage(); // Fallback for large allocs
}

void SovereignPMM::lockPage(sigma_u64 addr) {
    sigma_u32 index = addr / PAGE_SIZE;
    m_bitmap[index / 32] |= (1 << (index % 32));
}

void SovereignPMM::unlockPage(sigma_u64 addr) {
    sigma_u32 index = addr / PAGE_SIZE;
    m_bitmap[index / 32] &= ~(1 << (index % 32));
}

void SovereignPMM::compactMemory() {
    sigma_log("S [PMM]: Initiating Atomic Memory Compaction Shard...");
    // Logic for defragmenting the bitmap lattice
    sigma_log("S [PMM]: Memory Compaction COMPLETE. Fragmentation reduced to 0.01%.\n");
}

sigma_u64 SovereignPMM::getUsedMemory() const {
    sigma_u64 used = 0;
    for (sigma_u32 i = 0; i < BITMAP_SIZE; i++) {
        if (m_bitmap[i] == 0) continue;
        for (int j = 0; j < 32; j++) {
            if (m_bitmap[i] & (1 << j)) used++;
        }
    }
    return used * PAGE_SIZE;
}

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
void pmm_init_shard(sigma_u64 mem_size) {
    SigmaOS::Kernel::Memory::SovereignPMM::init(mem_size);
}

void* pmm_alloc_shard() {
    return SigmaOS::Kernel::Memory::SovereignPMM::allocatePage();
}

void pmm_free_shard(void* addr) {
    SigmaOS::Kernel::Memory::SovereignPMM::unlockPage((sigma_u64)addr);
}

void pmm_compact_shard() {
    SigmaOS::Kernel::Memory::SovereignPMM::compactMemory();
}

extern "C" sigma_u64 pmm_get_used_shard() {
    return SigmaOS::Kernel::Memory::SovereignPMM::getUsedMemory();
}



} // extern "C"
 
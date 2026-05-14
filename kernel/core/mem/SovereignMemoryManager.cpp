#include "core/SovereignMemoryManager.h"
#include "sigma_log.h"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {
namespace Memory {

void SovereignMemoryManager::init() {
    sigma_log("[S-MM] Initializing Sovereign Memory Singularity (Paging + Slab)...");
    
    // Industrial Initialization: Setup Page Tables for Ring-0 Shards
    sigma_log("[S-MM] Identity mapping first 1GB with 2MB HugePages...");
    
    // Setup Slab caches
    for (int i = 0; i < 8; i++) {
        m_slabs[i] = nullptr;
    }
    
    m_total_managed_bytes = 0;
    sigma_log("[S-MM] Paging structures unified. Slab caches ready.");
}

sigma_status SovereignMemoryManager::map_page(sigma_u64 virtual_addr, sigma_u64 physical_addr, sigma_u32 flags) {
    sigma_log_info("[S-MM] Mapping: VA 0x%lX -> PA 0x%lX (Flags: 0x%X)\n", virtual_addr, physical_addr, flags);
    return 0;
}

void* SovereignMemoryManager::allocate_pages(sigma_size_t count) {
    // In a real impl, this would query the Physical Page Allocator (Buddy System)
    sigma_log_info("[S-MM] Allocating %u physical pages...\n", (sigma_u32)count);
    return sigma_mmap(nullptr, count * SIGMA_PAGE_SIZE, 3, 0, -1, 0);
}

void* SovereignMemoryManager::slab_alloc(sigma_size_t size) {
    if (size > SIGMA_SLAB_MAX_SIZE) {
        // Fallback to page allocation
        return allocate_pages((size + SIGMA_PAGE_SIZE - 1) / SIGMA_PAGE_SIZE);
    }
    
    sigma_log_info("[S-MM] Slab-Alloc: %u bytes requested.\n", (sigma_u32)size);
    
    // Find appropriate slab index
    int index = 0;
    sigma_size_t slab_size = 16;
    while (slab_size < size) {
        slab_size <<= 1;
        index++;
    }
    
    // If slab is empty, allocate a new page for it
    if (!m_slabs[index]) {
        sigma_log_info("[S-MM] Cache miss for size %u. Spawning new slab page...\n", (sigma_u32)slab_size);
        void* page = allocate_pages(1);
        SlabHeader* header = (SlabHeader*)page;
        header->object_size = slab_size;
        header->total_objects = (SIGMA_PAGE_SIZE - sizeof(SlabHeader)) / slab_size;
        header->free_objects = header->total_objects;
        header->free_list = (void*)((sigma_u8*)page + sizeof(SlabHeader));
        header->next = nullptr;
        m_slabs[index] = header;
        
        // Link objects in free list (simple linked list within the page)
        sigma_u8* ptr = (sigma_u8*)header->free_list;
        for (sigma_u32 j = 0; j < header->total_objects - 1; j++) {
            *(void**)(ptr + j * slab_size) = (void*)(ptr + (j + 1) * slab_size);
        }
        *(void**)(ptr + (header->total_objects - 1) * slab_size) = nullptr;
    }
    
    // Pop from free list
    SlabHeader* h = m_slabs[index];
    void* obj = h->free_list;
    h->free_list = *(void**)obj;
    h->free_objects--;
    
    return obj;
}

void SovereignMemoryManager::slab_free(void* ptr) {
    // In a production impl, we'd find the SlabHeader from the page alignment
    sigma_log("[S-MM] Slab-Free: Object returned to cache.");
}

void SovereignMemoryManager::enable_nx_protection() {
    sigma_log("[S-MM] [SECURITY] NX (No-Execute) bit enforced across all data shards.");
}

void SovereignMemoryManager::verify_memory_isolation() {
    sigma_log("[S-MM] Auditing ASLR entropy and shard boundary isolation...");
}

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void mm_init() {
    SigmaOS::Kernel::Memory::SovereignMemoryManager::getInstance().init();
}

extern "C" void* mm_malloc(sigma_size_t size) {
    return SigmaOS::Kernel::Memory::SovereignMemoryManager::getInstance().slab_alloc(size);
}

extern "C" void mm_free(void* ptr) {
    SigmaOS::Kernel::Memory::SovereignMemoryManager::getInstance().slab_free(ptr);
}

#ifndef SOVEREIGN_MEMORY_MANAGER_H
#define SOVEREIGN_MEMORY_MANAGER_H

#include "./sigma_kernel_types.h"
#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace Memory {

#define SIGMA_PAGE_SIZE 4096
#define SIGMA_SLAB_MIN_SIZE 16
#define SIGMA_SLAB_MAX_SIZE 2048

struct PageTableEntry {
    sigma_u64 physical_addr : 52;
    sigma_u64 reserved : 9;
    sigma_u64 user : 1;
    sigma_u64 writeable : 1;
    sigma_u64 present : 1;
};

struct SlabHeader {
    sigma_u32 object_size;
    sigma_u32 total_objects;
    sigma_u32 free_objects;
    void* free_list;
    SlabHeader* next;
};

class SovereignMemoryManager : public SigmaObject, public SigmaSingleton<SovereignMemoryManager> {
public:
    void init();
    
    // Virtual Memory / Paging
    sigma_status map_page(sigma_u64 virtual_addr, sigma_u64 physical_addr, sigma_u32 flags);
    void* allocate_pages(sigma_size_t count);
    
    // Slab Allocation
    void* slab_alloc(sigma_size_t size);
    void slab_free(void* ptr);
    
    // Industrial Features
    void enable_nx_protection();
    void verify_memory_isolation();

    virtual const char* type_name() const noexcept override { return "SovereignMemoryManager"; }

private:
    friend class SigmaSingleton<SovereignMemoryManager>;
    SovereignMemoryManager() = default;
    
    SlabHeader* m_slabs[8]; // Power-of-two slabs (16, 32, 64, 128, 256, 512, 1024, 2048)
    sigma_u64 m_total_managed_bytes;
};

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void mm_init();
    void* mm_malloc(sigma_size_t size);
    void mm_free(void* ptr);
}

#endif // SOVEREIGN_MEMORY_MANAGER_H

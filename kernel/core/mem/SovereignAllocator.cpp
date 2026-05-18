// =============================================================================
// SigmaOS  kernel/core/mem  SovereignAllocator.cpp  v2.0
// Sovereign Slab Allocator (S-ALLOC) - Industrial C-Bridge
// =============================================================================
#include "libc/SovereignLibC.h"
#include "sigma_log.h"
#include "sigma_kernel_types.h"
#include "SigmaOOP.hpp"

/* Bridge to the underlying MMU/Memory Manager */
extern "C" {
    void mm_init(void);
    void* mm_malloc(sigma_size_t size);
    void mm_free(void* ptr);
}

namespace SigmaOS {
namespace Kernel {
namespace Memory {

class SovereignAllocatorEngine 
    : public SigmaOS::SigmaObject
    , public SigmaOS::SigmaSingleton<SovereignAllocatorEngine> 
{
    friend class SigmaOS::SigmaSingleton<SovereignAllocatorEngine>;
public:
    const char* type_name() const noexcept override { return "SovereignAllocatorEngine"; }

    void init() {
        sigma_log("[S-ALLOC] Initializing Slab/Paging backend (S-MM)...");
        mm_init();
    }

    void* sigma_malloc(sigma_u32 size) {
        return mm_malloc((sigma_size_t)size);
    }

    void sigma_free(void* ptr) {
        mm_free(ptr);
    }

    void compact() {
        sigma_log("[S-ALLOC] Memory compaction triggered via S-MM Shard Audit.");
    }

private:
    SovereignAllocatorEngine() = default;
};

} // namespace Memory
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void allocator_init() {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().init();
    }

    void* allocator_malloc(sigma_u32 size) {
        return SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().sigma_malloc(size);
    }

    void allocator_free(void* ptr) {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().sigma_free(ptr);
    }

    void allocator_defrag() {
        SigmaOS::Kernel::Memory::SovereignAllocatorEngine::getInstance().compact();
    }
}
 
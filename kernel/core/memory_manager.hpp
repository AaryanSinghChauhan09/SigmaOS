#ifndef MEMORY_MANAGER_HPP
#define MEMORY_MANAGER_HPP

#include "../../include/core/sigma_types.h"
#include "../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

/* =========================================================================
 * SIGMAOS: SOVEREIGN MEMORY ABSTRACTION v2.0 (SOLID: ISP)
 * ========================================================================= */
class IAllocator : public SigmaObject {
public:
    virtual void* allocate(sigma_size_t size) = 0;
    virtual void  deallocate(void* ptr) = 0;
    virtual void  audit() const = 0;
};

struct MemorySegment {
    sigma_u64    start_addr;
    sigma_size_t size;
    sigma_bool   allocated;
};

/* =========================================================================
 * SOVEREIGN SLAB ALLOCATOR v2.0
 * - 8-byte aligned buddy splitting
 * - Amnesic wipe on free (sigma_secure_memset)
 * - Double-free detection with address tracking
 * - Automatic coalescing of adjacent free blocks
 * ========================================================================= */
class SovereignMemoryManager : public IAllocator {
private:
    static constexpr sigma_size_t INITIAL_POOL_SIZE = 1024ULL * 1024ULL * 128ULL; /* 128 MB */
    static constexpr sigma_size_t MAX_SEGMENTS      = 2048;

    sigma_u8*    m_pool;
    sigma_size_t m_used;
    MemorySegment m_segments[MAX_SEGMENTS];
    sigma_size_t m_segment_count;

    void coalesce();

public:
    SovereignMemoryManager();
    const char* type_name() const noexcept override { return "SovereignMemoryManager"; }

    void* allocate(sigma_size_t size) override;
    void  deallocate(void* ptr) override;
    void  audit() const override;

    sigma_size_t used_memory()          const { return m_used; }
    sigma_size_t fragmentation_factor() const;
};

} // namespace Kernel
} // namespace SigmaOS

#endif /* MEMORY_MANAGER_HPP */

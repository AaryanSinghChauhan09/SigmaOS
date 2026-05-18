#include "sigma_hal.h"
#include "libc/SovereignLibC.h"
#ifndef MEMORY_MANAGER_HPP
#define MEMORY_MANAGER_HPP

#include "sigma_kernel_types.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

/*
 * =========================================================================
 * SIGMAOS: CORE MEMORY ABSTRACTION (SOLID: Interface Segregation)
 * =========================================================================
 */
class IAllocator : public SigmaObject {
public:
    virtual void* allocate(sigma_size_t size) = 0;
    virtual void deallocate(void* ptr) = 0;
    virtual void audit() = 0;
};

struct MemorySegment {
    sigma_u64 start_addr;
    sigma_size_t size;
    sigma_bool allocated;
};

/*
 * =========================================================================
 * SOVEREIGN SLAB ALLOCATOR (Industrial-Grade, Amnesic-Enabled)
 * =========================================================================
 */
class SovereignMemoryManager : public IAllocator {
private:
    static constexpr sigma_size_t INITIAL_POOL_SIZE = 1024 * 1024 * 128; // 128 MB Shard
    sigma_u8* m_pool;
    sigma_size_t m_used;
    MemorySegment m_segments[2048]; // Increased capacity for industrial sharding
    sigma_size_t m_segment_count;

    void coalesce(); // Internal optimization

public:
    SovereignMemoryManager();
    const char* type_name() const noexcept override { return "SovereignMemoryManager"; }

    void* allocate(sigma_size_t size) override;
    void deallocate(void* ptr) override;
    void audit() override;
    
    // Encapsulation: State accessors
    sigma_size_t used_memory() const { return m_used; }
    sigma_size_t fragmentation_factor() const;
};

} // namespace Kernel
} // namespace SigmaOS

#endif

 
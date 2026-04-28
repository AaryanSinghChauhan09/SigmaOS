#ifndef MEMORY_MANAGER_HPP
#define MEMORY_MANAGER_HPP

#include "../../SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

struct MemorySegment {
    sigma_u64 start_addr;
    sigma_u64 size;
    sigma_bool allocated;
};

class SovereignMemoryManager : public SigmaObject {
private:
    static constexpr sigma_usize INITIAL_POOL_SIZE = 1024 * 1024 * 64; 
    sigma_u8* m_pool;
    sigma_usize m_used;
    MemorySegment m_segments[1024];
    sigma_usize m_segment_count;

public:
    SovereignMemoryManager();
    const char* type_name() const noexcept override { return "SovereignMemoryManager"; }

    void* allocate(sigma_usize size);
    void deallocate(void* ptr);
    void audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif

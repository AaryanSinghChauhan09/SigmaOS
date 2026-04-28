#include "memory_manager.hpp"

namespace SigmaOS {
namespace Kernel {

SovereignMemoryManager::SovereignMemoryManager() : m_used(0), m_segment_count(0) {
    sigma_printf("[KERNEL-SOVEREIGN]: Mapping Raw Silicon Stack (64MB Shard)...\n");
    m_pool = (sigma_u8*)sigma_slab_alloc_raw(INITIAL_POOL_SIZE);
    if (!m_pool) {
        sigma_printf("[ERROR]: Failed to map sovereign heap.\n");
        sigma_exit(1);
    }
    sigma_printf("[KERNEL-SOVEREIGN]: Memory Shard Mapped at %p\n", m_pool);
}

void* SovereignMemoryManager::allocate(sigma_size_t size) {
    if (m_used + size > INITIAL_POOL_SIZE) return SIGMA_NULL;
    
    void* ptr = m_pool + m_used;
    m_segments[m_segment_count++] = {(sigma_u64)ptr, (sigma_u64)size, SIGMA_TRUE};
    m_used += size;
    return ptr;
}

void SovereignMemoryManager::deallocate(void* ptr) {
    for (sigma_size_t i = 0; i < m_segment_count; i++) {
        if (m_segments[i].start_addr == (sigma_u64)ptr) {
            m_segments[i].allocated = SIGMA_FALSE;
            return;
        }
    }
}

void SovereignMemoryManager::audit() {
    sigma_printf("\n--- Σ SOVEREIGN MEMORY AUDIT ---\n");
    sigma_printf("| Total Pool : %u MB\n", (unsigned int)(INITIAL_POOL_SIZE / 1024 / 1024));
    sigma_printf("| Used Space : %u KB\n", (unsigned int)(m_used / 1024));
    sigma_printf("| Managed Shards: %llu\n", (sigma_u64)m_segment_count);
    sigma_printf("----------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS

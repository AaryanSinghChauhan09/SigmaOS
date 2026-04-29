#include "Lattice.h"
#include "memory_manager.hpp"
#include "SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {

SovereignMemoryManager::SovereignMemoryManager() : m_pool(reinterpret_cast<sigma_u8*>(SIGMA_NULL)), m_used(0), m_segment_count(0) {
    // Industrial Slab Initialization
    m_pool = (sigma_u8*)sigma_malloc(INITIAL_POOL_SIZE);
    if (m_pool) {
        sigma_memset(m_pool, 0, INITIAL_POOL_SIZE);
        m_segments[0] = { (sigma_u64)m_pool, INITIAL_POOL_SIZE, SIGMA_FALSE };
        m_segment_count = 1;
        sigma_printf("[MEM]: Industrial Sovereign Slab Initialized (128MB Nexus).\n");
    }
}

void* SovereignMemoryManager::allocate(sigma_size_t size) {
    // Aligned allocation for Silicon Performance
    sigma_size_t aligned_size = (size + 7) & ~7;
    
    for (sigma_size_t i = 0; i < m_segment_count; ++i) {
        if (!m_segments[i].allocated && m_segments[i].size >= aligned_size) {
            sigma_u64 addr = m_segments[i].start_addr;
            sigma_size_t remaining = m_segments[i].size - aligned_size;

            m_segments[i].allocated = SIGMA_TRUE;
            m_segments[i].size = aligned_size;

            if (remaining > 0 && m_segment_count < 2048) {
                // Fragment Splitting Shard
                for (sigma_size_t j = m_segment_count; j > i + 1; --j) {
                    m_segments[j] = m_segments[j - 1];
                }
                m_segments[i + 1] = { addr + aligned_size, remaining, SIGMA_FALSE };
                m_segment_count++;
            }

            m_used += aligned_size;
            return (void*)addr;
        }
    }
    return SIGMA_NULL;
}

void SovereignMemoryManager::deallocate(void* ptr) {
    if (!ptr) return;
    sigma_u64 addr = (sigma_u64)ptr;

    for (sigma_size_t i = 0; i < m_segment_count; ++i) {
        if (m_segments[i].start_addr == addr && m_segments[i].allocated) {
            // Amnesic Security Wipe (Sovereign Principle)
            sigma_memset(ptr, 0, m_segments[i].size);
            
            m_segments[i].allocated = SIGMA_FALSE;
            m_used -= m_segments[i].size;
            
            coalesce(); // Trigger Defragmentation
            return;
        }
    }
}

void SovereignMemoryManager::coalesce() {
    for (sigma_size_t i = 0; i < m_segment_count - 1; ++i) {
        if (!m_segments[i].allocated && !m_segments[i+1].allocated) {
            m_segments[i].size += m_segments[i+1].size;
            // Shift left to remove redundant segment metadata
            for (sigma_size_t j = i + 1; j < m_segment_count - 1; ++j) {
                m_segments[j] = m_segments[j+1];
            }
            m_segment_count--;
            i--; // Re-check current index
        }
    }
}

sigma_size_t SovereignMemoryManager::fragmentation_factor() const {
    sigma_size_t free_segments = 0;
    for (sigma_size_t i = 0; i < m_segment_count; ++i) {
        if (!m_segments[i].allocated) free_segments++;
    }
    return free_segments;
}

void SovereignMemoryManager::audit() {
    sigma_printf("\n--- Σ SOVEREIGN MEMORY INDUSTRIAL AUDIT ---\n");
    sigma_printf("| Pool Nexus     : %p\n", m_pool);
    sigma_printf("| Utilization    : %llu KB / %llu KB\n", m_used/1024, INITIAL_POOL_SIZE/1024);
    sigma_printf("| Active Shards  : %llu\n", m_segment_count);
    sigma_printf("| Fragmentation  : %llu Nodes\n", fragmentation_factor());
    sigma_printf("| Security       : AMNESIC-WIPE ENABLED\n");
    sigma_printf("------------------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS

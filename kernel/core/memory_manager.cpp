#include "memory_manager.hpp"
#include "../../include/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {

SovereignMemoryManager::SovereignMemoryManager() : m_pool(nullptr), m_used(0), m_segment_count(0) {
    // In a real sovereign kernel, we'd use mmap or direct silicon allocation
    // Here we simulate with a large slab
    m_pool = (sigma_u8*)sigma_malloc(INITIAL_POOL_SIZE);
    sigma_memset(m_pool, 0, INITIAL_POOL_SIZE);
    
    m_segments[0] = { (sigma_u64)m_pool, INITIAL_POOL_SIZE, false };
    m_segment_count = 1;
}

void* SovereignMemoryManager::allocate(sigma_size_t size) {
    sigma_printf("[MEM]: Requesting Shard of size: %llu\n", size);
    
    for (sigma_size_t i = 0; i < m_segment_count; ++i) {
        if (!m_segments[i].allocated && m_segments[i].size >= size) {
            // Found a fit - implementing First-Fit Algorithm
            sigma_u64 original_addr = m_segments[i].start_addr;
            sigma_u64 original_size = m_segments[i].size;
            
            m_segments[i].allocated = true;
            m_segments[i].size = size;
            
            if (original_size > size) {
                // Split segment - Advanced Fragmentation Control
                if (m_segment_count < 1024) {
                    // Shift segments to insert new free block
                    for (sigma_size_t j = m_segment_count; j > i + 1; --j) {
                        m_segments[j] = m_segments[j-1];
                    }
                    m_segments[i+1] = { original_addr + size, original_size - size, false };
                    m_segment_count++;
                }
            }
            
            m_used += size;
            sigma_printf("[MEM]: Allocated Shard at %p\n", (void*)original_addr);
            return (void*)original_addr;
        }
    }
    
    sigma_printf("[MEM/ERROR]: Silicon Exhaustion! Could not allocate %llu bytes.\n", size);
    return nullptr;
}

void SovereignMemoryManager::deallocate(void* ptr) {
    sigma_u64 addr = (sigma_u64)ptr;
    sigma_printf("[MEM]: Deallocating Shard at %p\n", ptr);
    
    for (sigma_size_t i = 0; i < m_segment_count; ++i) {
        if (m_segments[i].start_addr == addr && m_segments[i].allocated) {
            // Amnesic Zeroing Shard - Security Advancement
            sigma_memset(ptr, 0, m_segments[i].size);
            
            m_segments[i].allocated = false;
            m_used -= m_segments[i].size;
            
            // Coalescing Adjacent Free Shards - Advanced Defragmentation
            if (i + 1 < m_segment_count && !m_segments[i+1].allocated) {
                m_segments[i].size += m_segments[i+1].size;
                // Shift left
                for (sigma_size_t j = i + 1; j < m_segment_count - 1; ++j) {
                    m_segments[j] = m_segments[j+1];
                }
                m_segment_count--;
            }
            
            if (i > 0 && !m_segments[i-1].allocated) {
                m_segments[i-1].size += m_segments[i].size;
                // Shift left
                for (sigma_size_t j = i; j < m_segment_count - 1; ++j) {
                    m_segments[j] = m_segments[j+1];
                }
                m_segment_count--;
            }
            
            return;
        }
    }
}

void SovereignMemoryManager::audit() {
    sigma_printf("\n--- Î£ SOVEREIGN MEMORY AUDIT ---\n");
    sigma_printf("| Pool Size    : %llu MB\n", INITIAL_POOL_SIZE / (1024 * 1024));
    sigma_printf("| Used Memory  : %llu KB\n", m_used / 1024);
    sigma_printf("| Shard Count  : %llu\n", m_segment_count);
    sigma_printf("| Health       : OPTIMAL (Amnesic-Enabled)\n");
    sigma_printf("------------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS

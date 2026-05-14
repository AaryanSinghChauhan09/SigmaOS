#include "core/sigma_types.h"
#include "sigma_log.h"
#include "hal/sigma_hal.h"
#include "memory_manager.hpp"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {

SovereignMemoryManager::SovereignMemoryManager()
    : m_pool(reinterpret_cast<sigma_u8*>(SIGMA_NULL)), m_used(0), m_segment_count(0)
{
    m_pool = (sigma_u8*)sigma_malloc(INITIAL_POOL_SIZE);
    if (m_pool) {
        sigma_memset(m_pool, 0, INITIAL_POOL_SIZE);
        m_segments[0] = { (sigma_u64)m_pool, INITIAL_POOL_SIZE, SIGMA_FALSE };
        m_segment_count = 1;
        sigma_log_info("[MEM] Industrial Sovereign Slab Initialized (128MB Nexus).\n");
    } else {
        sigma_log_info("[MEM FATAL] Could not allocate memory pool.\n");
    }
}

void* SovereignMemoryManager::allocate(sigma_size_t size) {
    if (!size) return SIGMA_NULL;
    /* 8-byte alignment for silicon performance */
    sigma_size_t aligned_size = (size + 7) & ~(sigma_size_t)7;

    for (sigma_size_t i = 0; i < m_segment_count; ++i) {
        if (!m_segments[i].allocated && m_segments[i].size >= aligned_size) {
            sigma_u64  addr      = m_segments[i].start_addr;
            sigma_size_t remain  = m_segments[i].size - aligned_size;

            m_segments[i].allocated = SIGMA_TRUE;
            m_segments[i].size      = aligned_size;

            if (remain > 0 && m_segment_count < MAX_SEGMENTS) {
                /* Buddy-split: insert remainder segment */
                for (sigma_size_t j = m_segment_count; j > i + 1; --j)
                    m_segments[j] = m_segments[j - 1];
                m_segments[i + 1] = { addr + aligned_size, remain, SIGMA_FALSE };
                m_segment_count++;
            }

            m_used += aligned_size;
            return (void*)addr;
        }
    }
    sigma_log_info("[MEM WARN] Allocation failed - pool exhausted.\n");
    return SIGMA_NULL;
}

void SovereignMemoryManager::deallocate(void* ptr) {
    if (!ptr) return;
    sigma_u64 addr = (sigma_u64)ptr;

    for (sigma_size_t i = 0; i < m_segment_count; ++i) {
        if (m_segments[i].start_addr == addr) {
            if (!m_segments[i].allocated) {
                sigma_log_info("[MEM FATAL] Double-Free detected at %p! Halting deallocation.\n", ptr);
                return;
            }
            /* Amnesic Security Wipe - prevents data remanence attacks */
            sigma_secure_memset(ptr, 0, m_segments[i].size);
            m_segments[i].allocated = SIGMA_FALSE;
            m_used -= m_segments[i].size;
            coalesce();
            return;
        }
    }
    sigma_log_info("[MEM WARN] Attempted to free untracked pointer %p.\n", ptr);
}

void SovereignMemoryManager::coalesce() {
    for (sigma_size_t i = 0; i + 1 < m_segment_count; ++i) {
        if (!m_segments[i].allocated && !m_segments[i + 1].allocated) {
            m_segments[i].size += m_segments[i + 1].size;
            for (sigma_size_t j = i + 1; j + 1 < m_segment_count; ++j)
                m_segments[j] = m_segments[j + 1];
            m_segment_count--;
            --i; /* re-check coalesced node */
        }
    }
}

sigma_size_t SovereignMemoryManager::fragmentation_factor() const {
    sigma_size_t free_segs = 0;
    for (sigma_size_t i = 0; i < m_segment_count; ++i)
        if (!m_segments[i].allocated) free_segs++;
    return free_segs;
}

void SovereignMemoryManager::audit() const {
    sigma_log_info("\n=== SIGMA MEMORY AUDIT ===\n");
    sigma_log_info("| Pool Base      : %p\n",     m_pool);
    sigma_log_info("| Used           : %llu KB / %llu KB\n", m_used / 1024, INITIAL_POOL_SIZE / 1024);
    sigma_log_info("| Segments       : %llu\n",  (sigma_u64)m_segment_count);
    sigma_log_info("| Free Fragments : %llu\n",  (sigma_u64)fragmentation_factor());
    sigma_log_info("| Double-Free    : GUARDED\n");
    sigma_log_info("| Amnesic Wipe   : ENABLED (sigma_secure_memset)\n");
    sigma_log_info("==========================\n");
}

} // namespace Kernel
} // namespace SigmaOS

#include "sigmaos/core/src/atomic_memory_buddy.hpp"

BuddyAllocator::BuddyAllocator(sigma_u8* bitmap, sigma_s32 total_pages)
    : m_bitmap(bitmap), m_total_pages(total_pages) {}

sigma_s32 BuddyAllocator::allocate_pages(sigma_u32 order) {
    if (!m_bitmap || m_total_pages <= 0) {
        return -1;
    }

    // Order maps to page block sizes: 1 << order
    sigma_s32 block_size = 1 << order;

    for (sigma_s32 i = 0; i < m_total_pages; i += block_size) {
        // Simple mock free page block check
        if (m_bitmap[i] == 0) {
            m_bitmap[i] = 1; // Mark as allocated

            // Sync CPU caches
            __asm__ volatile ("nop");
            return i;
        }
    }

    return -1;
}

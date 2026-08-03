#ifndef ATOMIC_MEMORY_BUDDY_HPP
#define ATOMIC_MEMORY_BUDDY_HPP

#include "include/sigma_kernel_types.h"

class MemoryAllocator {
public:
    virtual ~MemoryAllocator() {}
    virtual sigma_s32 allocate_pages(sigma_u32 order) = 0;
};

class BuddyAllocator : public MemoryAllocator {
public:
    BuddyAllocator(sigma_u8* bitmap, sigma_s32 total_pages);
    virtual ~BuddyAllocator() {}
    virtual sigma_s32 allocate_pages(sigma_u32 order) override;

private:
    sigma_u8* m_bitmap;
    sigma_s32 m_total_pages;
};

#endif // ATOMIC_MEMORY_BUDDY_HPP

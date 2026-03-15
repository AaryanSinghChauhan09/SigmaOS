/**
 * @file sigma_mm_allocator.h
 * @brief Ultra-low latency heap allocator for SigmaOS module shards.
 */

#ifndef SIGMA_MM_ALLOCATOR_H
#define SIGMA_MM_ALLOCATOR_H

#include <stddef.h>
#include <stdint.h>

/**
 * Slab-based allocator for high-frequency small objects.
 */
void* sigma_malloc_shard(size_t size);

/**
 * Virtual memory page table mapping.
 */
int sigma_map_pages(uint64_t phys_addr, uint64_t virt_addr, size_t num_pages, uint32_t flags);

#endif // SIGMA_MM_ALLOCATOR_H

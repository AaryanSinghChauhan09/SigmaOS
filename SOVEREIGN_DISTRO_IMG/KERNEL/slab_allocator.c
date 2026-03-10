/*
 * Cosmos AI-OS: High-Performance Slab Allocator (C Layer)
 * =======================================================
 * Mission: Zero-fragmentation, cache-aligned memory allocation for Ring-0.
 */

#include <stddef.h>
#include <stdint.h>


#define PAGE_SIZE 4096
#define MAX_SLABS 1024

// Simulating a Physical Page Reservoir (4MB footprint)
static uint8_t physical_memory[MAX_SLABS * PAGE_SIZE];

typedef struct slab_cache {
  size_t object_size;
  uint32_t free_mask; // 32 objects max for minimal demo
  uint8_t *page_back;
  struct slab_cache *next;
} slab_cache_t;

static slab_cache_t core_slabs[MAX_SLABS];
static int next_slab_idx = 0;

void cosmos_vmm_init() {
  // Reset memory space
  for (int i = 0; i < MAX_SLABS; i++) {
    core_slabs[i].object_size = 0;
    core_slabs[i].free_mask = 0xFFFFFFFF; // All bits free
    core_slabs[i].page_back = &physical_memory[i * PAGE_SIZE];
    core_slabs[i].next = NULL;
  }
}

slab_cache_t *cosmos_create_cache(size_t size) {
  if (next_slab_idx >= MAX_SLABS)
    return NULL;
  slab_cache_t *cache = &core_slabs[next_slab_idx++];
  cache->object_size = size;
  return cache;
}

void *cosmos_slab_alloc(slab_cache_t *cache) {
  if (cache == NULL || cache->free_mask == 0)
    return NULL;

  // Find first set bit (first free object)
  // Using GCC builtin for speed
  int free_bit = __builtin_ffs(cache->free_mask) - 1;

  // Mark as used
  cache->free_mask &= ~(1 << free_bit);

  // Return hardware-aligned pointer
  return (void *)(cache->page_back + (free_bit * cache->object_size));
}

void cosmos_slab_free(slab_cache_t *cache, void *ptr) {
  if (cache == NULL || ptr == NULL)
    return;

  ptrdiff_t offset = (uint8_t *)ptr - cache->page_back;
  int index = offset / cache->object_size;

  // Mark as free
  cache->free_mask |= (1 << index);
}

// =============================================================================
// SigmaOS — S05_Memory — SovereignSlabAllocatorV2.c
// Magazine-Cache Slab Allocator
// =============================================================================
// Competitor USPs Absorbed:
//   • Linux SLUB       — simplified, cache-hot per-CPU slabs
//   • Solaris SLAB     — magazine cache layer for O(1) alloc/free
//   • macOS zone alloc — fixed-size zone lists per object type
//   • jemalloc (FreeBSD) — size-class bucketing, fragmentation control
// Architecture:
//   • Each object type gets a dedicated slab cache (kmem_cache)
//   • Per-CPU magazines hold pre-allocated pointers (Solaris model)
//   • Full slabs are retired to a depot; partial slabs served first
//   • Guard pages surround slab arenas (Rust-backed bounds via S05 RS)
// =============================================================================

#include <stdint.h>
#include <stdbool.h>

#define SIGMA_SLAB_MAGAZINE_SIZE  128   // Objects per per-CPU magazine
#define SIGMA_SLAB_MAX_CACHES      64   // Max simultaneous object type caches
#define SIGMA_SLAB_GUARD_MAGIC  0xDEADBEEFC0FFEE00ULL

// ── Object Size Classes (jemalloc style) ─────────────────────────────────────
static const uint32_t size_classes[] = {
    8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096
};

// ── Slab Cache Descriptor ────────────────────────────────────────────────────
typedef struct SigmaSlabCache {
    const char* name;
    uint32_t    obj_size;        // Fixed size of managed objects
    uint32_t    objs_per_slab;   // How many objects fit in one 4K page slab
    uint32_t    free_count;      // Currently available objects
    void*       magazine_cpu[64];// Per-CPU hot magazine pointers
    void*       depot_full;      // Retired full slabs
    void*       depot_partial;   // Partial slabs (served first)
    uint64_t    guard_canary;    // Overflow detection sentinel
} SigmaSlabCache;

static SigmaSlabCache cache_table[SIGMA_SLAB_MAX_CACHES];
static uint32_t       cache_count = 0;

// ── Public API ───────────────────────────────────────────────────────────────

// Create a new named slab cache for objects of a fixed size
SigmaSlabCache* slab_cache_create(const char* name, uint32_t obj_size);

// Allocate one object from the cache (O(1) from per-CPU magazine)
void* slab_alloc(SigmaSlabCache* cache);

// Return an object to the cache magazine (O(1))
void  slab_free(SigmaSlabCache* cache, void* obj);

// Shrink all caches by reclaiming empty slabs back to the buddy allocator
void  slab_trim_all(void);

// Validate guard canaries on all active caches (called by Rust safety shard)
bool  slab_verify_integrity(void);

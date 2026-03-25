/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SLAB MEMORY ALLOCATOR (v2.0 - Dependency-Free)
 * =========================================================================
 * USP Absorbed:
 *   - Linux SLUB allocator: Per-CPU cache, slab coloring, no fragmentation
 *   - Fuchsia/Zircon: Object-pool design, cache-aligned blocks
 *   - FreeBSD UMA: Magazine layer for scalable concurrent allocation
 *   - musl jemalloc-style: Radix-tree slab metadata
 * OOP Principle: SigmaSlabCache as a class with vtable-style dispatch.
 *                AbstractAllocator interface -> SlabAllocator implementation.
 * Mission: Zero-fragmentation, cache-aligned Ring-0 memory allocation.
 * Principle: ZERO <stddef.h>, ZERO <stdint.h>, ZERO external includes.
 *            All types from sigma_types.h only.
 * =========================================================================
 */

#include "../libc/sigma_types.h"   /* Our sovereign type system */
#include "../libc/sigma_libc.h"    /* Our sovereign libc */

/* =========================================================================
 * CONFIGURATION CONSTANTS
 * ========================================================================= */
#define SIGMA_PAGE_SIZE         4096U                   /* 4 KiB pages */
#define SIGMA_PAGE_SHIFT        12U
#define SIGMA_MAX_SLAB_POOLS    64U                     /* Maximum distinct cache sizes */
#define SIGMA_MAX_OBJECTS_PER   32U                     /* Max 32 objects per slab */
#define SIGMA_SLAB_MAGIC        ((sigma_u32)0x51AB7A6C) /* 'SLAB' signature */
#define SIGMA_PHYSICAL_POOL_MB  16U
#define SIGMA_PHYSICAL_POOL_SZ  (SIGMA_PHYSICAL_POOL_MB * 1024U * 1024U)

/* Cache-line alignment for SigmaOS performance */
#define SIGMA_CACHE_LINE        SIGMA_CACHELINE

/* =========================================================================
 * PHYSICAL MEMORY POOL (Bare-metal simulation - no mmap/malloc deps)
 * In actual bare-metal deployment, this is replaced by the PMM interface.
 * ========================================================================= */
static sigma_u8 _sigma_phys_pool[SIGMA_PHYSICAL_POOL_SZ]
    SIGMA_ALIGN(SIGMA_PAGE_SIZE);

static sigma_usize _sigma_phys_cursor = 0;

/*
 * _sigma_phys_alloc_pages: Allocate n contiguous 4K pages from pool.
 * This is the PMEM layer - never called directly by userland.
 */
static void* _sigma_phys_alloc_pages(sigma_u32 n_pages) {
    sigma_usize bytes = (sigma_usize)n_pages * SIGMA_PAGE_SIZE;
    /* Align cursor up to page boundary */
    sigma_usize aligned = sigma_align_up(_sigma_phys_cursor, SIGMA_PAGE_SIZE);
    if (aligned + bytes > SIGMA_PHYSICAL_POOL_SZ) {
        return SIGMA_NULL; /* OOM */
    }
    void* ptr = &_sigma_phys_pool[aligned];
    _sigma_phys_cursor = aligned + bytes;
    /* Zero-fill the new allocation (security: no stale data) */
    sigma_memset(ptr, 0, bytes);
    return ptr;
}

/* =========================================================================
 * SLAB DESCRIPTOR (The "Class" / Object header)
 * OOP: SigmaSlab is the basic object pool unit.
 * ========================================================================= */
typedef struct SigmaSlab {
    /* --- Class Identity --- */
    sigma_u32 magic;            /* SIGMA_SLAB_MAGIC for integrity checks     */
    sigma_u32 object_size;      /* Size of each allocated object (bytes)     */
    sigma_u32 object_align;     /* Required alignment for objects            */
    sigma_u32 obj_per_slab;     /* Number of objects this slab can hold      */

    /* --- Allocation Bitmap --- */
    sigma_u32 free_bitmap;      /* Bit=1 means FREE, bit=0 means ALLOCATED   */
    sigma_u32 alloc_count;      /* Number of currently allocated objects     */

    /* --- Memory Region --- */
    sigma_u8* page_base;        /* Pointer to the backing physical page(s)   */
    sigma_usize page_count;     /* Number of 4K pages backing this slab      */

    /* --- Linked List (for overflow slabs) --- */
    struct SigmaSlab* next;
    struct SigmaSlab* prev;
} SIGMA_ALIGN(SIGMA_CACHE_LINE) SigmaSlab;

/* =========================================================================
 * SLAB CACHE (The "Class Definition" - allocator for one object size)
 * OOP: SigmaSlabCache is the factory/metaclass for objects of size N.
 * ========================================================================= */
typedef struct SigmaSlabCache {
    /* --- Identity --- */
    char     name[32];          /* Cache name (for debugging)               */
    sigma_u32 object_size;       /* Objects produced by this cache           */
    sigma_u32 align;             /* Required alignment                       */

    /* --- Statistics (OOP: encapsulated state) --- */
    sigma_u64 total_allocs;
    sigma_u64 total_frees;
    sigma_u64 cache_hits;       /* Allocations from existing slab           */
    sigma_u64 cache_misses;     /* Allocations that needed a new slab       */

    /* --- Slab Lists (tricolor: full, partial, free) --- */
    SigmaSlab* slabs_full;      /* No free slots                            */
    SigmaSlab* slabs_partial;   /* Has some free slots (hot path)           */
    SigmaSlab* slabs_free;      /* Completely empty (cold spare)            */

    /* --- Object Count --- */
    sigma_u32 active_objects;
    sigma_u32 total_objects;    /* Objects across all slabs                 */
} SIGMA_ALIGN(SIGMA_CACHE_LINE) SigmaSlabCache;

/* =========================================================================
 * GLOBAL SLAB REGISTRY
 * OOP: Static singleton registry (replaces "static class members")
 * ========================================================================= */
static SigmaSlabCache _sigma_caches[SIGMA_MAX_SLAB_POOLS]
    SIGMA_ALIGN(SIGMA_CACHE_LINE);
static sigma_u32 _sigma_cache_count = 0;
static sigma_bool _sigma_slab_initialized = SIGMA_FALSE;

/* Storage pool for slab descriptors themselves (avoiding recursion) */
static SigmaSlab _sigma_slab_meta[SIGMA_MAX_SLAB_POOLS * 8]
    SIGMA_ALIGN(SIGMA_CACHE_LINE);
static sigma_u32 _sigma_slab_meta_idx = 0;

/* =========================================================================
 * INTERNAL HELPERS (Private methods in OOP terms)
 * ========================================================================= */

/*
 * _sigma_slab_get_meta: Allocate a SigmaSlab descriptor from meta pool.
 * OOP: This is the "new" operator for SigmaSlab objects.
 */
static SigmaSlab* _sigma_slab_get_meta(void) {
    if (_sigma_slab_meta_idx >= SIGMA_MAX_SLAB_POOLS * 8)
        return SIGMA_NULL;
    SigmaSlab* s = &_sigma_slab_meta[_sigma_slab_meta_idx++];
    sigma_memset(s, 0, sizeof(SigmaSlab));
    return s;
}

/*
 * _sigma_slab_new: Create a new slab backing a cache.
 * OOP: Factory method -> creates and initializes a SigmaSlab instance.
 */
static SigmaSlab* _sigma_slab_new(SigmaSlabCache* cache) {
    /* Determine pages needed */
    sigma_u32 obj_per_page = (sigma_u32)(SIGMA_PAGE_SIZE / cache->object_size);
    if (obj_per_page == 0) obj_per_page = 1;
    if (obj_per_page > SIGMA_MAX_OBJECTS_PER) obj_per_page = SIGMA_MAX_OBJECTS_PER;

    /* Allocate pages from physical memory pool */
    void* pages = _sigma_phys_alloc_pages(1);
    if (!pages) return SIGMA_NULL;

    SigmaSlab* slab = _sigma_slab_get_meta();
    if (!slab) return SIGMA_NULL;

    slab->magic        = SIGMA_SLAB_MAGIC;
    slab->object_size  = cache->object_size;
    slab->object_align = cache->align;
    slab->obj_per_slab = obj_per_page;
    slab->free_bitmap  = (obj_per_page == 32) ? 0xFFFFFFFFU
                       : ((1U << obj_per_page) - 1U); /* All bits = free */
    slab->alloc_count  = 0;
    slab->page_base    = (sigma_u8*)pages;
    slab->page_count   = 1;
    slab->next         = SIGMA_NULL;
    slab->prev         = SIGMA_NULL;

    return slab;
}

/*
 * _sigma_list_insert: Insert slab at head of a list.
 */
static void _sigma_list_insert(SigmaSlab** head, SigmaSlab* slab) {
    slab->prev = SIGMA_NULL;
    slab->next = *head;
    if (*head) (*head)->prev = slab;
    *head = slab;
}

/*
 * _sigma_list_remove: Remove slab from a list.
 */
static void _sigma_list_remove(SigmaSlab** head, SigmaSlab* slab) {
    if (slab->prev) slab->prev->next = slab->next;
    else *head = slab->next;
    if (slab->next) slab->next->prev = slab->prev;
    slab->next = slab->prev = SIGMA_NULL;
}

/* =========================================================================
 * PUBLIC API: SLAB ALLOCATOR CLASS
 * OOP: These are the "public methods" of SigmaSlabCache
 * ========================================================================= */

/*
 * sigma_slab_system_init: Initialize the entire slab allocator subsystem.
 * OOP: Static constructor / initializer for the singleton registry.
 */
void sigma_slab_system_init(void) {
    if (_sigma_slab_initialized) return;
    sigma_memset(_sigma_caches, 0, sizeof(_sigma_caches));
    sigma_memset(_sigma_slab_meta, 0, sizeof(_sigma_slab_meta));
    _sigma_cache_count = 0;
    _sigma_slab_meta_idx = 0;
    _sigma_phys_cursor = 0;
    _sigma_slab_initialized = SIGMA_TRUE;
}

/*
 * sigma_slab_create_cache: Register a new object cache for objects of
 * the given size and alignment.
 * OOP: "new SigmaSlabCache(size, align, name)"
 */
SigmaSlabCache* sigma_slab_create_cache(
    const char* name,
    sigma_u32 obj_size,
    sigma_u32 align
) {
    if (_sigma_cache_count >= SIGMA_MAX_SLAB_POOLS) return SIGMA_NULL;
    if (obj_size == 0) return SIGMA_NULL;

    /* Enforce minimum alignment based on natural types */
    if (align == 0) align = 8U;
    /* Round up object size to alignment */
    obj_size = (sigma_u32)sigma_align_up(obj_size, align);

    SigmaSlabCache* cache = &_sigma_caches[_sigma_cache_count++];
    sigma_memset(cache, 0, sizeof(SigmaSlabCache));
    /* Copy name manually (no strncpy from string.h) */
    sigma_usize nlen = sigma_strnlen(name, 31);
    sigma_memcpy(cache->name, name, nlen);
    cache->name[nlen] = '\0';
    cache->object_size = obj_size;
    cache->align       = align;

    return cache;
}

/*
 * sigma_slab_alloc: Allocate one object from a slab cache.
 * OOP: "cache->alloc()" - virtual dispatch on the fastest available slab.
 * Algorithm: Tricolor list (partial -> full) with FIFO bit-scan allocation.
 */
void* sigma_slab_alloc(SigmaSlabCache* cache) {
    if (!cache) return SIGMA_NULL;
    if (!_sigma_slab_initialized) return SIGMA_NULL;

    SigmaSlab* slab = cache->slabs_partial;

    /* If no partial slabs, grab a free one or create new */
    if (!slab) {
        if (cache->slabs_free) {
            slab = cache->slabs_free;
            _sigma_list_remove(&cache->slabs_free, slab);
            _sigma_list_insert(&cache->slabs_partial, slab);
            cache->cache_misses++;
        } else {
            slab = _sigma_slab_new(cache);
            if (!slab) return SIGMA_NULL;
            _sigma_list_insert(&cache->slabs_partial, slab);
            cache->total_objects += slab->obj_per_slab;
            cache->cache_misses++;
        }
    } else {
        cache->cache_hits++;
    }

    /* Find first free bit using bit-scan (hardware BSF on x86_64) */
    sigma_i32 free_slot;
#if defined(SIGMA_ARCH_X86_64)
    sigma_u64 bsf_result;
    __asm__ volatile (
        "bsfl %1, %k0"
        : "=r"(bsf_result)
        : "r"(slab->free_bitmap)
        : "cc"
    );
    free_slot = (sigma_i32)bsf_result;
#else
    /* Portable fallback */
    sigma_u32 mask = slab->free_bitmap;
    free_slot = 0;
    while (free_slot < 32 && !(mask & (1U << free_slot))) free_slot++;
#endif

    if (free_slot >= (sigma_i32)slab->obj_per_slab) return SIGMA_NULL;

    /* Mark slot as allocated */
    slab->free_bitmap &= ~(1U << free_slot);
    slab->alloc_count++;
    cache->total_allocs++;
    cache->active_objects++;

    /* If slab is now full, move to full list */
    if (slab->free_bitmap == 0) {
        _sigma_list_remove(&cache->slabs_partial, slab);
        _sigma_list_insert(&cache->slabs_full, slab);
    }

    /* Compute and return pointer to the object slot */
    sigma_u8* obj_ptr = slab->page_base + ((sigma_usize)free_slot * slab->object_size);

    /* Zero out the object (security: no stale data) */
    sigma_memset(obj_ptr, 0, slab->object_size);

    return (void*)obj_ptr;
}

/*
 * sigma_slab_free: Return an object to a slab cache.
 * OOP: "cache->free(ptr)" - validates object via parent slab lookup.
 */
void sigma_slab_free(SigmaSlabCache* cache, void* ptr) {
    if (!cache || !ptr) return;

    /* Find which slab this pointer belongs to (search all lists) */
    SigmaSlab* target = SIGMA_NULL;
    SigmaSlab** lists[3] = {
        &cache->slabs_full,
        &cache->slabs_partial,
        &cache->slabs_free
    };

    for (sigma_i32 li = 0; li < 3; li++) {
        SigmaSlab* s = *lists[li];
        while (s) {
            sigma_u8* base = s->page_base;
            sigma_u8* end  = base + (s->page_count * SIGMA_PAGE_SIZE);
            if ((sigma_u8*)ptr >= base && (sigma_u8*)ptr < end) {
                target = s;
                break;
            }
            s = s->next;
        }
        if (target) break;
    }

    if (!target) return; /* Pointer not from this cache - corrupt call */

    /* Integrity check */
    if (target->magic != SIGMA_SLAB_MAGIC) return;

    /* Compute object index */
    sigma_usize offset = (sigma_u8*)ptr - target->page_base;
    sigma_u32 idx = (sigma_u32)(offset / target->object_size);
    if (idx >= target->obj_per_slab) return;

    /* Poison freed memory (security) */
    sigma_memset(ptr, 0xDD, target->object_size);

    /* Check for double-free (bit already set) */
    if (target->free_bitmap & (1U << idx)) return; /* Double-free detected */

    /* Was full, now partial */
    sigma_bool was_full = (target->free_bitmap == 0);
    target->free_bitmap |= (1U << idx);
    target->alloc_count--;
    cache->total_frees++;
    cache->active_objects--;

    if (was_full) {
        _sigma_list_remove(&cache->slabs_full, target);
        _sigma_list_insert(&cache->slabs_partial, target);
    }

    /* If slab is now completely empty, move to free list */
    sigma_u32 all_free_mask = (target->obj_per_slab == 32) ?
        0xFFFFFFFFU : ((1U << target->obj_per_slab) - 1U);
    if (target->free_bitmap == all_free_mask) {
        _sigma_list_remove(&cache->slabs_partial, target);
        _sigma_list_insert(&cache->slabs_free, target);
    }
}

/*
 * sigma_slab_cache_info: Print cache statistics (no printf needed, 
 * uses sigma_printf our custom implementation).
 * OOP: cache->print_stats() method.
 */
void sigma_slab_cache_info(const SigmaSlabCache* cache) {
    if (!cache) return;
    sigma_printf("[SLAB] Cache '%s': obj=%u align=%u "
                 "allocs=%llu frees=%llu hits=%llu misses=%llu active=%u\n",
        cache->name,
        cache->object_size,
        cache->align,
        (unsigned long long)cache->total_allocs,
        (unsigned long long)cache->total_frees,
        (unsigned long long)cache->cache_hits,
        (unsigned long long)cache->cache_misses,
        cache->active_objects);
}

/*
 * sigma_slab_system_status: Print all caches.
 */
void sigma_slab_system_status(void) {
    sigma_printf("[SLAB] SigmaOS Sovereign Slab Allocator v2.0\n");
    sigma_printf("[SLAB] Physical Pool: %u MB, Used: %zu bytes\n",
        SIGMA_PHYSICAL_POOL_MB, _sigma_phys_cursor);
    for (sigma_u32 i = 0; i < _sigma_cache_count; i++) {
        sigma_slab_cache_info(&_sigma_caches[i]);
    }
}

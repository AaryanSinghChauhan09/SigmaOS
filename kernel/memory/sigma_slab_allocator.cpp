/*
 * Σ SigmaOS Zenith — Slab Allocator (Inspired by Linux SLUB)
 * Absorbs: Linux SLUB design, power-of-2 bucketing, Arch Linux minimal philosophy
 * Zero-Dependency: No libc, no stdlib, no predefined headers or functions.
 */

/* ─────────────── Sovereign Types ─────────────── */
typedef unsigned char      u8;
typedef unsigned short     u16;
typedef unsigned int       u32;
typedef unsigned long long u64;
typedef u64 size_t;
typedef bool               bool_t;
#define nullptr 0

/* ─────────────── Sovereign Utilities ─────────────── */
static void sovereign_memset(void* ptr, u8 val, size_t n) {
    u8* p = (u8*)ptr;
    while (n--) *p++ = val;
}

static void sovereign_memcpy(void* dst, const void* src, size_t n) {
    u8* d = (u8*)dst;
    const u8* s = (const u8*)src;
    while (n--) *d++ = *s++;
}

/* ─────────────── Heap Configuration ─────────────── */
/*
 * Physical heap starts at 4MB. 16MB total slab arena.
 * Inspired by Linux's SLAB/SLUB memory allocator design.
 */
#define SLAB_HEAP_BASE      0x400000ULL   /* 4 MB */
#define SLAB_HEAP_SIZE      0x1000000ULL  /* 16 MB */
#define SLAB_BUCKET_COUNT   12            /* Buckets: 8,16,32,64,...,16384 */
#define SLAB_MAGIC          0xC0FFEE42

/* ─────────────── Slab Cache Metadata ─────────────── */
struct SlabObject {
    struct SlabObject* next; /* Intrusive free list pointer */
};

struct SlabCache {
    u32 object_size;         /* Size this cache serves */
    u32 slab_capacity;       /* Objects per slab */
    struct SlabObject* free_list;
    u64 slab_base_addr;
    u32 allocated;
    u32 magic;
};

static struct SlabCache sigma_caches[SLAB_BUCKET_COUNT];
extern "C" void* sigma_malloc(u64 size);
extern "C" void sigma_free(void* ptr);

/* ─────────────── Internal: Allocate a Raw Slab via Buddy Allocator ─────────────── */
static void* slab_raw_alloc(size_t size) {
    return sigma_malloc((u64)size);
}

/* ─────────────── API: Initialize Slab System ─────────────── */
extern "C" void sigma_slab_init() {
    u32 size = 8;
    for (u32 i = 0; i < SLAB_BUCKET_COUNT; i++) {
        sigma_caches[i].object_size   = size;
        sigma_caches[i].slab_capacity = 4096 / size;
        sigma_caches[i].free_list     = nullptr;
        sigma_caches[i].allocated     = 0;
        sigma_caches[i].magic         = SLAB_MAGIC;

        /* Carve out one slab per bucket during init */
        void* slab_mem = slab_raw_alloc(4096);
        sigma_caches[i].slab_base_addr = (u64)slab_mem;

        /* Build the intrusive free list */
        u8* base = (u8*)slab_mem;
        struct SlabObject* prev = nullptr;
        for (u32 j = sigma_caches[i].slab_capacity; j > 0; j--) {
            struct SlabObject* obj = (struct SlabObject*)(base + (j - 1) * size);
            obj->next = prev;
            prev = obj;
        }
        sigma_caches[i].free_list = prev;

        size <<= 1; /* Next power of 2 */
    }
}

/* ─────────────── API: Sovereign Slab alloc() ─────────────── */
extern "C" void* sigma_slab_alloc(size_t bytes) {
    if (bytes == 0) return nullptr;

    u32 size = 8;
    for (u32 i = 0; i < SLAB_BUCKET_COUNT; i++) {
        if (bytes <= size) {
            struct SlabCache* cache = &sigma_caches[i];
            if (!cache->free_list) {
                /* Refill: Allocate a new slab page */
                void* slab_mem = slab_raw_alloc(4096);
                if (!slab_mem) return nullptr;

                u8* base = (u8*)slab_mem;
                struct SlabObject* prev = nullptr;
                for (u32 j = cache->slab_capacity; j > 0; j--) {
                    struct SlabObject* obj = (struct SlabObject*)(base + (j - 1) * cache->object_size);
                    obj->next = prev;
                    prev = obj;
                }
                cache->free_list = prev;
            }

            struct SlabObject* obj = cache->free_list;
            cache->free_list = obj->next;
            cache->allocated++;
            sovereign_memset(obj, 0, cache->object_size);
            return (void*)obj;
        }
        size <<= 1;
    }

    /* Large allocation: fallback to raw buddy allocator */
    return slab_raw_alloc(bytes);
}

/* ─────────────── API: Sovereign Slab free() ─────────────── */
extern "C" void sigma_slab_free(void* ptr, size_t bytes) {
    if (!ptr) return;

    u32 size = 8;
    for (u32 i = 0; i < SLAB_BUCKET_COUNT; i++) {
        if (bytes <= size) {
            struct SlabObject* obj = (struct SlabObject*)ptr;
            obj->next = sigma_caches[i].free_list;
            sigma_caches[i].free_list = obj;
            sigma_caches[i].allocated--;
            return;
        }
        size <<= 1;
    }
    /* Large frees: delegate back to buddy allocator */
    sigma_free(ptr);
}

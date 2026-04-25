// SigmaOS — Sigma-Cache: Adaptive Performance Cache
// Inspired by: Linux Page Cache, macOS Unified Buffer Cache, Redis
// Module: sigma-perf-cache
// USP: LRU eviction with hardware cycle timestamps, no malloc, no stdlib
// Fixed-size open-addressing hash table backed by slab allocator

#ifndef SIGMA_CACHE_H
#define SIGMA_CACHE_H

#define SIGMA_CACHE_SLOTS   256
#define SIGMA_CACHE_KEY_LEN  32
#define SIGMA_CACHE_VAL_LEN  64

typedef struct SigmaCacheEntry {
    char          key[SIGMA_CACHE_KEY_LEN];
    unsigned char value[SIGMA_CACHE_VAL_LEN];
    unsigned int  value_len;
    unsigned long last_access;  // RDTSC cycles
    unsigned char valid;
} SigmaCacheEntry;

typedef struct SigmaCache {
    SigmaCacheEntry slots[SIGMA_CACHE_SLOTS];
    unsigned long   hits;
    unsigned long   misses;
    unsigned long   evictions;
} SigmaCache;

static inline unsigned long cache_rdtsc(void) {
#if defined(__x86_64__) || defined(__i386__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

// FNV-1a hash of key string
static inline unsigned int cache_hash(const char* key) {
    unsigned int h = 2166136261U;
    while (*key) { h ^= (unsigned char)*key++; h *= 16777619U; }
    return h % SIGMA_CACHE_SLOTS;
}

static inline int cache_strcmp(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return *a - *b;
}

static inline void cache_init(SigmaCache* c) {
    c->hits = c->misses = c->evictions = 0;
    for (int i = 0; i < SIGMA_CACHE_SLOTS; i++) c->slots[i].valid = 0;
}

// Insert/update cache entry
static inline void cache_put(SigmaCache* c, const char* key,
                               const unsigned char* val, unsigned int vlen) {
    unsigned int idx = cache_hash(key);
    // Linear probe
    for (unsigned int i = 0; i < SIGMA_CACHE_SLOTS; i++) {
        unsigned int s = (idx + i) % SIGMA_CACHE_SLOTS;
        SigmaCacheEntry* e = &c->slots[s];
        if (!e->valid || cache_strcmp(e->key, key) == 0) {
            for (int k = 0; k < SIGMA_CACHE_KEY_LEN - 1 && key[k]; k++) e->key[k] = key[k];
            if (vlen > SIGMA_CACHE_VAL_LEN) vlen = SIGMA_CACHE_VAL_LEN;
            for (unsigned int k = 0; k < vlen; k++) e->value[k] = val[k];
            e->value_len  = vlen;
            e->last_access = cache_rdtsc();
            e->valid = 1;
            return;
        }
    }
    // Cache full — evict LRU
    unsigned int oldest = 0; unsigned long oldest_ts = c->slots[0].last_access;
    for (int i = 1; i < SIGMA_CACHE_SLOTS; i++) {
        if (c->slots[i].last_access < oldest_ts) { oldest = i; oldest_ts = c->slots[i].last_access; }
    }
    c->evictions++;
    SigmaCacheEntry* e = &c->slots[oldest];
    for (int k = 0; k < SIGMA_CACHE_KEY_LEN - 1 && key[k]; k++) e->key[k] = key[k];
    if (vlen > SIGMA_CACHE_VAL_LEN) vlen = SIGMA_CACHE_VAL_LEN;
    for (unsigned int k = 0; k < vlen; k++) e->value[k] = val[k];
    e->value_len = vlen; e->last_access = cache_rdtsc(); e->valid = 1;
}

// Lookup — returns pointer to value or null
static inline const unsigned char* cache_get(SigmaCache* c, const char* key,
                                               unsigned int* out_len) {
    unsigned int idx = cache_hash(key);
    for (unsigned int i = 0; i < SIGMA_CACHE_SLOTS; i++) {
        unsigned int s = (idx + i) % SIGMA_CACHE_SLOTS;
        SigmaCacheEntry* e = &c->slots[s];
        if (!e->valid) { c->misses++; return (void*)0; }
        if (cache_strcmp(e->key, key) == 0) {
            e->last_access = cache_rdtsc(); c->hits++;
            if (out_len) *out_len = e->value_len;
            return e->value;
        }
    }
    c->misses++;
    return (void*)0;
}

#endif /* SIGMA_CACHE_H */

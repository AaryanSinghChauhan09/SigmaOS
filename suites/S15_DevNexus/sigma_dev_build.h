// SigmaOS — sigma-dev-build: Native Build System
// Inspired by: GNU Make, Ninja, Meson
// Module: sigma-dev-build
// USP: Dependency graph in pure C — no Makefile parser, no Python runtime
// Builds only what changed — delta tracking via FNV-1a content hash

#ifndef SIGMA_DEV_BUILD_H
#define SIGMA_DEV_BUILD_H

#define SIGMA_BUILD_MAX_TARGETS  64
#define SIGMA_BUILD_NAME_LEN     48
#define SIGMA_BUILD_MAX_DEPS      8

typedef int (*build_fn)(void* ctx);

typedef struct SigmaBuildTarget {
    char          name[SIGMA_BUILD_NAME_LEN];
    unsigned int  target_id;
    unsigned long content_hash;      // FNV-1a of source — detect changes
    unsigned long prev_hash;         // last built hash
    unsigned int  deps[SIGMA_BUILD_MAX_DEPS];
    unsigned int  dep_count;
    build_fn      fn;
    void*         ctx;
    unsigned char built;
    unsigned char dirty;             // needs rebuild
} SigmaBuildTarget;

typedef struct SigmaBuildGraph {
    SigmaBuildTarget targets[SIGMA_BUILD_MAX_TARGETS];
    unsigned int     count;
    unsigned long    total_built;
    unsigned long    total_skipped;
} SigmaBuildGraph;

static inline unsigned long build_fnv1a(const unsigned char* d, unsigned long n) {
    unsigned long h = 14695981039346656037UL, p = 1099511628211UL;
    for (unsigned long i = 0; i < n; i++) { h ^= d[i]; h *= p; }
    return h;
}

static inline void build_graph_init(SigmaBuildGraph* g) {
    g->count = g->total_built = g->total_skipped = 0;
}

static inline unsigned int build_add_target(SigmaBuildGraph* g, const char* name,
                                              build_fn fn, void* ctx,
                                              const unsigned char* src, unsigned long src_len) {
    if (g->count >= SIGMA_BUILD_MAX_TARGETS) return 0xFFFFFFFF;
    SigmaBuildTarget* t = &g->targets[g->count];
    t->target_id    = g->count++;
    t->fn           = fn;
    t->ctx          = ctx;
    t->dep_count    = 0;
    t->built        = 0;
    t->content_hash = build_fnv1a(src, src_len);
    t->prev_hash    = 0; // never built
    t->dirty        = (t->content_hash != t->prev_hash) ? 1 : 0;
    for (int i = 0; i < SIGMA_BUILD_NAME_LEN - 1 && name[i]; i++) t->name[i] = name[i];
    return t->target_id;
}

static inline void build_add_dep(SigmaBuildGraph* g, unsigned int target_id,
                                   unsigned int dep_id) {
    SigmaBuildTarget* t = &g->targets[target_id];
    if (t->dep_count < SIGMA_BUILD_MAX_DEPS)
        t->deps[t->dep_count++] = dep_id;
}

// Recursive build — respects dependency order (topological)
static inline int build_target(SigmaBuildGraph* g, unsigned int id) {
    if (id >= g->count) return -1;
    SigmaBuildTarget* t = &g->targets[id];
    if (t->built) return 0;
    // Build deps first
    for (unsigned int i = 0; i < t->dep_count; i++) {
        int r = build_target(g, t->deps[i]);
        if (r != 0) return r;
    }
    // Skip if clean
    if (!t->dirty) { t->built = 1; g->total_skipped++; return 0; }
    // Execute build function
    int result = t->fn ? t->fn(t->ctx) : 0;
    if (result == 0) {
        t->prev_hash = t->content_hash;
        t->dirty     = 0;
        t->built     = 1;
        g->total_built++;
    }
    return result;
}

static inline int build_all(SigmaBuildGraph* g) {
    // Reset built flag for new build pass
    for (unsigned int i = 0; i < g->count; i++) g->targets[i].built = 0;
    g->total_built = g->total_skipped = 0;
    int r = 0;
    for (unsigned int i = 0; i < g->count; i++) {
        r = build_target(g, i);
        if (r != 0) return r;
    }
    return 0;
}

#endif /* SIGMA_DEV_BUILD_H */

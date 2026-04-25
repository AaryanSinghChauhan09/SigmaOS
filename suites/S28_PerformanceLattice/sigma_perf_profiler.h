// SigmaOS — sigma-perf-profiler: Native Performance Profiler
// Inspired by: Linux perf, Valgrind callgrind, Tracy profiler
// Module: sigma-perf-profiler
// USP: RDTSC-based nanosecond profiling zones, zero perf_event_open syscall
// Each profiling zone is atomic — begin/end pair stored in ring

#ifndef SIGMA_PERF_PROFILER_H
#define SIGMA_PERF_PROFILER_H

#define SIGMA_PROF_MAX_ZONES   64
#define SIGMA_PROF_NAME_LEN    32
#define SIGMA_PROF_RING_SIZE   512

typedef struct SigmaProfSample {
    unsigned long start_cycles;
    unsigned long end_cycles;
    unsigned int  zone_id;
} SigmaProfSample;

typedef struct SigmaProfZone {
    char          name[SIGMA_PROF_NAME_LEN];
    unsigned int  zone_id;
    unsigned long total_cycles;
    unsigned long call_count;
    unsigned long min_cycles;
    unsigned long max_cycles;
} SigmaProfZone;

typedef struct SigmaProfiler {
    SigmaProfZone   zones[SIGMA_PROF_MAX_ZONES];
    SigmaProfSample ring[SIGMA_PROF_RING_SIZE];
    unsigned int    zone_count;
    unsigned int    ring_head;
    unsigned int    ring_tail;
} SigmaProfiler;

static inline unsigned long prof_rdtsc(void) {
#if defined(__x86_64__) || defined(__i386__)
    unsigned int lo, hi;
    __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
    return ((unsigned long)hi << 32) | lo;
#else
    return 0;
#endif
}

static inline void profiler_init(SigmaProfiler* p) {
    p->zone_count = 0;
    p->ring_head = p->ring_tail = 0;
}

// Register a profiling zone — returns zone_id
static inline unsigned int prof_register_zone(SigmaProfiler* p, const char* name) {
    if (p->zone_count >= SIGMA_PROF_MAX_ZONES) return 0xFFFFFFFF;
    SigmaProfZone* z = &p->zones[p->zone_count];
    z->zone_id     = p->zone_count;
    z->total_cycles = 0;
    z->call_count  = 0;
    z->min_cycles  = 0xFFFFFFFFFFFFFFFFUL;
    z->max_cycles  = 0;
    for (int i = 0; i < SIGMA_PROF_NAME_LEN - 1 && name[i]; i++) z->name[i] = name[i];
    return p->zone_count++;
}

// Begin profiling a zone — push start timestamp
static inline unsigned long prof_begin(SigmaProfiler* p, unsigned int zone_id) {
    (void)p; (void)zone_id;
    return prof_rdtsc();
}

// End profiling a zone — compute delta and accumulate
static inline void prof_end(SigmaProfiler* p, unsigned int zone_id,
                              unsigned long start_cycles) {
    if (zone_id >= p->zone_count) return;
    unsigned long end   = prof_rdtsc();
    unsigned long delta = end - start_cycles;
    SigmaProfZone* z = &p->zones[zone_id];
    z->total_cycles += delta;
    z->call_count++;
    if (delta < z->min_cycles) z->min_cycles = delta;
    if (delta > z->max_cycles) z->max_cycles = delta;
    // Push to ring
    unsigned int next = (p->ring_head + 1) % SIGMA_PROF_RING_SIZE;
    if (next != p->ring_tail) {
        p->ring[p->ring_head].zone_id      = zone_id;
        p->ring[p->ring_head].start_cycles = start_cycles;
        p->ring[p->ring_head].end_cycles   = end;
        p->ring_head = next;
    }
}

// Get average cycles for a zone
static inline unsigned long prof_avg(SigmaProfiler* p, unsigned int zone_id) {
    if (zone_id >= p->zone_count || p->zones[zone_id].call_count == 0) return 0;
    return p->zones[zone_id].total_cycles / p->zones[zone_id].call_count;
}

#endif /* SIGMA_PERF_PROFILER_H */

// SigmaOS — sigma-mem-audit: Memory Audit Reporter
// Modularised from: SovereignMemoryZenith.c
// Single responsibility: report memory pool statistics only

#ifndef SIGMA_MEM_AUDIT_H
#define SIGMA_MEM_AUDIT_H

#include "sigma_mem_pool.h"
#include "sigma_libc.h"

static inline void mem_audit(SigmaMemPool* mp) {
    sigma_kprint("\n--- Sigma Memory Audit ---\n");
    sigma_kprint("[MEM] Pool size:   64 MB\n");
    sigma_kprint("[MEM] Used (KB):   ");
    sigma_kprint_int((int)mem_pool_used_kb(mp));
    sigma_kprint("\n[MEM] Free (KB):   ");
    sigma_kprint_int((int)mem_pool_free_kb(mp));
    sigma_kprint("\n[MEM] Segments:    ");
    sigma_kprint_int((int)mp->seg_count);
    sigma_kprint("\n[MEM] Live segs:   ");
    unsigned int live = 0;
    for (unsigned int i = 0; i < mp->seg_count; i++)
        if (mp->segs[i].in_use) live++;
    sigma_kprint_int((int)live);
    sigma_kprint("\n--------------------------\n");
}

// Detect potential leaks (segments still in_use on shutdown)
static inline unsigned int mem_detect_leaks(SigmaMemPool* mp) {
    unsigned int leaks = 0;
    for (unsigned int i = 0; i < mp->seg_count; i++)
        if (mp->segs[i].in_use) leaks++;
    return leaks;
}

#endif /* SIGMA_MEM_AUDIT_H */

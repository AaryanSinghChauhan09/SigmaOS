/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN OS BASICS — ZENITH EDITION (v2.0 PURE C11)
 * =============================================================================
 * ROOT FIX: Eliminated all C++/OOP/SigmaOOP.hpp dependencies.
 * This header is now a pure C11 compatibility shim over sigma_kernel_types.h.
 * All definitions use strictly User-Defined primitives — zero stdlib dependency.
 * =============================================================================
 */

#ifndef SOVEREIGN_OS_BASICS_ZENITH_H
#define SOVEREIGN_OS_BASICS_ZENITH_H

#include "sigma_kernel_types.h"   /* canonical zero-dep type foundation */

/* ---- Process Control Block (pure C11 struct, no class) ---- */
typedef struct {
    u32         pid;
    const char* state;    /* "READY" | "RUNNING" | "WAITING" | "TERMINATED" */
    u64         pc;       /* program counter */
    u64         regs[16];
    u8*         stack_ptr;
} SovereignPCB;

/* ---- Resource Descriptor (replaces OOP DeadlockAgent) ---- */
#define SIGMA_MAX_PROCS   5
#define SIGMA_MAX_RES     3

typedef struct {
    u32 max[SIGMA_MAX_PROCS][SIGMA_MAX_RES];
    u32 alloc[SIGMA_MAX_PROCS][SIGMA_MAX_RES];
    u32 available[SIGMA_MAX_RES];
} SovereignResourceTable;

/* ---- Banker's Safety Check (pure UDF, no stdlib) ---- */
static inline int sigma_is_safe_state(SovereignResourceTable* rt) {
    u32 work[SIGMA_MAX_RES];
    int finish[SIGMA_MAX_PROCS];
    int i, j, found;
    for (j = 0; j < SIGMA_MAX_RES; j++)  work[j] = rt->available[j];
    for (i = 0; i < SIGMA_MAX_PROCS; i++) finish[i] = 0;
    found = 1;
    while (found) {
        found = 0;
        for (i = 0; i < SIGMA_MAX_PROCS; i++) {
            if (finish[i]) continue;
            int ok = 1;
            for (j = 0; j < SIGMA_MAX_RES; j++) {
                u32 need = rt->max[i][j] - rt->alloc[i][j];
                if (need > work[j]) { ok = 0; break; }
            }
            if (ok) {
                for (j = 0; j < SIGMA_MAX_RES; j++)
                    work[j] += rt->alloc[i][j];
                finish[i] = 1; found = 1;
            }
        }
    }
    for (i = 0; i < SIGMA_MAX_PROCS; i++) if (!finish[i]) return 0;
    return 1; /* safe */
}

/* ---- Page/Frame table (replaces OOP MemoryZenith) ---- */
#define SIGMA_MAX_FRAMES  64

typedef struct {
    u64  frames[SIGMA_MAX_FRAMES]; /* physical frame numbers */
    u32  count;
} SovereignPageTable;

static inline u64 sigma_logical_to_physical(SovereignPageTable* pt, u64 logical) {
    u64 page   = logical / PAGE_SIZE;
    u64 offset = logical % PAGE_SIZE;
    if (page >= pt->count) return 0;
    return (pt->frames[page] * PAGE_SIZE) + offset;
}

#endif /* SOVEREIGN_OS_BASICS_ZENITH_H */

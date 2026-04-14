/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN BARRIER SHARD (v52.3-SUPREME-OMNIPOTENCE)
 * =========================================================================
 * Mission: Large-scale parallel task rendezvous and synchronization.
 * Principles: Multi-Processing, Computer Science, Distributed, Throughput.
 *
 * Implements a sense-reversing barrier for core synchronization.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

typedef struct {
    volatile int count;
    int          threshold;
    volatile int sense;
} SigmaBarrier_t;

/**
 * sigma_sync_barrier_wait: Blocks until all threads reach the barrier.
 * Principle: Multi-Processing / Throughput Optimization.
 */
void sigma_sync_barrier_wait(SigmaBarrier_t* barrier) {
    int local_sense = !barrier->sense;
    int current = __sync_fetch_and_add(&barrier->count, 1);
    
    if (current == barrier->threshold - 1) {
        barrier->count = 0;
        barrier->sense = local_sense;
        sigma_printf("[BARRIER]: Threshhold %d reached. Global Release dispatched.\n", barrier->threshold);
    } else {
        while (barrier->sense != local_sense) { /* Spin */ }
    }
}

/* --- Module Factory --- */

void SovereignBarrier_Register(void) {
    sigma_printf("[ORCHESTRATOR]: Sovereign Barrier Sync (Rendezvous Mastery) active.\n");
}

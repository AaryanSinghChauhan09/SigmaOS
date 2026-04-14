/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN SEQLOCK SHARD (v52.7-SUPREME-OLYMPUS)
 * =========================================================================
 * Mission: Ultra-low overhead concurrent reads for frequently updated data.
 * Principles: Multi-Processing, Computer Science, Real-Time, Performance.
 *
 * Implements a Sequence Lock (Seqlock) for writer-priority data access.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

typedef struct {
    volatile sigma_u32 seq;
} SigmaSeqlock_t;

/**
 * sigma_sync_seqlock_read_begin: Returns the current sequence number.
 * Principle: Multi-Processing / Real-Time / Performance.
 */
sigma_u32 sigma_sync_seqlock_read_begin(SigmaSeqlock_t* sl) {
    sigma_u32 s;
    while ((s = sl->seq) & 1) { /* Spin while writer is active */ }
    return s;
}

/**
 * sigma_sync_seqlock_read_retry: Checks if the sequence number has changed.
 */
int sigma_sync_seqlock_read_retry(SigmaSeqlock_t* sl, sigma_u32 old_seq) {
    return (sl->seq != old_seq);
}

/**
 * sigma_sync_seqlock_write_lock: Begins a write-side critical section.
 */
void sigma_sync_seqlock_write_lock(SigmaSeqlock_t* sl) {
    __sync_fetch_and_add(&sl->seq, 1); // Make it odd (writer active)
    sigma_printf("[SYNC-SEQ]: Writer active. Sequence incremented to %u.\n", sl->seq);
}

/* --- Module Factory --- */

void SovereignSeqlock_Register(void) {
    sigma_printf("[ORCHESTRATOR]: Sovereign Seqlock (Writer-Priority Sync) active.\n");
}

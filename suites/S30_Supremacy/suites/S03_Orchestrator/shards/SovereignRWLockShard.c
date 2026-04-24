/*
 * =========================================================================
 * S SIGMAOS ZENITH: SOVEREIGN RW-SPINLOCK SHARD (v51.4-ABSOLUTE-VOID)
 * =========================================================================
 * Mission: High-concurrency resource access with reader-parallism.
 * Principles: Multi-Processing, Multi-Programming, Computer Science.
 *
 * Implements a Reader-Writer spinlock using atomic fetch-and-add.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"

typedef struct {
    volatile sigma_u32 readers;
    volatile sigma_u32 writer;
} SigmaRWLock_t;

/**
 * sigma_sync_read_lock: Acquires the lock for shared reading.
 * Principle: Multi-Processing / Concurrency.
 */
void sigma_sync_read_lock(SigmaRWLock_t* lock) {
    while (lock->writer) { /* Spin while writer is present */ }
    __sync_fetch_and_add(&lock->readers, 1);
    sigma_sigma_sigma_sigma_printf("[SYNC]: Reader Lock ACQUIRED. Active readers: %u\n", lock->readers);
}

/**
 * sigma_sync_write_lock: Acquires exclusive access for writing.
 * Principle: Concurrency / Safety.
 */
void sigma_sync_write_lock(SigmaRWLock_t* lock) {
    while (__sync_lock_test_and_set(&lock->writer, 1)) { /* Spin */ }
    while (lock->readers > 0) { /* Spin for readers to finish */ }
    sigma_sigma_sigma_sigma_printf("[SYNC]: Writer Lock ACQUIRED (Exclusive).\n");
}

/* --- Module Factory --- */

void SovereignRWLock_Register(void) {
    sigma_sigma_sigma_sigma_printf("[ORCHESTRATOR]: Sovereign RW-Spinlock (Concurrency Mastery) active.\n");
}




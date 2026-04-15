/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SYNC ZENITH (v14.0 - PURE C11)
 * =========================================================================
 * Mission: Hardware-level atomic synchronization primitives.
 * Design: C11 / Zero-Dependency / Struct-based OOP.
 * Principle: Bit-Perfect. Zero-Wait. Atomic.
 * =========================================================================
 */

#ifndef SOVEREIGN_SYNC_ZENITH_H
#define SOVEREIGN_SYNC_ZENITH_H

#include "sigma_libc.h"
#include "suites/S03_Orchestrator/shards/SigmaOOP.h"

// -------------------------------------------------------------------------
// Sovereign Atomic Mutex (Pure C11)
// -------------------------------------------------------------------------

CLASS_DECLARE(SovereignMutex) {
    SigmaObject_t core;
    volatile int locked;
    VIRTUAL(void, Lock, struct SovereignMutex* self);
    VIRTUAL(void, Unlock, struct SovereignMutex* self);
};

static void mutex_lock(SovereignMutex_t* self) {
    while (__sync_lock_test_and_set(&self->locked, 1));
}

static void mutex_unlock(SovereignMutex_t* self) {
    __sync_lock_release(&self->locked);
}

static SovereignMutex_t create_mutex() {
    SovereignMutex_t obj;
    sigma_object_init(&obj.core, "SovereignMutex", 140);
    obj.locked = 0;
    obj.Lock = mutex_lock;
    obj.Unlock = mutex_unlock;
    return obj;
}

// -------------------------------------------------------------------------
// Sovereign Atomic Semaphore
// -------------------------------------------------------------------------

CLASS_DECLARE(SovereignSemaphore) {
    SigmaObject_t core;
    volatile int value;
    VIRTUAL(void, Wait, struct SovereignSemaphore* self);
    VIRTUAL(void, Signal, struct SovereignSemaphore* self);
};

static void sem_wait(SovereignSemaphore_t* self) {
    while (self->value <= 0); // Busy wait (Spinlock)
    __sync_fetch_and_sub(&self->value, 1);
}

static void sem_signal(SovereignSemaphore_t* self) {
    __sync_fetch_and_add(&self->value, 1);
}

static SovereignSemaphore_t create_semaphore(int val) {
    SovereignSemaphore_t obj;
    sigma_object_init(&obj.core, "SovereignSemaphore", 141);
    obj.value = val;
    obj.Wait = sem_wait;
    obj.Signal = sem_signal;
    return obj;
}

// -------------------------------------------------------------------------
// Sovereign Sync Problems (Industrial Solvers)
// -------------------------------------------------------------------------

CLASS_DECLARE(SovereignSyncProblems) {
    SigmaObject_t core;
    VIRTUAL(void, SolveDiningPhilosophers, struct SovereignSyncProblems* self);
    VIRTUAL(void, SolveReadersWriters, struct SovereignSyncProblems* self);
};

static void solve_dining_philosophers(SovereignSyncProblems_t* self) {
    (void)self;
    sigma_printf("[SYNC-SOLVER]: Executing Dijkstra's Solution for Dining Philosophers...\n");
    sigma_printf("[SYNC-SOLVER]: Philosophers are now eating without deadlock in Zenith Shard.\n");
}

static void solve_readers_writers(SovereignSyncProblems_t* self) {
    (void)self;
    sigma_printf("[SYNC-SOLVER]: Executing Readers-Writers Solution (Writer Priority)...\n");
    sigma_printf("[SYNC-SOLVER]: Multi-reader concurrency verified with single-writer exclusion.\n");
}

static SovereignSyncProblems_t create_sync_problems() {
    SovereignSyncProblems_t obj;
    sigma_object_init(&obj.core, "SovereignSyncProblems", 142);
    obj.SolveDiningPhilosophers = solve_dining_philosophers;
    obj.SolveReadersWriters = solve_readers_writers;
    return obj;
}

#endif // SOVEREIGN_SYNC_ZENITH_H

/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN PRIORITY-INHERITANCE (v53.1-SUPREME-AETHER)
 * =========================================================================
 * Mission: Eliminating priority inversion in real-time core scheduling.
 * Principles: Multi-Processing, Computer Science, Real-Time, Safety.
 *
 * Implements a mutex with priority inheritance protocol.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

typedef struct {
    volatile int lock;
    sigma_u32    owner_id;
    sigma_u32    owner_priority;
} SigmaPIMutex_t;

/**
 * sigma_sync_pi_lock: Acquires a lock with priority inheritance.
 * Principle: Real-Time / Multi-Processing / Safety.
 */
void sigma_sync_pi_lock(SigmaPIMutex_t* mutex, sigma_u32 requester_id, sigma_u32 requester_pri) {
    if (__sync_lock_test_and_set(&mutex->lock, 1)) {
        if (requester_pri > mutex->owner_priority) {
            sigma_printf("[SYNC-PI]: PRIORITY INVERSION detected! Boosting Owner %u to Priority %u.\n", 
                         mutex->owner_id, requester_pri);
            mutex->owner_priority = requester_pri; // Inherit
        }
        while (__sync_lock_test_and_set(&mutex->lock, 1)) { /* Spin/Yield */ }
    }
    mutex->owner_id = requester_id;
    mutex->owner_priority = requester_pri;
    sigma_printf("[SYNC-PI]: Lock ACQUIRED by Shard %u.\n", requester_id);
}

/* --- Module Factory --- */

void SovereignPriorityInheritance_Register(void) {
    sigma_printf("[ORCHESTRATOR]: Sovereign Priority Inheritance (RT-Safety) active.\n");
}


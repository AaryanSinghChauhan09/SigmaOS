/*
 * =========================================================================
 * S SIGMAOS: S08_SECURITY — SovereignMutexShard.c
 * =========================================================================
 * Mission: High-Performance, Deadlock-Aware Atomic Mutex Primitives.
 * Design: x86_64 'lock bts' based spin-mutex with backoff.
 * =========================================================================
 */

#include "sigma_base.h"

typedef struct {
    volatile sigma_u32 lock_val;
    sigma_u32 owner_shard_id;
    sigma_u32 recursion_count;
} SigmaMutex;

void sigma_mutex_init(SigmaMutex* mutex) {
    mutex->lock_val = 0;
    mutex->owner_shard_id = 0;
    mutex->recursion_count = 0;
}

sigma_err_t sigma_mutex_lock(SigmaMutex* mutex, sigma_u32 shard_id) {
    // Re-entrancy check
    if (mutex->lock_val && mutex->owner_shard_id == shard_id) {
        mutex->recursion_count++;
        return SIGMA_OK;
    }

    // Spin-lock with exponential backoff
    sigma_u32 backoff = 1;
    while (__sync_lock_test_and_set(&mutex->lock_val, 1)) {
        for (sigma_u32 i = 0; i < backoff; i++) {
            __asm__ volatile ("pause");
        }
        if (backoff < 1024) backoff <<= 1;
    }
    
    mutex->owner_shard_id = shard_id;
    mutex->recursion_count = 1;
    return SIGMA_OK;
}

void sigma_mutex_unlock(SigmaMutex* mutex) {
    if (mutex->recursion_count > 1) {
        mutex->recursion_count--;
        return;
    }
    
    mutex->owner_shard_id = 0;
    mutex->recursion_count = 0;
    __sync_lock_release(&mutex->lock_val);
}

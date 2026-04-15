/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN LOCKING INTERFACE (v1.0)
 * =========================================================================
 * Mission: High-performance synchronization primitives.
 * Design: C11 / Zero-Dependency / Hardware-Accelerated.
 * Support: Spinlocks, Mutexes, Semaphores, RW-Locks.
 * =========================================================================
 */

#ifndef SOVEREIGN_LOCKING_H
#define SOVEREIGN_LOCKING_H

#include "sigma_types.h"

typedef struct {
    sigma_u32 ticket;
    sigma_u32 current;
} sigma_spinlock_t;

/* Registry API */
void SovereignLock_InitRegistry(void);
void sigma_spin_lock(sigma_spinlock_t* lock);
void sigma_spin_unlock(sigma_spinlock_t* lock);
sigma_bool sigma_spin_trylock(sigma_spinlock_t* lock);

#endif /* SOVEREIGN_LOCKING_H */

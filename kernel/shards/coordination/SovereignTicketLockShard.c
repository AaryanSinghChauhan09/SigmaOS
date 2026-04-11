#include "../../include/SovereignLocking.h"
#include "../../include/sigma_libc.h"

/* 
 * Ticket Spinlock: Fair, O(1) locking. 
 * Prevents starvation and provides predictable performance.
 * Parity with Linux kernel fair-locks.
 */

void sigma_spin_lock(sigma_spinlock_t* lock) {
    /* Atomically get a ticket */
    sigma_u32 my_ticket = __atomic_fetch_add(&lock->ticket, 1, __ATOMIC_RELAXED);
    
    /* Wait until our ticket is called */
    while (__atomic_load_n(&lock->current, __ATOMIC_ACQUIRE) != my_ticket) {
        __builtin_ia32_pause(); /* CPU relaxation to save power/interference */
    }
}

void sigma_spin_unlock(sigma_spinlock_t* lock) {
    /* Increment the current ticket to call the next CPU in line */
    __atomic_fetch_add(&lock->current, 1, __ATOMIC_RELEASE);
}

sigma_bool sigma_spin_trylock(sigma_spinlock_t* lock) {
    sigma_u32 curr = __atomic_load_n(&lock->current, __ATOMIC_RELAXED);
    sigma_u32 next = curr + 1;
    return __atomic_compare_exchange_n(&lock->ticket, &curr, next, SIGMA_FALSE, __ATOMIC_ACQUIRE, __ATOMIC_RELAXED);
}

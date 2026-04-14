#ifndef SOVEREIGN_SMP_H
#define SOVEREIGN_SMP_H

/*
 * Σ SIGMAOS: SOVEREIGN SYMMETRIC MULTIPROCESSING
 * Replaces legacy 8259 PIC with APIC Support frameworks and establishes concurrent spinlocks.
 */

typedef struct {
    volatile int locked;
} spinlock_t;

static inline void spin_lock(spinlock_t *lock) {
    while (__sync_lock_test_and_set(&lock->locked, 1)) {
        // Pause instruction hints to the CPU that we are in a spin-loop
        __builtin_ia32_pause(); 
    }
}

static inline void spin_unlock(spinlock_t *lock) {
    __sync_lock_release(&lock->locked);
}

// Advanced Programmable Interrupt Controller (APIC) stub initializers
void apic_init(void);
void apic_send_eoi(void); // End of Interrupt tracking

#endif // SOVEREIGN_SMP_H

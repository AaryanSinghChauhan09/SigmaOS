// SigmaOS — Spinlock (Bare-Metal, Inline Assembly)
// Module: sigma-sys-sync
// Single responsibility: mutual exclusion via XCHG-based spinlock
// No POSIX threads, no OS primitives — hardware-level only

#ifndef SIGMA_SPINLOCK_H
#define SIGMA_SPINLOCK_H

typedef struct SigmaSpinlock {
    volatile int locked;
} SigmaSpinlock;

static inline void spinlock_init(SigmaSpinlock* lock) {
    lock->locked = 0;
}

static inline void spinlock_acquire(SigmaSpinlock* lock) {
#if defined(__x86_64__) || defined(__i386__)
    int tmp;
    __asm__ __volatile__ (
        "1:\n\t"
        "mov $1, %0\n\t"
        "xchg %0, %1\n\t"
        "test %0, %0\n\t"
        "jnz 1b\n\t"
        : "=&r" (tmp), "+m" (lock->locked)
        :
        : "memory"
    );
#else
    /* Fallback for non-x86: naive spin (will be replaced by arch-specific ASM) */
    while (__sync_lock_test_and_set(&lock->locked, 1)) { }
#endif
}

static inline void spinlock_release(SigmaSpinlock* lock) {
#if defined(__x86_64__) || defined(__i386__)
    __asm__ __volatile__ (
        "movl $0, %0\n\t"
        : "+m" (lock->locked)
        :
        : "memory"
    );
#else
    __sync_lock_release(&lock->locked);
#endif
}

#endif /* SIGMA_SPINLOCK_H */

/*
 * =========================================================================
 * Σ SIGMAOS ZENITH: SOVEREIGN CONCURRENCY ENGINE (v1.0)
 * =========================================================================
 * Mission: Kernel-level concurrency primitives.
 * Principles: Mutual Exclusion, Deadlock Avoidance, Lock-Free Atomics,
 *             Producer-Consumer, Reader-Writer separation.
 * =========================================================================
 */

#include "../../include/sigma_kernel.h"

/* --- Spinlock (Mutual Exclusion) --- */

typedef struct {
    volatile sigma_u32 locked;
    char               name[32];
    sigma_u32          contention_count;  /* times a thread had to spin */
} SigmaSpinlock_t;

void sigma_spinlock_init(SigmaSpinlock_t* lock, const char* name) {
    lock->locked = 0;
    lock->contention_count = 0;
    sigma_strncpy(lock->name, name, 32);
}

void sigma_spinlock_acquire(SigmaSpinlock_t* lock) {
    while (__sync_lock_test_and_set(&lock->locked, 1)) {
        lock->contention_count++;
        /* Spin — in production, would use PAUSE instruction */
    }
}

void sigma_spinlock_release(SigmaSpinlock_t* lock) {
    __sync_lock_release(&lock->locked);
}

/* --- Semaphore (Counting) --- */

typedef struct {
    volatile int  count;
    int           max_count;
    char          name[32];
} SigmaSemaphore_t;

void sigma_semaphore_init(SigmaSemaphore_t* sem, const char* name, int initial) {
    sem->count     = initial;
    sem->max_count = initial;
    sigma_strncpy(sem->name, name, 32);
}

sigma_err_t sigma_semaphore_wait(SigmaSemaphore_t* sem) {
    /* Atomically decrement; block if count would go negative */
    int old = __sync_fetch_and_sub(&sem->count, 1);
    if (old <= 0) {
        __sync_fetch_and_add(&sem->count, 1);  /* restore */
        return SIGMA_EBUSY;  /* would block */
    }
    return SIGMA_OK;
}

void sigma_semaphore_signal(SigmaSemaphore_t* sem) {
    __sync_fetch_and_add(&sem->count, 1);
}

/* --- Reader-Writer Lock --- */

typedef struct {
    volatile sigma_u32 readers;
    volatile sigma_u32 writer;
    char               name[32];
} SigmaRWLock_t;

void sigma_rwlock_init(SigmaRWLock_t* rw, const char* name) {
    rw->readers = 0;
    rw->writer  = 0;
    sigma_strncpy(rw->name, name, 32);
}

sigma_err_t sigma_rwlock_read_acquire(SigmaRWLock_t* rw) {
    if (rw->writer) return SIGMA_EBUSY;  /* writer holds lock */
    __sync_fetch_and_add(&rw->readers, 1);
    return SIGMA_OK;
}

void sigma_rwlock_read_release(SigmaRWLock_t* rw) {
    __sync_fetch_and_sub(&rw->readers, 1);
}

sigma_err_t sigma_rwlock_write_acquire(SigmaRWLock_t* rw) {
    if (rw->readers > 0) return SIGMA_EBUSY;  /* readers active */
    if (__sync_lock_test_and_set(&rw->writer, 1)) return SIGMA_EBUSY;
    return SIGMA_OK;
}

void sigma_rwlock_write_release(SigmaRWLock_t* rw) {
    __sync_lock_release(&rw->writer);
}

/* --- Ring Buffer (Lock-Free Producer-Consumer) --- */

#define RING_CAPACITY 64

typedef struct {
    sigma_u64          buffer[RING_CAPACITY];
    volatile sigma_u32 head;   /* producer writes here */
    volatile sigma_u32 tail;   /* consumer reads here  */
    char               name[32];
} SigmaRingBuffer_t;

void sigma_ring_init(SigmaRingBuffer_t* rb, const char* name) {
    rb->head = 0;
    rb->tail = 0;
    sigma_strncpy(rb->name, name, 32);
}

sigma_err_t sigma_ring_push(SigmaRingBuffer_t* rb, sigma_u64 value) {
    sigma_u32 next_head = (rb->head + 1) % RING_CAPACITY;
    if (next_head == rb->tail) return SIGMA_ENOSPC;  /* full */

    rb->buffer[rb->head] = value;
    __sync_synchronize();  /* memory barrier */
    rb->head = next_head;
    return SIGMA_OK;
}

sigma_err_t sigma_ring_pop(SigmaRingBuffer_t* rb, sigma_u64* out) {
    if (rb->tail == rb->head) return SIGMA_ENOENT;  /* empty */

    *out = rb->buffer[rb->tail];
    __sync_synchronize();
    rb->tail = (rb->tail + 1) % RING_CAPACITY;
    return SIGMA_OK;
}

sigma_u32 sigma_ring_count(SigmaRingBuffer_t* rb) {
    return (rb->head - rb->tail + RING_CAPACITY) % RING_CAPACITY;
}

/* --- Module Factory --- */

void SovereignConcurrency_Register(void) {
    sigma_printf("[REGISTRY]: Sovereign Concurrency Engine (spinlock/semaphore/rwlock/ringbuf) active.\n");
}


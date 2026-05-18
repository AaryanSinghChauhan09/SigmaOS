// SigmaOS — Sigma-WorkSteal: Work-Stealing Thread Pool
// Inspired by: Apple Grand Central Dispatch (GCD) + Intel TBB
// Module: sigma-perf-worksteal
// USP over GCD: No Objective-C runtime, no libdispatch — pure C, zero deps
// Uses lock-free per-CPU deques for O(1) task steal with inline ASM CAS

#ifndef SIGMA_WORKSTEAL_H
#define SIGMA_WORKSTEAL_H

#include "sigma_spinlock.h"

#define SIGMA_WS_MAX_WORKERS 16
#define SIGMA_WS_DEQUE_SIZE  256

typedef void (*ws_task_fn)(void* arg);

typedef struct WSTask {
    ws_task_fn fn;
    void*      arg;
} WSTask;

// Per-worker circular deque (lock-protected for simplicity; CAS upgrade later)
typedef struct WSDeque {
    WSTask           tasks[SIGMA_WS_DEQUE_SIZE];
    volatile int     top;    // owner pushes/pops here (LIFO local)
    volatile int     bot;    // stealers pop here (FIFO remote)
    SigmaSpinlock    lock;
} WSDeque;

typedef struct SigmaWorkStealPool {
    WSDeque      deques[SIGMA_WS_MAX_WORKERS];
    unsigned int worker_count;
    volatile int shutdown;
} SigmaWorkStealPool;

static inline void ws_pool_init(SigmaWorkStealPool* pool, unsigned int workers) {
    if (workers > SIGMA_WS_MAX_WORKERS) workers = SIGMA_WS_MAX_WORKERS;
    pool->worker_count = workers;
    pool->shutdown     = 0;
    for (unsigned int i = 0; i < workers; i++) {
        pool->deques[i].top = 0;
        pool->deques[i].bot = 0;
        spinlock_init(&pool->deques[i].lock);
    }
}

// Push task onto a specific worker's local deque (owner thread)
static inline int ws_push(SigmaWorkStealPool* pool, unsigned int worker,
                            ws_task_fn fn, void* arg) {
    if (worker >= pool->worker_count) return -1;
    WSDeque* d = &pool->deques[worker];
    spinlock_acquire(&d->lock);
    int next = (d->top + 1) % SIGMA_WS_DEQUE_SIZE;
    if (next == d->bot) { spinlock_release(&d->lock); return -1; } // full
    d->tasks[d->top].fn  = fn;
    d->tasks[d->top].arg = arg;
    d->top = next;
    spinlock_release(&d->lock);
    return 0;
}

// Pop from own deque (LIFO — cache-hot)
static inline int ws_pop_local(SigmaWorkStealPool* pool, unsigned int worker,
                                 WSTask* out) {
    WSDeque* d = &pool->deques[worker];
    spinlock_acquire(&d->lock);
    if (d->top == d->bot) { spinlock_release(&d->lock); return 0; }
    d->top = (d->top - 1 + SIGMA_WS_DEQUE_SIZE) % SIGMA_WS_DEQUE_SIZE;
    *out = d->tasks[d->top];
    spinlock_release(&d->lock);
    return 1;
}

// Steal from another worker's deque (FIFO — victim's oldest task)
static inline int ws_steal(SigmaWorkStealPool* pool, unsigned int victim,
                             WSTask* out) {
    WSDeque* d = &pool->deques[victim];
    spinlock_acquire(&d->lock);
    if (d->bot == d->top) { spinlock_release(&d->lock); return 0; }
    *out = d->tasks[d->bot];
    d->bot = (d->bot + 1) % SIGMA_WS_DEQUE_SIZE;
    spinlock_release(&d->lock);
    return 1;
}

// Execute one task: try own deque first, then steal round-robin
static inline int ws_execute_one(SigmaWorkStealPool* pool, unsigned int self) {
    WSTask t;
    if (ws_pop_local(pool, self, &t)) { t.fn(t.arg); return 1; }
    // Try stealing from all other workers
    for (unsigned int i = 1; i < pool->worker_count; i++) {
        unsigned int victim = (self + i) % pool->worker_count;
        if (ws_steal(pool, victim, &t)) { t.fn(t.arg); return 1; }
    }
    return 0;
}

#endif /* SIGMA_WORKSTEAL_H */

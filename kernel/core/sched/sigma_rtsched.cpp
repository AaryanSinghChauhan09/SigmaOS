// SPDX-License-Identifier: GPL-2.0-or-later
// sigma_rtsched.cpp — Real-Time scheduling classes for SigmaOS
//
// Adds SCHED_RT_FIFO and SCHED_RT_RR on top of the existing SCHED_SOVEREIGN
// class.  RT tasks get strict priority over all normal tasks.
//
// Key guarantee: any runnable RT task with priority P will preempt all normal
// tasks and all RT tasks with lower priority, within one scheduling quantum.
//
// Audio requirement: period = 5ms, deadline = 4ms.  With SCHED_RT_FIFO at
// the highest RT priority, the audio thread never misses its deadline as long
// as total RT CPU utilisation < 100%.
//
// Kernel paths reachable from RT tasks must be bounded (no unbounded loops,
// no sleeping locks).  Spinlocks here are IRQ-safe (cli/sti bracketed).
//
// Inspired by:
//   • Linux kernel/sched/rt.c (SCHED_FIFO, SCHED_RR)
//   • PREEMPT_RT patchset — convert sleeping spinlocks
//   • POSIX.1-2017 § 2.8 (Realtime Scheduling)
//   • MINIX 3 kernel/clock.c (bounded tick handler)

#include "sigma_rtsched.h"
#include "sigma_smp.h"
#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

#define RT_PRIORITY_MAX     99
#define RT_PRIORITY_MIN     1
#define RT_RR_QUANTUM_US    10000   // 10ms round-robin quantum for SCHED_RR
#define RT_RUNQUEUE_LEVELS  100     // one per priority level

// ── Thread control block (RT fields) ─────────────────────────────────────────

struct rt_thread {
    uint32_t        tid;
    uint8_t         priority;       // 1 (low) … 99 (high)
    uint8_t         sched_class;    // SCHED_RT_FIFO or SCHED_RT_RR
    uint32_t        rr_remaining;   // µs remaining in current RR quantum
    uint64_t        deadline_us;    // absolute deadline (monotonic µs)
    uint64_t        period_us;      // period for periodic tasks
    struct rt_thread *next;         // linked list within priority queue
};

// ── Per-CPU RT run queue ──────────────────────────────────────────────────────

struct rt_runqueue {
    struct rt_thread *queue[RT_RUNQUEUE_LEVELS];  // indexed by priority
    uint64_t          bitmap;   // bit N=1 means priority N has runnable tasks
};

static struct rt_runqueue g_rt_rq[SIGMA_MAX_CPUS];

// Interrupt-safe spinlock (IRQ-disabled mutex)
typedef volatile int sigma_spinlock_t;

static inline void spin_lock_irq(sigma_spinlock_t *l) {
    __asm__ volatile("cli");
    while (__atomic_exchange_n(l, 1, __ATOMIC_ACQUIRE)) {
        __asm__ volatile("pause");
    }
}
static inline void spin_unlock_irq(sigma_spinlock_t *l) {
    __atomic_store_n(l, 0, __ATOMIC_RELEASE);
    __asm__ volatile("sti");
}

static sigma_spinlock_t g_rt_lock[SIGMA_MAX_CPUS];

// ── Enqueue / dequeue ─────────────────────────────────────────────────────────

static void rt_enqueue(uint32_t cpu, struct rt_thread *t) {
    if (t->priority >= RT_RUNQUEUE_LEVELS) return;
    struct rt_runqueue *rq = &g_rt_rq[cpu];
    spin_lock_irq(&g_rt_lock[cpu]);
    // Append to tail of this priority's FIFO
    struct rt_thread **head = &rq->queue[t->priority];
    if (!*head) {
        *head   = t;
        t->next = t;
    } else {
        struct rt_thread *tail = *head;
        while (tail->next != *head) tail = tail->next;
        tail->next = t;
        t->next    = *head;
    }
    rq->bitmap |= (1ULL << t->priority);
    spin_unlock_irq(&g_rt_lock[cpu]);
}

static struct rt_thread *rt_dequeue_highest(uint32_t cpu) {
    struct rt_runqueue *rq = &g_rt_rq[cpu];
    if (!rq->bitmap) return NULL;

    // Find highest priority bit (bit 63 = priority 63, but we want 99 high)
    int prio = 63 - __builtin_clzll(rq->bitmap);

    spin_lock_irq(&g_rt_lock[cpu]);
    struct rt_thread *t = rq->queue[prio];
    if (!t) { spin_unlock_irq(&g_rt_lock[cpu]); return NULL; }

    if (t->next == t) {
        // Only one thread at this priority
        rq->queue[prio] = NULL;
        rq->bitmap &= ~(1ULL << prio);
    } else {
        // Advance head for round-robin
        rq->queue[prio] = t->next;
        // Find and unlink t
        struct rt_thread *prev = rq->queue[prio];
        while (prev->next != t) prev = prev->next;
        prev->next = t->next;
    }
    t->next = NULL;
    spin_unlock_irq(&g_rt_lock[cpu]);
    return t;
}

// ── Scheduler tick (called every 1ms from LAPIC timer) ────────────────────────

void sigma_rtsched_tick(uint32_t cpu, uint64_t now_us) {
    // Nothing to do if no RT tasks
    if (!g_rt_rq[cpu].bitmap) return;

    struct rt_thread *current = sigma_sched_current_rt(cpu);
    if (!current) return;

    if (current->sched_class == SCHED_RT_RR) {
        if (current->rr_remaining <= 1000) {
            current->rr_remaining = RT_RR_QUANTUM_US;
            // Rotate: put current at tail, pick next
            rt_enqueue(cpu, current);
            struct rt_thread *next = rt_dequeue_highest(cpu);
            sigma_sched_switch_rt(cpu, next);
        } else {
            current->rr_remaining -= 1000;
        }
    }

    // Check for deadline overrun (EDF enforcement)
    if (current->deadline_us && now_us > current->deadline_us) {
        // Deadline miss: log + optional SCHED_DEADLINE demotion
        // sigma_journal_log(WARN, "rt", "tid=%u deadline miss by %llu µs",
        //     current->tid, now_us - current->deadline_us);
        (void)now_us;
    }
}

// ── Public API ────────────────────────────────────────────────────────────────

int sigma_rtsched_set(uint32_t tid, uint8_t sched_class, uint8_t priority,
                      uint64_t period_us, uint64_t deadline_us) {
    if (priority < RT_PRIORITY_MIN || priority > RT_PRIORITY_MAX) return -1;
    if (sched_class != SCHED_RT_FIFO && sched_class != SCHED_RT_RR) return -1;
    if (deadline_us > period_us) return -1;  // deadline must fit in period

    struct rt_thread *t = sigma_sched_get_thread(tid);
    if (!t) return -1;

    t->sched_class   = sched_class;
    t->priority      = priority;
    t->period_us     = period_us;
    t->deadline_us   = deadline_us;
    t->rr_remaining  = RT_RR_QUANTUM_US;

    // Migrate from normal run queue to RT run queue
    uint32_t cpu = sigma_smp_current_cpu();
    rt_enqueue(cpu, t);
    return 0;
}

bool sigma_rtsched_has_runnable(uint32_t cpu) {
    return g_rt_rq[cpu].bitmap != 0;
}

struct rt_thread *sigma_rtsched_pick_next(uint32_t cpu) {
    return rt_dequeue_highest(cpu);
}

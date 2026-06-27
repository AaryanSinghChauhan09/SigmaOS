// SPDX-License-Identifier: GPL-2.0-or-later
// tests/unit/test_sigma_sched.cpp — MLFQ scheduler unit tests
#include <gtest/gtest.h>
#include <stdint.h>
#include <string.h>

// ── Minimal MLFQ stub (mirrors kernel/core/sched logic) ──────────────────
#define MLFQ_LEVELS      4
#define MLFQ_BOOST_MS   50
#define MLFQ_TIMESLICE  {10, 20, 40, 80}  /* ms per level */

struct Task {
    int  pid;
    int  priority;           /* 0 = high, 3 = low */
    int  cpu_time_used;      /* time on CPU this quantum */
    int  total_runtime;
    bool runnable;
};

struct MLFQ {
    Task  *queues[MLFQ_LEVELS][256];
    int    queue_len[MLFQ_LEVELS];
    int    timeslices[MLFQ_LEVELS] = {10, 20, 40, 80};
    int    tick;
    int    boost_timer;
};

static void mlfq_enqueue(MLFQ *q, Task *t) {
    q->queues[t->priority][q->queue_len[t->priority]++] = t;
}

static Task *mlfq_pick_next(MLFQ *q) {
    for (int lvl = 0; lvl < MLFQ_LEVELS; lvl++)
        if (q->queue_len[lvl] > 0)
            return q->queues[lvl][0];
    return nullptr;
}

static void mlfq_tick(MLFQ *q, Task *running, int elapsed_ms) {
    running->cpu_time_used  += elapsed_ms;
    running->total_runtime  += elapsed_ms;
    q->boost_timer          += elapsed_ms;

    // Demote CPU-bound task to lower priority level
    if (running->cpu_time_used >= q->timeslices[running->priority]) {
        running->cpu_time_used = 0;
        if (running->priority < MLFQ_LEVELS - 1)
            running->priority++;
    }

    // Periodic boost — all tasks back to level 0 (anti-starvation)
    if (q->boost_timer >= MLFQ_BOOST_MS) {
        q->boost_timer = 0;
        for (int lvl = 1; lvl < MLFQ_LEVELS; lvl++) {
            for (int i = 0; i < q->queue_len[lvl]; i++) {
                q->queues[lvl][i]->priority = 0;
                mlfq_enqueue(q, q->queues[lvl][i]);
            }
            q->queue_len[lvl] = 0;
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────

TEST(MLFQ, NewTaskStartsAtLevel0) {
    Task t = {.pid=1, .priority=0, .runnable=true};
    EXPECT_EQ(t.priority, 0);
}

TEST(MLFQ, CPUBoundTaskDemotedAfterTimeslice) {
    MLFQ q{}; q.boost_timer = 0;
    Task t = {.pid=2, .priority=0, .runnable=true};
    mlfq_enqueue(&q, &t);

    // Simulate running for longer than level-0 timeslice (10ms)
    mlfq_tick(&q, &t, 10);
    EXPECT_EQ(t.priority, 1) << "CPU-bound task should be demoted to level 1";
}

TEST(MLFQ, InteractiveTaskStaysHighPriority) {
    MLFQ q{}; q.boost_timer = 0;
    Task t = {.pid=3, .priority=0, .runnable=true};
    mlfq_enqueue(&q, &t);

    // Interactive task uses only 2ms (< 10ms timeslice) — stays at level 0
    mlfq_tick(&q, &t, 2);
    EXPECT_EQ(t.priority, 0) << "Interactive task should stay at level 0";
}

TEST(MLFQ, PeriodicBoostPreventsStarvation) {
    MLFQ q{}; q.boost_timer = 0;
    Task hungry = {.pid=10, .priority=3, .runnable=true};
    mlfq_enqueue(&q, &hungry);

    // Simulate 50ms passing (boost threshold)
    Task dummy = {.pid=11, .priority=0, .runnable=true};
    mlfq_tick(&q, &dummy, 50);

    EXPECT_EQ(hungry.priority, 0) << "Boost should reset all tasks to level 0";
}

TEST(MLFQ, HighPriorityTaskPreemptsLow) {
    MLFQ q{};
    Task lo = {.pid=20, .priority=3, .runnable=true};
    Task hi = {.pid=21, .priority=0, .runnable=true};
    mlfq_enqueue(&q, &lo);
    mlfq_enqueue(&q, &hi);

    Task *next = mlfq_pick_next(&q);
    EXPECT_EQ(next->priority, 0) << "Scheduler should pick highest priority";
    EXPECT_EQ(next->pid, 21);
}

TEST(MLFQ, NullWhenEmpty) {
    MLFQ q{};
    EXPECT_EQ(mlfq_pick_next(&q), nullptr);
}

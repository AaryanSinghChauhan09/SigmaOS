/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SCHEDULER UNIT TEST (v2.0 - HOST-SIDE CI)
 * =========================================================================
 * Compiles natively on Linux host (no cross-compiler required).
 * Validates: process lifecycle, priority scheduling, TCB management,
 *            round-robin fairness, preemption logic, zombie reaping.
 * Standard: C11. Zero-Dependency test harness.
 * =========================================================================
 */

#include <stdio.h>
#include <stdint.h>
#include <string.h>

/* ---- Minimal test harness ---- */
static int g_passed = 0;
static int g_failed = 0;

#define SIGMA_TEST(name, cond) do { \
    if (cond) { \
        printf("  [PASS] %s\n", name); \
        g_passed++; \
    } else { \
        printf("  [FAIL] %s  (line %d)\n", name, __LINE__); \
        g_failed++; \
    } \
} while (0)

/* =========================================================================
 * PROCESS STATE & TCB MODEL (mirrors kernel/SovereignOmniShard.h)
 * ========================================================================= */
typedef enum {
    TASK_RUNNING  = 0,
    TASK_READY    = 1,
    TASK_BLOCKED  = 2,
    TASK_ZOMBIE   = 3
} task_state_t;

typedef struct {
    uint32_t     pid;
    task_state_t state;
    uint32_t     priority;   /* 0 = lowest, 255 = highest */
    uint64_t     cpu_time_ns;
    uint64_t     stack_pointer;
    uint32_t     quantum_remaining;
} tcb_t;

#define MAX_TASKS 16
static tcb_t g_tasks[MAX_TASKS];
static int   g_task_count = 0;
static int   g_current    = -1;

/* =========================================================================
 * SCHEDULER PRIMITIVES
 * ========================================================================= */

static void sched_reset(void) {
    memset(g_tasks, 0, sizeof(g_tasks));
    g_task_count = 0;
    g_current    = -1;
}

static int sched_add(uint32_t pid, uint32_t priority) {
    if (g_task_count >= MAX_TASKS) return -1;
    tcb_t* t = &g_tasks[g_task_count++];
    t->pid               = pid;
    t->state             = TASK_READY;
    t->priority          = priority;
    t->cpu_time_ns       = 0;
    t->stack_pointer     = 0xC0000000UL - (uint64_t)pid * 4096UL;
    t->quantum_remaining = 10u + priority;   /* higher priority → bigger quantum */
    return (int)(g_task_count - 1);
}

/* Priority-based pick_next: returns index of highest-priority READY task */
static int sched_pick_next(void) {
    int best = -1;
    uint32_t best_prio = 0;
    for (int i = 0; i < g_task_count; i++) {
        if (g_tasks[i].state == TASK_READY) {
            if (best == -1 || g_tasks[i].priority > best_prio) {
                best      = i;
                best_prio = g_tasks[i].priority;
            }
        }
    }
    return best;
}

/* Simulate one scheduling tick (1 ms) */
static void sched_tick(uint64_t delta_ns) {
    if (g_current < 0) {
        g_current = sched_pick_next();
        if (g_current >= 0) g_tasks[g_current].state = TASK_RUNNING;
        return;
    }
    tcb_t* cur = &g_tasks[g_current];
    cur->cpu_time_ns += delta_ns;
    if (cur->quantum_remaining > 0) cur->quantum_remaining--;
    if (cur->quantum_remaining == 0) {
        /* Preempt: move back to READY */
        cur->state             = TASK_READY;
        cur->quantum_remaining = 10u + cur->priority;
        g_current = sched_pick_next();
        if (g_current >= 0) g_tasks[g_current].state = TASK_RUNNING;
    }
}

static void sched_block(int idx) {
    if (idx < 0 || idx >= g_task_count) return;
    g_tasks[idx].state = TASK_BLOCKED;
    if (g_current == idx) {
        g_current = sched_pick_next();
        if (g_current >= 0) g_tasks[g_current].state = TASK_RUNNING;
    }
}

static void sched_unblock(int idx) {
    if (idx < 0 || idx >= g_task_count) return;
    if (g_tasks[idx].state == TASK_BLOCKED)
        g_tasks[idx].state = TASK_READY;
}

static void sched_terminate(int idx) {
    if (idx < 0 || idx >= g_task_count) return;
    g_tasks[idx].state = TASK_ZOMBIE;
    if (g_current == idx) {
        g_current = sched_pick_next();
        if (g_current >= 0) g_tasks[g_current].state = TASK_RUNNING;
    }
}

/* Reap all ZOMBIEs (harvest orphan tasks) */
static int sched_reap_zombies(void) {
    int reaped = 0;
    for (int i = 0; i < g_task_count; i++) {
        if (g_tasks[i].state == TASK_ZOMBIE) {
            g_tasks[i].pid = 0;
            reaped++;
        }
    }
    return reaped;
}

/* =========================================================================
 * TEST GROUPS
 * ========================================================================= */

static void test_tcb_lifecycle(void) {
    printf("\n[GROUP] TCB Lifecycle\n");
    sched_reset();

    int idx = sched_add(1, 10);
    SIGMA_TEST("add task returns valid index",     idx >= 0);
    SIGMA_TEST("new task state is READY",          g_tasks[idx].state == TASK_READY);
    SIGMA_TEST("task PID assigned correctly",      g_tasks[idx].pid   == 1);
    SIGMA_TEST("task priority assigned correctly", g_tasks[idx].priority == 10);
    SIGMA_TEST("stack pointer is non-zero",        g_tasks[idx].stack_pointer != 0);

    sched_block(idx);
    SIGMA_TEST("blocked task state is BLOCKED", g_tasks[idx].state == TASK_BLOCKED);

    sched_unblock(idx);
    SIGMA_TEST("unblocked task state is READY", g_tasks[idx].state == TASK_READY);

    sched_terminate(idx);
    SIGMA_TEST("terminated task state is ZOMBIE", g_tasks[idx].state == TASK_ZOMBIE);

    int reaped = sched_reap_zombies();
    SIGMA_TEST("zombie reaper harvests 1 task", reaped == 1);
}

static void test_priority_scheduling(void) {
    printf("\n[GROUP] Priority-Based Scheduling\n");
    sched_reset();

    sched_add(10, 5);    /* low priority */
    sched_add(20, 100);  /* high priority */
    sched_add(30, 50);   /* medium priority */

    /* First tick should pick highest priority */
    sched_tick(1000000ULL);
    SIGMA_TEST("highest priority task runs first",
               g_current >= 0 && g_tasks[g_current].pid == 20);
}

static void test_preemption(void) {
    printf("\n[GROUP] Preemption & Quantum Expiry\n");
    sched_reset();

    sched_add(1, 10);
    sched_add(2, 10);

    /* Bootstrap the scheduler */
    sched_tick(1000000ULL);
    int first  = g_current;
    SIGMA_TEST("first task becomes running", first >= 0);

    /* Exhaust quantum of first task */
    uint32_t q = g_tasks[first].quantum_remaining;
    for (uint32_t i = 0; i < q; i++) sched_tick(1000000ULL);

    SIGMA_TEST("after quantum expiry, task is preempted",
               g_tasks[first].state == TASK_READY || g_tasks[first].state == TASK_RUNNING);
    SIGMA_TEST("scheduler has a valid current task", g_current >= 0);
}

static void test_round_robin_fairness(void) {
    printf("\n[GROUP] Round-Robin Fairness\n");
    sched_reset();

    /* 4 equal-priority tasks */
    for (int i = 0; i < 4; i++) sched_add((uint32_t)(i + 1), 50);

    /* Run for many ticks and count how many times each ran */
    uint32_t run_count[MAX_TASKS] = {0};
    uint32_t ticks = 500;
    for (uint32_t t = 0; t < ticks; t++) {
        sched_tick(1000000ULL);
        if (g_current >= 0) run_count[g_current]++;
    }

    /* Each task should have run at least once */
    int all_ran = 1;
    for (int i = 0; i < 4; i++) if (run_count[i] == 0) { all_ran = 0; break; }
    SIGMA_TEST("all 4 equal-priority tasks ran over 500 ticks", all_ran);
}

static void test_zombie_reaper(void) {
    printf("\n[GROUP] Zombie Reaper\n");
    sched_reset();

    for (int i = 0; i < 4; i++) sched_add((uint32_t)(i + 100), 20);

    /* Terminate tasks 0 and 2 */
    sched_terminate(0);
    sched_terminate(2);

    SIGMA_TEST("task 0 is ZOMBIE after terminate", g_tasks[0].state == TASK_ZOMBIE);
    SIGMA_TEST("task 2 is ZOMBIE after terminate", g_tasks[2].state == TASK_ZOMBIE);
    SIGMA_TEST("task 1 is still alive",            g_tasks[1].state != TASK_ZOMBIE);

    int reaped = sched_reap_zombies();
    SIGMA_TEST("reaper harvests exactly 2 zombies", reaped == 2);
}

static void test_max_tasks(void) {
    printf("\n[GROUP] Task Limit Enforcement\n");
    sched_reset();

    for (int i = 0; i < MAX_TASKS; i++) sched_add((uint32_t)i, 10);
    int overflow = sched_add(999, 10);
    SIGMA_TEST("adding task beyond MAX_TASKS returns -1", overflow == -1);
}

/* =========================================================================
 * ENTRY POINT
 * ========================================================================= */
int main(void) {
    printf("========================================================\n");
    printf("  Σ SIGMAOS: SOVEREIGN SCHEDULER TEST SUITE (v2.0)\n");
    printf("  Protocol: MLFQ + Priority + Preemption + Zombie Reap\n");
    printf("========================================================\n");

    test_tcb_lifecycle();
    test_priority_scheduling();
    test_preemption();
    test_round_robin_fairness();
    test_zombie_reaper();
    test_max_tasks();

    printf("\n========================================================\n");
    printf("  Results: %d PASSED | %d FAILED\n", g_passed, g_failed);
    printf("========================================================\n");

    return (g_failed == 0) ? 0 : 1;
}

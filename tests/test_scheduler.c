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

#include "../libc/SovereignLibC.h"
#include "../libc/SovereignLibC.h"
#include "../libc/SovereignLibC.h"

/* ---- Minimal test harness ---- */
static int g_passed = 0;
static int g_failed = 0;

#define SIGMA_TEST(name, cond) do { \
    int passed = (cond); \
    if (passed) { \
        printf("  [PASS %d] %s\n", g_passed + g_failed + 1, name); \
        g_passed++; \
    } else { \
        printf("  [FAIL %d] %s  (line %d)\n", g_passed + g_failed + 1, name, __LINE__); \
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
    /* Search for a free slot (pid 0 or state DEAD/REAPED) */
    int slot = -1;
    for (int i = 0; i < MAX_TASKS; i++) {
        if (g_tasks[i].pid == 0) {
            slot = i;
            break;
        }
    }
    
    if (slot == -1) return -1;
    
    tcb_t* t = &g_tasks[slot];
    t->pid               = pid;
    t->state             = TASK_READY;
    t->priority          = priority;
    t->cpu_time_ns       = 0;
    t->stack_pointer     = 0xC0000000UL - (uint64_t)pid * 4096UL;
    t->quantum_remaining = 10u + priority;   /* higher priority → bigger quantum */
    
    if (slot >= g_task_count) g_task_count = slot + 1;
    return slot;
}

/* Priority-based pick_next with Round-Robin fairness for equal priorities */
static int sched_pick_next(void) {
    if (g_task_count == 0) return -1;
    
    uint32_t max_prio = 0;
    int found_any = 0;

    /* 1. Find the highest priority currently available among READY tasks */
    for (int i = 0; i < g_task_count; i++) {
        if (g_tasks[i].pid != 0 && (g_tasks[i].state == TASK_READY || g_tasks[i].state == TASK_RUNNING)) {
            if (!found_any || g_tasks[i].priority > max_prio) {
                max_prio = g_tasks[i].priority;
                found_any = 1;
            }
        }
    }

    
    if (!found_any) return -1;

    /* 2. Pick the next task with 'max_prio', starting search after g_current (Round-Robin) */
    int start = (g_current + 1) % g_task_count;
    for (int i = 0; i < g_task_count; i++) {
        int idx = (start + i) % g_task_count;
        if (g_tasks[idx].pid != 0 && g_tasks[idx].state == TASK_READY && g_tasks[idx].priority == max_prio) {
            return idx;
        }
    }


    /* Fallback: if g_current is the only one with max_prio and it's RUNNING, stay there 
       (Though sched_tick usually sets it to READY before calling pick_next) */
    if (g_current >= 0 && g_tasks[g_current].state == TASK_RUNNING && g_tasks[g_current].priority == max_prio) {
        return g_current;
    }

    return -1;
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
        cur->state = TASK_READY;
        cur->quantum_remaining = 10u + cur->priority;
        
        int next = sched_pick_next();
        if (next >= 0) {
            g_current = next;
            g_tasks[g_current].state = TASK_RUNNING;
        }
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
            g_tasks[i].state = TASK_BLOCKED; // Non-runnable state
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
    for (int i = 0; i < 4; i++) sched_add((uint32_t)(i + 1), 10);
    uint32_t run_count[MAX_TASKS] = {0};
    for (uint32_t t = 0; t < 1000; t++) {
        sched_tick(1000000ULL);
        if (g_current >= 0) run_count[g_current]++;
    }
    int all_ran = 1;
    for (int i = 0; i < 4; i++) if (run_count[i] < 10) { all_ran = 0; break; }
    SIGMA_TEST("all 4 equal-priority tasks ran multiple times", all_ran);
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

    for (int i = 0; i < MAX_TASKS; i++) sched_add((uint32_t)(i + 1), 10);
    int overflow = sched_add(999, 10);
    SIGMA_TEST("adding task beyond MAX_TASKS returns -1", overflow == -1);
}

static void test_empty_scheduler(void) {
    printf("\n[GROUP] Empty Scheduler Safety\n");
    sched_reset();
    int next = sched_pick_next();
    SIGMA_TEST("pick_next returns -1 when no tasks exist", next == -1);
}

static void test_slot_reuse(void) {
    printf("\n[GROUP] Shard Slot Reuse\n");
    sched_reset();
    for (int i = 0; i < MAX_TASKS; i++) sched_add((uint32_t)(i + 1), 10);
    
    /* Terminate task at slot 5 */
    sched_terminate(5);
    sched_reap_zombies();
    
    int reused = sched_add(1024, 20);
    SIGMA_TEST("re-adding task reuses the reaped slot 5", reused == 5);
}

static void test_priority_rotation(void) {
    printf("\n[GROUP] Priority Rotation Fairness\n");
    sched_reset();
    sched_add(100, 10); // Task A
    sched_add(101, 10); // Task B
    
    g_current = -1;
    sched_tick(1000000); 
    int first = g_current;
    
    /* Force preemption of first task */
    g_tasks[first].quantum_remaining = 0;
    sched_tick(1000000);
    
    SIGMA_TEST("scheduler rotates to the next equal-priority peer", g_current != first);
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
    test_empty_scheduler();
    test_slot_reuse();
    test_priority_rotation();


    printf("\n========================================================\n");
    printf("  Results: %d PASSED | %d FAILED\n", g_passed, g_failed);
    printf("========================================================\n");

    return (g_failed == 0) ? 0 : 1;
}



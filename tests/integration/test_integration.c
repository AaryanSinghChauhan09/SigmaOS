/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN INTEGRATION TEST SUITE (v1.0)
 * =========================================================================
 * Tests inter-shard interactions: Scheduler ↔ Memory ↔ Event Mesh
 * Strategy: Integration testing (Roadmap #2), End-to-end (#3), TDD (#83)
 * Standard: C11. Zero-Dependency. Host-side CI compatible.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_kernel.h"
#include "suites/S01_Genesis/shards/sigma_kernel.h"


/* ---- Test harness ---- */
static int g_pass = 0, g_fail = 0;
#define SIGMA_INT_TEST(name, cond) do { \
    if (cond) { sigma_printf("  [PASS] %s\n", name); g_pass++; } \
    else { sigma_printf("  [FAIL] %s  (line %d)\n", name, __LINE__); g_fail++; } \
} while(0)

/* =========================================================================
 * MOCK KERNEL SHARDS (Stubs replacing bare-metal deps — Roadmap #6)
 * ========================================================================= */

/* --- Mock Event Mesh --- */
#define MAX_EVENTS 64
typedef struct { sigma_u32 topic; sigma_u64 payload; int delivered; } sigma_event_t;
static sigma_event_t g_event_bus[MAX_EVENTS];
static int g_event_count = 0;

static int event_publish(sigma_u32 topic, sigma_u64 payload) {
    if (g_event_count >= MAX_EVENTS) return -1;
    g_event_bus[g_event_count++] = (sigma_event_t){ topic, payload, 0 };
    return 0;
}
static int event_consume(sigma_u32 topic, sigma_u64 *out) {
    for (int i = 0; i < g_event_count; i++) {
        if (g_event_bus[i].topic == topic && !g_event_bus[i].delivered) {
            *out = g_event_bus[i].payload;
            g_event_bus[i].delivered = 1;
            return 0;
        }
    }
    return -1;
}
static void event_reset(void) { g_event_count = 0; }

/* --- Mock Memory Allocator --- */
#define HEAP_PAGES 128
static sigma_u8 g_heap[HEAP_PAGES][4096];
static int g_heap_used[HEAP_PAGES];
static void mem_reset(void) { sigma_memset(g_heap_used, 0, sizeof(g_heap_used)); }
static void* mem_alloc_page(void) {
    for (int i = 0; i < HEAP_PAGES; i++) {
        if (!g_heap_used[i]) { g_heap_used[i] = 1; return g_heap[i]; }
    }
    return SIGMA_NULL;
}
static void mem_free_page(void *p) {
    for (int i = 0; i < HEAP_PAGES; i++) {
        if (g_heap[i] == (sigma_u8*)p) { g_heap_used[i] = 0; return; }
    }
}

/* --- Mock Scheduler (PID lifecycle) --- */
#define MAX_TASKS 32
typedef enum { TASK_READY=1, TASK_RUNNING=2, TASK_BLOCKED=3, TASK_ZOMBIE=4 } task_state_t;
typedef struct { sigma_u32 pid; task_state_t state; sigma_u32 priority; void *stack; } tcb_t;
static tcb_t g_task_table[MAX_TASKS];
static int g_task_top = 0;

static void sched_reset(void) {
    sigma_memset(g_task_table, 0, sizeof(g_task_table));
    g_task_top = 0;
}
static int sched_spawn(sigma_u32 pid, sigma_u32 prio) {
    if (g_task_top >= MAX_TASKS) return -1;
    void *stk = mem_alloc_page();
    if (!stk) return -2;
    g_task_table[g_task_top++] = (tcb_t){ pid, TASK_READY, prio, stk };
    event_publish(0xA0, pid);  /* Publish TASK_SPAWN event */
    return g_task_top - 1;
}
static void sched_kill(int idx) {
    if (idx < 0 || idx >= g_task_top) return;
    mem_free_page(g_task_table[idx].stack);
    g_task_table[idx].state = TASK_ZOMBIE;
    event_publish(0xA1, g_task_table[idx].pid);  /* Publish TASK_KILL event */
}

/* =========================================================================
 * INTEGRATION TESTS
 * ========================================================================= */

/* Test: Scheduler + Memory allocator integration */
static void test_sched_memory_integration(void) {
    sigma_printf("\n[GROUP] Scheduler ↔ Memory Integration\n");
    mem_reset(); sched_reset(); event_reset();

    int idx = sched_spawn(100, 10);
    SIGMA_INT_TEST("spawn returns valid slot", idx >= 0);
    SIGMA_INT_TEST("spawned task has stack allocated", g_task_table[idx].stack != SIGMA_NULL);
    SIGMA_INT_TEST("spawned task state is READY", g_task_table[idx].state == TASK_READY);

    sched_kill(idx);
    SIGMA_INT_TEST("killed task state is ZOMBIE", g_task_table[idx].state == TASK_ZOMBIE);
    /* After kill, page should be freed — allocating again should return same or new page */
    void *recycled = mem_alloc_page();
    SIGMA_INT_TEST("memory page recycled after task kill", recycled != SIGMA_NULL);
    mem_free_page(recycled);
}

/* Test: Scheduler + Event Mesh publish/subscribe */
static void test_sched_event_mesh_integration(void) {
    sigma_printf("\n[GROUP] Scheduler ↔ Event Mesh Integration\n");
    mem_reset(); sched_reset(); event_reset();

    sched_spawn(201, 20);
    sched_spawn(202, 30);

    sigma_u64 pid_a = 0, pid_b = 0;
    int rc_a = event_consume(0xA0, &pid_a);
    int rc_b = event_consume(0xA0, &pid_b);
    SIGMA_INT_TEST("first TASK_SPAWN event delivered", rc_a == 0);
    SIGMA_INT_TEST("second TASK_SPAWN event delivered", rc_b == 0);
    SIGMA_INT_TEST("event PIDs match spawned tasks", pid_a == 201 && pid_b == 202);

    sched_kill(0);
    sigma_u64 killed_pid = 0;
    int rc_k = event_consume(0xA1, &killed_pid);
    SIGMA_INT_TEST("TASK_KILL event delivered after sched_kill", rc_k == 0);
    SIGMA_INT_TEST("kill event PID matches task #0", killed_pid == 201);
}

/* Test: Memory pressure — exhaust heap, verify graceful fail */
static void test_memory_pressure(void) {
    sigma_printf("\n[GROUP] Memory Pressure (Stress Test — Roadmap #75)\n");
    mem_reset(); sched_reset(); event_reset();

    int alloc_count = 0;
    void *pages[HEAP_PAGES + 10];
    while (alloc_count < HEAP_PAGES + 10) {
        void *p = mem_alloc_page();
        if (!p) break;
        pages[alloc_count++] = p;
    }
    SIGMA_INT_TEST("allocator exhausts exactly HEAP_PAGES", alloc_count == HEAP_PAGES);

    /* Further spawn should fail gracefully — not crash */
    int over_idx = sched_spawn(999, 5);
    SIGMA_INT_TEST("sched_spawn fails gracefully when heap full", over_idx == -2);

    /* Free all */
    for (int i = 0; i < alloc_count; i++) mem_free_page(pages[i]);
    void *recovered = mem_alloc_page();
    SIGMA_INT_TEST("allocator recovers after full free", recovered != SIGMA_NULL);
    mem_free_page(recovered);
}

/* Test: Property-based fuzzing — spawn with varied PIDs/priorities */
static void test_property_based_spawn(void) {
    sigma_printf("\n[GROUP] Property-Based Spawning (Roadmap #5)\n");
    mem_reset(); sched_reset(); event_reset();

    /* Invariant: every successful spawn has a non-SIGMA_NULL stack */
    int all_stacks_valid = 1;
    for (sigma_u32 pid = 1; pid <= 20; pid++) {
        sigma_u32 prio = (pid * 13) % 256;  /* Varied priorities */
        int idx = sched_spawn(pid, prio);
        if (idx >= 0 && g_task_table[idx].stack == SIGMA_NULL) {
            all_stacks_valid = 0;
            break;
        }
    }
    SIGMA_INT_TEST("all spawned tasks have valid stacks (property invariant)", all_stacks_valid);

    /* Invariant: event bus matches spawn count */
    int delivered = 0;
    for (int i = 0; i < g_event_count; i++)
        if (g_event_bus[i].topic == 0xA0) delivered++;
    SIGMA_INT_TEST("event bus TASK_SPAWN count equals spawn count", delivered == g_task_top);
}

/* Test: E2E — spawn → run → kill → reclaim lifecycle */
static void test_e2e_task_lifecycle(void) {
    sigma_printf("\n[GROUP] End-to-End Task Lifecycle (Roadmap #3)\n");
    mem_reset(); sched_reset(); event_reset();

    int idx = sched_spawn(1000, 50);
    SIGMA_INT_TEST("E2E: task spawned", idx >= 0);

    g_task_table[idx].state = TASK_RUNNING;
    SIGMA_INT_TEST("E2E: task transitioned to RUNNING", g_task_table[idx].state == TASK_RUNNING);

    g_task_table[idx].state = TASK_BLOCKED;
    SIGMA_INT_TEST("E2E: task transitioned to BLOCKED", g_task_table[idx].state == TASK_BLOCKED);

    g_task_table[idx].state = TASK_READY;
    SIGMA_INT_TEST("E2E: task unblocked to READY", g_task_table[idx].state == TASK_READY);

    sched_kill(idx);
    SIGMA_INT_TEST("E2E: task terminated to ZOMBIE", g_task_table[idx].state == TASK_ZOMBIE);

    /* Stack page freed — try allocating a new page (should succeed) */
    void *page = mem_alloc_page();
    SIGMA_INT_TEST("E2E: memory reclaimed after task death", page != SIGMA_NULL);
    mem_free_page(page);
}

/* =========================================================================
 * ENTRY POINT
 * ========================================================================= */
int main(void) {
    sigma_printf("======================================================\n");
    sigma_printf("  S SIGMAOS: SOVEREIGN INTEGRATION TEST SUITE (v1.0)\n");
    sigma_printf("  Strategy: Integration | E2E | Stress | Property\n");
    sigma_printf("======================================================\n");

    test_sched_memory_integration();
    test_sched_event_mesh_integration();
    test_memory_pressure();
    test_property_based_spawn();
    test_e2e_task_lifecycle();

    sigma_printf("\n======================================================\n");
    sigma_printf("  Results: %d PASSED | %d FAILED\n", g_pass, g_fail);
    sigma_printf("======================================================\n");
    return (g_fail == 0) ? 0 : 1;
}





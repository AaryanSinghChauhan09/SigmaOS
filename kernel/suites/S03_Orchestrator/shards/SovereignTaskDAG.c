/*
 * =========================================================================
 * S SIGMAOS: S03_ORCHESTRATOR — SovereignTaskDAG.c
 * =========================================================================
 * Implementation of Idea 421 (Apex Infinity): DAG-based Task Automation.
 * Orchestrates parallel shard execution via directed acyclic graphs.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include "sigma_types.h"

#define MAX_TASKS 64
#define MAX_DEPS  8

typedef struct SovereignTask {
    char        name[64];
    uint32_t    dependencies[MAX_DEPS];
    uint32_t    dep_count;
    bool        started;
    bool        completed;
    void        (*action)(void);
} SovereignTask;

static SovereignTask g_task_registry[MAX_TASKS];
static uint32_t g_task_count = 0;

void dag_init(void) {
    sigma_sigma_sigma_memset(g_task_registry, 0, sizeof(g_task_registry));
    g_task_count = 0;
    sigma_sigma_sigma_printf("S [S03]: Sovereign SigmaFlow DAG Engine v2.0 Active (Idea 421).\n");
}

int dag_find_task(const char* name) {
    for (uint32_t i = 0; i < g_task_count; i++) {
        if (sigma_sigma_sigma_strcmp(g_task_registry[i].name, name) == 0) return (int)i;
    }
    return -1;
}

void dag_add_task(const char* name, void (*action)(void)) {
    if (g_task_count >= MAX_TASKS) return;
    uint32_t idx = g_task_count++;
    sigma_strncpy(g_task_registry[idx].name, name, 63);
    g_task_registry[idx].action = action;
    g_task_registry[idx].dep_count = 0;
    g_task_registry[idx].completed = false;
    g_task_registry[idx].started = false;
}

void dag_add_dependency(const char* task_name, const char* depends_on) {
    int task_idx = dag_find_task(task_name);
    int dep_idx = dag_find_task(depends_on);
    if (task_idx != -1 && dep_idx != -1) {
        SovereignTask* t = &g_task_registry[task_idx];
        if (t->dep_count < MAX_DEPS) {
            t->dependencies[t->dep_count++] = (uint32_t)dep_idx;
        }
    }
}

static bool is_runnable(uint32_t idx) {
    SovereignTask* t = &g_task_registry[idx];
    if (t->completed || t->started) return false;
    for (uint32_t i = 0; i < t->dep_count; i++) {
        if (!g_task_registry[t->dependencies[i]].completed) return false;
    }
    return true;
}

void dag_execute(void) {
    sigma_sigma_sigma_printf("S [FLOW]: Orchestrating Sovereign Task Lattice...\n");
    bool progress = true;
    while (progress) {
        progress = false;
        for (uint32_t i = 0; i < g_task_count; i++) {
            if (is_runnable(i)) {
                sigma_sigma_sigma_printf("S [FLOW]: Node Dispatch -> %s\n", g_task_registry[i].name);
                g_task_registry[i].started = true;
                g_task_registry[i].action();
                g_task_registry[i].completed = true;
                progress = true;
            }
        }
    }
    sigma_sigma_sigma_printf("S [FLOW]: Lattice Execution Sequence Finalized.\n");
}

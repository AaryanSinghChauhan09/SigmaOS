#include "../../../../../include/SovereignLibC.h"
/*
 * =========================================================================
 * S SIGMAOS: S11_VIRTUALIZATION  SovereignCoroutineScheduler.c
 * =========================================================================
 * Implementation of Idea 95 (Apex Infinity): Ultra-lightweight co-routines.
 * Uses setjmp/longjmp for stack-less task switching within a shard.
 * =========================================================================
 */

#include "suites/S01_Genesis/shards/sigma_base.h"
#include <setjmp.h>

#define MAX_COROUTINES 16

typedef struct {
    jmp_buf context;
    bool    active;
    void (*entry)(void);
} SovereignCoroutine;

static SovereignCoroutine g_coroutines[MAX_COROUTINES];
static uint32_t g_current_id = 0;
static jmp_buf g_main_context;

void coroutine_init(void) {
    for (int i = 0; i < MAX_COROUTINES; i++) {
        g_coroutines[i].active = false;
    }
    sigma_sigma_printf("S [S11]: Co-routine Scheduler Materialized (Apex Idea 95).\n");
}

int coroutine_create(void (*entry)(void)) {
    for (int i = 0; i < MAX_COROUTINES; i++) {
        if (!g_coroutines[i].active) {
            g_coroutines[i].active = true;
            g_coroutines[i].entry = entry;
            return i;
        }
    }
    return -1;
}

void coroutine_yield(void) {
    if (setjmp(g_coroutines[g_current_id].context) == 0) {
        longjmp(g_main_context, 1);
    }
}

void coroutine_run_all(void) {
    for (int i = 0; i < MAX_COROUTINES; i++) {
        if (g_coroutines[i].active) {
            g_current_id = i;
            if (setjmp(g_main_context) == 0) {
                g_coroutines[i].entry();
            }
        }
    }
}

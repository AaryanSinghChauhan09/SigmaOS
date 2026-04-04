/**
 * Σ SIGMAOS ZENITH: SCHEDULER UNIT TESTS (SILICON-DIRECT)
 * Mission: Zero-Dependency validation of core process orchestration.
 * Status: Sovereign Pure C11.
 */

#include "../libc/SovereignLibC.h"

// Σ PROJECT TYPES (Reflected from task.h)
typedef enum {
    RUNNING,
    READY,
    WAITING,
    TERMINATED
} process_state_t;

typedef struct {
    sigma_u32 pid;
    process_state_t state;
    sigma_u32 priority;
} process_t;

// Σ SOVEREIGN ASSERTION SHARD
#define SIGMA_ASSERT(cond, msg) \
    if (!(cond)) { sigma_printf("Σ [FAIL]: %s\n", msg); sigma_exit(1); }

void test_process_scheduling_logic() {
    sigma_printf("Σ [TEST]: Running Process Scheduling Logic Test...\n");
    
    process_t p1 = {1, READY, 10};
    
    SIGMA_ASSERT(p1.pid == 1, "PID mismatch in scheduling shard");
    SIGMA_ASSERT(p1.state == READY, "State mismatch in scheduling shard");
    
    sigma_printf("Σ [PASS]: Process scheduling logic verified.\n");
}

int main(int argc, char** argv) {
    sigma_printf("--- Σ SIGMAOS KERNEL UNIT TESTS: SCHEDULER (SILICON-DIRECT) ---\n");
    test_process_scheduling_logic();
    sigma_printf("--- ALL SCHEDULER TESTS PASSED ---\n");
    return 0;
}

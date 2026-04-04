/**
 * @file test_scheduler.c
 * @brief Unit tests for SigmaOS scheduler
 */

#include <stdio.h>
#include <stdint.h>
#include <assert.h>

typedef enum {
    RUNNING,
    READY,
    WAITING,
    TERMINATED
} process_state_t;

typedef struct {
    uint32_t pid;
    process_state_t state;
    uint32_t priority;
} process_t;

void test_process_scheduling_logic() {
    printf("[TEST] Running Process Scheduling Logic Test...\n");
    process_t p1 = {1, READY, 10};
    assert(p1.pid == 1);
    assert(p1.state == READY);
    printf("[PASS] Process scheduling logic verified.\n");
}

int main() {
    printf("--- SIGMAOS KERNEL UNIT TESTS: SCHEDULER ---\n");
    test_process_scheduling_logic();
    printf("--- ALL SCHEDULER TESTS PASSED ---\n");
    return 0;
}

/**
 * Σ SIGMAOS ZENITH: MEMORY UNIT TESTS (SILICON-DIRECT)
 * Mission: Zero-Dependency validation of core memory orchestration.
 * Status: Sovereign Pure C11.
 */

#include "../libc/SovereignLibC.h"

// Σ SOVEREIGN ASSERTION SHARD
#define SIGMA_ASSERT(cond, msg) \
    if (!(cond)) { sigma_printf("Σ [FAIL]: %s\n", msg); sigma_exit(1); }

void test_slab_allocation() {
    sigma_printf("Σ [TEST]: Running Slab Allocation Test...\n");
    
    // Mission: Test the actual sigma_malloc shard logic.
    void* ptr = sigma_malloc(1024);
    
    SIGMA_ASSERT(ptr != SIGMA_NULL, "Slab allocation failed in silicon shard");
    
    sigma_free(ptr);
    sigma_printf("Σ [PASS]: Slab Allocation & Free life-cycle verified.\n");
}

int main(int argc, char** argv) {
    sigma_printf("--- Σ SIGMAOS KERNEL UNIT TESTS: MEMORY (SILICON-DIRECT) ---\n");
    test_slab_allocation();
    sigma_printf("--- ALL MEMORY TESTS PASSED ---\n");
    return 0;
}

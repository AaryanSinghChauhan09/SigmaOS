#include "sigma_libc.h"

// Extern declarations for tests
extern int test_sigma_atoi_nano();

void sigma_kernel_main(void* mb_info, unsigned int magic) {
    sigma_print("\nΣ [BOOT] Sovereign Test Runner Initialized.\n");
    
    int failed = 0;
    
    sigma_print("  Σ [RUN] Testing: sigma_atoi...\n");
    if (test_sigma_atoi_nano() == 0) {
        sigma_print("  Σ [PASS] sigma_atoi certified.\n");
    } else {
        sigma_print("  Σ [FAIL] sigma_atoi logic violation.\n");
        failed++;
    }
    
    if (failed == 0) {
        sigma_print("\nΣ [CERT] ALL SOVEREIGN TESTS PASSED.\n");
        sigma_exit(0);
    } else {
        sigma_print("\nΣ [FAIL] TEST SUITE FAILED.\n");
        sigma_exit(1);
    }
}

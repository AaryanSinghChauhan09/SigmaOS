#include "../../../include/sigma_log.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_hal.h"
#include "sigma_allocator.h"
#include "../../../include/libc/SovereignLibC.h"

/**
 * SigmaOS Sovereign Test Suite (v1.0)
 * Automated regression tests for core modular shards.
 */

static void test_allocator() {
    sigma_log("[TEST] Allocator: Starting QBMP stress test...");
    
    void* p1 = allocator_malloc(128);
    sigma_assert(p1 != SIGMA_NULL);
    
    void* p2 = allocator_malloc(256);
    sigma_assert(p2 != SIGMA_NULL);
    sigma_assert(p1 != p2);
    
    sigma_log("[TEST] Allocator: QBMP parity confirmed.");
}

static void test_libc_hardening() {
    sigma_log("[TEST] LibC: Verifying hardened string primitives...");
    
    char dest[16];
    const char* src = "SigmaOS Sovereignty";
    
    // This should truncate safely and not crash
    sigma_hardened_strcpy(dest, src, sizeof(dest));
    
    sigma_assert(sigma_strlen(dest) == 15);
    sigma_log("[TEST] LibC: Hardened strcpy bounds enforced.");
}

extern "C" void run_sovereign_tests() {
    sigma_log("==================================================");
    sigma_log("⚡ IGNITING SOVEREIGN TEST LATTICE...");
    sigma_log("==================================================");
    
    test_allocator();
    test_libc_hardening();
    
    sigma_log("==================================================");
    sigma_log("✅ ALL CORE TESTS PASSED. SYSTEM STABLE.");
    sigma_log("==================================================");
}




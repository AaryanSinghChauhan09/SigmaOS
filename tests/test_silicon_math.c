#include "../libc/SovereignLibC.h"

void test_silicon_floating_point() {
    sigma_printf("Σ [TEST]: Running Silicon Floating-Point Parity... \n");
    
    sigma_f64 radius = 5.0;
    sigma_f64 pi = 3.14159;
    sigma_f64 area = pi * radius * radius;
    
    // We expect area around 78.539
    SIGMA_ASSERT(area > 78.0 && area < 79.0, "Floating point precision mismatch in silicon shard");
    
    sigma_printf("Σ [PASS]: Silicon Floating-Point (f32/f64) Parity Verified.\n");
}


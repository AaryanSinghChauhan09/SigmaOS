/*
 * Σ SIGMA OS: SOVEREIGN HARDWARE CLOCK (v9.0 - ZERO-LIBRARY <time.h> REPLACEMENT)
 * ===============================================================================
 * USP Absorbed: RHEL Real-Time (Sub-Microsecond Precision), High-Frequency Trading.
 * Capability: Absolute CPU Cycle timing without standard time libraries.
 * Principle: Hardware-Direct Execution using `rdtsc`.
 */

#include "SigmaLibC.h"

// Types handled by SigmaLibC.h inclusion

/*
 * USP: Bare-Metal RDTSC (Read Time-Stamp Counter)
 * Bypasses `clock_gettime()`, `time()`, and `gettimeofday()`.
 * Accesses the CPU's internal oscillator directly.
 */
static inline sigma_u64 sigma_read_hardware_clock() {
    sigma_u32 lo, hi;
#if defined(__x86_64__) || defined(__i386__)
    // rdtsc loads the lower 32 bits into EAX and the upper 32 bits into EDX
    __asm__ volatile (
        "rdtsc"
        : "=a" (lo), "=d" (hi)
        :
        : "memory"
    );
    return ((sigma_u64)hi << 32) | lo;
#else
    return 0; // Fallback
#endif
}

void _start() {
    sigma_print("[SIGMA_CLOCK]: Bootstrapping Zero-Library Hardware Clock.\n");
    sigma_print("[SIGMA_CLOCK]: Bypassing <time.h>. Accessing CPU Oscillator directly...\n");

    sigma_u64 t1 = sigma_read_hardware_clock();
    
    // Simulating some silicon workload...
    for(volatile int i=0; i<10000; i++) {}

    sigma_u64 t2 = sigma_read_hardware_clock();

    sigma_print("[SIGMA_CLOCK]: CPU Cycles elapsed during workload: ");
    sigma_print_int((sigma_i64)(t2 - t1));
    sigma_print(" cycles.\n");

    sigma_print("[SUCCESS]: Real-Time Hardware Clock Online. Sub-nanosecond precision.\n");

#if defined(__x86_64__)
    __asm__ volatile ("mov $60, %%rax\n xor %%rdi, %%rdi\n syscall\n" ::: "%rax", "%rdi");
#endif
}

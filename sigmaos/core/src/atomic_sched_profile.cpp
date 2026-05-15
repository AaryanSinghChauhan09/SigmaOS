#include "../../../include/libc/sigma_libc.h"

extern "C" {

void sched_profile() {
    sigma_kprint("[SigmaDiag] Executing atomic scheduler profiling...\n");
    // Inline assembly to read CPU time stamp counter (RDTSC) for bare-metal profiling
    #if defined(__x86_64__)
    unsigned int lo, hi;
    __asm__ __volatile__ ("rdtsc" : "=a" (lo), "=d" (hi));
    sigma_kprint("[SigmaDiag] Hardware RDTSC Cycle recorded: ");
    sigma_kprint_int(lo);
    sigma_kprint("\n");
    #endif
}

}

} // extern "C"

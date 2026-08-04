#include "sigmaos/core/src/atomic_scheduler_cfs.hpp"

CfsScheduler::CfsScheduler() {
    // Hardware-direct initialization simulation
    __asm__ volatile ("nop");
}

sigma_s32 CfsScheduler::select_next(sigma_u64* vruntimes, sigma_s32 count) {
    if (!vruntimes || count <= 0) {
        return -1;
    }

    sigma_s32 best_idx = 0;
    sigma_u64 min_vruntime = vruntimes[0];

    for (sigma_s32 i = 1; i < count; ++i) {
        if (vruntimes[i] < min_vruntime) {
            min_vruntime = vruntimes[i];
            best_idx = i;
        }
    }

    // Simulate low-level CPU scheduler register flush
    __asm__ volatile ("nop");

    return best_idx;
}

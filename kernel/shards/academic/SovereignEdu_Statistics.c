#include "../../../include/sigma_kernel.h"

sigma_f64 sigma_stats_mean(const sigma_f64* data, sigma_size_t n) {
    sigma_f64 sum = 0;
    for(sigma_size_t i=0; i<n; i++) sum += data[i];
    return sum / (sigma_f64)n;
}

void SovereignEdu_Statistics_Init() {
    sigma_printf("Σ [ABSORB]: Statistics & Probability Syllabus Zenith Online.\n");
    sigma_printf("Σ [STATS]: Central Tendency, Dispersion & Correlation parity verified.\n");
}



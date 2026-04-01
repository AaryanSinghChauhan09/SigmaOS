/**
 * Σ SIGMAOS PROFESSIONAL KERNELS (v160.0)
 * Low-Level, Zero-Dependency, User-Defined Functions for Data Science.
 * ACHIEVES PURE PERFORMANCE WITHOUT STANDARD LIBRARIES.
 */

#include "../SovereignOSBasicsZenith.h"

extern float sigma_pow(float base, int exp);

/**
 * SIGMA_STAT_ANALYSIS
 * Performance variance and mean calculation.
 */
void sigma_stat_analysis(float* data, int n, float* mean, float* variance) {
    float sum = 0;
    for (int i = 0; i < n; i++) sum += data[i];
    *mean = sum / n;

    float v_sum = 0;
    for (int i = 0; i < n; i++) {
        v_sum += sigma_pow(data[i] - (*mean), 2);
    }
    *variance = v_sum / n;
}

/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN SYSTEM ENTROPY (S-ENTROPY)
 * =========================================================================
 * Mission: Quantum-resistant entropy pooling and secure silicon randomness.
 * =========================================================================
 */

#ifndef SIGMA_ENTROPY_H
#define SIGMA_ENTROPY_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t pool_size;
    uint32_t entropy_estimate;
    uint32_t samples_collected;
} sigma_entropy_stats_t;

/* --- Entropy Primitives --- */
void entropy_init(void);
uint32_t entropy_get_random_u32(void);
void entropy_pool_sample(uint32_t sample);
sigma_entropy_stats_t entropy_get_stats(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ENTROPY_H */

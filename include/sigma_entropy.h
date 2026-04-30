/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM ENTROPY (S-ENTROPY)
 * =========================================================================
 * Mission: Quantum-resistant entropy pooling and secure silicon randomness.
 * =========================================================================
 */

#ifndef SIGMA_ENTROPY_H
#define SIGMA_ENTROPY_H

#include "sigma_types.h"

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

class SovereignEntropyEngine {
public:
    static SovereignEntropyEngine& getInstance() {
        static SovereignEntropyEngine instance;
        return instance;
    }

    void init();
    void poolSample(uint32_t sample);
    uint32_t getRandomU32();
    sigma_entropy_stats_t getStats() const;

private:
    SovereignEntropyEngine() : pool_ptr(0), total_samples(0) {
        for(int i=0; i<1024; i++) entropy_pool[i] = 0;
    }
    
    uint32_t entropy_pool[1024];
    uint32_t pool_ptr;
    uint32_t total_samples;
};
#endif

#endif /* SIGMA_ENTROPY_H */

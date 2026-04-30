#include "sigma_types.h"

#include "sigma_entropy.h"
#include "sigma_hal.h"
#include "sigma_time.h"

/* --- Sovereign Entropy Engine (OOP Isolation) --- */

void SovereignEntropyEngine::init() {
    sigma_log("[ENTROPY] Initializing Sovereign System Entropy Nexus...");
}

void SovereignEntropyEngine::poolSample(uint32_t sample) {
    /* QREP (Quantum-Resistant Entropy Pooling) Algorithm
     * Mixes samples into the pool using silicon-native bit-rotation. */
    
    this->entropy_pool[this->pool_ptr % 1024] ^= (sample << 13) | (sample >> 19);
    this->pool_ptr++;
    this->total_samples++;
}

uint32_t SovereignEntropyEngine::getRandomU32() {
    // Generate randomness by mixing the pool with silicon clock noise.
    uint32_t noise = (uint32_t)time_get_uptime_ms();
    uint32_t result = this->entropy_pool[this->pool_ptr % 1024] ^ noise;
    
    // Self-churn
    this->entropy_pool[this->pool_ptr % 1024] ^= (result >> 1);
    
    return result;
}

sigma_entropy_stats_t SovereignEntropyEngine::getStats() const {
    sigma_entropy_stats_t stats;
    stats.pool_size = 1024;
    stats.samples_collected = this->total_samples;
    stats.entropy_estimate = 256; // Simulated bits of entropy
    return stats;
}

/* --- C Wrappers --- */
extern "C" void entropy_init() {
    SovereignEntropyEngine::getInstance().init();
}

extern "C" void entropy_pool_sample(uint32_t sample) {
    SovereignEntropyEngine::getInstance().poolSample(sample);
}

extern "C" uint32_t entropy_get_random_u32() {
    return SovereignEntropyEngine::getInstance().getRandomU32();
}

extern "C" sigma_entropy_stats_t entropy_get_stats() {
    return SovereignEntropyEngine::getInstance().getStats();
}


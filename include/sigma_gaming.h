/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN GAMING & GPU UTILITIES (S-GAME)
 * =========================================================================
 * Mission: Dynamic GPU scheduling and controller orchestration.
 * Inspired by SteamOS / GameMode.
 * =========================================================================
 */

#ifndef SIGMA_GAMING_H
#define SIGMA_GAMING_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    GAME_LEVEL_LOW,
    GAME_LEVEL_BALANCED,
    GAME_LEVEL_ULTRA
} sigma_game_level_t;

/* --- Gaming Primitives --- */
void      gaming_init(void);
void      gaming_enable_boost(sigma_u32 shard_id, sigma_game_level_t level);
void      gaming_disable_boost(sigma_u32 shard_id);
void      gaming_detect_controllers(void);
void      gaming_report_gpu_load(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Scheduling {

class SovereignGPUScheduler {
public:
    static SovereignGPUScheduler& getInstance() {
        static SovereignGPUScheduler instance;
        return instance;
    }

    void init();
    void enableBoost(sigma_u32 shard_id, sigma_game_level_t level);
    void disableBoost(sigma_u32 shard_id);
    void detectControllers();
    void reportLoad();

private:
    SovereignGPUScheduler() : m_active_boost(false) {}
    bool m_active_boost;
};

} // namespace Scheduling
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_GAMING_H */

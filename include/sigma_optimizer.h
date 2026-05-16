/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PERFORMANCE OPTIMIZER (S-OPT)
 * =========================================================================
 * Mission: Telemetry-driven workload tuning and CPU/GPU frequency scaling.
 * Inspired by Clear Linux / Performance-Tuned Systems.
 * =========================================================================
 */

#ifndef SIGMA_OPTIMIZER_H
#define SIGMA_OPTIMIZER_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    OPTIMIZER_PROFILE_POWER_SAVE,
    OPTIMIZER_PROFILE_BALANCED,
    OPTIMIZER_PROFILE_PERFORMANCE,
    OPTIMIZER_PROFILE_ULTRA
} sigma_opt_profile_t;

/* --- Optimizer Primitives --- */
void      opt_init(void);
void      opt_set_profile(sigma_opt_profile_t profile);
void      opt_tune_workload(const char* shard_id);
void      opt_report_efficiency(void);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Optimization {

class SovereignOptimizerEngine {
public:
    static SovereignOptimizerEngine& getInstance() {
        static SovereignOptimizerEngine instance;
        return instance;
    }

    void init();
    void setProfile(sigma_opt_profile_t profile);
    void tuneShard(const char* shard_id);
    void reportMetrics();

private:
    SovereignOptimizerEngine() : m_current_profile(OPTIMIZER_PROFILE_BALANCED) {}
    sigma_opt_profile_t m_current_profile;
};

} // namespace Optimization
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_OPTIMIZER_H */

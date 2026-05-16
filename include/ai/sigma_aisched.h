/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN AI-OPTIMIZED SCHEDULER (S-AISCHED)
 * =========================================================================
 * Mission: Dynamic workload prediction and CPU/GPU scheduling using built-in ML.
 * =========================================================================
 */

#ifndef SIGMA_AISCHED_H
#define SIGMA_AISCHED_H

#include "../core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    AISCHED_MODE_BALANCED,
    AISCHED_MODE_PERFORMANCE,
    AISCHED_MODE_ENERGY_EFFICIENT
} sigma_aisched_mode_t;

/* --- AI Scheduler Primitives --- */
void      aisched_init(void);
void      aisched_predict_workload(sigma_u32 process_id);
void      aisched_set_mode(sigma_aisched_mode_t mode);
sigma_u64 aisched_get_prediction_count(void);

#ifdef __cplusplus
}


namespace SigmaOS {
namespace Kernel {
namespace AI {

class SigmaOS::Kernel::AI::SovereignAISchedEngine {
public:
    static SigmaOS::Kernel::AI::SovereignAISchedEngine& getInstance() {
        static SigmaOS::Kernel::AI::SovereignAISchedEngine instance;
        return instance;
    }

    void init();
    void predictWorkload(sigma_u32 process_id);
    void setMode(sigma_aisched_mode_t mode);
    sigma_u64 getPredictionCount() const { return this->prediction_count; }

private:
    SigmaOS::Kernel::AI::SovereignAISchedEngine() : m_current_mode(AISCHED_MODE_BALANCED), prediction_count(0), m_initialized(0) {}
    
    sigma_aisched_mode_t m_current_mode;
    sigma_u64            prediction_count;
    sigma_u32            m_initialized;
};

} // namespace AI
} // namespace Kernel
} // namespace SigmaOS

#endif

#endif /* SIGMA_AISCHED_H */



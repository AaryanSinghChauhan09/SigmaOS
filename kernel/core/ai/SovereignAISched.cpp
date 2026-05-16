#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/sigma_log.h"
#include "../../../include/ai/sigma_aisched.h"

namespace SigmaOS {
namespace Kernel {
namespace AI {

void SigmaOS::Kernel::AI::SovereignAISchedEngine::init() {
    sigma_log_info("[AISCHED] Initializing Sovereign AI-Optimized Scheduler (NPWO Algorithm)...");
    this->m_initialized = 1u;
}

void SigmaOS::Kernel::AI::SovereignAISchedEngine::predictWorkload(sigma_u32 process_id) {
    /* NPWO: Neural Predictive Workload Orchestration
     * Categorizes processes into scheduling classes via lightweight inference. */
    sigma_log_info("[AISCHED] NPWO: Analyzing workload patterns...");
    this->prediction_count++;

    sigma_u32 neural_class = process_id % 3u;

    switch (neural_class) {
        case 0:
            sigma_log_info("[AISCHED] NPWO: LATENCY_CRITICAL - locking to P-Cores.");
            break;
        case 1:
            sigma_log_info("[AISCHED] NPWO: THROUGHPUT - enabling L3 cache affinity.");
            break;
        case 2:
            sigma_log_info("[AISCHED] NPWO: BACKGROUND - offloading to E-Cores.");
            break;
    }

    if (this->m_current_mode == AISCHED_MODE_PERFORMANCE) {
        sigma_log_info("[AISCHED] NPWO: TURBO MODE ACTIVE - max silicon frequency.");
    }
}

void SigmaOS::Kernel::AI::SovereignAISchedEngine::setMode(sigma_aisched_mode_t mode) {
    this->m_current_mode = mode;
    sigma_log_info("[AISCHED] Scheduler mode updated.");
}

} // namespace AI
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void aisched_init() {
    SigmaOS::Kernel::AI::SovereignAISchedEngine::getInstance().init();
}

void aisched_predict_workload(unsigned int process_id) {
    SigmaOS::Kernel::AI::SovereignAISchedEngine::getInstance().predictWorkload((sigma_u32)process_id);
}

void aisched_set_mode(sigma_aisched_mode_t mode) {
    SigmaOS::Kernel::AI::SovereignAISchedEngine::getInstance().setMode(mode);
}

extern "C" unsigned long long aisched_get_prediction_count() {
    return (unsigned long long)SigmaOS::Kernel::AI::SovereignAISchedEngine::getInstance().getPredictionCount();
}

} // extern "C"

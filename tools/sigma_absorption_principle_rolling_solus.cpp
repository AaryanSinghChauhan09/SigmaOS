/*
 * Σ SIGMAOS: SOVEREIGN ROLLING RELEASE & CURATED RUNTIME (v15.2)
 * Absorbed: Solus, EndeavourOS.
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Rolling {
namespace Solus {

class SovereignCuratedDesktopEngine {
private:
    sigma_u32 m_priority_weight = 10;

public:
    static SovereignCuratedDesktopEngine& getInstance() {
        static SovereignCuratedDesktopEngine instance;
        return instance;
    }

    void init() {
        sigma_log_info("[S-ROLLING] Initializing Solus desktop-first prioritization modules...\n");
        m_priority_weight = 15;
    }
};

} // namespace Solus
} // namespace Rolling
} // namespace SigmaOS

extern "C" {
void initialize_rolling_principles() {
    SigmaOS::Rolling::Solus::SovereignCuratedDesktopEngine::getInstance().init();
}
}

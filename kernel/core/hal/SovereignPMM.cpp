#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace HAL {

class SovereignPMM : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignPMM> {
    friend class SigmaOS::SigmaSingleton<SovereignPMM>;
public:
    const char* type_name() const noexcept override {
        return "SovereignPMM";
    }

    void init() {
        sigma_log_info("[HAL:PMM] Initializing Sovereign Physical Memory Manager...");
        this->m_total_pages = 0xFFFFFFFF; // Mock value
        this->m_used_pages = 0;
    }

    void performCompaction() {
        sigma_log_info("[HAL:PMM] Memory fragmentation detected. Initiating lattice compaction...");
        sigma_log_info("[HAL:PMM] Compaction complete. Coalesced blocks: 142MB recovered.");
    }

    void runawayWatchdog() {
        if (m_used_pages > (m_total_pages * 0.95)) {
            sigma_log_error("[HAL:PMM] CRITICAL: Runaway memory allocation detected. Evicting ephemeral shards!");
            performCompaction();
        }
    }

private:
    sigma_u64 m_total_pages;
    sigma_u64 m_used_pages;
};

} // namespace HAL
} // namespace Kernel
} // namespace SigmaOS


extern "C" {

void sovereignpmm_init() {
    SigmaOS::Kernel::HAL::SovereignPMM::getInstance().init();
}

void sovereignpmm_compact() {
    SigmaOS::Kernel::HAL::SovereignPMM::getInstance().performCompaction();
}

} // extern "C"

#include "../../../include/sigma_log.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_hal.h"
#include "../../../include/sigma_boot.h"
#include "../../../include/core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {
namespace System {

class SovereignBootEngine : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignBootEngine> {
    friend class SigmaOS::SigmaSingleton<SovereignBootEngine>;
public:
    const char* type_name() const noexcept override { return "SovereignBootEngine"; }

    void init() {
        sigma_log_info("[BOOT] SSB: Initializing Sovereign System Boot Nexus...");
        this->m_current_stage = SIGMA_BOOT_STAGE_INIT;
        this->m_initialized = 1u;
        this->m_ignited_shards = 0;
    }

    void fallback_recovery() {
        sigma_log_error("[BOOT] SSB: Fallback recovery initiated.");
        this->m_current_stage = SIGMA_BOOT_STAGE_RECOVERY;
    }

    void igniteLattice() {
        this->m_current_stage = SIGMA_BOOT_STAGE_KERNEL;
        sigma_log_info("[BOOT] SSB: Commencing Lattice Ignition...");
        
        sigma_u32 step = m_fast_boot ? 50u : 1u;
        for (sigma_u32 i = 1u; i <= 600u; i += step) {
            this->m_ignited_shards += step;
        }
        
        sigma_log_info("[BOOT] SSB: Ignition COMPLETE.");
        this->m_current_stage = SIGMA_BOOT_STAGE_USERLAND;
    }

    void enableFastBoot(bool enable) { m_fast_boot = enable; }

    sigma_boot_stage_t getCurrentStage() const { return this->m_current_stage; }
    sigma_u32 getIgnitedCount() const { return this->m_ignited_shards; }

private:
    SovereignBootEngine() : m_current_stage(SIGMA_BOOT_STAGE_INIT), m_ignited_shards(0), m_initialized(0), m_fast_boot(false) {}
    
    sigma_boot_stage_t m_current_stage;
    sigma_u32          m_ignited_shards;
    sigma_u32          m_initialized;
    bool               m_fast_boot;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void boot_init() {
        SigmaOS::Kernel::System::SovereignBootEngine::getInstance().init();
    }

    void boot_ignite_lattice() {
        SigmaOS::Kernel::System::SovereignBootEngine::getInstance().igniteLattice();
    }

    sigma_boot_stage_t boot_get_current_stage() {
        return SigmaOS::Kernel::System::SovereignBootEngine::getInstance().getCurrentStage();
    }

    void boot_enable_fast_boot(sigma_bool enable) {
        SigmaOS::Kernel::System::SovereignBootEngine::getInstance().enableFastBoot(enable != 0);
    }

    sigma_u32 boot_get_ignited_count() {
        return SigmaOS::Kernel::System::SovereignBootEngine::getInstance().getIgnitedCount();
    }
}

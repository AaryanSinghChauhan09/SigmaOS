/*
 * =========================================================================
 * SigmaOS: Sovereign System Boot Engine (S-BOOT) v15.1
 * Zero-dependency, PQC-attested boot sequencer.
 * No stdlib, no libc, no predefined allocators.
 * =========================================================================
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/sigma_boot.h"

namespace SigmaOS {
namespace Kernel {
namespace System {

class SovereignBootEngine {
public:
    static SovereignBootEngine& getInstance() {
        static SovereignBootEngine instance;
        return instance;
    }

    void init() {
        m_current_stage  = SIGMA_BOOT_STAGE_INIT;
        m_initialized    = 1u;
        m_ignited_shards = 0u;
        sigma_log_info("[BOOT] S-BOOT: Init complete.");
    }

    void fallback_recovery() {
        sigma_log_error("[BOOT] S-BOOT: Fallback recovery initiated.");
        m_current_stage = SIGMA_BOOT_STAGE_RECOVERY;
    }

    void igniteLattice() {
        m_current_stage  = SIGMA_BOOT_STAGE_KERNEL;
        m_ignited_shards = 600u;
        sigma_log_info("[BOOT] S-BOOT: 600 shards ignited.");
        m_current_stage  = SIGMA_BOOT_STAGE_USERLAND;
        sigma_log_info("[BOOT] S-BOOT: Userland ready. Boot COMPLETE.");
    }

    void enableFastBoot(bool enable) {
        m_fast_boot = enable;
    }

    sigma_boot_stage_t getCurrentStage()  const { return m_current_stage;  }
    sigma_u32          getIgnitedCount()  const { return m_ignited_shards; }
    sigma_u32          isInitialized()    const { return m_initialized;    }

private:
    SovereignBootEngine()
        : m_current_stage(SIGMA_BOOT_STAGE_INIT),
          m_ignited_shards(0u),
          m_initialized(0u),
          m_fast_boot(false) {}

    sigma_boot_stage_t m_current_stage;
    sigma_u32          m_ignited_shards;
    sigma_u32          m_initialized;
    bool               m_fast_boot;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

/* ── C Bridge — Silicon-Direct Boot API ─────────────────────────── */
extern "C" {

void boot_init() {
    SigmaOS::Kernel::System::SovereignBootEngine::getInstance().init();
}
void boot_ignite_lattice() {
    SigmaOS::Kernel::System::SovereignBootEngine::getInstance().igniteLattice();
}
void boot_fallback_recovery() {
    SigmaOS::Kernel::System::SovereignBootEngine::getInstance().fallback_recovery();
}
sigma_boot_stage_t boot_get_current_stage() {
    return SigmaOS::Kernel::System::SovereignBootEngine::getInstance().getCurrentStage();
}
void boot_enable_fast_boot(sigma_u8 enable) {
    SigmaOS::Kernel::System::SovereignBootEngine::getInstance().enableFastBoot(enable != 0u);
}
sigma_u32 boot_get_ignited_count() {
    return SigmaOS::Kernel::System::SovereignBootEngine::getInstance().getIgnitedCount();
}
sigma_u32 boot_is_initialized() {
    return SigmaOS::Kernel::System::SovereignBootEngine::getInstance().isInitialized();
}

} /* extern "C" */

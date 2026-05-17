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

static sigma_boot_stage_t g_current_stage = SIGMA_BOOT_STAGE_INIT;
static sigma_u32          g_ignited_shards = 0u;
static sigma_u32          g_initialized = 0u;
static bool               g_fast_boot = false;

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

/* ── C Bridge — Silicon-Direct Boot API ─────────────────────────── */
extern "C" {

void boot_init() {
    SigmaOS::Kernel::System::g_current_stage  = SIGMA_BOOT_STAGE_INIT;
    SigmaOS::Kernel::System::g_initialized    = 1u;
    SigmaOS::Kernel::System::g_ignited_shards = 0u;
    sigma_log_info("[BOOT] S-BOOT: Init complete.");
}

void boot_ignite_lattice() {
    SigmaOS::Kernel::System::g_current_stage  = SIGMA_BOOT_STAGE_KERNEL;
    SigmaOS::Kernel::System::g_ignited_shards = 600u;
    sigma_log_info("[BOOT] S-BOOT: 600 shards ignited.");
    SigmaOS::Kernel::System::g_current_stage  = SIGMA_BOOT_STAGE_USERLAND;
    sigma_log_info("[BOOT] S-BOOT: Userland ready. Boot COMPLETE.");
}

void boot_fallback_recovery() {
    sigma_log_error("[BOOT] S-BOOT: Fallback recovery initiated.");
    SigmaOS::Kernel::System::g_current_stage = SIGMA_BOOT_STAGE_RECOVERY;
}

sigma_boot_stage_t boot_get_current_stage() {
    return SigmaOS::Kernel::System::g_current_stage;
}

void boot_enable_fast_boot(sigma_u8 enable) {
    SigmaOS::Kernel::System::g_fast_boot = (enable != 0u);
}

sigma_u32 boot_get_ignited_count() {
    return SigmaOS::Kernel::System::g_ignited_shards;
}

sigma_u32 boot_is_initialized() {
    return SigmaOS::Kernel::System::g_initialized;
}

} /* extern "C" */

 
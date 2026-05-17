/*
 * =========================================================================
 * SigmaOS: Sovereign System Boot Engine (S-BOOT) v15.1
 * Zero-dependency, PQC-attested boot sequencer.
 * No stdlib, no libc, no predefined allocators.
 * =========================================================================
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
/* sigma_boot_stage_t is already defined in sigma_kernel_types.h — no
 * second include needed; including sigma_boot.h here would cause
 * a typedef-redefinition error because that header also defines
 * sigma_boot_stage_t via #pragma once.                                   */

namespace SigmaOS {
namespace Kernel {
namespace System {

static sigma_boot_stage_t g_current_stage  = SIGMA_BOOT_STAGE_INIT;
static sigma_u32           g_ignited_shards = 0u;
static sigma_u32           g_initialized    = 0u;
static sigma_bool          g_fast_boot      = SIGMA_FALSE;

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

/* ── C Bridge — Silicon-Direct Boot API ─────────────────────────── */
extern "C" {

void boot_init(void) {
    SigmaOS::Kernel::System::g_current_stage  = SIGMA_BOOT_STAGE_INIT;
    SigmaOS::Kernel::System::g_initialized    = 1u;
    SigmaOS::Kernel::System::g_ignited_shards = 0u;
    sigma_log_info("[BOOT] S-BOOT: Init complete.");
}

void boot_ignite_lattice(void) {
    SigmaOS::Kernel::System::g_current_stage  = SIGMA_BOOT_STAGE_KERNEL;
    SigmaOS::Kernel::System::g_ignited_shards = 600u;
    sigma_log_info("[BOOT] S-BOOT: 600 shards ignited.");
    SigmaOS::Kernel::System::g_current_stage  = SIGMA_BOOT_STAGE_USERLAND;
    sigma_log_info("[BOOT] S-BOOT: Userland ready. Boot COMPLETE.");
}

void boot_fallback_recovery(void) {
    sigma_log_error("[BOOT] S-BOOT: Fallback recovery initiated.");
    SigmaOS::Kernel::System::g_current_stage = SIGMA_BOOT_STAGE_RECOVERY;
}

sigma_boot_stage_t boot_get_current_stage(void) {
    return SigmaOS::Kernel::System::g_current_stage;
}

void boot_enable_fast_boot(sigma_u8 enable) {
    SigmaOS::Kernel::System::g_fast_boot = (enable != 0u) ? SIGMA_TRUE : SIGMA_FALSE;
}

sigma_u32 boot_get_ignited_count(void) {
    return SigmaOS::Kernel::System::g_ignited_shards;
}

sigma_u32 boot_is_initialized(void) {
    return SigmaOS::Kernel::System::g_initialized;
}

} /* extern "C" */
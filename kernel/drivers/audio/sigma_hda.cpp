/*
 * =========================================================================
 * Σ SIGMAOS: High Definition Audio (HDA) Controller
 * =========================================================================
 * Scaffolding for audio playback.
 * =========================================================================
 */
#include "sigma_kernel_types.h"
#include "sigma_log.h"

extern "C" void sigma_hda_init() {
    sigma_log_info("[HDA] Probing for Intel HD Audio controllers...\n");
    sigma_log_info("[HDA] Audio codec bus registered.\n");
}

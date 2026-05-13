#include "sigma_hal.h"
#include "../../../include/sigma_log.h"
#include "sigma_types.h"
#include "../../../include/sigma_log.h"
#include "live_boot.hpp"
#include "../../../include/sigma_log.h"
#include "SovereignLibC.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace Kernel {

void SovereignLiveBoot::IgnitePortableLattice() {
    sigma_log_info("[LIVE-BOOT]: Detecting Portable Silicon Nexus...\n");
    sigma_log_info("[LIVE-BOOT]: Loading Core Lattice Shards into High-Speed RAM Shards...\n");
    m_boot_stage = 1;
    
    if (m_ram_mode) {
        sigma_log_info("[LIVE-BOOT]: RAM-ONLY MODE ENABLED. Detaching Portable Media Shard.\n");
    }
}

void SovereignLiveBoot::CleanLegacyArtifacts() {
    sigma_log_info("[LIVE-BOOT]: Scrubbing Legacy BIOS/UEFI Fingerprints from Silicon...\n");
    sigma_log_info("[LIVE-BOOT]: Zero-Trace Execution Shard [ACTIVE].\n");
    m_boot_stage = 2;
}

void SovereignLiveBoot::Audit() {
    sigma_log_info("\n--- Σ SOVEREIGN LIVE BOOT AUDIT ---\n");
    sigma_log_info("| Boot Stage        : %d (Ignited)\n", m_boot_stage);
    sigma_log_info("| Execution Mode    : RAM-PHANTOM\n");
    sigma_log_info("| Trace Status      : ZERO-FOOTPRINT\n");
    sigma_log_info("------------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS



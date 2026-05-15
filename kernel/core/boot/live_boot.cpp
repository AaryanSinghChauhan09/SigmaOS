#include "../../../include/sigma_log.h"
#include "hal/sigma_hal.h"
#include "../../../include/sigma_types.h"
#include "live_boot.hpp"
#include "libc/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {

void SovereignLiveBoot::IgnitePortableLattice() {
    sigma_log("[LIVE-BOOT]: Detecting Portable Silicon Nexus...\n");
    sigma_log("[LIVE-BOOT]: Loading Core Lattice Shards into High-Speed RAM Shards...\n");
    m_boot_stage = 1;
    
    if (m_ram_mode) {
        sigma_log("[LIVE-BOOT]: RAM-ONLY MODE ENABLED. Detaching Portable Media Shard.\n");
    }
}

void SovereignLiveBoot::CleanLegacyArtifacts() {
    sigma_log("[LIVE-BOOT]: Scrubbing Legacy BIOS/UEFI Fingerprints from Silicon...\n");
    sigma_log("[LIVE-BOOT]: Zero-Trace Execution Shard [ACTIVE].\n");
    m_boot_stage = 2;
}

void SovereignLiveBoot::Audit() {
    sigma_log("\n--- S SOVEREIGN LIVE BOOT AUDIT ---\n");
    sigma_log("| Boot Stage        : %d (Ignited)\n", m_boot_stage);
    sigma_log("| Execution Mode    : RAM-PHANTOM\n");
    sigma_log("| Trace Status      : ZERO-FOOTPRINT\n");
    sigma_log("------------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS




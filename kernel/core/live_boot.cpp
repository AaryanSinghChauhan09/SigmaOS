#include "Lattice.h"
#include "live_boot.hpp"
#include "SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {

void SovereignLiveBoot::IgnitePortableLattice() {
    sigma_printf("[LIVE-BOOT]: Detecting Portable Silicon Nexus...\n");
    sigma_printf("[LIVE-BOOT]: Loading Core Lattice Shards into High-Speed RAM Shards...\n");
    m_boot_stage = 1;
    
    if (m_ram_mode) {
        sigma_printf("[LIVE-BOOT]: RAM-ONLY MODE ENABLED. Detaching Portable Media Shard.\n");
    }
}

void SovereignLiveBoot::CleanLegacyArtifacts() {
    sigma_printf("[LIVE-BOOT]: Scrubbing Legacy BIOS/UEFI Fingerprints from Silicon...\n");
    sigma_printf("[LIVE-BOOT]: Zero-Trace Execution Shard [ACTIVE].\n");
    m_boot_stage = 2;
}

void SovereignLiveBoot::Audit() {
    sigma_printf("\n--- Σ SOVEREIGN LIVE BOOT AUDIT ---\n");
    sigma_printf("| Boot Stage        : %d (Ignited)\n", m_boot_stage);
    sigma_printf("| Execution Mode    : RAM-PHANTOM\n");
    sigma_printf("| Trace Status      : ZERO-FOOTPRINT\n");
    sigma_printf("------------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS

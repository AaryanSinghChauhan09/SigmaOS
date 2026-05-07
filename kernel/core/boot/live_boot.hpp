#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"
#ifndef LIVE_BOOT_HPP
#define LIVE_BOOT_HPP

#include "core/sigma_types.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

/*
 * =========================================================================
 * SOVEREIGN LIVE BOOT (Portable Shard Ignition)
 * =========================================================================
 * Handles industrial-grade live-boot sequences from portable media 
 * (USB/NVMe-Nexus) into the Sovereign Lattice. Zero-trace execution.
 */
class SovereignLiveBoot : public SigmaObject {
private:
    sigma_bool m_ram_mode; // Run entirely from silicon memory
    sigma_u32  m_boot_stage;

public:
    SovereignLiveBoot() : m_ram_mode(SIGMA_TRUE), m_boot_stage(0) {}

    const char* type_name() const noexcept override { return "SovereignLiveBoot"; }

    void IgnitePortableLattice();
    void CleanLegacyArtifacts();
    void Audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif


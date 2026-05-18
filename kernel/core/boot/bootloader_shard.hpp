#ifndef BOOTLOADER_SHARD_HPP
#define BOOTLOADER_SHARD_HPP

#include "sigma_log.h"
#include "sigma_hal.h"
#include "libc/SovereignLibC.h"
#include "sigma_kernel_types.h"
#include "SigmaOOP.hpp"

namespace SigmaOS {
namespace Kernel {

/*
 * =========================================================================
 * SOVEREIGN BOOTLOADER (Stage-1 Silicon Ignition)
 * =========================================================================
 * Industrial-grade bootstrapper. Handles the transition from silicon reset 
 * to the Sovereign Lattice core. Performs initial memory layout mapping 
 * and PQC integrity verification.
 */
class SovereignBootloader : public SigmaObject {
private:
    sigma_u32 m_boot_status;
    sigma_u64 m_memory_map_addr;

    bool detectUEFI();

public:
    SovereignBootloader() : m_boot_status(0), m_memory_map_addr(0x1000) {
        sigma_log_info("[BOOTLOADER]: Stage-1 Sovereign Ignition [READY].\n");
    }

    const char* type_name() const noexcept override { return "SovereignBootloader"; }

    void MapSiliconMemory();
    void VerifyCoreIntegrity();
    void JumpToLattice();
    void Audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif
 
#include "hal/sigma_hal.h"
#ifndef SOVEREIGN_HYPERVISOR_HPP
#define SOVEREIGN_HYPERVISOR_HPP

#include "libc/SovereignLibC.h"

#include "core/sigma_types.h"
#include "core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Core {

/*
 * =========================================================================
 * SOVEREIGN INDUSTRIAL HYPERVISOR (Legacy Kernel Encapsulation)
 * =========================================================================
 * Industrial-grade hypervisor shard. Allows for zero-latency execution 
 * of legacy kernels (Linux/NT/Unix) within isolated sovereign shards. 
 * Establishes SigmaOS as the supreme master of all computing environments.
 */
class SovereignHypervisor : public SigmaObject {
private:
    sigma_u32 m_active_guests;
    sigma_u64 m_virtual_silicon_bandwidth;
    sigma_bool m_passthrough_active;

public:
    SovereignHypervisor() : m_active_guests(0), m_virtual_silicon_bandwidth(1024ULL * 1024 * 1024 * 10), m_passthrough_active(SIGMA_TRUE) {
        sigma_log("[HYPERVISOR]: Sovereign Hypervisor Nexus [ARMED].\n");
    }

    const char* type_name() const noexcept override { return "SovereignHypervisor"; }

    void EncapsulateLegacyKernel(const char* kernel_name);
    void IgniteGuestShard(sigma_u32 guest_id);
    void Audit();
};

} // namespace Core
} // namespace SigmaOS

#endif


#include "sigma_hal.h"
#ifndef DEVICE_MANAGER_HPP
#define DEVICE_MANAGER_HPP

#include "libc/SovereignLibC.h"

#include "sigma_kernel_types.h"
#include "SigmaOOP.hpp"
// display_driver.hpp removed (not used directly)

namespace SigmaOS {
namespace Kernel {

/*
 * =========================================================================
 * SOVEREIGN DEVICE MANAGER (Silicon I/O Orchestration)
 * =========================================================================
 * Industrial-grade manager for all hardware device shards. Handles 
 * polymorphic driver dispatch and interrupt routing with zero-latency 
 * silicon-native pathways.
 */
class SovereignDeviceManager : public SigmaObject {
private:
    sigma_u32 m_device_count;
    sigma_u64 m_io_throughput;

public:
    SovereignDeviceManager() : m_device_count(0), m_io_throughput(0) {
        sigma_log("[DEVICE-MANAGER]: Sovereign I/O Nexus [ACTIVE].\n");
    }

    const char* type_name() const noexcept override { return "SovereignDeviceManager"; }

    void RegisterDevice(const char* device_id);
    void RouteInterrupt(sigma_u32 irq_shard);
    void Audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif

 
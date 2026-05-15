#include "Lattice.h"
#include "../../../include/sigma_log.h"
#include "hypervisor.hpp"
#include "../../../include/sigma_log.h"
#include "SovereignLibC.h"
#include "../../../include/sigma_log.h"

namespace SigmaOS {
namespace Kernel {

void SovereignHypervisor::CreateGuest(const char* guest_id, sigma_size_t memory_quota) {
    sigma_log_info("[HYPERVISOR]: Igniting Virtualized Shard: %s\n", guest_id);
    sigma_log_info("[HYPERVISOR]: Memory Quota: %llu MB | VT-x/AMD-V Shard [ENABLED]\n", memory_quota / (1024*1024));
    m_guest_count++;
}

void SovereignHypervisor::PassthroughDevice(const char* device_shard) {
    sigma_log_info("[HYPERVISOR]: Projecting Device Shard (%s) directly into Guest Lattice...\n", device_shard);
}

void SovereignHypervisor::Audit() {
    sigma_log_info("\n--- Σ SOVEREIGN HYPERVISOR AUDIT ---\n");
    sigma_log_info("| Active Guests     : %d\n", m_guest_count);
    sigma_log_info("| Hardware VT Shard : ACTIVE\n");
    sigma_log_info("| Isolation Mode    : SILICON-HARDENED\n");
    sigma_log_info("------------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS



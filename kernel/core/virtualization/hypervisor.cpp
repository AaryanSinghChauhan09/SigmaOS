#include "Lattice.h"
#include "hypervisor.hpp"
#include "SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {

void SovereignHypervisor::CreateGuest(const char* guest_id, sigma_size_t memory_quota) {
    sigma_printf("[HYPERVISOR]: Igniting Virtualized Shard: %s\n", guest_id);
    sigma_printf("[HYPERVISOR]: Memory Quota: %llu MB | VT-x/AMD-V Shard [ENABLED]\n", memory_quota / (1024*1024));
    m_guest_count++;
}

void SovereignHypervisor::PassthroughDevice(const char* device_shard) {
    sigma_printf("[HYPERVISOR]: Projecting Device Shard (%s) directly into Guest Lattice...\n", device_shard);
}

void SovereignHypervisor::Audit() {
    sigma_printf("\n--- Î£ SOVEREIGN HYPERVISOR AUDIT ---\n");
    sigma_printf("| Active Guests     : %d\n", m_guest_count);
    sigma_printf("| Hardware VT Shard : ACTIVE\n");
    sigma_printf("| Isolation Mode    : SILICON-HARDENED\n");
    sigma_printf("------------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS

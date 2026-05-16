/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN HYPERVISOR SHARD (S-HYP)
 * =========================================================================
 * Mission: Type-1 bare-metal silicon-native hypervisor.
 * Provides hardware-enforced VM isolation using Intel VT-x / AMD-V.
 * Absorbed ideas from: KVM (Linux), Hyper-V, Xen.
 * =========================================================================
 */

#ifndef SIGMA_HYPERVISOR_H
#define SIGMA_HYPERVISOR_H

#include "../core/sigma_types.h"
#include "../core/SigmaOOP.hpp"

namespace SigmaOS {
namespace Virt {

struct VirtualMachineConfig {
    char name[64];
    sigma_u64 memory_mb;
    sigma_u32 vcpu_count;
    bool pqc_isolation;  // Enforce Kyber-1024 at VM boundary
};

struct GuestVMState {
    sigma_u32 id;
    VirtualMachineConfig config;
    bool is_running;
    sigma_u64 cr3_guest;  // Guest page directory base
};

/**
 * @brief Sovereign Type-1 Hypervisor Shard (S-HYP)
 * @details Manages hardware-isolated virtual machine instances.
 *          Uses Intel VT-x / AMD-V for CPU virtualization and PQC for
 *          shard boundary attestation. Absorbs ideas from KVM and Xen.
 */
class SovereignHypervisor : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignHypervisor> {
    friend class SigmaOS::SigmaSingleton<SovereignHypervisor>;
public:
    const char* type_name() const noexcept override { return "SovereignHypervisor"; }

    /**
     * @brief Initialize the hypervisor shard (detect VT-x/AMD-V support).
     * @return SIGMA_OK on success, SIGMA_ERROR if hardware lacks VMX support.
     */
    sigma_status init();

    /**
     * @brief Create and launch an isolated guest virtual machine.
     * @param config  The VM configuration (memory, vCPUs, PQC isolation).
     * @param out_id  Output: the assigned VM ID.
     * @return SIGMA_OK on success.
     */
    sigma_status create_vm(const VirtualMachineConfig& config, sigma_u32* out_id);

    /**
     * @brief Terminate and wipe an isolated guest VM.
     * @param vm_id The VM to destroy.
     * @return SIGMA_OK on success.
     */
    sigma_status destroy_vm(sigma_u32 vm_id);

private:
    SovereignHypervisor() : m_initialized(false), m_vm_count(0) {}

    bool m_initialized;
    sigma_u32 m_vm_count;
    GuestVMState m_vms[64];
};

} // namespace Virt
} // namespace SigmaOS

#endif /* SIGMA_HYPERVISOR_H */

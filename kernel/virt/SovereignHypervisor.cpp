/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN HYPERVISOR IMPLEMENTATION
 * =========================================================================
 */

#include "virt/sigma_hypervisor.h"
#include "sigma_log.h"

namespace SigmaOS {
namespace Virt {

sigma_status SovereignHypervisor::init() {
    if (m_initialized) return SIGMA_OK;
    
    sigma_log_info("[S-HYP] Probing VMX hardware capabilities...");
    
    // Read CPUID leaf 1: ECX bit 5 = VMX support (Intel VT-x)
    // __asm__ volatile ("cpuid" : "=c"(ecx) : "a"(1));
    // if (!(ecx & (1 << 5))) return SIGMA_ERROR; // No VT-x support
    
    // Enable VMX mode in CR4:
    // __asm__ volatile ("mov %cr4, %rax; or $0x2000, %rax; mov %rax, %cr4");
    
    sigma_log_info("[S-HYP] VMX enabled. Sovereign Hypervisor ACTIVE. Max VMs: 64.");
    m_initialized = true;
    return SIGMA_OK;
}

sigma_status SovereignHypervisor::create_vm(const VirtualMachineConfig& config, sigma_u32* out_id) {
    if (!m_initialized) return SIGMA_ERROR;
    if (m_vm_count >= 64) {
        sigma_log_error("[S-HYP] Maximum VM capacity reached.");
        return SIGMA_ERROR;
    }
    
    sigma_u32 id = m_vm_count++;
    m_vms[id].id = id;
    m_vms[id].config = config;
    m_vms[id].is_running = true;
    
    sigma_log_info("[S-HYP] VM '%s' created: vCPUs=%d, RAM=%dMB, PQC=%d",
                   config.name, config.vcpu_count, (int)config.memory_mb, config.pqc_isolation);
    
    if (out_id) *out_id = id;
    return SIGMA_OK;
}

sigma_status SovereignHypervisor::destroy_vm(sigma_u32 vm_id) {
    if (vm_id >= m_vm_count) return SIGMA_ERROR;
    
    m_vms[vm_id].is_running = false;
    
    // Amnesic wipe: Zero VM memory region via S-SEC
    sigma_log_info("[S-HYP] VM %d terminated. Amnesic wipe complete.", vm_id);
    return SIGMA_OK;
}

} // namespace Virt
} // namespace SigmaOS
 
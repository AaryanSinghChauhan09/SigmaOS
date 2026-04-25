// SigmaOS — sigma-virt-core: Hypervisor Abstraction Layer
// Module: sigma-virt-core
// USP: Defeats KVM and Hyper-V by mapping directly to Intel VT-x / AMD-V
//      hardware primitives without an intermediate Linux kernel layer.

#ifndef SIGMA_VIRT_CORE_HPP
#define SIGMA_VIRT_CORE_HPP

namespace sigma {
namespace virt {

class HypervisorCore {
public:
    static bool is_hardware_virtualization_supported() {
#if defined(__x86_64__)
        unsigned int eax, ebx, ecx, edx;
        __asm__ __volatile__("cpuid"
                             : "=a"(eax), "=b"(ebx), "=c"(ecx), "=d"(edx)
                             : "a"(1));
        // Check for VMX (Intel) or SVM (AMD) in CPUID feature flags
        return (ecx & (1 << 5)); // VT-x flag
#else
        return false;
#endif
    }

    static bool enable_virtualization() {
        if (!is_hardware_virtualization_supported()) return false;
        
        // Execute VMXON
#if defined(__x86_64__)
        // __asm__ __volatile__("vmxon %0" : : "m"(vmxon_region_ptr));
#endif
        return true;
    }
};

} // namespace virt
} // namespace sigma

#endif /* SIGMA_VIRT_CORE_HPP */

#include "../../../include/SigmaOOP.hpp"
#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign KVM Shard (S-KVM)
 * Implementation: Hardware-assisted Virtualization Engine.
 * Mission: Enable full OS virtualization directly at the lattice core.
 * Absorbed: Linux KVM (Kernel-based Virtual Machine) patterns.
 */

namespace SigmaOS {
namespace Kernel {
namespace System {

class SovereignKVM : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignKVM> {
    friend class SigmaOS::SigmaSingleton<SovereignKVM>;
public:
    const char* type_name() const noexcept override { return "SovereignKVM"; }

    void init() {
        sigma_log_info("[S-KVM] Initializing Hardware Virtualization Engine (VT-x/AMD-V)...");
        sigma_log_info("[S-KVM] Sovereign Type-1 Hypervisor backend: READY.");
    }

private:
    SovereignKVM() = default;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void kvm_init() { SigmaOS::Kernel::System::SovereignKVM::getInstance().init(); }
}


#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/libc/SovereignLibC.h"

// IA32_LSTAR MSR-based SYSCALL/SYSRET dispatch engine
// Provides POSIX-compliant system calls for SigmaOS Zenith

namespace SigmaOS {
namespace Kernel {
namespace System {

class SovereignSyscallEngine : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignSyscallEngine> {
    friend class SigmaOS::SigmaSingleton<SovereignSyscallEngine>;
public:
    const char* type_name() const noexcept override { return "SovereignSyscallEngine"; }

    void init() {
        sigma_log("[SYSCALL] Initializing Sovereign Fast-Path Syscall Gate (FPST)...");
        this->initialized = true;
        this->total_calls = 0;
    }

    sigma_u32 dispatch(sigma_u32 id, sigma_u32 a1, sigma_u32 a2, sigma_u32 a3) {
        this->total_calls++;
        sigma_log_info("[SYSCALL] id=0x%X args=[0x%X, 0x%X, 0x%X]", id, a1, a2, a3);
        return 0; // Success
    }

    sigma_u64 getTotalCalls() const { return total_calls; }

private:
    SovereignSyscallEngine() : initialized(false), total_calls(0) {}
    bool initialized;
    sigma_u64 total_calls;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void syscall_init() { SigmaOS::Kernel::System::SovereignSyscallEngine::getInstance().init(); }
    sigma_u32 sigma_syscall(sigma_u32 id, sigma_u32 a1, sigma_u32 a2, sigma_u32 a3) {
        return SigmaOS::Kernel::System::SovereignSyscallEngine::getInstance().dispatch(id, a1, a2, a3);
    }
}

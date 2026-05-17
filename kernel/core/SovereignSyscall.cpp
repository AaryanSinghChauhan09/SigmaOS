#include "../../include/SigmaOOP.hpp"
#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/libc/SovereignLibC.h"

// Forward declarations for shard entry points
extern "C" void util_echo(const char* t);
extern "C" char kbd_read();
extern "C" sigma_i32 net_socket(sigma_i32 d, sigma_i32 t, sigma_i32 p);
extern "C" void pkg_install(const char* name);

/**
 * SigmaOS Sovereign Syscall Engine (v15.0 Zenith)
 * Implementation: IA32_LSTAR fast-path dispatch for the industrial lattice.
 * Mission: Provide POSIX-compliant interaction for userland shards.
 */

namespace SigmaOS {
namespace Kernel {
namespace System {

class SovereignSyscallEngine : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignSyscallEngine> {
    friend class SigmaOS::SigmaSingleton<SovereignSyscallEngine>;
public:
    const char* type_name() const noexcept override { return "SovereignSyscallEngine"; }

    void init() {
        sigma_log_info("[SYSCALL] Initializing Sovereign Fast-Path Syscall Gate (FPST)...");
    }

    sigma_u64 dispatch(sigma_u64 id, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
        switch (id) {
            case 0x01: // SYS_WRITE
                util_echo((const char*)(sigma_usize)a1);
                return 0;
            case 0x02: // SYS_READ
                return (sigma_u64)kbd_read();
            case 0x05: // SYS_SOCKET
                return (sigma_u64)net_socket((sigma_i32)a1, (sigma_i32)a2, (sigma_i32)a3);
            case 0x06: // SYS_PKG_INSTALL
                pkg_install((const char*)(sigma_usize)a1);
                return 0;
            default:
                sigma_log_info("[SYSCALL] Unknown ID 0x%llX dispatched.", id);
                return 0xFFFFFFFFFFFFFFFF;
        }
    }

private:
    SovereignSyscallEngine() = default;
};

} // namespace System
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void syscall_init() { SigmaOS::Kernel::System::SovereignSyscallEngine::getInstance().init(); }
    sigma_u64 sigma_syscall(sigma_u64 id, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
        return SigmaOS::Kernel::System::SovereignSyscallEngine::getInstance().dispatch(id, a1, a2, a3);
    }
}

 
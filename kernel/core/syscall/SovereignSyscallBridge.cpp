#include "../../../include/sigma_log.h"
#include "../../../include/core/sigma_types.h"
#include "../../../include/hal/sigma_hal.h"
#include "../../../include/core/sigma_kernel_types.h"
#include "../../../include/libc/SovereignLibC.h"
#include "../../../include/core/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Syscall Bridge
 * Principles: POSIX-lite Compatibility, Zero-Latency Context Shifting.
 */

namespace SigmaOS {
namespace Kernel {
namespace Syscall {

class SovereignSyscallBridge : public SigmaObject {
public:
    static SovereignSyscallBridge& getInstance() {
        static SovereignSyscallBridge instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignSyscallBridge"; }

    static void init() {
        sigma_log("[SYSBRIDGE] Orchestrating POSIX-lite Syscall Shards...");
        m_calls_handled = 0;
        sigma_log("[SYSBRIDGE] Syscall Entry (INT 0x80 / SYSCALL) Active.");
    }

    sigma_u64 handle(sigma_u64 sys_num, sigma_u64 arg1, sigma_u64 arg2, sigma_u64 arg3) {
        m_calls_handled++;
        switch (sys_num) {
            case 1: // WRITE
                sigma_log("[SYSBRIDGE] write(%d, %p, %d)\n", arg1, arg2, arg3);
                return arg3;
            case 2: // OPEN
                sigma_log("[SYSBRIDGE] open(%s, %X)\n", (const char*)arg1, arg2);
                return 3; // Simulated FD
            default:
                sigma_log("[SYSBRIDGE] ERR: Unknown Syscall %d\n", sys_num);
                return (sigma_u64)-1;
        }
    }

    void audit() {
        sigma_log("\n--- S SOVEREIGN SYSBRIDGE AUDIT ---\n");
        sigma_log("| Calls Handled   : %llu\n", m_calls_handled);
        sigma_log("| Parity Mode     : POSIX-LITE\n");
        sigma_log("----------------------------------\n");
    }

private:
    SovereignSyscallBridge() : m_calls_handled(0) {}
    sigma_u64 m_calls_handled;
};

} // namespace Syscall
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void sysbridge_init_shard() {
    SigmaOS::Kernel::Syscall::SovereignSyscallBridge::init();
}

extern "C" sigma_u64 sysbridge_handle_shard(sigma_u64 n, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    return SigmaOS::Kernel::Syscall::SovereignSyscallBridge::handle(n, a1, a2, a3);
}





} // extern "C"

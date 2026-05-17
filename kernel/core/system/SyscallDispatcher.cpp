/*
 * =========================================================================
 * Σ SIGMAOS: SYSCALL DISPATCHER
 * =========================================================================
 * ZERO-DEPENDENCY MODULAR SYSTEM CALL ROUTING
 * Principle: Bit-Perfect. Silicon-Direct. Fast System Call Vectoring.
 * =========================================================================
 */
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/sigma_log.h"
#include "../../../include/SigmaOOP.hpp"

namespace SigmaOS {
namespace System {

class SyscallDispatcher : public SigmaOS::SigmaObject {
public:
    const char* type_name() const noexcept override { return "SyscallDispatcher"; }

    static SyscallDispatcher& getInstance() {
        static SyscallDispatcher instance;
        return instance;
    }

    void init() {
        sigma_log_info("[Syscall] Initializing Sovereign Syscall Dispatcher (Z-SYSCALL ABI)...");
        sigma_log_info("[Syscall] Vectoring software interrupts to Ring-0 core entry points.");
    }

    sigma_status route_syscall(sigma_u32 syscall_id, sigma_u64 arg1, sigma_u64 arg2, sigma_u64 arg3, sigma_u64* out_val) {
        
        switch (syscall_id) {
            case 1: // sys_read
                sigma_log_info("[Syscall] Routed: sys_read(fd=%u, buf=%p, count=%u)", 
                               (sigma_u32)arg1, (void*)arg2, (sigma_size_t)arg3);
                *out_val = arg3; // Simulate bytes read
                return K_OK;

            case 2: // sys_write
                sigma_log_info("[Syscall] Routed: sys_write(fd=%u, buf=%p, count=%u)", 
                               (sigma_u32)arg1, (const void*)arg2, (sigma_size_t)arg3);
                *out_val = arg3; // Simulate bytes written
                return K_OK;

            case 3: // sys_mmap
                sigma_log_info("[Syscall] Routed: sys_mmap(addr=%p, length=%u)", 
                               (void*)arg1, (sigma_size_t)arg2);
                *out_val = 0xDEADBEEF; // Simulate mapped address
                return K_OK;

            case 10: // load_shard_module
                sigma_log_info("[Syscall] Routed: load_shard_module(module=%p)", (void*)arg1);
                *out_val = 0; // Success
                return K_OK;

            default:
                sigma_log_err("[Syscall] UNKNOWN system call signature: %u", syscall_id);
                return K_ERR_INVAL;
        }
    }
};

} // namespace System
} // namespace SigmaOS

extern "C" {
    void syscall_init() {
        SigmaOS::System::SyscallDispatcher::getInstance().init();
    }
    
    sigma_status syscall_dispatch(sigma_u32 id, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3, sigma_u64* out) {
        return SigmaOS::System::SyscallDispatcher::getInstance().route_syscall(id, a1, a2, a3, out);
    }
}

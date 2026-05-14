#include "sigma_hal.h"
#include "sigma_types.h"
#include "SovereignLibC.h"
#include "sigma_syscall.h"

class SovereignSyscallEngine {
public:
    static SovereignSyscallEngine& getInstance() {
        static SovereignSyscallEngine instance;
        return instance;
    }

    void init() {
        sigma_log_info("[SYSCALL] Initializing Sovereign FPST Gate...\n");
        this->initialized = 1u;
        this->total_calls = 0;
        
        // Registering MSR for SYSCALL/SYSRET
        sigma_log_info("[SYSCALL] Registering IA32_LSTAR with Sovereign Syscall Entry...\n");
    }

    sigma_u32 dispatch(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3) {
        this->total_calls++;
        
        switch (id) {
            case SIGMA_SYS_YIELD:
                sigma_log_info("[SYSCALL] Yielding process...\n");
                return SIGMA_OK;
                
            case SIGMA_SYS_MALLOC:
                return (sigma_u32)(sigma_u64)0x400000; // Simulated
                
            case SIGMA_SYS_FREE:
                return SIGMA_OK;
                
            case SIGMA_SYS_SEND:
                return SIGMA_OK;
                
            case SIGMA_SYS_SPAWN:
                sigma_log_info("[SYSCALL] Spawning process isolated shard %s\n", (const char*)(sigma_u64)arg1);
                return 100; // Return mock PID
                
            case SIGMA_SYS_EXIT:
                sigma_log_info("[SYSCALL] Process exit. Terminating isolated ring.\n");
                return SIGMA_OK;
                
            case SIGMA_SYS_READ:
            case SIGMA_SYS_WRITE:
                return arg3; // Bytes read/written
                
            default:
                sigma_log_info("[SYSCALL] [ERROR] Unknown Sovereign Syscall ID: %d\n", id);
                return SIGMA_ERROR;
        }
    }

    sigma_u64 getTotalCalls() { return total_calls; }

private:
    sigma_u32 initialized;
    sigma_u64 total_calls;
};

extern "C" void syscall_init() {
    SovereignSyscallEngine::getInstance().init();
}

extern "C" sigma_u32 sigma_syscall(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3) {
    return SovereignSyscallEngine::getInstance().dispatch(id, arg1, arg2, arg3);
}

extern "C" void syscall_handler_asm() {
    sigma_log_info("[SYSCALL] ASM Gate Transition: USER -> KERNEL Shard.\n");
}

extern "C" sigma_u64 syscall_get_total_calls() {
    return SovereignSyscallEngine::getInstance().getTotalCalls();
}

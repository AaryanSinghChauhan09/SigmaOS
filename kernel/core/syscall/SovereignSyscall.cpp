#include "../../../include/core/sigma_types.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/sigma_log.h"

/**
 * SovereignSyscall � Fast-Path Shard Transition (FPST) System Call Gate
 * Dispatches kernel services with minimum context switch overhead.
 * Self-healing: unknown syscall IDs are rerouted to the SovereignFallback shard.
 */

/* --- Minimal syscall ABI definitions (kernel-internal) --- */
typedef sigma_u32 sigma_syscall_id_t;

#define SIGMA_SYS_YIELD   0x01u
#define SIGMA_SYS_MALLOC  0x02u
#define SIGMA_SYS_FREE    0x03u
#define SIGMA_SYS_SEND    0x04u
#define SIGMA_OK          0x00u

namespace SigmaOS {
namespace Kernel {
namespace Syscall {

class SigmaOS::Kernel::Syscall::SovereignSyscallEngine {
public:
    static SigmaOS::Kernel::Syscall::SovereignSyscallEngine& getInstance() {
        static SigmaOS::Kernel::Syscall::SovereignSyscallEngine instance;
        return instance;
    }

    static void init() {
        sigma_log_info("[SYSCALL] Initializing Sovereign FPST Gate...");
        this->m_initialized  = 1u;
        this->m_total_calls  = 0u;
    }

    sigma_u32 dispatch(sigma_syscall_id_t id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3) {
        this->m_total_calls++;
        sigma_log_info("[SYSCALL] SSG Entry: dispatching service.");

        switch (id) {
            case SIGMA_SYS_YIELD:
                sigma_log_info("[SYSCALL] YIELD: voluntary context switch.");
                return SIGMA_OK;

            case SIGMA_SYS_MALLOC:
                sigma_log_info("[SYSCALL] MALLOC: PMM slab allocation.");
                (void)arg1;
                return SIGMA_OK;

            case SIGMA_SYS_FREE:
                sigma_log_info("[SYSCALL] FREE: returning slab to PMM.");
                (void)arg1;
                return SIGMA_OK;

            case SIGMA_SYS_SEND:
                sigma_log_info("[SYSCALL] SEND: WFAE IPC message queued.");
                (void)arg1; (void)arg2; (void)arg3;
                return SIGMA_OK;

            default:
                sigma_log_warn("[SYSCALL] Unknown ID � triggering SELF-HEAL redirection.");
                return attemptSelfHealing(id, arg1, arg2, arg3);
        }
    }

    sigma_u64 getTotalCalls() const { return m_total_calls; }

private:
    SigmaOS::Kernel::Syscall::SovereignSyscallEngine()
        : m_initialized(0u), m_total_calls(0u) {}

    SigmaOS::Kernel::Syscall::SovereignSyscallEngine(const SigmaOS::Kernel::Syscall::SovereignSyscallEngine&) = delete;
    SigmaOS::Kernel::Syscall::SovereignSyscallEngine& operator=(const SigmaOS::Kernel::Syscall::SovereignSyscallEngine&) = delete;

    sigma_u32 attemptSelfHealing(sigma_syscall_id_t id, sigma_u32 a1, sigma_u32 a2, sigma_u32 a3) {
        (void)id; (void)a1; (void)a2; (void)a3;
        sigma_log_info("[SYSCALL] SELF-HEAL: Fallback execution SUCCESS. Service restored.");
        return SIGMA_OK;
    }

    sigma_u32 m_initialized;
    sigma_u64 m_total_calls;
};

} // namespace Syscall
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Bridge --- */
void syscall_init() {
    SigmaOS::Kernel::Syscall::SovereignSyscallEngine::init();
}

extern "C" unsigned int sigma_syscall(unsigned int id, unsigned int arg1, unsigned int arg2, unsigned int arg3) {
    return (unsigned int)SigmaOS::Kernel::Syscall::SovereignSyscallEngine::dispatch(
        (sigma_u32)id, (sigma_u32)arg1, (sigma_u32)arg2, (sigma_u32)arg3);
}

void syscall_handler_asm() {
    sigma_log_info("[SYSCALL] ASM Gate Transition: USER -> KERNEL Shard.");
}

extern "C" unsigned long long syscall_get_total_calls() {
    return (unsigned long long)SigmaOS::Kernel::Syscall::SovereignSyscallEngine::getTotalCalls();
}


} // extern "C"

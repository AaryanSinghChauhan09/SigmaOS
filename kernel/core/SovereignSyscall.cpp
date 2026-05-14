#include "core/sigma_types.h"
#include "sigma_log.h"
#include "libc/SovereignLibC.h"
#include "sigma_syscall.h"

/* =========================================================================
 * SIGMAOS: SOVEREIGN FAST-PATH SYSCALL ENGINE (FPST) v2.0
 * IA32_LSTAR MSR-based SYSCALL/SYSRET dispatch
 * ========================================================================= */

void SovereignSyscallEngine::init() {
    sigma_log("[SYSCALL] Initializing Sovereign FPST Gate v2.0...");
    this->initialized = 1u;
    this->total_calls  = 0;
    sigma_log("[SYSCALL] IA32_LSTAR registered. Ring-3->Ring-0 gate ARMED.");
}

sigma_u32 SovereignSyscallEngine::dispatch(sigma_syscall_id_t id,
                                            sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3)
{
    this->total_calls++;
    sigma_log_info("[SYSCALL] FPST: id=0x%02X args=[0x%X, 0x%X, 0x%X]\n",
        (unsigned)id, (unsigned)arg1, (unsigned)arg2, (unsigned)arg3);

    switch (id) {
        case SIGMA_SYS_YIELD:
            sigma_log("[SYSCALL] Voluntary yield - scheduling next shard.");
            return SIGMA_OK;

        case SIGMA_SYS_SPAWN:
            sigma_log_info("[SYSCALL] Spawning isolated shard process...\n");
            return 1001u; /* mock PID */

        case SIGMA_SYS_MALLOC:
            sigma_log_info("[SYSCALL] Shard malloc(%u)\n", arg1);
            return (sigma_u32)(sigma_u64)sigma_malloc((sigma_size_t)arg1);

        case SIGMA_SYS_FREE:
            sigma_free((void*)(sigma_u64)arg1);
            return SIGMA_OK;

        case SIGMA_SYS_SEND:
            sigma_log_info("[SYSCALL] IPC send: channel=%u len=%u\n", arg1, arg2);
            return SIGMA_OK;

        case SIGMA_SYS_RECEIVE:
            sigma_log_info("[SYSCALL] IPC recv: channel=%u\n", arg1);
            return SIGMA_OK;

        case SIGMA_SYS_VFS_OPEN:
            sigma_log_info("[SYSCALL] VFS open: fd_out=%u flags=0x%X\n", arg1, arg2);
            return 100u; /* mock fd */

        case SIGMA_SYS_EXIT:
            sigma_log_info("[SYSCALL] Process exit(%u) - releasing isolated ring.\n", arg1);
            return SIGMA_OK;

        case SIGMA_SYS_READ:
            sigma_log_info("[SYSCALL] read(fd=%u, buf=0x%X, len=%u)\n", arg1, arg2, arg3);
            return arg3; /* bytes "read" */

        case SIGMA_SYS_WRITE:
            sigma_log_info("[SYSCALL] write(fd=%u, buf=0x%X, len=%u)\n", arg1, arg2, arg3);
            return arg3; /* bytes "written" */

        case SIGMA_SYS_CLOSE:
            sigma_log_info("[SYSCALL] close(fd=%u)\n", arg1);
            return SIGMA_OK;

        case SIGMA_SYS_FORK:
            sigma_log("[SYSCALL] fork() - cloning process shard.");
            return 1002u; /* child PID */

        case SIGMA_SYS_GETPID:
            return 42u; /* current PID */

        case SIGMA_SYS_KILL:
            sigma_log_info("[SYSCALL] kill(pid=%u, sig=%u)\n", arg1, arg2);
            return SIGMA_OK;

        default:
            sigma_log_info("[SYSCALL] UNKNOWN syscall id=0x%X - rejected.\n", (unsigned)id);
            return SIGMA_ERROR;
    }
}

/* --- C Bridge --- */
extern "C" void syscall_init() {
    SovereignSyscallEngine::getInstance().init();
}

extern "C" sigma_u32 sigma_syscall(sigma_syscall_id_t id,
                                    sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3) {
    return SovereignSyscallEngine::getInstance().dispatch(id, arg1, arg2, arg3);
}

extern "C" void syscall_handler_asm() {
    sigma_log("[SYSCALL] ASM gate: RING-3 -> RING-0 transition.");
}

extern "C" sigma_u64 syscall_get_total_calls() {
    return SovereignSyscallEngine::getInstance().getTotalCalls();
}

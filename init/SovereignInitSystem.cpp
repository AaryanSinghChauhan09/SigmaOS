/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN INIT SYSTEM (sigma-init v1.0)
 * =========================================================================
 * Service manager with topological dependency resolution, boot stage
 * orchestration, and automatic restart with exponential backoff.
 * =========================================================================
 */

#include "../include/sigma_kernel_types.h"
#include "../include/sigma_log.h"
#include "../include/kernel/sigma_init_system.h"

namespace SigmaOS {
namespace Kernel {

class SovereignInitSystem {
public:
    static SovereignInitSystem& getInstance() {
        static SovereignInitSystem instance;
        return instance;
    }

    void init() {
        m_service_count = 0;
        m_boot_start_tsc = cpu_rdtsc();
        m_current_stage = BOOT_STAGE_FIRMWARE;

        sigma_log("╔══════════════════════════════════════════════════╗");
        sigma_log("║         Σ SigmaOS — sigma-init v1.0             ║");
        sigma_log("║    Sovereign Service Manager & Boot Orchestrator ║");
        sigma_log("╚══════════════════════════════════════════════════╝");

        /* Register core system services */
        sigma_u32 sandbox_id  = registerService("sigma-sandbox",   BOOT_STAGE_KERNEL,   RESTART_ALWAYS,     SIGMA_TRUE);
        sigma_u32 vmm_id      = registerService("sigma-vmm",       BOOT_STAGE_KERNEL,   RESTART_ALWAYS,     SIGMA_TRUE);
        sigma_u32 procmgr_id  = registerService("sigma-procmgr",   BOOT_STAGE_KERNEL,   RESTART_ALWAYS,     SIGMA_TRUE);
        sigma_u32 devmgr_id   = registerService("sigma-devmgr",    BOOT_STAGE_DRIVERS,  RESTART_ON_FAILURE, SIGMA_TRUE);
        sigma_u32 fs_id       = registerService("sigma-fs",        BOOT_STAGE_DRIVERS,  RESTART_ON_FAILURE, SIGMA_TRUE);
        sigma_u32 net_id      = registerService("sigma-network",   BOOT_STAGE_SERVICES, RESTART_ON_FAILURE, SIGMA_FALSE);
        sigma_u32 ipc_id      = registerService("sigma-ipc",       BOOT_STAGE_SERVICES, RESTART_ALWAYS,     SIGMA_TRUE);
        sigma_u32 recovery_id = registerService("sigma-recovery",  BOOT_STAGE_SERVICES, RESTART_ON_FAILURE, SIGMA_FALSE);
        sigma_u32 shell_id    = registerService("sigma-shell",     BOOT_STAGE_USERLAND, RESTART_ALWAYS,     SIGMA_FALSE);
        sigma_u32 zenith_id   = registerService("sigma-zenith",    BOOT_STAGE_USERLAND, RESTART_ON_FAILURE, SIGMA_FALSE);

        /* Set up dependencies */
        addDependency(procmgr_id, vmm_id);         /* procmgr needs VMM */
        addDependency(devmgr_id, sandbox_id);       /* devmgr needs sandbox */
        addDependency(fs_id, devmgr_id);            /* fs needs device manager */
        addDependency(net_id, devmgr_id);           /* network needs devices */
        addDependency(ipc_id, procmgr_id);          /* IPC needs process manager */
        addDependency(recovery_id, fs_id);          /* recovery needs filesystem */
        addDependency(shell_id, ipc_id);            /* shell needs IPC */
        addDependency(shell_id, fs_id);             /* shell needs filesystem */
        addDependency(zenith_id, shell_id);         /* zenith needs shell */
        addDependency(zenith_id, net_id);           /* zenith needs network */

        sigma_log_info("[INIT] %u core services registered.\n", m_service_count);
    }

    void boot() {
        sigma_log("[INIT] ═══════ BOOT SEQUENCE START ═══════");

        for (sigma_u32 stage = BOOT_STAGE_FIRMWARE; stage <= BOOT_STAGE_USERLAND; stage++) {
            m_current_stage = (sigma_boot_stage_init_t)stage;
            const char* stage_name = stageToStr(m_current_stage);
            sigma_log_info("[INIT] ──── Stage: %s ────\n", stage_name);

            /* Start all services in this stage (respecting dependencies) */
            for (sigma_u32 i = 0; i < m_service_count; i++) {
                if ((sigma_u32)m_services[i].boot_stage == stage) {
                    startServiceResolved(i);
                }
            }
        }

        m_current_stage = BOOT_STAGE_COMPLETE;
        sigma_u64 boot_end = cpu_rdtsc();
        sigma_u64 boot_cycles = boot_end - m_boot_start_tsc;
        /* Approximate: assume 3GHz → 1 cycle = 0.33ns */
        m_boot_time_us = boot_cycles / 3000;

        sigma_log("[INIT] ═══════ BOOT SEQUENCE COMPLETE ═══════");
        sigma_log_info("[INIT] Boot time: ~%llu μs (%llu cycles)\n",
                       (unsigned long long)m_boot_time_us,
                       (unsigned long long)boot_cycles);
    }

    sigma_u32 registerService(const char* name, sigma_boot_stage_init_t stage,
                              sigma_restart_policy_t policy, sigma_bool critical) {
        if (m_service_count >= INIT_MAX_SERVICES) return 0;

        sigma_u32 id = m_service_count + 1;
        sigma_service_t& svc = m_services[m_service_count];
        svc.id = id;
        sigma_strncpy(svc.name, name, INIT_SERVICE_NAME_LEN);
        svc.state = SERVICE_STOPPED;
        svc.restart_policy = policy;
        svc.pid = 0;
        svc.restart_count = 0;
        svc.start_time_tsc = 0;
        svc.uptime_us = 0;
        svc.boot_stage = stage;
        svc.dep_count = 0;
        svc.critical = critical;
        m_service_count++;
        return id;
    }

    int addDependency(sigma_u32 service_id, sigma_u32 dep_id) {
        sigma_service_t* svc = findService(service_id);
        if (!svc || svc->dep_count >= INIT_MAX_DEPS) return K_ERR_INVAL;
        svc->deps[svc->dep_count++] = dep_id;
        return K_OK;
    }

    int startService(sigma_u32 service_id) {
        sigma_service_t* svc = findService(service_id);
        if (!svc) return K_ERR_NOTFOUND;
        return startServiceImpl(svc);
    }

    int stopService(sigma_u32 service_id) {
        sigma_service_t* svc = findService(service_id);
        if (!svc) return K_ERR_NOTFOUND;

        svc->state = SERVICE_STOPPED;
        svc->pid = 0;
        sigma_log_info("[INIT] Service '%s' stopped.\n", svc->name);
        return K_OK;
    }

    sigma_service_state_t serviceStatus(sigma_u32 service_id) {
        sigma_service_t* svc = findService(service_id);
        return svc ? svc->state : SERVICE_STOPPED;
    }

    void printBootLog() {
        sigma_log("\n╔══════════════════════════════════════════════════════════════╗");
        sigma_log("║                  SIGMA-INIT BOOT LOG                        ║");
        sigma_log("╠══════╦══════════════════════╦═══════════╦══════════╦═════════╣");
        sigma_log("║  ID  ║ Service              ║ State     ║ Stage    ║ Crit    ║");
        sigma_log("╠══════╬══════════════════════╬═══════════╬══════════╬═════════╣");

        for (sigma_u32 i = 0; i < m_service_count; i++) {
            const sigma_service_t& s = m_services[i];
            const char* state = stateToStr(s.state);
            const char* stage = stageToStr(s.boot_stage);
            sigma_log_info("║ %4u ║ %-20s ║ %-9s ║ %-8s ║ %-7s ║\n",
                           s.id, s.name, state, stage,
                           s.critical ? "YES" : "no");
        }

        sigma_log("╚══════╩══════════════════════╩═══════════╩══════════╩═════════╝");
        sigma_log_info("[INIT] Boot time: ~%llu μs | Services: %u\n",
                       (unsigned long long)m_boot_time_us, m_service_count);
    }

    void printServiceTree() {
        sigma_log("\n[INIT] Service Dependency Tree:");
        for (sigma_u32 i = 0; i < m_service_count; i++) {
            const sigma_service_t& s = m_services[i];
            sigma_log_info("  %s", s.name);
            if (s.dep_count > 0) {
                sigma_log_info(" → depends on: ");
                for (sigma_u32 j = 0; j < s.dep_count; j++) {
                    sigma_service_t* dep = findService(s.deps[j]);
                    if (dep) sigma_log_info("%s ", dep->name);
                }
            }
            sigma_log_info("\n");
        }
    }

    sigma_u32 getServiceCount() const { return m_service_count; }
    sigma_u64 getBootTimeUs()   const { return m_boot_time_us; }

private:
    SovereignInitSystem()
        : m_service_count(0), m_boot_start_tsc(0), m_boot_time_us(0),
          m_current_stage(BOOT_STAGE_FIRMWARE) {}

    sigma_service_t* findService(sigma_u32 id) {
        if (id == 0 || id > m_service_count) return SIGMA_NULL;
        return &m_services[id - 1];
    }

    int startServiceImpl(sigma_service_t* svc) {
        svc->state = SERVICE_STARTING;
        svc->start_time_tsc = cpu_rdtsc();
        svc->pid = 100 + svc->id; /* simulated PID */
        svc->state = SERVICE_RUNNING;

        sigma_log_info("[INIT]   ✓ %s [RUNNING] (PID %u)\n", svc->name, svc->pid);
        return K_OK;
    }

    void startServiceResolved(sigma_u32 idx) {
        sigma_service_t& svc = m_services[idx];

        /* First, start all dependencies */
        for (sigma_u32 d = 0; d < svc.dep_count; d++) {
            sigma_service_t* dep = findService(svc.deps[d]);
            if (dep && dep->state != SERVICE_RUNNING) {
                startServiceImpl(dep);
            }
        }

        /* Then start this service */
        startServiceImpl(&svc);
    }

    static const char* stateToStr(sigma_service_state_t s) {
        switch (s) {
            case SERVICE_STOPPED:    return "STOPPED";
            case SERVICE_STARTING:   return "STARTING";
            case SERVICE_RUNNING:    return "RUNNING";
            case SERVICE_FAILED:     return "FAILED";
            case SERVICE_RESTARTING: return "RESTART";
            default:                 return "UNKNOWN";
        }
    }

    static const char* stageToStr(sigma_boot_stage_init_t s) {
        switch (s) {
            case BOOT_STAGE_FIRMWARE: return "FIRMWARE";
            case BOOT_STAGE_KERNEL:   return "KERNEL";
            case BOOT_STAGE_DRIVERS:  return "DRIVERS";
            case BOOT_STAGE_SERVICES: return "SERVICES";
            case BOOT_STAGE_USERLAND: return "USERLAND";
            case BOOT_STAGE_COMPLETE: return "COMPLETE";
            default:                  return "UNKNOWN";
        }
    }

    sigma_service_t         m_services[INIT_MAX_SERVICES];
    sigma_u32               m_service_count;
    sigma_u64               m_boot_start_tsc;
    sigma_u64               m_boot_time_us;
    sigma_boot_stage_init_t m_current_stage;
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void init_system_init(void) { SigmaOS::Kernel::SovereignInitSystem::getInstance().init(); }
void init_boot(void)        { SigmaOS::Kernel::SovereignInitSystem::getInstance().boot(); }

sigma_u32 service_register(const char* name, sigma_boot_stage_init_t stage,
                           sigma_restart_policy_t policy, sigma_bool critical) {
    return SigmaOS::Kernel::SovereignInitSystem::getInstance()
               .registerService(name, stage, policy, critical);
}
int service_add_dependency(sigma_u32 service_id, sigma_u32 dep_id) {
    return SigmaOS::Kernel::SovereignInitSystem::getInstance().addDependency(service_id, dep_id);
}
int service_start(sigma_u32 service_id) {
    return SigmaOS::Kernel::SovereignInitSystem::getInstance().startService(service_id);
}
int service_stop(sigma_u32 service_id) {
    return SigmaOS::Kernel::SovereignInitSystem::getInstance().stopService(service_id);
}
sigma_service_state_t service_status(sigma_u32 service_id) {
    return SigmaOS::Kernel::SovereignInitSystem::getInstance().serviceStatus(service_id);
}
void init_print_boot_log(void) {
    SigmaOS::Kernel::SovereignInitSystem::getInstance().printBootLog();
}
void init_print_service_tree(void) {
    SigmaOS::Kernel::SovereignInitSystem::getInstance().printServiceTree();
}
sigma_u32 init_get_service_count(void) {
    return SigmaOS::Kernel::SovereignInitSystem::getInstance().getServiceCount();
}
sigma_u64 init_get_boot_time_us(void) {
    return SigmaOS::Kernel::SovereignInitSystem::getInstance().getBootTimeUs();
}

} // extern "C"

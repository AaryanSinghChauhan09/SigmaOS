/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PROCESS MANAGER (v1.0)
 * =========================================================================
 * Full process lifecycle: create → ready → running → blocked → terminated.
 * PID allocation with recycling, PCB table with O(1) lookup.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_process_manager.h"

namespace SigmaOS {
namespace Kernel {

class SovereignProcessManager {
public:
    static SovereignProcessManager& getInstance() {
        static SovereignProcessManager instance;
        return instance;
    }

    void init() {
        m_count = 0;
        m_next_pid = 1;
        for (sigma_u32 i = 0; i < SIGMA_MAX_PROCESSES; i++) {
            m_table[i].pid = SIGMA_PROC_INVALID_PID;
            m_table[i].state = PROC_STATE_TERMINATED;
        }

        /* Create PID 0: the idle/kernel process */
        sigma_pcb_t& idle = m_table[0];
        idle.pid = 0;
        idle.parent_pid = 0;
        idle.state = PROC_STATE_RUNNING;
        idle.priority = 255;
        idle.is_kernel = SIGMA_TRUE;
        idle.mem_allocated = 0;
        idle.cpu_time_us = 0;
        idle.start_tsc = cpu_rdtsc();
        sigma_strncpy(idle.name, "sigma-idle", SIGMA_PROC_NAME_LEN);

        /* Create PID 1: init process */
        createProcess("sigma-init", 0, SIGMA_TRUE);

        sigma_log("[PROCMGR] Sovereign Process Manager initialized.");
        sigma_log_info("[PROCMGR] PID 0 (idle) and PID 1 (init) created.\n");
    }

    sigma_u32 createProcess(const char* name, sigma_u8 priority, sigma_bool is_kernel) {
        if (m_count >= SIGMA_MAX_PROCESSES - 1) {
            sigma_log("[PROCMGR] ERROR: Process table full.");
            return SIGMA_PROC_INVALID_PID;
        }

        sigma_u32 pid = allocPid();
        if (pid == SIGMA_PROC_INVALID_PID) return pid;

        sigma_pcb_t& pcb = m_table[pid % SIGMA_MAX_PROCESSES];
        pcb.pid = pid;
        pcb.parent_pid = 1; /* default parent: init */
        pcb.state = PROC_STATE_CREATED;
        pcb.priority = priority;
        pcb.is_kernel = is_kernel;
        pcb.mem_allocated = PAGE_SIZE * 4; /* 16KB initial allocation */
        pcb.cpu_time_us = 0;
        pcb.start_tsc = cpu_rdtsc();
        pcb.stack_base = 0x00007FFF00000000ULL - (pid * PAGE_SIZE * 8);
        pcb.stack_size = PAGE_SIZE * 8; /* 32KB stack */
        pcb.page_table_root = 0;
        sigma_strncpy(pcb.name, name, SIGMA_PROC_NAME_LEN);

        m_count++;
        pcb.state = PROC_STATE_READY;

        sigma_log_info("[PROCMGR] Created PID %u '%s' (priority=%u, kernel=%d)\n",
                       pid, name, (unsigned)priority, (int)is_kernel);
        return pid;
    }

    int killProcess(sigma_u32 pid) {
        if (pid == 0) {
            sigma_log("[PROCMGR] ERROR: Cannot kill PID 0 (idle).");
            return K_ERR_PERM;
        }
        sigma_pcb_t* pcb = findPcb(pid);
        if (!pcb) return K_ERR_NOTFOUND;

        pcb->state = PROC_STATE_TERMINATED;
        m_count--;
        sigma_log_info("[PROCMGR] Killed PID %u '%s'.\n", pid, pcb->name);
        return K_OK;
    }

    int setState(sigma_u32 pid, sigma_proc_state_t new_state) {
        sigma_pcb_t* pcb = findPcb(pid);
        if (!pcb) return K_ERR_NOTFOUND;
        pcb->state = new_state;
        return K_OK;
    }

    int setPriority(sigma_u32 pid, sigma_u8 priority) {
        sigma_pcb_t* pcb = findPcb(pid);
        if (!pcb) return K_ERR_NOTFOUND;
        pcb->priority = priority;
        return K_OK;
    }

    const sigma_pcb_t* getInfo(sigma_u32 pid) {
        return findPcb(pid);
    }

    void listProcesses() {
        sigma_log("\n╔═══════════════════════════════════════════════════════════════╗");
        sigma_log("║              SOVEREIGN PROCESS TABLE                        ║");
        sigma_log("╠══════╦═══════════════════╦═══════════╦═══════╦══════════════╣");
        sigma_log("║  PID ║ Name              ║ State     ║ Prio  ║ Memory       ║");
        sigma_log("╠══════╬═══════════════════╬═══════════╬═══════╬══════════════╣");

        for (sigma_u32 i = 0; i < SIGMA_MAX_PROCESSES; i++) {
            if (m_table[i].pid != SIGMA_PROC_INVALID_PID &&
                m_table[i].state != PROC_STATE_TERMINATED) {
                const sigma_pcb_t& p = m_table[i];
                const char* state_str = "UNKNOWN";
                switch (p.state) {
                    case PROC_STATE_CREATED:    state_str = "CREATED";    break;
                    case PROC_STATE_READY:      state_str = "READY";      break;
                    case PROC_STATE_RUNNING:    state_str = "RUNNING";    break;
                    case PROC_STATE_BLOCKED:    state_str = "BLOCKED";    break;
                    case PROC_STATE_TERMINATED: state_str = "TERMINATED"; break;
                }
                sigma_log_info("║ %4u ║ %-17s ║ %-9s ║  %3u  ║ %8lluKB   ║\n",
                    p.pid, p.name, state_str, (unsigned)p.priority,
                    (unsigned long long)(p.mem_allocated / 1024));
            }
        }
        sigma_log("╚══════╩═══════════════════╩═══════════╩═══════╩══════════════╝");
        sigma_log_info("[PROCMGR] Total active processes: %u\n", m_count);
    }

    sigma_u32 forkProcess(sigma_u32 parent_pid) {
        sigma_pcb_t* parent = findPcb(parent_pid);
        if (!parent) return SIGMA_PROC_INVALID_PID;

        sigma_u32 child_pid = createProcess(parent->name, parent->priority, parent->is_kernel);
        if (child_pid == SIGMA_PROC_INVALID_PID) return child_pid;

        sigma_pcb_t* child = findPcb(child_pid);
        if (child) {
            child->parent_pid = parent_pid;
            child->mem_allocated = parent->mem_allocated;
            /* CoW: child shares parent page table until write */
            child->page_table_root = parent->page_table_root;
        }
        sigma_log_info("[PROCMGR] Forked PID %u → child PID %u\n", parent_pid, child_pid);
        return child_pid;
    }

    sigma_u32 getCount() const { return m_count; }

private:
    SovereignProcessManager() : m_count(0), m_next_pid(1) {}

    sigma_u32 allocPid() {
        sigma_u32 start = m_next_pid;
        do {
            sigma_u32 slot = m_next_pid % SIGMA_MAX_PROCESSES;
            if (m_table[slot].state == PROC_STATE_TERMINATED ||
                m_table[slot].pid == SIGMA_PROC_INVALID_PID) {
                sigma_u32 pid = m_next_pid++;
                return pid;
            }
            m_next_pid++;
        } while (m_next_pid != start);
        return SIGMA_PROC_INVALID_PID;
    }

    sigma_pcb_t* findPcb(sigma_u32 pid) {
        sigma_u32 slot = pid % SIGMA_MAX_PROCESSES;
        if (m_table[slot].pid == pid &&
            m_table[slot].state != PROC_STATE_TERMINATED) {
            return &m_table[slot];
        }
        return SIGMA_NULL;
    }

    sigma_pcb_t m_table[SIGMA_MAX_PROCESSES];
    sigma_u32 m_count;
    sigma_u32 m_next_pid;
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void process_manager_init(void) {
    SigmaOS::Kernel::SovereignProcessManager::getInstance().init();
}

sigma_u32 process_create(const char* name, sigma_u8 priority, sigma_bool is_kernel) {
    return SigmaOS::Kernel::SovereignProcessManager::getInstance()
               .createProcess(name, priority, is_kernel);
}

int process_kill(sigma_u32 pid) {
    return SigmaOS::Kernel::SovereignProcessManager::getInstance().killProcess(pid);
}

int process_set_state(sigma_u32 pid, sigma_proc_state_t new_state) {
    return SigmaOS::Kernel::SovereignProcessManager::getInstance().setState(pid, new_state);
}

int process_set_priority(sigma_u32 pid, sigma_u8 priority) {
    return SigmaOS::Kernel::SovereignProcessManager::getInstance().setPriority(pid, priority);
}

const sigma_pcb_t* process_getinfo(sigma_u32 pid) {
    return SigmaOS::Kernel::SovereignProcessManager::getInstance().getInfo(pid);
}

void process_list(void) {
    SigmaOS::Kernel::SovereignProcessManager::getInstance().listProcesses();
}

sigma_u32 process_get_count(void) {
    return SigmaOS::Kernel::SovereignProcessManager::getInstance().getCount();
}

sigma_u32 process_fork(sigma_u32 parent_pid) {
    return SigmaOS::Kernel::SovereignProcessManager::getInstance().forkProcess(parent_pid);
}

} // extern "C"

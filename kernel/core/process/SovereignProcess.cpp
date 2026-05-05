#include "../../../include/sigma_types.h""
#include "sigma_proc.h"
#include "../../../include/sigma_hal.h""
#include "../../../include/SovereignLibC.h""

/**
 * SigmaOS Sovereign Process Manager
 * Implements a Priority-Aware Task Switching (PATS) algorithm.
 * ZERO-DEPENDENCY: Strictly bare-metal process orchestration.
 *
 * Design: OOP-isolated singleton — SovereignProcessEngine.
 */

namespace SigmaOS {
namespace Kernel {
namespace Process {

class SovereignProcessEngine {
public:
    static SovereignProcessEngine& getInstance() {
        static SovereignProcessEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[PROC] Initializing Sovereign Scheduler (PATS Algorithm)...");
        // Spawn PID 0: Sovereign Genesis Task
        this->spawn("SovereignGenesis", 0u);
        this->m_initialized = 1u;
    }

    sigma_u32 spawn(const char* name, sigma_u32 priority) {
        if (this->m_active_count >= 64u) return 0u;
        
        sigma_process_t* proc = &this->m_table[this->m_active_count++];
        proc->pid = this->m_active_count;
        sigma_strncpy(proc->name, name, 32);
        proc->state = SIGMA_PROC_READY;
        proc->priority = priority;
        proc->cpu_time = 0u;
        proc->capability_mask = 0xFFFFFFFFu; // Full sovereignty by default
        
        sigma_printf("[PROC] Spawned Task: %s (PID: %u, Priority: %u)\n", name, (unsigned)proc->pid, (unsigned)priority);
        return proc->pid;
    }

    void yield() {
        /* PATS (Priority-Aware Task Switching) Algorithm */
        
        if (this->m_current_pid > 0u) {
            sigma_process_t* current = &this->m_table[this->m_current_pid - 1u];
            if (current->state == SIGMA_PROC_RUNNING) {
                current->cpu_time++;
                if (current->cpu_time > this->m_quantum_limit) {
                    sigma_printf("[PROC] [WATCHDOG] PID %u ('%s') exceeded quota. Deprioritizing.\n", 
                                 (unsigned)current->pid, current->name);
                    current->priority++;
                    current->cpu_time = 0u;
                }
                current->state = SIGMA_PROC_READY;
            }
        }

        sigma_process_t* next = (sigma_process_t*)SIGMA_NULL;
        sigma_u32 highest_priority = 0xFFFFFFFFu;
        
        uint32_t start_idx = this->m_current_pid; 
        for (uint32_t offset = 0u; offset < this->m_active_count; offset++) {
            uint32_t i = (start_idx + offset) % this->m_active_count;
            
            if (this->m_table[i].state == SIGMA_PROC_READY && 
                this->m_table[i].priority < highest_priority) {
                highest_priority = this->m_table[i].priority;
                next = &this->m_table[i];
            }
        }
        
        if (next) {
            sigma_printf("[PROC] PATS Context Switch: PID %u -> PID %u (%s)\n", 
                         (unsigned)this->m_current_pid, (unsigned)next->pid, next->name);
            this->m_current_pid = next->pid;
            next->state = SIGMA_PROC_RUNNING;
            this->m_switches_performed++;
        }
    }

    sigma_process_t* getCurrent() {
        if (this->m_current_pid == 0u) return (sigma_process_t*)SIGMA_NULL;
        return &this->m_table[this->m_current_pid - 1u];
    }

    sigma_u64 getSwitchCount() const { return this->m_switches_performed; }

private:
    SovereignProcessEngine() : m_active_count(0), m_current_pid(0), m_quantum_limit(1000), m_switches_performed(0), m_initialized(0) {}
    
    sigma_process_t m_table[64];
    sigma_u32 m_active_count;
    sigma_u32 m_current_pid;
    sigma_u32 m_quantum_limit;
    sigma_u64 m_switches_performed;
    sigma_u32 m_initialized;
};

} // namespace Process
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" void proc_init() {
    SigmaOS::Kernel::Process::SovereignProcessEngine::getInstance().init();
}

extern "C" sigma_u32 proc_spawn(const char* name, sigma_u32 priority) {
    return SigmaOS::Kernel::Process::SovereignProcessEngine::getInstance().spawn(name, priority);
}

extern "C" void proc_yield() {
    SigmaOS::Kernel::Process::SovereignProcessEngine::getInstance().yield();
}

extern "C" sigma_process_t* proc_get_current() {
    return SigmaOS::Kernel::Process::SovereignProcessEngine::getInstance().getCurrent();
}

extern "C" sigma_u64 proc_get_switch_count() {
    return SigmaOS::Kernel::Process::SovereignProcessEngine::getInstance().getSwitchCount();
}




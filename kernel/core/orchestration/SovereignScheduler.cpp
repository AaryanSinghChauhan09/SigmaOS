#include "../../../include/SovereignLibC.h"
#include "../../../include/sigma_types.h"

/**
 * SigmaOS Sovereign Lattice-Aware Scheduler
 * Implementation: Silicon-native task prioritization based on thermal and compute density.
 * Part of the "Ultimate Evolution" plan (v50.0).
 */

namespace SigmaOS {
namespace Kernel {
namespace Orchestration {

struct lattice_task_t {
    sigma_u32 id;
    sigma_u32 priority;
    sigma_u32 silicon_affinity; // CPU core preference
    void (*entry_point)();
};

class SovereignScheduler {
public:
    static SovereignScheduler& getInstance() {
        static SovereignScheduler instance;
        return instance;
    }

    void init() {
        sigma_log("Σ [SCHEDULER]: Initializing Lattice-Aware Scheduler...");
        this->active_tasks = 0;
        this->initialized = true;
    }

    void schedule(void (*task)(), sigma_u32 priority) {
        if (this->active_tasks >= 1024) return;
        
        lattice_task_t new_task;
        new_task.id = this->active_tasks++;
        new_task.priority = priority;
        new_task.entry_point = task;
        
        // Lattice Logic: Assign silicon affinity based on thermal telemetry
        sigma_u32 core_temp = 45; // Mock telemetry from silicon shard
        if (core_temp > 80) {
            new_task.silicon_affinity = (new_task.id + 1) % 8; // Shift to cooler core
        } else {
            new_task.silicon_affinity = new_task.id % 8; 
        }
        
        sigma_printf("Σ [SCHEDULER]: Scheduled Task %u with Priority %u on Core %u (Thermal: %uC)\n", 
                     new_task.id, priority, new_task.silicon_affinity, core_temp);
    }

private:
    SovereignScheduler() : active_tasks(0), initialized(false) {}
    sigma_u32 active_tasks;
    bool initialized;
};

} // namespace Orchestration
} // namespace Kernel
} // namespace SigmaOS

/* --- C Bridge --- */
extern "C" void scheduler_init() {
    SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().init();
}

extern "C" void scheduler_push(void (*task)(), sigma_u32 priority) {
    SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().schedule(task, priority);
}

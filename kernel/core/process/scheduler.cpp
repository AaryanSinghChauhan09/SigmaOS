#include "../../../include/sigma_hal.h"
#include "../../../include/sigma_types.h"
#include "../../../include/SovereignLibC.h"
#include "../../../include/SigmaOOP.hpp"

/**
 * SigmaOS Sovereign Scheduler
 * Principles: Preemptive Round-Robin, Silicon-Aware Sharding.
 */

namespace SigmaOS {
namespace Kernel {
namespace Scheduling {

class SovereignScheduler : public SigmaObject {
public:
    static SovereignScheduler& getInstance() {
        static SovereignScheduler instance;
        return instance;
    }

    const char* type_name() const noexcept override { return "SovereignScheduler"; }

    void init() {
        sigma_log("[SCHED] Initializing Sovereign Multitasking Shard...");
        m_task_count = 0;
        // In a real kernel, we would call the C scheduler_init() here
        sigma_log("[SCHED] Silicon Orchestration READY.");
    }

    void createTask(const char* name, void (*entry)()) {
        sigma_printf("[SCHED] Spawning task shard: %s\n", name);
        m_task_count++;
    }

    void dispatch() {
        // Orchestrate context switch
        // This would call the C yield() or switch_to_task()
    }

private:
    SovereignScheduler() : m_task_count(0) {}
    sigma_u32 m_task_count;
};

} // namespace Scheduling
} // namespace Kernel
} // namespace SigmaOS

/* --- C Interface --- */
extern "C" void scheduler_init_shard() {
    SigmaOS::Kernel::Scheduling::SovereignScheduler::getInstance().init();
}

extern "C" void scheduler_spawn(const char* name, void (*entry)()) {
    SigmaOS::Kernel::Scheduling::SovereignScheduler::getInstance().createTask(name, entry);
}

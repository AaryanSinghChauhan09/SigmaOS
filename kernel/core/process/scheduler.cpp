#include "../../../include/sigma_log.h"
#include "../../../include/sigma_hal.h"
#include "../../../include/sigma_kernel_types.h"
#include "../../../include/libc/SovereignLibC.h"
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

    static void init() {
        sigma_log("[SCHED] Initializing Sovereign Multitasking Shard...");
        m_tasks = new SigmaVector<SovereignTask*>();
        sigma_log("[SCHED] Silicon Orchestration READY.");
    }

    void createTask(const char* name, void (*entry)()) {
        sigma_log("[SCHED] Spawning task shard: %s\n", name);
        SovereignTask* task = new SovereignTask(m_tasks->size(), name, entry);
        m_tasks->push_back(task);
    }

    void dispatch() {
        if (m_tasks->size() == 0) return;
        
        static sigma_u32 current_idx = 0;
        current_idx = (current_idx + 1) % m_tasks->size();
        
        SovereignTask* task = (*m_tasks)[current_idx];
        sigma_log("[SCHED] Silicon Dispatch -> %s [0x%p]\n", task->name, task->entry);
        
        // In a real kernel, this would call the ASM switch_to_task
    }

    void audit() {
        sigma_log("\n--- S SOVEREIGN SCHEDULER AUDIT ---\n");
        sigma_log("| Active Shards : %d\n", m_tasks->size());
        for (sigma_u32 i = 0; i < m_tasks->size(); i++) {
            sigma_log("| [%d] %-15s | Entry: %p\n", i, (*m_tasks)[i]->name, (*m_tasks)[i]->entry);
        }
    }

private:
    struct SovereignTask {
        sigma_u32 id;
        const char* name;
        void (*entry)();
        SovereignTask(sigma_u32 i, const char* n, void (*e)()) : id(i), name(n), entry(e) {}
    };

    SovereignScheduler() : m_tasks(SIGMA_NULL) {}
    SigmaVector<SovereignTask*>* m_tasks;
};

} // namespace Scheduling
} // namespace Kernel
} // namespace SigmaOS

extern "C" {

/* --- C Interface --- */
void scheduler_init_shard() {
    SigmaOS::Kernel::Scheduling::SovereignScheduler::init();
}

void scheduler_spawn(const char* name, void (*entry)()) {
    SigmaOS::Kernel::Scheduling::SovereignScheduler::createTask(name, entry);
}





} // extern "C"

} // extern "C"
 
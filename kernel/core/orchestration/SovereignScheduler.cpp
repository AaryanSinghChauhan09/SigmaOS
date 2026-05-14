#include "sigma_log.h"
#include "core/sigma_types.h"
#include "hal/sigma_hal.h"
#include "libc/SovereignLibC.h"
#include "system/sigma_scheduler.h"

namespace SigmaOS {
namespace Kernel {
namespace Orchestration {

struct SovereignTask {
    sigma_u32 id;
    sigma_u64 vruntime; // Virtual Runtime for CFS fairness
    sigma_u32 priority;
    bool active;
};

class SovereignScheduler {
public:
    static SovereignScheduler& getInstance();
    void init();
    void schedule(void (*task)(), sigma_u32 priority);

private:
    SovereignScheduler() : m_task_count(0), m_min_vruntime(0) {}
    SovereignTask m_tasks[1024];
    sigma_u32 m_task_count;
    sigma_u64 m_min_vruntime;
};

SovereignScheduler& SovereignScheduler::getInstance() {
    static SovereignScheduler instance;
    return instance;
}

void SovereignScheduler::init() {
    sigma_log_info("[S-SCHED] Initializing Completely Fair Scheduler (S-CFS)...");
    m_task_count = 0;
    m_min_vruntime = 0;
}

void SovereignScheduler::schedule(void (*task)(), sigma_u32 priority) {
    if (m_task_count >= 1024) return;

    // 1. Task Registration with Fair Initial vruntime
    SovereignTask& t = m_tasks[m_task_count++];
    t.id = m_task_count;
    t.priority = priority;
    t.active = true;
    
    // Weighted vruntime based on priority (lower priority = higher runtime penalty)
    sigma_u64 weight = (100 - priority); 
    t.vruntime = m_min_vruntime + weight;

    sigma_log_info("[S-SCHED] Task %u Registered | vruntime: %llu | Priority: %u", 
                   t.id, t.vruntime, priority);

    // 2. CFS Selection (Find task with minimal vruntime)
    sigma_u32 best_task = 0;
    sigma_u64 min_v = 0xFFFFFFFFFFFFFFFF;
    
    for(sigma_u32 i = 0; i < m_task_count; i++) {
        if (m_tasks[i].active && m_tasks[i].vruntime < min_v) {
            min_v = m_tasks[i].vruntime;
            best_task = i;
        }
    }

    m_min_vruntime = min_v;
    sigma_log_info("[S-SCHED] Dispatching Task %u (Lowest vruntime: %llu)", 
                   m_tasks[best_task].id, m_tasks[best_task].vruntime);
    
    // Simulate runtime increment
    m_tasks[best_task].vruntime += 10; 
}

} // namespace Orchestration
} // namespace Kernel
} // namespace SigmaOS

extern "C" {
    void scheduler_init() { SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().init(); }
    void scheduler_push(void (*task)(), sigma_u32 priority) { 
        SigmaOS::Kernel::Orchestration::SovereignScheduler::getInstance().schedule(task, priority); 
    }
}

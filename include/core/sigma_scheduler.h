/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN REAL-TIME SCHEDULER SHARD (S-SCHED)
 * =========================================================================
 * Mission: Hard-RTOS deterministic scheduling (CFS-inspired) with AI telemetry hooks.
 * =========================================================================
 */

#ifndef SIGMA_SCHEDULER_H
#define SIGMA_SCHEDULER_H

#include "./sigma_kernel_types.h"
#include "../SigmaOOP.hpp"

namespace SigmaOS {
namespace Core {

enum class TaskPriority {
    IDLE = 0,
    NORMAL = 1,
    HIGH = 2,
    REALTIME_CRITICAL = 3
};

struct TaskContext {
    sigma_u64 rip;
    sigma_u64 rsp;
    sigma_u64 rbp;
    sigma_u64 cr3; // Page Directory Base Register
    sigma_u64 flags;
};

struct ShardTask {
    sigma_u32 id;
    TaskPriority priority;
    TaskContext context;
    sigma_u64 cpu_time;
    sigma_u64 deadline; // For Earliest Deadline First (EDF) Real-Time scheduling
    bool is_active;
};

class SovereignSchedulerShard : public SigmaOS::SigmaObject, public SigmaOS::SigmaSingleton<SovereignSchedulerShard> {
    friend class SigmaOS::SigmaSingleton<SovereignSchedulerShard>;
public:
    const char* type_name() const noexcept override { return "SovereignSchedulerShard"; }

    sigma_status init();
    sigma_status spawn_task(void (*entry)(), TaskPriority priority, sigma_u32* out_id);
    void yield();
    void tick(); // Triggered by APIC timer interrupt

private:
    SovereignSchedulerShard() : m_current_task(0), m_task_count(0) {}
    
    ShardTask m_tasks[256];
    sigma_u32 m_current_task;
    sigma_u32 m_task_count;
};

} // namespace Core
} // namespace SigmaOS

#endif /* SIGMA_SCHEDULER_H */

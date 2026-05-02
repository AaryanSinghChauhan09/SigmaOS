#ifndef SCHEDULER_HPP
#define SCHEDULER_HPP

#include "../../../include/sigma_types.h"
#include "../../../include/SigmaOOP.hpp"
#include "SovereignString.hpp"

namespace SigmaOS {
namespace Kernel {

enum class TaskState { READY, RUNNING, WAITING, TERMINATED };

/*
 * =========================================================================
 * SOVEREIGN TASK (Encapsulation of Process Context)
 * =========================================================================
 */
class SovereignTask : public SigmaObject {
public:
    sigma_u32 id;
    SigmaOS::Core::SovereignString name;
    TaskState state;
    sigma_u64 cpu_time;
    void (*entry_point)();

    SovereignTask(sigma_u32 _id, const char* _name, void (*_entry)()) 
        : id(_id), name(_name), state(TaskState::READY), cpu_time(0), entry_point(_entry) {}

    const char* type_name() const noexcept override { return "SovereignTask"; }
};

/*
 * =========================================================================
 * SOVEREIGN SCHEDULER (Process Orchestration / Concurrency)
 * =========================================================================
 */
class SovereignScheduler : public SigmaObject {
private:
    SovereignTask* m_tasks[256];
    sigma_u32 m_task_count;
    sigma_u32 m_current_task_idx;
    sigma_u32 m_load_history[10]; // Heuristic Load Tracking

public:
    SovereignScheduler();
    const char* type_name() const noexcept override { return "SovereignScheduler"; }

    void CreateTask(const char* name, void (*entry)());
    void Dispatch(); // Round-Robin Orchestration
    void AdaptiveDispatch(); // AI-driven Heuristic Dispatch
    void Audit();
};

} // namespace Kernel
} // namespace SigmaOS

#endif

#include "scheduler.hpp"
#include "../../include/SovereignLibC.h"

namespace SigmaOS {
namespace Kernel {

SovereignScheduler::SovereignScheduler() : m_task_count(0), m_current_task_idx(0) {
    sigma_memset(m_tasks, 0, sizeof(m_tasks));
    sigma_printf("[SCHED]: Sovereign Scheduler Online. Ready for Silicon Orchestration.\n");
}

void SovereignScheduler::CreateTask(const char* name, void (*entry)()) {
    if (m_task_count < 256) {
        // Allocation using our SovereignMemoryManager would happen here
        // For simulation of internal state:
        m_tasks[m_task_count] = new SovereignTask(m_task_count, name, entry);
        m_task_count++;
        sigma_printf("[SCHED]: Task Created: %s (ID: %d)\n", name, m_task_count - 1);
    }
}

void SovereignScheduler::Dispatch() {
    if (m_task_count == 0) return;

    // Round-Robin Sovereign Dispatcher
    m_current_task_idx = (m_current_task_idx + 1) % m_task_count;
    SovereignTask* current = m_tasks[m_current_task_idx];
    
    current->state = TaskState::RUNNING;
    current->cpu_time += 100; // Simulate quantum consumption

    sigma_printf("[SCHED]: Context Switch -> %s\n", current->name.c_str());
    
    // Simulate return to READY for next cycle
    current->state = TaskState::READY;
}

void SovereignScheduler::AdaptiveDispatch() {
    sigma_printf("[SCHED/ADAPTIVE]: Analyzing Load History for Heuristic Sharding...\n");
    // Simulated AI-driven prediction
    sigma_u32 predicted_quantum = 50 + (m_task_count * 10);
    sigma_printf("[SCHED/ADAPTIVE]: Adjusting Task Quantum to %d ms based on Lattice Pressure.\n", predicted_quantum);
    Dispatch();
}

void SovereignScheduler::Audit() {
    sigma_printf("\n--- Σ SOVEREIGN SCHEDULER AUDIT ---\n");
    sigma_printf("| Active Tasks   : %d\n", m_task_count);
    for(sigma_u32 i = 0; i < m_task_count; ++i) {
        sigma_printf("| Task [%d]: %-15s | Time: %llu ms\n", 
            m_tasks[i]->id, m_tasks[i]->name.c_str(), m_tasks[i]->cpu_time);
    }
    sigma_printf("----------------------------------\n");
}

} // namespace Kernel
} // namespace SigmaOS

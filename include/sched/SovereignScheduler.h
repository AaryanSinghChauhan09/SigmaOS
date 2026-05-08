/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SCHEDULER (v1.0 - AI-NATIVE CFS EVOLUTION)
 * =========================================================================
 * Inspired by: Linux CFS (Completely Fair Scheduler) + seL4 MCS scheduler
 * Purpose: Priority-aware, cgroup-like task scheduling for kernel shards.
 * Design:  Fixed-capacity run queue, virtual runtime fair scheduling,
 *          zero dynamic allocation, AI-hint integration via KernelBridge.
 * =========================================================================
 */

#pragma once

#include "sigma_types.h"
#include "sigma_log.h"

#define SIGMA_SCHED_MAX_TASKS  256u
#define SIGMA_SCHED_TICK_MS    4u      /* 4ms scheduling quantum */

namespace SigmaOS {
namespace Kernel {
namespace Scheduler {

/* ─── Task Priority Levels (inspired by Linux nice values) ─────────────── */
enum class TaskPriority : sigma_u32 {
    REALTIME   = 0u,   /* RT tasks — AI inference, audio, security */
    HIGH       = 1u,   /* Interactive tasks — UI, input events */
    NORMAL     = 2u,   /* Standard workloads */
    BATCH      = 3u,   /* Background indexing, compression */
    IDLE       = 4u,   /* Run only when CPU is idle */
};

/* ─── Task States ───────────────────────────────────────────────────────── */
enum class TaskState : sigma_u32 {
    RUNNABLE  = 0u,
    RUNNING   = 1u,
    BLOCKED   = 2u,
    SLEEPING  = 3u,
    ZOMBIE    = 4u,
};

/* ─── Task Control Block ────────────────────────────────────────────────── */
struct SovereignTask {
    sigma_u32     pid;
    TaskPriority  priority;
    TaskState     state;
    sigma_u64     vruntime_ns;     /* Virtual runtime (CFS-style fairness) */
    sigma_u64     deadline_ns;     /* EDF deadline for RT tasks */
    sigma_u64     slice_remaining_ms;
    const char*   name;
    sigma_u32     shard_id;        /* Owning shard for MAC enforcement */
    sigma_u32     valid;
};

/**
 * @brief SovereignScheduler — CFS-inspired fair scheduler with AI-hint
 *        integration. Absorbs ideas from Linux CFS, seL4 MCS, and
 *        the Zircon (Fuchsia) scheduler's deadline-based RT path.
 */
class SovereignScheduler {
public:
    static SovereignScheduler& getInstance() {
        static SovereignScheduler instance;
        return instance;
    }

    SovereignScheduler(const SovereignScheduler&)            = delete;
    SovereignScheduler& operator=(const SovereignScheduler&) = delete;

    /**
     * @brief Enqueue a task into the run queue.
     */
    bool enqueue(const SovereignTask& task) {
        if (m_task_count >= SIGMA_SCHED_MAX_TASKS) {
            sigma_log_warn("[SCHED] Run queue full — task rejected.");
            return false;
        }
        m_tasks[m_task_count] = task;
        m_tasks[m_task_count].valid = 1u;
        m_task_count++;
        return true;
    }

    /**
     * @brief Select the next task to run (CFS: lowest vruntime among RUNNABLE).
     * RT tasks (priority == REALTIME) always preempt normal tasks.
     */
    SovereignTask* pickNext() {
        SovereignTask* best = nullptr;

        /* 1. Check for RT task (EDF — earliest deadline first) */
        for (sigma_u32 i = 0u; i < m_task_count; i++) {
            auto& t = m_tasks[i];
            if (!t.valid || t.state != TaskState::RUNNABLE) continue;
            if (t.priority != TaskPriority::REALTIME) continue;
            if (!best || t.deadline_ns < best->deadline_ns) {
                best = &t;
            }
        }
        if (best) return best;

        /* 2. CFS: pick the RUNNABLE task with lowest vruntime */
        for (sigma_u32 i = 0u; i < m_task_count; i++) {
            auto& t = m_tasks[i];
            if (!t.valid || t.state != TaskState::RUNNABLE) continue;
            if (!best || t.vruntime_ns < best->vruntime_ns) {
                best = &t;
            }
        }
        return best;
    }

    /**
     * @brief Account for time spent; update vruntime and yield if slice expires.
     */
    void tick(sigma_u64 elapsed_ms) {
        if (!m_current) return;
        m_current->slice_remaining_ms = (elapsed_ms < m_current->slice_remaining_ms)
            ? m_current->slice_remaining_ms - elapsed_ms : 0u;

        /* Weight vruntime by inverse priority (lower priority = faster aging) */
        sigma_u64 weight = 1u + static_cast<sigma_u32>(m_current->priority);
        m_current->vruntime_ns += elapsed_ms * 1000000ULL * weight;

        if (m_current->slice_remaining_ms == 0u) {
            sigma_log_info("[SCHED] Timeslice expired — preempting task.");
            m_current->state = TaskState::RUNNABLE;
            m_current = pickNext();
            if (m_current) {
                m_current->state = TaskState::RUNNING;
                m_current->slice_remaining_ms = SIGMA_SCHED_TICK_MS;
            }
        }
    }

    /**
     * @brief Receive an AI-generated scheduling hint.
     * Inspired by Google's ghOSt OS-as-a-service scheduler.
     */
    void applyAIHint(sigma_u32 pid, TaskPriority suggested_priority) {
        for (sigma_u32 i = 0u; i < m_task_count; i++) {
            if (m_tasks[i].valid && m_tasks[i].pid == pid) {
                m_tasks[i].priority = suggested_priority;
                sigma_log_info("[SCHED] AI scheduling hint applied.");
                return;
            }
        }
    }

    /**
     * @brief Block a task (e.g. waiting on IPC or I/O).
     */
    void blockTask(sigma_u32 pid) {
        setTaskState(pid, TaskState::BLOCKED);
    }

    /**
     * @brief Wake a blocked task.
     */
    void wakeTask(sigma_u32 pid) {
        setTaskState(pid, TaskState::RUNNABLE);
    }

    sigma_u32 taskCount() const { return m_task_count; }

private:
    SovereignScheduler() : m_task_count(0u), m_current(nullptr) {}

    void setTaskState(sigma_u32 pid, TaskState state) {
        for (sigma_u32 i = 0u; i < m_task_count; i++) {
            if (m_tasks[i].valid && m_tasks[i].pid == pid) {
                m_tasks[i].state = state;
                return;
            }
        }
    }

    SovereignTask  m_tasks[SIGMA_SCHED_MAX_TASKS];
    sigma_u32      m_task_count;
    SovereignTask* m_current;
};

} // namespace Scheduler
} // namespace Kernel
} // namespace SigmaOS

/* ─── C Bridge ─────────────────────────────────────────────────────────── */
extern "C" {

inline void sigma_sched_tick(unsigned long long elapsed_ms) {
    SigmaOS::Kernel::Scheduler::SovereignScheduler::getInstance().tick(elapsed_ms);
}

inline void sigma_sched_block(unsigned int pid) {
    SigmaOS::Kernel::Scheduler::SovereignScheduler::getInstance().blockTask(pid);
}

inline void sigma_sched_wake(unsigned int pid) {
    SigmaOS::Kernel::Scheduler::SovereignScheduler::getInstance().wakeTask(pid);
}

} // extern "C"

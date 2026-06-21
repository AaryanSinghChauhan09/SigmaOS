/**
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN TASK SCHEDULER (v1.0)
 * =========================================================================
 * MLFQ (Multi-Level Feedback Queue) + EDF (Earliest Deadline First) hybrid.
 * Per-CPU run queues, context switch tracking, and starvation prevention.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_scheduler.h"

namespace SigmaOS {
namespace Kernel {

struct sigma_spinlock_t {
    volatile int lock_state;
    void init() { lock_state = 0; }
    void acquire() {
        while (__sync_lock_test_and_set(&lock_state, 1)) {
            __asm__ volatile("pause");
        }
    }
    void release() {
        __sync_lock_release(&lock_state);
    }
};

class SovereignScheduler {
public:
    static SovereignScheduler& getInstance() {
        static SovereignScheduler instance;
        return instance;
    }

    void init(sigma_u32 num_cpus) {
        m_lock.init();
        m_num_cpus = (num_cpus > SCHED_MAX_CPUS) ? SCHED_MAX_CPUS : num_cpus;
        m_task_count = 0;
        m_total_switches = 0;
        m_tick_count = 0;

        for (sigma_u32 i = 0; i < SCHED_MAX_TASKS; i++) {
            m_tasks[i].tid = 0;
            m_tasks[i].state = TASK_STATE_TERMINATED;
        }

        for (sigma_u32 i = 0; i < m_num_cpus; i++) {
            m_cpu_states[i].cpu_id = i;
            m_cpu_states[i].current_tid = 0;
            m_cpu_states[i].idle_time_us = 0;
            m_cpu_states[i].total_context_switches = 0;
            m_cpu_states[i].last_tick_tsc = cpu_rdtsc();
        }

        sigma_log("[SCHED] Sovereign Scheduler (MLFQ+EDF) initialized.");
        sigma_log_info("[SCHED] Active CPUs: %u | Quantum: %u μs\n",
                       m_num_cpus, SCHED_BASE_QUANTUM_US);
    }

    sigma_u32 addTask(sigma_u32 pid, sigma_sched_policy_t policy,
                      sigma_u8 priority, sigma_u64 deadline_us) {
        m_lock.acquire();
        if (m_task_count >= SCHED_MAX_TASKS) {
            m_lock.release();
            return 0;
        }

        /* Find empty slot */
        sigma_u32 tid = 0;
        for (sigma_u32 i = 0; i < SCHED_MAX_TASKS; i++) {
            if (m_tasks[i].state == TASK_STATE_TERMINATED) {
                tid = i + 1;
                break;
            }
        }
        if (tid == 0) {
            m_lock.release();
            return 0;
        }

        sigma_task_t& t = m_tasks[tid - 1];
        t.tid = tid;
        t.pid = pid;
        t.state = TASK_STATE_READY;
        t.policy = policy;
        t.mlfq_level = 0; /* start at highest queue */
        t.base_priority = priority;
        t.deadline_us = deadline_us;
        t.period_us = 0;
        t.time_slice_us = getQuantumForLevel(0);
        t.total_cpu_us = 0;
        t.cpu_affinity = 0xFFFFFFFF; /* Run on any CPU */
        t.last_run_tsc = 0;
        t.wake_time_us = 0;
        t.stack_ptr = 0;

        m_task_count++;
        sigma_log_info("[SCHED] Task added: TID %u (PID %u) Policy: %u\n",
                       tid, pid, (unsigned)policy);
        m_lock.release();
        return tid;
    }

    int removeTask(sigma_u32 tid) {
        m_lock.acquire();
        sigma_task_t* t = findTask(tid);
        if (!t) {
            m_lock.release();
            return K_ERR_NOTFOUND;
        }

        t->state = TASK_STATE_TERMINATED;
        m_task_count--;
        sigma_log_info("[SCHED] Task removed: TID %u\n", tid);
        m_lock.release();
        return K_OK;
    }

    void tick(sigma_u32 cpu_id) {
        if (cpu_id >= m_num_cpus) return;
        sigma_cpu_state_t& cpu = m_cpu_states[cpu_id];
        m_tick_count++;

        sigma_u64 now_tsc = cpu_rdtsc();
        sigma_u64 elapsed_us = (now_tsc - cpu.last_tick_tsc) / 3000; /* rough 3GHz calc */
        cpu.last_tick_tsc = now_tsc;

        /* Update sleeping tasks with safety lock */
        m_lock.acquire();
        for (sigma_u32 i = 0; i < SCHED_MAX_TASKS; i++) {
            if (m_tasks[i].state == TASK_STATE_SLEEPING) {
                if (m_tick_count * 1000 >= m_tasks[i].wake_time_us) {
                    m_tasks[i].state = TASK_STATE_READY;
                }
            }
        }
        m_lock.release();

        /* Update current task status */
        bool need_yield = false;
        m_lock.acquire();
        if (cpu.current_tid != 0) {
            sigma_task_t* curr = findTask(cpu.current_tid);
            if (curr && curr->state == TASK_STATE_RUNNING) {
                curr->total_cpu_us += elapsed_us;
                if (curr->time_slice_us > elapsed_us) {
                    curr->time_slice_us -= elapsed_us;
                } else {
                    curr->time_slice_us = 0;
                    /* Time slice exhausted: demote in MLFQ */
                    if (curr->policy == SCHED_POLICY_MLFQ && curr->mlfq_level < SCHED_MLFQ_LEVELS - 1) {
                        curr->mlfq_level++;
                    }
                    curr->state = TASK_STATE_READY;
                    need_yield = true;
                }
            } else {
                cpu.current_tid = 0;
            }
        } else {
            cpu.idle_time_us += elapsed_us;
            /* Attempt to pick a new task if idle */
            need_yield = true;
        }
        m_lock.release();

        if (need_yield) {
            yield(cpu_id);
        }

        /* Priority Boost: Starvation prevention every ~1 second */
        if (m_tick_count % 1000 == 0) {
            priorityBoost();
        }
    }

    void yield(sigma_u32 cpu_id) {
        if (cpu_id >= m_num_cpus) return;
        m_lock.acquire();
        sigma_cpu_state_t& cpu = m_cpu_states[cpu_id];

        /* Put current task back in ready queue if it was running */
        if (cpu.current_tid != 0) {
            sigma_task_t* curr = findTask(cpu.current_tid);
            if (curr && curr->state == TASK_STATE_RUNNING) {
                curr->state = TASK_STATE_READY;
            }
        }

        /* Scheduler Algorithm: Pick next task */
        sigma_u32 next_tid = pickNextTask(cpu_id);

        if (next_tid != 0) {
            sigma_task_t* next = findTask(next_tid);
            next->state = TASK_STATE_RUNNING;
            if (next->time_slice_us == 0) {
                next->time_slice_us = getQuantumForLevel(next->mlfq_level);
            }
            next->last_run_tsc = cpu_rdtsc();
            
            if (cpu.current_tid != next_tid) {
                cpu.total_context_switches++;
                m_total_switches++;
                cpu.current_tid = next_tid;

                /* Enforce process isolation: Reload page directory (CR3 register) on context switch */
                #if defined(__x86_64__)
                sigma_u64 pml4_phys = 0x100000ULL + (sigma_u64)(next->pid - 1) * 4096;
                __asm__ volatile("mov %0, %%cr3" : : "r"(pml4_phys) : "memory");
                #endif
            }
        } else {
            cpu.current_tid = 0; /* Idle */
        }
        m_lock.release();
    }

    void yieldGlobal() {
        /* Used by tasks voluntarily yielding (calls yield on CPU 0 for now) */
        yield(0);
    }

    void priorityBoost() {
        m_lock.acquire();
        priorityBoostLocked();
        m_lock.release();
    }

    void priorityBoostLocked() {
        /* Move all MLFQ tasks back to top queue to prevent starvation */
        for (sigma_u32 i = 0; i < SCHED_MAX_TASKS; i++) {
            if (m_tasks[i].state != TASK_STATE_TERMINATED && m_tasks[i].policy == SCHED_POLICY_MLFQ) {
                m_tasks[i].mlfq_level = 0;
                m_tasks[i].time_slice_us = getQuantumForLevel(0);
            }
        }
        sigma_log("[SCHED] Global priority boost triggered.");
    }

    void printQueues() {
        sigma_log("\n--- SCHEDULER QUEUES ---");
        for (sigma_u32 lvl = 0; lvl < SCHED_MLFQ_LEVELS; lvl++) {
            int count = 0;
            for (sigma_u32 i = 0; i < SCHED_MAX_TASKS; i++) {
                if (m_tasks[i].state == TASK_STATE_READY && 
                    m_tasks[i].policy == SCHED_POLICY_MLFQ &&
                    m_tasks[i].mlfq_level == lvl) count++;
            }
            if (count > 0) {
                sigma_log_info("MLFQ Level %u (quantum %u μs): %d tasks ready\n",
                               lvl, getQuantumForLevel(lvl), count);
            }
        }
        
        int edf_count = 0;
        for (sigma_u32 i = 0; i < SCHED_MAX_TASKS; i++) {
            if (m_tasks[i].state == TASK_STATE_READY && m_tasks[i].policy == SCHED_POLICY_EDF) {
                edf_count++;
            }
        }
        if (edf_count > 0) {
            sigma_log_info("EDF Queue: %d real-time tasks ready\n", edf_count);
        }
        sigma_log("------------------------");
    }

    void printCpuStats() {
        sigma_log("\n--- CPU SCHEDULER STATS ---");
        for (sigma_u32 i = 0; i < m_num_cpus; i++) {
            sigma_cpu_state_t& c = m_cpu_states[i];
            sigma_log_info("CPU %u: Curr TID: %u | Switches: %llu | Idle time: %llu μs\n",
                           i, c.current_tid,
                           (unsigned long long)c.total_context_switches,
                           (unsigned long long)c.idle_time_us);
        }
        sigma_log_info("Total System Switches: %llu\n", (unsigned long long)m_total_switches);
        sigma_log("---------------------------");
    }

    sigma_u32 getTaskCount() const { return m_task_count; }
    sigma_u64 getTotalSwitches() const { return m_total_switches; }
    sigma_u32 getCurrentTid(sigma_u32 cpu_id) {
        if (cpu_id >= m_num_cpus) return 0;
        return m_cpu_states[cpu_id].current_tid;
    }

private:
    SovereignScheduler() : m_num_cpus(1), m_task_count(0), m_total_switches(0), m_tick_count(0) {}

    sigma_task_t* findTask(sigma_u32 tid) {
        if (tid == 0 || tid > SCHED_MAX_TASKS) return SIGMA_NULL;
        sigma_task_t& t = m_tasks[tid - 1];
        return (t.tid == tid) ? &t : SIGMA_NULL;
    }

    sigma_u32 getQuantumForLevel(sigma_u8 level) {
        /* 4ms, 8ms, 16ms, 32ms... */
        return SCHED_BASE_QUANTUM_US * (1 << level);
    }

    sigma_u32 pickNextTask(sigma_u32 cpu_id) {
        sigma_u32 cpu_mask = 1 << cpu_id;

        /* 1. Check EDF (Earliest Deadline First) queue for real-time tasks */
        sigma_u32 best_edf_tid = 0;
        sigma_u64 earliest_deadline = 0xFFFFFFFFFFFFFFFFULL;
        for (sigma_u32 i = 0; i < SCHED_MAX_TASKS; i++) {
            sigma_task_t& t = m_tasks[i];
            if (t.state == TASK_STATE_READY && t.policy == SCHED_POLICY_EDF && (t.cpu_affinity & cpu_mask)) {
                if (t.deadline_us < earliest_deadline) {
                    earliest_deadline = t.deadline_us;
                    best_edf_tid = t.tid;
                }
            }
        }
        if (best_edf_tid != 0) return best_edf_tid;

        /* 2. Check MLFQ queues (highest priority = 0) */
        for (sigma_u32 lvl = 0; lvl < SCHED_MLFQ_LEVELS; lvl++) {
            for (sigma_u32 i = 0; i < SCHED_MAX_TASKS; i++) {
                sigma_task_t& t = m_tasks[i];
                if (t.state == TASK_STATE_READY && t.policy == SCHED_POLICY_MLFQ && 
                    t.mlfq_level == lvl && (t.cpu_affinity & cpu_mask)) {
                    /* Basic Round-Robin within queue */
                    return t.tid;
                }
            }
        }
        
        return 0; /* Nothing to run (idle) */
    }

    sigma_task_t      m_tasks[SCHED_MAX_TASKS];
    sigma_cpu_state_t m_cpu_states[SCHED_MAX_CPUS];
    sigma_spinlock_t  m_lock;
    sigma_u32         m_num_cpus;
    sigma_u32         m_task_count;
    sigma_u64         m_total_switches;
    sigma_u64         m_tick_count;
};

} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" {

void sched_init(sigma_u32 num_cpus) { SigmaOS::Kernel::SovereignScheduler::getInstance().init(num_cpus); }

sigma_u32 sched_add_task(sigma_u32 pid, sigma_sched_policy_t policy,
                         sigma_u8 priority, sigma_u64 deadline_us) {
    return SigmaOS::Kernel::SovereignScheduler::getInstance().addTask(pid, policy, priority, deadline_us);
}

int sched_remove_task(sigma_u32 tid) {
    return SigmaOS::Kernel::SovereignScheduler::getInstance().removeTask(tid);
}

void sched_tick(sigma_u32 cpu_id) {
    SigmaOS::Kernel::SovereignScheduler::getInstance().tick(cpu_id);
}

void sched_yield(void) {
    SigmaOS::Kernel::SovereignScheduler::getInstance().yieldGlobal();
}

sigma_u32 sched_get_current(sigma_u32 cpu_id) {
    return SigmaOS::Kernel::SovereignScheduler::getInstance().getCurrentTid(cpu_id);
}

void sched_priority_boost(void) {
    SigmaOS::Kernel::SovereignScheduler::getInstance().priorityBoost();
}

void sched_print_queues(void) {
    SigmaOS::Kernel::SovereignScheduler::getInstance().printQueues();
}

void sched_print_cpu_stats(void) {
    SigmaOS::Kernel::SovereignScheduler::getInstance().printCpuStats();
}

sigma_u32 sched_get_task_count(void) {
    return SigmaOS::Kernel::SovereignScheduler::getInstance().getTaskCount();
}

sigma_u64 sched_get_total_switches(void) {
    return SigmaOS::Kernel::SovereignScheduler::getInstance().getTotalSwitches();
}

} // extern "C"
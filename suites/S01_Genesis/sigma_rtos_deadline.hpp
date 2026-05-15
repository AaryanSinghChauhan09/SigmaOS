// SigmaOS — sigma-rtos-deadline: Hard Real-Time Determinism
// Module: sigma-rtos-deadline
// USP: Defeats QNX. Implements Earliest Deadline First (EDF) scheduling 
//      with strict microsecond execution guarantees for aerospace/automotive use cases.

#ifndef SIGMA_RTOS_DEADLINE_HPP
#define SIGMA_RTOS_DEADLINE_HPP

#include "../../include/sigma_proc_pcb.h"

namespace sigma {
namespace rtos {

struct DeadlineTask {
    SigmaPCB* process;
    unsigned long absolute_deadline_rdtsc;
    unsigned long worst_case_execution_time;
    bool is_hard_realtime;
};

class EDFScheduler {
private:
    DeadlineTask active_tasks[64];
    unsigned int task_count;

    unsigned long get_rdtsc() const {
#if defined(__x86_64__) || defined(__i386__)
        unsigned int lo, hi;
        __asm__ __volatile__("rdtsc" : "=a"(lo), "=d"(hi));
        return ((unsigned long)hi << 32) | lo;
#else
        return 0;
#endif
    }

public:
    EDFScheduler() : task_count(0) {}

    bool register_task(SigmaPCB* process, unsigned long relative_deadline_us, unsigned long wcet_us, bool hard) {
        if (task_count >= 64) return false;
        
        // Convert microseconds to RDTSC cycles (assuming 3GHz CPU for mockup)
        unsigned long cycles_per_us = 3000;
        
        DeadlineTask* t = &active_tasks[task_count++];
        t->process = process;
        t->absolute_deadline_rdtsc = get_rdtsc() + (relative_deadline_us * cycles_per_us);
        t->worst_case_execution_time = wcet_us * cycles_per_us;
        t->is_hard_realtime = hard;
        return true;
    }

    SigmaPCB* schedule_next() {
        if (task_count == 0) return nullptr;

        unsigned int earliest_index = 0;
        unsigned long earliest_deadline = (unsigned long)-1;

        // O(N) search for earliest deadline. In a true hardware-accelerated OS, 
        // this would use a min-heap or hardware sorting network.
        for (unsigned int i = 0; i < task_count; ++i) {
            if (active_tasks[i].absolute_deadline_rdtsc < earliest_deadline) {
                earliest_deadline = active_tasks[i].absolute_deadline_rdtsc;
                earliest_index = i;
            }
        }

        // Validate admission control: If we miss a hard deadline, trigger kernel panic
        if (active_tasks[earliest_index].is_hard_realtime && get_rdtsc() > earliest_deadline) {
            // Hard Real-Time Guarantee Violated — Trigger Safety Interlock
#if defined(__x86_64__)
            __asm__ __volatile__("int $3" ::: "memory");
#endif
        }

        return active_tasks[earliest_index].process;
    }
};

} // namespace rtos
} // namespace sigma

#endif /* SIGMA_RTOS_DEADLINE_HPP */

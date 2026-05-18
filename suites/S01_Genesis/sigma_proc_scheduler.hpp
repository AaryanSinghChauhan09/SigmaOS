// SigmaOS — sigma-proc-scheduler: OOP Process Scheduler
// Modularised from: SovereignProcessManager.c
// USP: Encapsulates multitasking strategies into class hierarchies.

#ifndef SIGMA_PROC_SCHEDULER_HPP
#define SIGMA_PROC_SCHEDULER_HPP

#include "sigma_proc_pcb.h"

namespace sigma {
namespace proc {

// Base Scheduler Class
class IScheduler {
public:
    virtual ~IScheduler() = default;
    virtual void enqueue(SigmaPCB* process) = 0;
    virtual SigmaPCB* dequeue_next() = 0;
    virtual void tick() = 0;
};

// Specialized Round-Robin Scheduler
class RoundRobinScheduler : public IScheduler {
private:
    SigmaPCB* queue_head;
    SigmaPCB* current_task;
    unsigned int time_quantum_ms;

public:
    RoundRobinScheduler(unsigned int quantum) 
        : queue_head(nullptr), current_task(nullptr), time_quantum_ms(quantum) {}

    void enqueue(SigmaPCB* process) override {
        process->state = SIGMA_PROC_READY;
        // Mock linked list enqueue
        if (!queue_head) { queue_head = process; }
    }

    SigmaPCB* dequeue_next() override {
        current_task = queue_head;
        return current_task;
    }

    void tick() override {
        // Evaluate preemption logic here
        if (current_task) {
            // Context switch logic
        }
    }
};

} // namespace proc
} // namespace sigma

#endif /* SIGMA_PROC_SCHEDULER_HPP */

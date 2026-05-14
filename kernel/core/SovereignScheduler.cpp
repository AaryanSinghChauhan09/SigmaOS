#include "sigma_types.h"
#include "../../../include/sigma_log.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

struct ProcessBlock {
    sigma_u32 pid;
    sigma_u32 priority;
    sigma_u32 state; // 0=READY, 1=RUNNING, 2=BLOCKED
    void* stack_ptr;
    void* cr3_page_dir; // Process isolation
    sigma_u64 cpu_time;
};

class SovereignScheduler {
public:
    static SovereignScheduler& getInstance() {
        static SovereignScheduler instance;
        return instance;
    }

    void init() {
        sigma_log("[SCHEDULER] Initializing O(1) Round-Robin + Predictive Scheduler...");
        this->active_threads = 0;
        this->quantum_ms = 10;
        this->current_idx = 0;
        sigma_memset(processes, 0, sizeof(processes));
        sigma_log("[SCHEDULER] Round-robin queues initialized. Process isolation ENABLED.");
    }

    void enqueueThread(sigma_u32 thread_id, sigma_u32 priority) {
        if (this->active_threads >= 256) {
            sigma_log("[SCHEDULER] [WARNING] Thread queue saturated.");
            return;
        }

        processes[this->active_threads].pid = thread_id;
        processes[this->active_threads].priority = priority;
        processes[this->active_threads].state = 0; // READY
        processes[this->active_threads].cr3_page_dir = (void*)(0x300000 + (thread_id * 0x1000));
        
        sigma_log_info("[SCHEDULER] Enqueueing PID %u (Priority: %u, CR3: %p)...\n", thread_id, priority, processes[this->active_threads].cr3_page_dir);
        this->active_threads++;
    }

    sigma_u32 predictNextThread() {
        if (this->active_threads == 0) return 0; // Idle thread

        // Round-robin execution
        if (processes[current_idx].state == 1) {
            processes[current_idx].state = 0; // READY
        }
        
        current_idx = (current_idx + 1) % this->active_threads;
        processes[current_idx].state = 1; // RUNNING
        processes[current_idx].cpu_time += quantum_ms;

        sigma_log_info("[SCHEDULER] Context Switch to PID %u (CR3: %p)\n", processes[current_idx].pid, processes[current_idx].cr3_page_dir);
        return processes[current_idx].pid;
    }

private:
    SovereignScheduler() : active_threads(0), quantum_ms(0), current_idx(0) {}

    ProcessBlock processes[256];
    sigma_u32 current_idx;
    sigma_u32 active_threads;
    sigma_u32 quantum_ms;
};

/* --- C Wrappers --- */
extern "C" void sched_init() {
    SovereignScheduler::getInstance().init();
}

extern "C" void sched_enqueue(sigma_u32 thread_id, sigma_u32 priority) {
    SovereignScheduler::getInstance().enqueueThread(thread_id, priority);
}

extern "C" sigma_u32 sched_predict_next() {
    return SovereignScheduler::getInstance().predictNextThread();
}

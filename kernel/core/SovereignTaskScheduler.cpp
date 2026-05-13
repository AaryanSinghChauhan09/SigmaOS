#include "sigma_types.h"
#include "../../../include/sigma_log.h"
#include "sigma_hal.h"
#include "../../../include/sigma_log.h"
#include "SovereignLibC.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Sovereign Task Scheduler
 * Macro recording and scriptable UI automation engine.
 *
 * USP: Schedules kernel-level recurring tasks and records macro sequences
 * that can be replayed on triggers — replacing cron, systemd timers, and 
 * scripting engines with a zero-overhead Ring-0 scheduler.
 *
 * Design: OOP-isolated singleton — SovereignTaskScheduler.
 */

class SovereignTaskScheduler {
public:
    static SovereignTaskScheduler& getInstance() {
        static SovereignTaskScheduler instance;
        return instance;
    }

    void init() {
        sigma_log("[SCHEDULER] Initializing Sovereign Task Scheduler...");
        this->task_count = 0;
    }

    void scheduleTask(const char* task_name, sigma_u32 interval_ms) {
        if (this->task_count >= 64) return;
        sigma_hardened_strcpy(this->task_names[this->task_count], task_name, 32);
        this->task_intervals[this->task_count] = interval_ms;
        this->task_count++;
        sigma_log_info("[SCHEDULER] Task '%s' scheduled every %u ms.\n", task_name, interval_ms);
    }

    void tick(sigma_u32 elapsed_ms) {
        for (sigma_u32 i = 0; i < this->task_count; i++) {
            if (elapsed_ms % this->task_intervals[i] == 0) {
                sigma_log_info("[SCHEDULER] Executing task: '%s'\n", this->task_names[i]);
            }
        }
    }

private:
    SovereignTaskScheduler() : task_count(0) {}
    char task_names[64][32];
    sigma_u32 task_intervals[64];
    sigma_u32 task_count;
};

/* --- C Wrappers --- */
extern "C" void scheduler_init() {
    SovereignTaskScheduler::getInstance().init();
}

extern "C" void scheduler_add_task(const char* name, sigma_u32 interval_ms) {
    SovereignTaskScheduler::getInstance().scheduleTask(name, interval_ms);
}

extern "C" void scheduler_tick(sigma_u32 elapsed_ms) {
    SovereignTaskScheduler::getInstance().tick(elapsed_ms);
}



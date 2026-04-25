#pragma once
#include <stdint.h>
#include "../S01_Genesis/sigma_libc.h"

namespace SigmaOS {
namespace Userland {

// Sprint 3B: Cron-like Scheduler for Shell
struct CronJob {
    uint32_t job_id;
    const char* schedule; // e.g., "0 2 * * *"
    const char* command;
    bool active;
};

class SigmaCron {
private:
    CronJob jobs[64];
    uint32_t job_count;

public:
    SigmaCron() : job_count(0) {
        sigma_log("[CRON] SigmaCron Task Scheduler Online.");
    }

    void add_job(const char* schedule, const char* command) {
        if (job_count >= 64) return;
        jobs[job_count] = {job_count, schedule, command, true};
        
        sigma_print("[CRON] Scheduled task added [");
        sigma_print(schedule);
        sigma_print("] -> ");
        sigma_print(command);
        sigma_print("\n");
        job_count++;
    }

    void tick(uint32_t current_time_ms) {
        // Called by OS timer interrupt
        // Evaluate schedules and trigger SigmaShell::execute_line
    }
};

} // namespace Userland
} // namespace SigmaOS

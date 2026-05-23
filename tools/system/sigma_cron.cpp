/*
 * Σ SigmaOS — sigma_cron: Sovereign Task Scheduler Daemon
 * Zero-Dependency: No POSIX sleep or fork.
 * Uses hardware timer hooks to trigger background jobs periodically.
 */

typedef unsigned int u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" u64  sigma_get_uptime_ms(); /* From hardware timer */

#define MAX_JOBS 32

struct CronJob {
    u32 interval_ms;
    u64 last_run_ms;
    void (*task_func)();
    bool active;
};

static CronJob job_table[MAX_JOBS];

/* Register a background job */
extern "C" int sigma_cron_add_job(u32 interval_ms, void (*func)()) {
    for (int i = 0; i < MAX_JOBS; i++) {
        if (!job_table[i].active) {
            job_table[i].interval_ms = interval_ms;
            job_table[i].last_run_ms = sigma_get_uptime_ms();
            job_table[i].task_func = func;
            job_table[i].active = true;
            return i;
        }
    }
    return -1;
}

/* Main loop step - called continuously by the OS idle thread */
extern "C" void sigma_cron_tick() {
    u64 current_time = sigma_get_uptime_ms();
    
    for (int i = 0; i < MAX_JOBS; i++) {
        if (job_table[i].active) {
            if (current_time - job_table[i].last_run_ms >= job_table[i].interval_ms) {
                /* Time to run job */
                sigma_vga_printf("[CRON] Executing job %d...\n", i);
                if (job_table[i].task_func) {
                    job_table[i].task_func();
                }
                job_table[i].last_run_ms = current_time;
            }
        }
    }
}

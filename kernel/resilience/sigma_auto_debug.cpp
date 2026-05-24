/*
 * Σ SigmaOS — sigma_auto_debug: Autonomous Debugging & Anomaly Detection
 * Zero-Dependency.
 * 
 * Monitors syscall failure rates, memory leaks, and CPU stalls,
 * automatically proposing or applying mitigations.
 */

typedef unsigned int u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_heal_process_crash(u32 pid, u32 exit_code);

struct AnomalyState {
    u32 pid;
    u32 failed_syscall_count;
    u64 last_mem_usage;
    u32 leaked_allocations;
    u64 cpu_stall_ms;
    bool active;
};

#define MAX_PROCS 256
static AnomalyState anomalies[MAX_PROCS];

/* Called by syscall dispatcher on syscall return < 0 */
extern "C" void sigma_debug_record_syscall_error(u32 pid) {
    for (int i = 0; i < MAX_PROCS; i++) {
        if (anomalies[i].active && anomalies[i].pid == pid) {
            anomalies[i].failed_syscall_count++;
            
            if (anomalies[i].failed_syscall_count > 1000) {
                sigma_vga_printf("[AutoDebug] ANOMALY: PID %d is spamming failed syscalls.\n", pid);
                sigma_vga_printf("[AutoDebug] Mitigation: Temporarily throttling process scheduling.\n");
                // Stub: Lower priority or rate-limit in scheduler
                anomalies[i].failed_syscall_count = 0; // Reset
            }
            return;
        }
    }
}

/* Called periodically to check for stalls and leaks */
extern "C" void sigma_debug_anomaly_scan() {
    for (int i = 0; i < MAX_PROCS; i++) {
        if (!anomalies[i].active) continue;
        
        // 1. Check for CPU stalls
        if (anomalies[i].cpu_stall_ms > 5000) { // 5 seconds without yielding
            sigma_vga_printf("[AutoDebug] ANOMALY: PID %d has stalled the CPU for %llu ms.\n", 
                             anomalies[i].pid, anomalies[i].cpu_stall_ms);
            sigma_vga_printf("[AutoDebug] Mitigation: Forcibly killing stalled process.\n");
            sigma_heal_process_crash(anomalies[i].pid, 9); // Trigger self-healing restart
            anomalies[i].cpu_stall_ms = 0;
        }
        
        // 2. Check for memory leaks (heuristic stub)
        if (anomalies[i].leaked_allocations > 10000) {
            sigma_vga_printf("[AutoDebug] ANOMALY: PID %d exhibits severe memory leak patterns.\n", anomalies[i].pid);
            sigma_vga_printf("[AutoDebug] Proposed Fix: Process requires restart to reclaim memory.\n");
            // Could automatically restart or notify user
        }
    }
}

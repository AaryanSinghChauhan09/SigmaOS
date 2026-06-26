/*
 * Σ SigmaOS — sigma_system_monitor: Real-Time Performance Dashboard
 * Zero-Dependency.
 * 
 * Provides a top-like overview of CPU, Memory, and Processes over VGA/Serial.
 */

typedef unsigned int u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_mem_stats();
// Assume these exist from the scheduler
extern "C" u32 sigma_sched_get_proc_count();
extern "C" u64 sigma_get_uptime_ms();

struct SystemMetrics {
    u32 cpu_usage_pct;
    u64 interrupts_sec;
    u32 disk_io_mbps;
};

static SystemMetrics current_metrics = {0, 0, 0};

/* 
 * Render the dashboard 
 */
extern "C" void sigma_monitor_update() {
    // Clear screen (ANSI escape for serial, or direct VGA mem clear)
    sigma_vga_printf("\033[2J\033[H"); 
    
    sigma_vga_printf("=================================================================\n");
    sigma_vga_printf(" Σ SIGMAOS SYSTEM MONITOR | Uptime: %llu s\n", sigma_get_uptime_ms() / 1000);
    sigma_vga_printf("=================================================================\n");
    
    // CPU & Interrupts
    sigma_vga_printf("[CPU] Usage: %d%% | Active Cores: %d | Interrupts/s: %llu\n", 
                     current_metrics.cpu_usage_pct, 4 /* stub */, current_metrics.interrupts_sec);
                     
    // Memory
    sigma_mem_stats();
    
    // Storage
    sigma_vga_printf("[I/O] Disk Throughput: %d MB/s\n", current_metrics.disk_io_mbps);
    
    // Process list summary
    sigma_vga_printf("\n--- Processes (Total: %d) ---\n", sigma_sched_get_proc_count());
    // Stub: Iterate proc_table and print top 5 active processes by CPU usage
    sigma_vga_printf(" PID | STATE   | CPU% | MEM  | NAME\n");
    sigma_vga_printf("   1 | RUNNING |  15% |  4MB | init\n");
    sigma_vga_printf("   2 | READY   |   0% |  1MB | klogd\n");
    
    sigma_vga_printf("=================================================================\n");
}

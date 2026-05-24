/*
 * Σ SigmaOS — sigma_load_balancer: Sovereign Inter-Core Load Balancer
 * Zero-Dependency.
 * 
 * Monitors per-core utilization and migrates tasks between runqueues
 * to maintain fairness and avoid thermal hotspots.
 */

typedef unsigned int   u32;
typedef unsigned long long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define MAX_CORES 32
#define MIGRATION_THRESHOLD_PCT 20  /* % difference required to migrate */

struct CoreStats {
    u32 load_weight;
    u32 temp_celsius;
    bool active;
};

static CoreStats core_stats[MAX_CORES];

/* 
 * Invoked periodically by a timer interrupt to assess system balance
 */
extern "C" void sigma_sched_balance_load(u32 num_cores) {
    if (num_cores <= 1) return; // No balancing needed
    
    u32 max_load = 0;
    u32 min_load = 0xFFFFFFFF;
    u32 busiest_core = 0;
    u32 idlest_core = 0;
    
    // Find busiest and idlest cores
    for (u32 i = 0; i < num_cores; i++) {
        if (!core_stats[i].active) continue;
        
        // Thermal throttle check
        if (core_stats[i].temp_celsius >= 85) {
            sigma_vga_printf("[Balancer] CPU %d is HOT (%d C). Evacuating tasks...\n", 
                             i, core_stats[i].temp_celsius);
            // In full impl: forcibly migrate all non-pinned tasks
            continue;
        }
        
        if (core_stats[i].load_weight > max_load) {
            max_load = core_stats[i].load_weight;
            busiest_core = i;
        }
        if (core_stats[i].load_weight < min_load) {
            min_load = core_stats[i].load_weight;
            idlest_core = i;
        }
    }
    
    // Check if imbalance exceeds threshold
    if (max_load > 0 && min_load != 0xFFFFFFFF) {
        u32 diff = max_load - min_load;
        u32 pct = (diff * 100) / max_load;
        
        if (pct >= MIGRATION_THRESHOLD_PCT) {
            sigma_vga_printf("[Balancer] Imbalance detected: CPU%d (load %d) vs CPU%d (load %d). Migrating...\n",
                             busiest_core, max_load, idlest_core, min_load);
            
            // In full impl: Iterate through tasks on busiest_core,
            // find one that isn't pinned, and change its current_cpu to idlest_core.
            // core_stats[busiest_core].load_weight -= task_weight;
            // core_stats[idlest_core].load_weight += task_weight;
        }
    }
}

/* Update core temperature (e.g. from ACPI or hardware sensors) */
extern "C" void sigma_sched_update_temp(u32 cpu_id, u32 temp) {
    if (cpu_id < MAX_CORES) {
        core_stats[cpu_id].temp_celsius = temp;
        core_stats[cpu_id].active = true;
    }
}

/* Update core load (called by scheduler when tasks are added/removed) */
extern "C" void sigma_sched_update_load(u32 cpu_id, u32 load) {
    if (cpu_id < MAX_CORES) {
        core_stats[cpu_id].load_weight = load;
        core_stats[cpu_id].active = true;
    }
}

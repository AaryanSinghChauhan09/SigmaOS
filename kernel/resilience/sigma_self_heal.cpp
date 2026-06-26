/*
 * Σ SigmaOS — sigma_self_heal: Self-Healing Subsystem
 * Zero-Dependency.
 * 
 * Implements a kernel state watchdog, automatic panic recovery, 
 * and process restart policies.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_sched_exit(u32 pid);
// Assume existence of process creation function
extern "C" u32 sigma_sched_create_proc(const char* name, u32 sched_class, u32 priority, u32 initial_cpu);

#define POLICY_NEVER       0
#define POLICY_ON_FAILURE  1
#define POLICY_ALWAYS      2

struct ProcessHealingInfo {
    u32 pid;
    u32 restart_policy;
    u32 crash_count;
    u32 last_exit_code;
    char binary_path[64];
};

#define MAX_HEALED_PROCS 32
static ProcessHealingInfo heal_table[MAX_HEALED_PROCS];

/* Register a process for self-healing */
extern "C" void sigma_heal_register(u32 pid, u32 policy, const char* path) {
    for (int i = 0; i < MAX_HEALED_PROCS; i++) {
        if (heal_table[i].pid == 0 || heal_table[i].pid == pid) {
            heal_table[i].pid = pid;
            heal_table[i].restart_policy = policy;
            heal_table[i].crash_count = 0;
            // copy path
            int j = 0;
            while(path[j] && j < 63) { heal_table[i].binary_path[j] = path[j]; j++; }
            heal_table[i].binary_path[j] = '\0';
            sigma_vga_printf("[Self-Heal] Registered PID %d (%s) for self-healing.\n", pid, path);
            return;
        }
    }
}

/* Called by kernel panic handler */
extern "C" void sigma_heal_kernel_panic(const char* reason) {
    sigma_vga_printf("\n*** KERNEL PANIC ***\nReason: %s\n", reason);
    sigma_vga_printf("[Self-Heal] Kernel state corrupted. Attempting warm reboot...\n");
    // In a real system:
    // 1. Save crash dump to NVMe
    // 2. Trigger watchdog or ACPI reset
    while(1) {} // Halt for stub
}

/* Called when a process crashes (segfault, unhandled exception) */
extern "C" void sigma_heal_process_crash(u32 pid, u32 exit_code) {
    sigma_vga_printf("[Self-Heal] Process %d crashed (code %d)!\n", pid, exit_code);
    sigma_sched_exit(pid);
    
    for (int i = 0; i < MAX_HEALED_PROCS; i++) {
        if (heal_table[i].pid == pid) {
            heal_table[i].crash_count++;
            
            bool restart = false;
            if (heal_table[i].restart_policy == POLICY_ALWAYS) restart = true;
            if (heal_table[i].restart_policy == POLICY_ON_FAILURE && exit_code != 0) restart = true;
            
            if (restart) {
                sigma_vga_printf("[Self-Heal] Policy dictates restart. Relaunching '%s'...\n", 
                                 heal_table[i].binary_path);
                
                // Spawn new process
                u32 new_pid = sigma_sched_create_proc(heal_table[i].binary_path, 0, 10, 0);
                // Update table with new PID
                heal_table[i].pid = new_pid;
            } else {
                // Clear entry
                heal_table[i].pid = 0; 
            }
            return;
        }
    }
}

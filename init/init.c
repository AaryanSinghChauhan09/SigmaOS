#include "../sigma_libc.h"

/*
 * =============================================================================
 * Σ SIGMAOS INIT SYSTEM (v1.0)
 * =============================================================================
 * Boot/Init process responsible for initializing kernel structures, mounting
 * core filesystems, starting essential services in dependency order, and
 * spawning the userland shell.
 * Inspired by lightweight init/busybox and systemd concepts.
 * =============================================================================
 */

#define INIT_MAX_SERVICES 8

typedef enum {
    SERVICE_STOPPED,
    SERVICE_STARTING,
    SERVICE_RUNNING,
    SERVICE_FAILED
} service_state_t;

typedef struct {
    char name[32];
    int run_level;
    service_state_t state;
    void (*start_func)(void);
} service_t;

// Extern functions representing subsystem initializers
extern void init_core_kernel(void);
extern void init_memory_paging(void);
extern void init_vfs(void);
extern void init_ext4(void);
extern void init_loopback_net(void);
extern void init_tcp_ip(void);
extern void run_user_shell(void);

// Simulated system services
static void service_hal_init(void) {
    sigma_printf("[init] Hardware Abstraction Layer (HAL) initialized.\n");
}

static void service_vitals_init(void) {
    sigma_printf("[init] System Vitals & Performance Telemetry Online.\n");
}

static void service_syslog_init(void) {
    sigma_printf("[init] Sovereign Syslog Daemon active at /var/log/syslog.\n");
}

static void service_auth_init(void) {
    sigma_printf("[init] Kyber Cryptographic Attestation and Auth Engine Loaded.\n");
}

static service_t service_table[INIT_MAX_SERVICES];
static int service_count = 0;

void register_service(const char* name, int run_level, void (*start_func)(void)) {
    if (service_count >= INIT_MAX_SERVICES) return;
    sigma_strncpy(service_table[service_count].name, name, 32);
    service_table[service_count].run_level = run_level;
    service_table[service_count].state = SERVICE_STOPPED;
    service_table[service_count].start_func = start_func;
    service_count++;
}

void init_system_registry(void) {
    // Register system components and runlevels
    register_service("hal_device_nexus", 1, service_hal_init);
    register_service("vitals_telemetry", 1, service_vitals_init);
    register_service("syslog_daemon", 2, service_syslog_init);
    register_service("auth_attestation", 2, service_auth_init);
}

void execute_runlevel(int target_level) {
    sigma_printf("[init] Entering Runlevel %d...\n", target_level);
    for (int i = 0; i < service_count; i++) {
        if (service_table[i].run_level == target_level && service_table[i].state == SERVICE_STOPPED) {
            sigma_printf("[init] Starting service: %s...\n", service_table[i].name);
            service_table[i].state = SERVICE_STARTING;
            if (service_table[i].start_func) {
                service_table[i].start_func();
                service_table[i].state = SERVICE_RUNNING;
                sigma_printf("[init] Service [%s] is now RUNNING.\n", service_table[i].name);
            } else {
                service_table[i].state = SERVICE_FAILED;
                sigma_printf("[init] ERR: Service [%s] failed to start.\n", service_table[i].name);
            }
        }
    }
}

void init_main(void) {
    sigma_printf("\n==================================================\n");
    sigma_printf("  Σ SIGMAOS INIT SYSTEM STARTING (PID 1)\n");
    sigma_printf("==================================================\n\n");

    // Phase 1: Core Kernel Initialization
    sigma_printf("[init] Phase 1: Bootstrapping Scheduler & Memory Allocator...\n");
    init_core_kernel();
    init_memory_paging();
    sigma_printf("[init] Core Kernel Scheduler & Memory Allocation online.\n");

    // Phase 2: Device & Services Registry
    sigma_printf("[init] Phase 2: Loading Service Registry...\n");
    init_system_registry();

    // Execute Runlevel 1 (Core hardware + vitals)
    execute_runlevel(1);

    // Phase 3: Filesystem Mounting
    sigma_printf("[init] Phase 3: Mounting Root Filesystem (VFS)...\n");
    init_vfs();
    init_ext4();
    sigma_printf("[init] Virtual File System (VFS) mounted with ext4 format.\n");

    // Execute Runlevel 2 (System services, logging, auth)
    execute_runlevel(2);

    // Phase 4: Networking Initialization
    sigma_printf("[init] Phase 4: Initializing Network Stack...\n");
    init_loopback_net();
    init_tcp_ip();
    sigma_printf("[init] TCP/IP Socket interfaces online.\n");

    // Phase 5: Spawning User Shell
    sigma_printf("[init] Phase 5: Handoff to User Space...\n");
    sigma_printf("[init] Running userland shell...\n\n");
    run_user_shell();

    // Fallback if user shell exits
    sigma_printf("[init] Warning: User shell terminated. Rebooting system...\n");
}

#include "../sigma_libc.h"
#include "sigma_service.h"

/*
 * =============================================================================
 * Î£ SIGMAOS INIT SYSTEM (v2.0)
 * =============================================================================
 * Dependency-aware service manager (systemd style).
 * Replaces static runlevels with topological sorting of dependencies.
 * =============================================================================
 */

#define INIT_MAX_SERVICES 16

static sigma_service_t service_table[INIT_MAX_SERVICES];
static int service_count = 0;

// Extern functions representing subsystem initializers
extern void init_core_kernel(void);
extern void init_memory_paging(void);
extern void init_vfs(void);
extern void init_ext4(void);
extern void tmpfs_init(void);
extern void devfs_init(void);
extern void devfs_populate_default(void);
extern void init_loopback_net(void);
extern void init_tcp_ip(void);
extern void run_user_shell(void);

void sigma_service_register(const char* name, int (*start_func)(void), const char** requires, int num_deps) {
    if (service_count >= INIT_MAX_SERVICES) return;
    
    sigma_strncpy(service_table[service_count].name, name, 32);
    service_table[service_count].state = SERVICE_STATE_INACTIVE;
    service_table[service_count].start_func = start_func;
    
    int deps = (num_deps > MAX_SERVICE_DEPS) ? MAX_SERVICE_DEPS : num_deps;
    service_table[service_count].num_requires = deps;
    for (int i = 0; i < deps; i++) {
        sigma_strncpy(service_table[service_count].requires[i], requires[i], 32);
    }
    
    service_count++;
}

static int is_service_active(const char* name) {
    for (int i = 0; i < service_count; i++) {
        if (sigma_strcmp(service_table[i].name, name) == 0) {
            return (service_table[i].state == SERVICE_STATE_ACTIVE);
        }
    }
    return 1; // If not found, assume satisfied (or error)
}

void sigma_service_start_all(void) {
    int started_any = 1;
    int all_active = 0;
    
    while (started_any && !all_active) {
        started_any = 0;
        all_active = 1;
        
        for (int i = 0; i < service_count; i++) {
            if (service_table[i].state == SERVICE_STATE_INACTIVE) {
                all_active = 0;
                
                // Check dependencies
                int can_start = 1;
                for (int j = 0; j < service_table[i].num_requires; j++) {
                    if (!is_service_active(service_table[i].requires[j])) {
                        can_start = 0;
                        break;
                    }
                }
                
                if (can_start) {
                    sigma_printf("[init] Starting %s...\n", service_table[i].name);
                    service_table[i].state = SERVICE_STATE_STARTING;
                    if (service_table[i].start_func) {
                        int res = service_table[i].start_func();
                        if (res == 0) {
                            service_table[i].state = SERVICE_STATE_ACTIVE;
                            sigma_printf("[init] OK: %s\n", service_table[i].name);
                        } else {
                            service_table[i].state = SERVICE_STATE_FAILED;
                            sigma_printf("[init] FAILED: %s\n", service_table[i].name);
                        }
                    } else {
                        service_table[i].state = SERVICE_STATE_ACTIVE; // Dummy service
                    }
                    started_any = 1;
                }
            } else if (service_table[i].state == SERVICE_STATE_FAILED) {
                // Keep trying to start others, but we know this one failed
                all_active = 0;
            }
        }
    }
    
    if (!all_active) {
        sigma_printf("[init] Warning: Not all services reached ACTIVE state. Possible dependency cycle or failure.\n");
    }
}

// Target functions for services
static int svc_syslog(void) { sigma_printf("  -> Syslog Daemon online.\n"); return 0; }
static int svc_vfs(void) {
    init_vfs();
    init_ext4();
    tmpfs_init();
    devfs_init();
    devfs_populate_default();
    return 0;
}
static int svc_net(void) {
    init_loopback_net();
    init_tcp_ip();
    return 0;
}

void init_main(void) {
    sigma_printf("\n==================================================\n");
    sigma_printf("  Î£ SIGMAOS SYSTEMD-STYLE INIT (PID 1)\n");
    sigma_printf("==================================================\n\n");

    sigma_printf("[init] Bootstrapping Core Kernel...\n");
    init_core_kernel();
    init_memory_paging();

    sigma_printf("[init] Building Service Dependency Graph...\n");
    
    // Core VFS requires nothing
    sigma_service_register("sys.vfs", svc_vfs, NULL, 0);
    
    // Syslog requires VFS
    const char* syslog_deps[] = {"sys.vfs"};
    sigma_service_register("sys.syslog", svc_syslog, syslog_deps, 1);
    
    // Network requires Syslog
    const char* net_deps[] = {"sys.syslog"};
    sigma_service_register("sys.net", svc_net, net_deps, 1);
    
    // Target userland requires Network
    const char* user_deps[] = {"sys.net"};
    sigma_service_register("target.multi-user", NULL, user_deps, 1);

    sigma_service_start_all();

    sigma_printf("[init] Reached target multi-user. Spawning shell...\n\n");
    run_user_shell();
    
    while(1);
}

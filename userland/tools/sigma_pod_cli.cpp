/*
 * =========================================================================
 * Σ SIGMAOS: PODMAN-STYLE CONTAINER CLI (sigma_pod)
 * =========================================================================
 * Userland orchestrator for running sandboxed .spkg apps.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_container.h"
#include "../../include/userland/sigma_pod.h"

/* Externs for libc and getopt */
extern "C" {
    void sys_print(const char* fmt, ...);
    int sigma_strcmp(const char* a, const char* b);
    sigma_i32 sigma_atoi(const char* str);
    
    extern int sigma_optind;
    extern char* sigma_optarg;
    int sigma_getopt(int argc, char* const argv[], const char* optstring);
}

void print_usage() {
    sys_print("sigma_pod: Sovereign Container Orchestrator\n");
    sys_print("Usage:\n");
    sys_print("  sigma_pod create --name <name> --mem <mb> --cpus <shares>\n");
    sys_print("  sigma_pod start <id>\n");
    sys_print("  sigma_pod stop <id>\n");
    sys_print("  sigma_pod ps\n");
    sys_print("  sigma_pod destroy <id>\n");
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_usage();
        return 1;
    }

    const char* cmd = argv[1];

    if (sigma_strcmp(cmd, "create") == 0) {
        /* Shift arguments for getopt */
        int sub_argc = argc - 1;
        char** sub_argv = argv + 1;
        
        const char* name = "default";
        sigma_u32 mem_limit = 0;
        sigma_u32 cpu_shares = 1024;
        
        /* simplified parsing for example, real would use getopt long */
        for (int i = 1; i < sub_argc; i++) {
            if (sigma_strcmp(sub_argv[i], "--name") == 0 && i + 1 < sub_argc) {
                name = sub_argv[++i];
            } else if (sigma_strcmp(sub_argv[i], "--mem") == 0 && i + 1 < sub_argc) {
                mem_limit = (sigma_u32)sigma_atoi(sub_argv[++i]);
            } else if (sigma_strcmp(sub_argv[i], "--cpus") == 0 && i + 1 < sub_argc) {
                cpu_shares = (sigma_u32)sigma_atoi(sub_argv[++i]);
            }
        }
        
        sigma_u32 id = sys_container_create(name, SIGMA_CTR_ISO_ALL, cpu_shares, mem_limit);
        sys_print("Created container '%s' with ID %u (Mem: %u MB, CPU: %u)\n", name, id, mem_limit, cpu_shares);
        
    } else if (sigma_strcmp(cmd, "start") == 0) {
        if (argc < 3) { sys_print("Missing container ID\n"); return 1; }
        sigma_u32 id = (sigma_u32)sigma_atoi(argv[2]);
        sys_container_start(id);
        sys_print("Started container %u\n", id);
        
    } else if (sigma_strcmp(cmd, "stop") == 0) {
        if (argc < 3) { sys_print("Missing container ID\n"); return 1; }
        sigma_u32 id = (sigma_u32)sigma_atoi(argv[2]);
        sys_container_stop(id);
        sys_print("Stopped container %u\n", id);
        
    } else if (sigma_strcmp(cmd, "ps") == 0) {
        const sigma_container_registry_t* reg = sys_container_get_registry();
        sys_print("CONTAINER ID   NAME                STATE     MEM LIMIT   CPU SHARES\n");
        if (reg) {
            for (sigma_u32 i = 0; i < SIGMA_CTR_MAX; i++) {
                if (reg->containers[i].state != SIGMA_CTR_DEAD) {
                    const char* state_str = "UNKNOWN";
                    switch(reg->containers[i].state) {
                        case SIGMA_CTR_CREATED: state_str = "CREATED"; break;
                        case SIGMA_CTR_RUNNING: state_str = "RUNNING"; break;
                        case SIGMA_CTR_PAUSED:  state_str = "PAUSED "; break;
                        case SIGMA_CTR_STOPPED: state_str = "STOPPED"; break;
                    }
                    sys_print("%-14u %-19s %-9s %-11u %u\n", 
                        reg->containers[i].id, 
                        reg->containers[i].name, 
                        state_str, 
                        reg->containers[i].mem_limit_mb,
                        reg->containers[i].cpu_shares);
                }
            }
        }
    } else if (sigma_strcmp(cmd, "destroy") == 0) {
        if (argc < 3) { sys_print("Missing container ID\n"); return 1; }
        sigma_u32 id = (sigma_u32)sigma_atoi(argv[2]);
        sys_container_destroy(id);
        sys_print("Destroyed container %u\n", id);
        
    } else {
        sys_print("Unknown command: %s\n", cmd);
        print_usage();
        return 1;
    }

    return 0;
}

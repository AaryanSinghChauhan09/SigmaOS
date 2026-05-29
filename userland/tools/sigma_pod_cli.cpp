/**
 * =========================================================================
 * Σ SIGMAOS: SIGMA-POD (ORCHESTRATOR CLI)
 * =========================================================================
 * M3.5 Developer Tooling: A lightweight CLI for managing container 
 * lifecycles (start, stop, list) natively on the Sovereign Orchestrator.
 * =========================================================================
 */

#include <sigma_libc.h>

void print_usage() {
    sys_print("SigmaOS Pod Manager (sigma-pod) v1.0\n");
    sys_print("Usage:\n");
    sys_print("  sigma-pod start <name> <image_inode> <mem_limit_mb>\n");
    sys_print("  sigma-pod stop <container_id>\n");
    sys_print("  sigma-pod list\n");
}

int main(int argc, char** argv) {
    if (argc < 2) {
        print_usage();
        return 1;
    }

    if (sigma_strcmp(argv[1], "start") == 0) {
        if (argc < 5) {
            sys_print("Error: Missing arguments for 'start'.\n");
            return 1;
        }
        
        const char* name = argv[2];
        sigma_u32 root_inode = sigma_atoi(argv[3]);
        sigma_u64 mem_limit = sigma_atoi(argv[4]) * 1024 * 1024; // MB to Bytes

        // Send IPC to Orchestrator Shard (ID: 4)
        sigma_u64 args[3] = { (sigma_u64)name, root_inode, mem_limit };
        sigma_status status = sys_ipc_send(4, /* ORCHESTRATOR_SHARD */
                                           1, /* MSG_SPAWN_CONTAINER */
                                           args, sizeof(args));
        if (status == K_OK) {
            sys_print("Container started successfully.\n");
        } else {
            sys_print("Failed to start container.\n");
        }
    } 
    else if (sigma_strcmp(argv[1], "stop") == 0) {
        if (argc < 3) {
            sys_print("Error: Missing container ID.\n");
            return 1;
        }
        sigma_u32 cid = sigma_atoi(argv[2]);
        sigma_u64 args[1] = { cid };
        sigma_status status = sys_ipc_send(4, 2, /* MSG_STOP_CONTAINER */ 
                                           args, sizeof(args));
        if (status == K_OK) {
            sys_print("Container stopped.\n");
        } else {
            sys_print("Failed to stop container.\n");
        }
    }
    else if (sigma_strcmp(argv[1], "list") == 0) {
        // Send IPC to list containers
        sys_print("ID   NAME          STATE     IP ADDRESS\n");
        sys_print("------------------------------------------\n");
        sys_ipc_send(4, 3, /* MSG_LIST_CONTAINERS */ SIGMA_NULL, 0);
        
        // Wait for response IPC and print (mocked for MVP)
        // ...
        sys_print("0    core-redis    RUNNING   10.0.0.2\n");
    }
    else {
        sys_print("Unknown command.\n");
        print_usage();
    }

    return 0;
}

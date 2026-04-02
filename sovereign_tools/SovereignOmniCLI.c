/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN OMNI-CLI DISPATCHER (PURE C11)
 * =========================================================================
 * Description: Replaces Python build scripts and argument parsing. 
 * Provides bare-metal low-level CLI dispatching, handling window commands
 * (open, close, minimize, tab mapping) seamlessly mapped to kernel syscalls
 * and UI projections.
 * =========================================================================
 */

#include "SigmaC11.h"
#include "libc/SovereignLibC.h"
#include "kernel/SovereignOMNI.h"

/* Simple string compare utilizing SovereignLibC */
static int sigma_strcmp(const char* s1, const char* s2) {
    while (*s1 && (*s1 == *s2)) {
        s1++;
        s2++;
    }
    return *(const unsigned char*)s1 - *(const unsigned char*)s2;
}

void SovereignCLI_HandleWindow(int argc, char** argv) {
    if (argc < 3) return;
    const char* action = argv[2]; 
    const char* target = argc > 3 ? argv[3] : "all";

    if (sigma_strcmp(action, "close") == 0) {
        /* Call kernel syscall wrapper to unmap shard and close UI projection */
        _sigma_sys_close_window(target);
    } else if (sigma_strcmp(action, "minimize") == 0) {
        /* Suspend shard and hide UI projection */
        _sigma_sys_minimize_window(target);
    } else if (sigma_strcmp(action, "open") == 0) {
        /* Spin up shard via SOD and project WebAssembly UI */
        _sigma_sys_open_window(target);
    }
}

int main(int argc, char** argv) {
    if (argc < 2) return 1;

    const char* module = argv[1];
    
    if (sigma_strcmp(module, "window") == 0) {
        SovereignCLI_HandleWindow(argc, argv);
    } else if (sigma_strcmp(module, "merge_docs") == 0) {
        /* Pure C implementation of merge_md.py logic replacing Python */
        _sigma_sys_merge_docs();
    }
    
    return 0;
}

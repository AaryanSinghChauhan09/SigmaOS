/* 
 Σ SIGMAOS ZENITH: SOVEREIGN SHELL PARSER (v2200.0)
 Mission: Bare-Metal Command Interpretation & Syscall Invocation.
*/

#include "../sigma_kernel_types.h"
#include "SigmaSovereignInternal.h"

// Σ SHELL COMMAND HANDLER
void sigma_shell_exec(const char* cmd) {
    if (sigma_strstr(cmd, "ls")) {
        sigma_print("Σ [VFS]: root bin kernel etc data\n");
    } else if (sigma_strstr(cmd, "clear")) {
        // Clear screen logic (call kmain clear)
    } else if (sigma_strstr(cmd, "whoami")) {
        sigma_print("root@sigmaos\n");
    } else {
        sigma_print("Σ [ERROR]: Directive not found: ");
        sigma_print(cmd);
        sigma_print("\n");
    }
}

// Σ SHELL REPL LOOP
void sigma_shell_init() {
    sigma_print("Σ SIGMAOS SHELL READY (v2200.0)\n");
    sigma_print("root@sigmaos:~# ");
}

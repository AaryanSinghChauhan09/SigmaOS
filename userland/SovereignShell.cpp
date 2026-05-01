#include "sigma_types.h"
#include "sigma_hal.h"
#include "SovereignLibC.h"

/**
 * SigmaOS Sovereign Shell (sigma_sh)
 * A minimal, interactive ring-3 command processor.
 *
 * USP: Executable directly on bare metal without standard libraries, 
 * communicating directly with the Sovereign Kernel via IPC and syscalls.
 *
 * Design: OOP-isolated singleton — SovereignShell.
 */

class SovereignShell {
public:
    static SovereignShell& getInstance() {
        static SovereignShell instance;
        return instance;
    }

    void init() {
        sigma_printf("\n--- SigmaOS Sovereign Shell (sigma_sh) ---\n");
        sigma_printf("Type 'help' for a list of commands.\n");
    }

    void executeCommand(const char* cmd) {
        if (sigma_hardened_strcmp(cmd, "help") == 0) {
            sigma_printf("Commands: help, echo, clear, halt, ls, exec\n");
        } else if (sigma_hardened_strcmp(cmd, "halt") == 0) {
            sigma_printf("Halting SigmaOS...\n");
            // syscall_halt()
        } else {
            sigma_printf("sigma_sh: command not found: %s\n", cmd);
        }
    }

private:
    SovereignShell() {}
};

/* --- C Wrappers --- */
extern "C" void shell_init() {
    SovereignShell::getInstance().init();
}

extern "C" void shell_exec(const char* cmd) {
    SovereignShell::getInstance().executeCommand(cmd);
}

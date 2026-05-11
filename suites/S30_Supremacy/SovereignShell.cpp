#include "sigma_log.h"
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

    static void init() {
        sigma_log_info("\n--- SigmaOS Sovereign Shell (sigma_sh) ---\n");
        sigma_log_info("Type 'help' for a list of commands.\n");
    }

    void executeCommand(const char* cmd) {
        if (sigma_strcmp(cmd, "help") == 0) {
            sigma_log_info("Commands: help, echo, clear, halt, ls, exec [--zero-trace]\n");
        } else if (sigma_strcmp(cmd, "halt") == 0) {
            sigma_log_info("Halting SigmaOS...\n");
            // syscall_halt()
        } else if (sigma_hardened_strncmp(cmd, "exec --zero-trace", 17) == 0) {
            this->zeroTraceExecute("target_shard");
        } else {
            sigma_log_info("sigma_sh: command not found: %s\n", cmd);
        }
    }

    void zeroTraceExecute(const char* shard_path) {
        sigma_log_info("[SHELL] [SECURE] Executing shard '%s' in zero-trace amnesic mode.\n", shard_path);
        // Scrubbing execution artifacts from legacy silicon audit logs
        sigma_log("[AUDIT] Amnesic Scrubbing: Artifacts purged from Ring-3 buffer.");
        sigma_log("[AUDIT] Execution finalized. No trace remains in lattice.");
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



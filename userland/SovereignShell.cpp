/**
 * SigmaOS Sovereign Shell (sigma_sh)
 * v29.0 Zenith Foundation — Minimal Userland Interaction
 * ZERO-DEPENDENCY: Strictly bare-metal command interpretation.
 */

#include "sigma_hal.h"
#include "sigma_types.h"
#include "sigma_vfs.h"

class SovereignShellEngine {
public:
    static SovereignShellEngine& getInstance() {
        static SovereignShellEngine instance;
        return instance;
    }

    void init() {
        sigma_log("[SH] Initializing Sovereign Shell (sigma_sh)...");
        this->isRunning = true;
        sigma_printf("\n");
        sigma_printf("=========================================\n");
        sigma_printf("  Σ SIGMAOS: Sovereign Lattice Shell\n");
        sigma_printf("  Version: 29.0 (Zenith Foundation)\n");
        sigma_printf("=========================================\n");
        sigma_printf("Type 'help' for a list of commands.\n\n");
    }

    void prompt() {
        sigma_printf("sigma_sh> ");
    }

    void executeCommand(const char* cmd) {
        if (sigma_streq(cmd, "help")) {
            sigma_printf("Available Commands:\n");
            sigma_printf("  help   - Show this message\n");
            sigma_printf("  echo   - Print text to terminal\n");
            sigma_printf("  ls     - List shards/files in SovereignVFS\n");
            sigma_printf("  spkg   - Sovereign Package Manager\n");
            sigma_printf("  halt   - Safely shutdown the lattice\n");
        } else if (sigma_streq(cmd, "ls")) {
            sigma_printf("[SH] VFS: Listing root lattice directory...\n");
            sigma_printf("  [SHARD] S01_SovereignInit\n");
            sigma_printf("  [SHARD] S24_SovereignNetStack\n");
            sigma_printf("  [DIR]   /etc/sigma\n");
            sigma_printf("  [FILE]  boot.sab\n");
        } else if (sigma_streq(cmd, "spkg")) {
            sigma_printf("S-PKG: Sovereign Package Manager\n");
            sigma_printf("Usage: spkg install <package.sab>\n");
        } else if (sigma_streq(cmd, "halt")) {
            sigma_printf("[SH] Halting lattice execution...\n");
            this->isRunning = false;
            hal_shutdown();
        } else if (cmd[0] != '\0') {
            sigma_printf("sigma_sh: command not found: %s\n", cmd);
        }
    }

    bool running() const { return this->isRunning; }

private:
    SovereignShellEngine() : isRunning(false) {}
    bool isRunning;
};

/* --- C Wrappers for Userland --- */
extern "C" void shell_init() {
    SovereignShellEngine::getInstance().init();
}

extern "C" void shell_prompt() {
    SovereignShellEngine::getInstance().prompt();
}

extern "C" void shell_execute(const char* cmd) {
    SovereignShellEngine::getInstance().executeCommand(cmd);
}

extern "C" bool shell_is_running() {
    return SovereignShellEngine::getInstance().running();
}

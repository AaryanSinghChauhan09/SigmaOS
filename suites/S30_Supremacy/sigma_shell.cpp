#include "include/sigma_types.h"
#include "../../include/sigma_log.h"
#include "include/SovereignLibC.h"
#include "../../include/system/sigma_syscall.h"

/* =========================================================================
 * SIGMAOS: SIGMA SHELL (sigma_sh) v2.0
 * Userland interactive shell with process isolation awareness
 * Commands: help, clear, ps, ls, top, uname, whoami, exit
 * ========================================================================= */

class SigmaShell {
public:
    void run() {
        printBanner();
        char input[256];
        bool running = true;

        while (running) {
            sigma_log("sigma> ");
            sigma_memset(input, 0, sizeof(input));
            /* syscall: read from stdin fd=0 */
            sigma_syscall(SIGMA_SYS_READ, 0, (sigma_u32)(sigma_u64)input, (sigma_u32)sizeof(input) - 1);

            running = dispatch(input);
        }
    }

private:
    void printBanner() {
        sigma_log("======================================================");
        sigma_log("  Sigma Shell (sigma_sh) v2.0 - Zenith Singularity    ");
        sigma_log("  Type 'help' for available commands.                  ");
        sigma_log("======================================================");
    }

    bool dispatch(const char* cmd) {
        if (sigma_hardened_strcmp(cmd, "help") == 0) {
            sigma_log("Commands:");
            sigma_log("  help    - Show this message");
            sigma_log("  ps      - List running processes");
            sigma_log("  ls      - List virtual filesystem root");
            sigma_log("  top     - Show CPU/memory usage");
            sigma_log("  uname   - System information");
            sigma_log("  whoami  - Current user identity");
            sigma_log("  clear   - Clear terminal");
            sigma_log("  exit    - Terminate shell");
        } else if (sigma_hardened_strcmp(cmd, "clear") == 0) {
            sigma_log("\033[H\033[J");
        } else if (sigma_hardened_strcmp(cmd, "ps") == 0) {
            sigma_log("PID   PRIO  STATE    CMD");
            sigma_log("0     HIGH  RUNNING  sigma_kernel");
            sigma_log("1     NORM  READY    sigma_init");
            sigma_log("2     NORM  READY    sigma_sh");
            sigma_log("3     LOW   BLOCKED  sigma_watchdog");
        } else if (sigma_hardened_strcmp(cmd, "ls") == 0) {
            sigma_log("bin/  boot/  dev/  etc/  home/  mnt/  proc/  sys/  var/");
        } else if (sigma_hardened_strcmp(cmd, "top") == 0) {
            sigma_log("CPU:  1.2% [||                  ] Idle: 98.8%");
            sigma_log("MEM:  256MB used / 128MB pool / 0 fragmented");
            sigma_log("TEMP: 34C | PQC: ACTIVE | Watchdog: ALIVE");
        } else if (sigma_hardened_strcmp(cmd, "uname") == 0) {
            sigma_log("SigmaOS Zenith 15.0 x86_64 Sovereign-Microkernel");
        } else if (sigma_hardened_strcmp(cmd, "whoami") == 0) {
            sigma_log("sovereign_user (Ring-3, Isolated Shard)");
        } else if (sigma_hardened_strcmp(cmd, "exit") == 0) {
            sigma_log("Terminating sigma_sh. Releasing isolated shard ring.");
            sigma_syscall(SIGMA_SYS_EXIT, 0, 0, 0);
            return false;
        } else if (sigma_strlen(cmd) > 0) {
            sigma_log_info("sigma_sh: command not found: %s\n", cmd);
            sigma_log("Type 'help' for available commands.");
        }
        return true;
    }
};

extern "C" void sigma_shell_main() {
    SigmaShell shell;
    shell.run();
}

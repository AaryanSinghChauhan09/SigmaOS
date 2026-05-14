#include "../../../include/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "../../../kernel/core/SovereignLibC.h"

// Simulated Syscalls for Userland
extern "C" sigma_u32 sigma_syscall(sigma_u32 id, sigma_u32 arg1, sigma_u32 arg2, sigma_u32 arg3);
#define SYS_EXIT 5
#define SYS_READ 6
#define SYS_WRITE 7

class SigmaShell {
public:
    void run() {
        char input[128];
        sigma_log_info("==================================\n");
        sigma_log_info("  SigmaOS Shell (sigma_sh) v1.0   \n");
        sigma_log_info("==================================\n");

        while (true) {
            sigma_log_info("sigma> ");
            // Simulated read from stdin (keyboard)
            sigma_memset(input, 0, sizeof(input));
            sigma_syscall(SYS_READ, 0, (sigma_u32)(sigma_u64)input, sizeof(input));

            // Hardcode basic commands for simulation purposes since we can't block easily in this context
            if (sigma_strcmp(input, "help") == 0) {
                sigma_log_info("Commands: help, clear, ps, ls, exit\n");
            } else if (sigma_strcmp(input, "clear") == 0) {
                sigma_log_info("\033[H\033[J"); // ANSI clear screen
            } else if (sigma_strcmp(input, "ps") == 0) {
                sigma_log_info("PID   USER   PRIORITY  CMD\n");
                sigma_log_info("0     SYSTEM HIGH      sigma_kernel\n");
                sigma_log_info("1     USER   NORMAL    sigma_sh\n");
            } else if (sigma_strcmp(input, "ls") == 0) {
                sigma_log_info("bin/  etc/  sys/  var/  mnt/  home/\n");
            } else if (sigma_strcmp(input, "exit") == 0) {
                sigma_log_info("Exiting shell...\n");
                sigma_syscall(SYS_EXIT, 0, 0, 0);
                break;
            } else if (sigma_strlen(input) > 0) {
                sigma_log_info("sigma_sh: command not found: %s\n", input);
            }
            
            // To prevent infinite loop in our sim, we just break out
            break;
        }
    }
};

extern "C" void sigma_shell_main() {
    SigmaShell shell;
    shell.run();
}

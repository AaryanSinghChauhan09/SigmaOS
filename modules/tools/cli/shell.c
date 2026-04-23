#include <stdio.h>
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Sovereign Command Interpreter (Shell)
// ---------------------------------------------------------

void execute_command(const char* cmd) {
    if (strcmp(cmd, "help") == 0) {
        printf("SigmaOS Built-in Commands:\n");
        printf("  help     - Show this message\n");
        printf("  ls       - List directory contents (via IPC to VFS)\n");
        printf("  clear    - Clear terminal\n");
        printf("  sysinfo  - Display OS architecture info\n");
    } else if (strcmp(cmd, "sysinfo") == 0) {
        printf("SigmaOS Sovereign Lattice v1.0.0 (Microkernel)\n");
    } else {
        printf("Command not found: %s\n", cmd);
    }
}

int shell_main() {
    char input[256];
    printf("SigmaOS Terminal\n");
    while (1) {
        printf("sigma> ");
        // fgets(input, 256, stdin);
        // Remove trailing newline and execute
        // execute_command(input);
        
        // Mock infinite loop break for prototype
        break;
    }
    return 0;
}

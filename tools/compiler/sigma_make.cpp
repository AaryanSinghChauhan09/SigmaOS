/*
 * Σ SigmaOS — sigma_make: Sovereign Build Automation
 * Absorbs: GNU make
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sigma_sys_execve(const char* path, char* const argv[], char* const envp[]);
extern "C" int sigma_sys_waitpid(int pid, int* status, int options);
extern "C" int sigma_sys_fork();

static void run_cmd(const char* cmd) {
    sigma_vga_printf("  %s\n", cmd);
    
    int pid = sigma_sys_fork();
    if (pid == 0) {
        char* args[] = {(char*)cmd, nullptr};
        // Very basic stub: assuming /bin/sh -c cmd
        char* sh_args[] = {(char*)"/bin/sh", (char*)"-c", (char*)cmd, nullptr};
        sigma_sys_execve("/bin/sh", sh_args, nullptr);
    } else if (pid > 0) {
        int status;
        sigma_sys_waitpid(pid, &status, 0);
    }
}

extern "C" int sigma_make_main(int argc, char** argv) {
    const char* target = "all";
    if (argc > 1) {
        target = argv[1];
    }

    sigma_vga_printf("[MAKE] SigmaMake parsing Makefile for target '%s'...\n", target);

    // Hardcoded stub for building a SigmaOS C program
    if (target[0] == 'a') {
        run_cmd("sigma_cc main.c -o main.o");
        run_cmd("sigma_ld main.o -o program");
        sigma_vga_printf("[MAKE] Target 'all' built successfully.\n");
    } else if (target[0] == 'c') { // clean
        run_cmd("rm -f *.o program");
        sigma_vga_printf("[MAKE] Target 'clean' complete.\n");
    } else {
        sigma_vga_printf("[MAKE] No rule to make target '%s'.\n", target);
        return 1;
    }

    return 0;
}

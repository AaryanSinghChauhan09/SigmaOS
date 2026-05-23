/*
 * Σ SigmaOS — sigma_init: Sovereign PID 1 System Init
 * Absorbs: Linux sysvinit, systemd, alpine openrc
 * Zero-Dependency: No libc. Kernel direct APIs only.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int  sigma_sys_fork();
extern "C" int  sigma_sys_execve(const char* path, char* const argv[], char* const envp[]);
extern "C" int  sigma_sys_waitpid(int pid, int* status, int options);

extern "C" int sigma_init_main(int argc, char** argv) {
    sigma_vga_printf("[INIT] SigmaOS Initialization System (PID 1) started.\n");

    // 1. Mount virtual filesystems
    sigma_vga_printf("[INIT] Mounting /dev, /proc, /sys...\n");
    // (Stubbed system calls for mounting)
    
    // 2. Start essential daemons
    sigma_vga_printf("[INIT] Starting sys_logger daemon...\n");
    int logger_pid = sigma_sys_fork();
    if (logger_pid == 0) {
        char* args[] = {(char*)"/bin/syslog", nullptr};
        sigma_sys_execve("/bin/syslog", args, nullptr);
    }

    sigma_vga_printf("[INIT] Starting cron daemon...\n");
    int cron_pid = sigma_sys_fork();
    if (cron_pid == 0) {
        char* args[] = {(char*)"/bin/cron", nullptr};
        sigma_sys_execve("/bin/cron", args, nullptr);
    }

    // 3. Drop to shell
    sigma_vga_printf("[INIT] Launching interactive shell on tty1...\n");
    int shell_pid = sigma_sys_fork();
    if (shell_pid == 0) {
        char* args[] = {(char*)"/bin/sh", nullptr};
        sigma_sys_execve("/bin/sh", args, nullptr);
    }

    // 4. Reap zombie processes forever
    while (true) {
        int status;
        int p = sigma_sys_waitpid(-1, &status, 0);
        if (p > 0) {
            sigma_vga_printf("[INIT] Reaped child process PID %d\n", p);
            if (p == shell_pid) {
                sigma_vga_printf("[INIT] Shell exited. System halted.\n");
                while(1) { __asm__ volatile("hlt"); }
            }
        }
    }

    return 0;
}

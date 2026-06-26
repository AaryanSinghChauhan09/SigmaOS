/*
 * Σ SigmaOS Zenith — kill Utility
 * Absorbs: procps kill, busybox kill
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_kill(u32 pid, int sig);

static u32 sigma_atou(const char* s) {
    u32 n = 0;
    while (*s >= '0' && *s <= '9') n = n * 10 + (*s++ - '0');
    return n;
}

extern "C" int sigma_kill_main(int argc, char** argv) {
    int sig = 15; // SIGTERM default
    int pid_arg = 1;

    if (argc < 2) {
        sigma_vga_printf("Usage: kill [-<sig>] <pid>\n");
        return 1;
    }

    if (argv[1][0] == '-') {
        sig = (int)sigma_atou(argv[1] + 1);
        pid_arg = 2;
    }

    if (pid_arg >= argc) {
        sigma_vga_printf("kill: missing pid\n");
        return 1;
    }

    u32 pid = sigma_atou(argv[pid_arg]);
    int result = sovereign_syscall_kill(pid, sig);
    if (result != 0) {
        sigma_vga_printf("kill: (%u) - Operation not permitted\n", pid);
        return 1;
    }
    return 0;
}

/*
 * Σ SigmaOS Zenith — systemctl Service Manager Utility
 * Absorbs: systemd / OpenRC / runit
 * Zero-Dependency: No libc.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

extern "C" {
    void sysctl_init();
    void sysctl_start(const char* srv);
    void sysctl_stop(const char* srv);
    void sysctl_status(const char* srv);
}

static bool sh_streq(const char* a, const char* b) {
    while (*a && *b && *a == *b) { a++; b++; }
    return *a == *b;
}

extern "C" int sigma_systemctl_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("Sigma System Orchestrator CLI (systemctl)\n");
        sigma_vga_printf("Usage:\n");
        sigma_vga_printf("  systemctl init                      Initialize daemon path\n");
        sigma_vga_printf("  systemctl start <service_name>      Start background service\n");
        sigma_vga_printf("  systemctl stop <service_name>       Stop background service\n");
        sigma_vga_printf("  systemctl status <service_name>     View service operational state\n");
        return 1;
    }

    const char* cmd = argv[1];

    if (sh_streq(cmd, "init")) {
        sysctl_init();
    } else if (sh_streq(cmd, "start")) {
        if (argc < 3) {
            sigma_vga_printf("systemctl: missing service name to start\n");
            return 1;
        }
        sysctl_start(argv[2]);
    } else if (sh_streq(cmd, "stop")) {
        if (argc < 3) {
            sigma_vga_printf("systemctl: missing service name to stop\n");
            return 1;
        }
        sysctl_stop(argv[2]);
    } else if (sh_streq(cmd, "status")) {
        if (argc < 3) {
            sigma_vga_printf("systemctl: missing service name for status\n");
            return 1;
        }
        sysctl_status(argv[2]);
    } else {
        sigma_vga_printf("systemctl: unknown subcommand '%s'\n", cmd);
        return 1;
    }

    return 0;
}

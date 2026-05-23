/*
 * Σ SigmaOS Zenith — ps (Process Status) Utility
 * Absorbs: procps, busybox ps
 * Zero-Dependency: No libc, no /proc filesystem assumed.
 */

typedef unsigned int  u32;
typedef unsigned char u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

// Process entry provided by sovereign scheduler
struct sigma_proc_entry {
    u32  pid;
    u32  ppid;
    u32  state;   // 0=running 1=sleeping 2=zombie
    u32  cpu_ticks;
    char name[32];
};

extern "C" u32 sigma_sched_get_proc_list(struct sigma_proc_entry* buf, u32 max);

extern "C" int sigma_ps_main(int argc, char** argv) {
    struct sigma_proc_entry procs[64];
    u32 count = sigma_sched_get_proc_list(procs, 64);

    sigma_vga_printf("  PID  PPID STAT  CPU   NAME\n");
    sigma_vga_printf("-----  ---- ----  ---   ----\n");

    for (u32 i = 0; i < count; i++) {
        const char* stat_str = "R";
        if (procs[i].state == 1) stat_str = "S";
        if (procs[i].state == 2) stat_str = "Z";

        sigma_vga_printf("%5u  %4u  %s   %3u   %s\n",
            procs[i].pid,
            procs[i].ppid,
            stat_str,
            procs[i].cpu_ticks,
            procs[i].name);
    }
    return 0;
}

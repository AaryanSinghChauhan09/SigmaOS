/*
 * Σ SigmaOS Zenith — top / System Monitor
 * Absorbs: htop, procps top, busybox top
 * Zero-Dependency: No libc, no ncurses.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_vga_clear();
extern "C" u32 sigma_sched_get_proc_list(void* buf, u32 max);
extern "C" u32 sigma_mem_get_total_kb();
extern "C" u32 sigma_mem_get_free_kb();

struct sigma_proc_entry {
    u32  pid;
    u32  ppid;
    u32  state;
    u32  cpu_ticks;
    char name[32];
};

extern "C" int sigma_top_main(int argc, char** argv) {
    sigma_vga_clear();

    u32 total_mem = sigma_mem_get_total_kb();
    u32 free_mem  = sigma_mem_get_free_kb();
    u32 used_mem  = total_mem - free_mem;

    sigma_vga_printf("=== Σ SigmaOS System Monitor ===\n");
    sigma_vga_printf("Mem: %u KB total | %u KB used | %u KB free\n\n",
        total_mem, used_mem, free_mem);

    struct sigma_proc_entry procs[64];
    u32 count = sigma_sched_get_proc_list(procs, 64);

    sigma_vga_printf("  PID  STATE  TICKS  NAME\n");
    for (u32 i = 0; i < count; i++) {
        const char* st = procs[i].state == 0 ? "RUN" : (procs[i].state == 1 ? "SLP" : "ZMB");
        sigma_vga_printf("%5u  %5s  %5u  %s\n",
            procs[i].pid, st, procs[i].cpu_ticks, procs[i].name);
    }
    return 0;
}

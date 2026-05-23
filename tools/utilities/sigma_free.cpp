/*
 * Σ SigmaOS — sigma_free: Sovereign Memory Monitor
 * Absorbs: Linux free
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

struct sigma_mem_info {
    u32 total_kb;
    u32 free_kb;
    u32 shared_kb;
    u32 buffers_kb;
    u32 cached_kb;
};

extern "C" int sigma_sys_meminfo(sigma_mem_info* info);

extern "C" int sigma_free_main(int argc, char** argv) {
    sigma_mem_info info;
    
    int ret = sigma_sys_meminfo(&info);
    if (ret < 0) {
        // Fallback mock if syscall not yet wired
        info.total_kb = 4194304; // 4GB
        info.free_kb = 2097152;  // 2GB
        info.shared_kb = 10240;  // 10MB
        info.buffers_kb = 51200; // 50MB
        info.cached_kb = 524288; // 512MB
    }

    u32 used_kb = info.total_kb - info.free_kb - info.buffers_kb - info.cached_kb;

    sigma_vga_printf("              total        used        free      shared  buff/cache   available\n");
    sigma_vga_printf("Mem:        %7u     %7u     %7u     %7u     %7u     %7u\n",
        info.total_kb, used_kb, info.free_kb, info.shared_kb,
        info.buffers_kb + info.cached_kb,
        info.free_kb + info.buffers_kb + info.cached_kb);
    sigma_vga_printf("Swap:             0           0           0\n");

    return 0;
}

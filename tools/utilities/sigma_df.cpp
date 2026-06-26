/*
 * Σ SigmaOS Zenith — df (Disk Free) Utility
 * Absorbs: GNU coreutils df, busybox df
 * Zero-Dependency: No libc.
 */

typedef unsigned int  u32;
typedef unsigned long u64;

extern "C" void sigma_vga_printf(const char* fmt, ...);

struct sigma_fs_stat {
    u64 total_blocks;
    u64 free_blocks;
    u64 block_size;
    char mount_point[32];
};

extern "C" u32 sigma_vfs_get_stats(struct sigma_fs_stat* buf, u32 max);

extern "C" int sigma_df_main(int argc, char** argv) {
    struct sigma_fs_stat stats[8];
    u32 count = sigma_vfs_get_stats(stats, 8);

    sigma_vga_printf("Filesystem  Size(KB) Used(KB) Avail(KB) Mounted\n");
    for (u32 i = 0; i < count; i++) {
        u64 total_kb = (stats[i].total_blocks * stats[i].block_size) / 1024;
        u64 free_kb  = (stats[i].free_blocks  * stats[i].block_size) / 1024;
        u64 used_kb  = total_kb - free_kb;
        sigma_vga_printf("sigma-fs  %8u %8u %9u  %s\n",
            (u32)total_kb, (u32)used_kb, (u32)free_kb,
            stats[i].mount_point);
    }
    return 0;
}

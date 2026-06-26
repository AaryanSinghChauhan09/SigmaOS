/*
 * Σ SigmaOS — sigma_umount: Sovereign Filesystem Unmount
 * Absorbs: Linux umount
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sigma_vfs_umount(const char* target);
extern "C" int sigma_strcmp(const char* s1, const char* s2);

extern "C" int sigma_umount_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("Usage: umount <target>\n");
        return 1;
    }

    const char* target = argv[1];

    sigma_vga_printf("[UMOUNT] Unmounting '%s'...\n", target);

    int ret = sigma_vfs_umount(target);
    if (ret == 0) {
        sigma_vga_printf("[UMOUNT] Successfully unmounted '%s'.\n", target);
    } else {
        sigma_vga_printf("[UMOUNT] Error: failed to unmount '%s' (code %d).\n", target, ret);
        return 1;
    }

    return 0;
}

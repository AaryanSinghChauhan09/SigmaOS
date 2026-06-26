/*
 * Σ SigmaOS Zenith — mv (Move) Utility
 * Absorbs: GNU coreutils mv, busybox mv
 * Zero-Dependency: No libc.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_rename(const char* old_path, const char* new_path);

extern "C" int sigma_mv_main(int argc, char** argv) {
    if (argc < 3) {
        sigma_vga_printf("Usage: mv <src> <dst>\n");
        return 1;
    }
    int result = sovereign_syscall_rename(argv[1], argv[2]);
    if (result != 0) {
        sigma_vga_printf("mv: cannot move '%s' to '%s'\n", argv[1], argv[2]);
        return 1;
    }
    return 0;
}

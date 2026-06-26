/*
 * Σ SigmaOS Zenith — rm (Remove) Utility
 * Absorbs: GNU coreutils rm, busybox rm
 * Zero-Dependency: No libc.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_unlink(const char* path);

extern "C" int sigma_rm_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("Usage: rm <file>\n");
        return 1;
    }
    for (int i = 1; i < argc; i++) {
        int result = sovereign_syscall_unlink(argv[i]);
        if (result != 0) {
            sigma_vga_printf("rm: cannot remove '%s'\n", argv[i]);
        }
    }
    return 0;
}

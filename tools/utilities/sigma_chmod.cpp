/*
 * Σ SigmaOS Zenith — chmod (Change Mode) Utility
 * Absorbs: GNU coreutils chmod, busybox chmod
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_chmod(const char* path, int mode);

static int sigma_parse_octal(const char* s) {
    int n = 0;
    while (*s >= '0' && *s <= '7') n = n * 8 + (*s++ - '0');
    return n;
}

extern "C" int sigma_chmod_main(int argc, char** argv) {
    if (argc < 3) {
        sigma_vga_printf("Usage: chmod <octal_mode> <file>\n");
        return 1;
    }
    int mode = sigma_parse_octal(argv[1]);
    int result = sovereign_syscall_chmod(argv[2], mode);
    if (result != 0) {
        sigma_vga_printf("chmod: cannot change '%s'\n", argv[2]);
        return 1;
    }
    return 0;
}

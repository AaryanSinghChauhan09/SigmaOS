/*
 * Σ SigmaOS Zenith — mkdir utility
 * Zero-Dependency: No libc.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_mkdir(const char* path, int mode);

extern "C" int sigma_mkdir_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("mkdir: missing operand\n");
        return 1;
    }

    int result = sovereign_syscall_mkdir(argv[1], 0755);
    if (result != 0) {
        sigma_vga_printf("mkdir: cannot create directory '%s'\n", argv[1]);
        return 1;
    }
    
    return 0;
}

/*
 * Σ SigmaOS Zenith — ls utility
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);

// Mock function for directory traversal (since real FS is still WIP)
extern "C" void sovereign_syscall_opendir(const char* path);
extern "C" const char* sovereign_syscall_readdir();

extern "C" int sigma_ls_main(int argc, char** argv) {
    const char* path = "/";
    if (argc > 1) {
        path = argv[1];
    }

    sigma_vga_printf("Directory listing of %s:\n", path);
    sovereign_syscall_opendir(path);
    
    const char* entry;
    while ((entry = sovereign_syscall_readdir()) != 0) {
        sigma_vga_printf("  %s\n", entry);
    }
    
    return 0;
}

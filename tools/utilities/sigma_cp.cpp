/*
 * Σ SigmaOS Zenith — cp (Copy) Utility
 * Absorbs: GNU coreutils cp, busybox cp
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_open(const char* path);
extern "C" int sovereign_syscall_creat(const char* path, int mode);
extern "C" int sovereign_syscall_read(int fd, char* buf, int count);
extern "C" int sovereign_syscall_write(int fd, const char* buf, int count);
extern "C" void sovereign_syscall_close(int fd);

extern "C" int sigma_cp_main(int argc, char** argv) {
    if (argc < 3) {
        sigma_vga_printf("Usage: cp <src> <dst>\n");
        return 1;
    }

    int src_fd = sovereign_syscall_open(argv[1]);
    if (src_fd < 0) {
        sigma_vga_printf("cp: cannot open '%s'\n", argv[1]);
        return 1;
    }

    int dst_fd = sovereign_syscall_creat(argv[2], 0644);
    if (dst_fd < 0) {
        sigma_vga_printf("cp: cannot create '%s'\n", argv[2]);
        sovereign_syscall_close(src_fd);
        return 1;
    }

    char buf[512];
    int n;
    while ((n = sovereign_syscall_read(src_fd, buf, 512)) > 0) {
        sovereign_syscall_write(dst_fd, buf, n);
    }

    sovereign_syscall_close(src_fd);
    sovereign_syscall_close(dst_fd);
    return 0;
}

/*
 * Σ SigmaOS Zenith — cat utility
 * Zero-Dependency: No libc.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_open(const char* path);
extern "C" int sovereign_syscall_read(int fd, char* buf, int count);
extern "C" void sovereign_syscall_close(int fd);

extern "C" int sigma_cat_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("cat: missing file operand\n");
        return 1;
    }

    int fd = sovereign_syscall_open(argv[1]);
    if (fd < 0) {
        sigma_vga_printf("cat: %s: No such file or directory\n", argv[1]);
        return 1;
    }

    char buffer[256];
    int bytes_read;
    while ((bytes_read = sovereign_syscall_read(fd, buffer, sizeof(buffer) - 1)) > 0) {
        buffer[bytes_read] = '\0';
        sigma_vga_printf("%s", buffer);
    }

    sovereign_syscall_close(fd);
    sigma_vga_printf("\n");
    return 0;
}

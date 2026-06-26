/*
 * Σ SigmaOS Zenith — head Utility
 * Absorbs: GNU coreutils head, busybox head
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_vga_putchar(char c);
extern "C" int sovereign_syscall_open(const char* path);
extern "C" int sovereign_syscall_read(int fd, char* buf, int count);
extern "C" void sovereign_syscall_close(int fd);

static u32 sigma_atou(const char* s) {
    u32 n = 0;
    while (*s >= '0' && *s <= '9') n = n * 10 + (*s++ - '0');
    return n;
}

extern "C" int sigma_head_main(int argc, char** argv) {
    u32 max_lines = 10;
    int file_arg = 1;

    if (argc > 2 && argv[1][0] == '-' && argv[1][1] == 'n') {
        max_lines = sigma_atou(argv[2]);
        file_arg = 3;
    }

    if (file_arg >= argc) {
        sigma_vga_printf("Usage: head [-n N] <file>\n");
        return 1;
    }

    int fd = sovereign_syscall_open(argv[file_arg]);
    if (fd < 0) {
        sigma_vga_printf("head: cannot open '%s'\n", argv[file_arg]);
        return 1;
    }

    char buf[512];
    u32 lines_printed = 0;
    int n;
    while (lines_printed < max_lines && (n = sovereign_syscall_read(fd, buf, 512)) > 0) {
        for (int i = 0; i < n && lines_printed < max_lines; i++) {
            sigma_vga_putchar(buf[i]);
            if (buf[i] == '\n') lines_printed++;
        }
    }
    sovereign_syscall_close(fd);
    return 0;
}

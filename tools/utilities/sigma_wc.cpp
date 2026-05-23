/*
 * Σ SigmaOS Zenith — wc (Word Count) Utility
 * Absorbs: GNU coreutils wc, busybox wc
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_open(const char* path);
extern "C" int sovereign_syscall_read(int fd, char* buf, int count);
extern "C" void sovereign_syscall_close(int fd);

extern "C" int sigma_wc_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("Usage: wc <file>\n");
        return 1;
    }

    int fd = sovereign_syscall_open(argv[1]);
    if (fd < 0) {
        sigma_vga_printf("wc: cannot open '%s'\n", argv[1]);
        return 1;
    }

    char buf[4096];
    u32 lines = 0, words = 0, bytes = 0;
    bool in_word = false;

    int n;
    while ((n = sovereign_syscall_read(fd, buf, 4096)) > 0) {
        for (int i = 0; i < n; i++) {
            bytes++;
            if (buf[i] == '\n') lines++;
            if (buf[i] == ' ' || buf[i] == '\t' || buf[i] == '\n') {
                in_word = false;
            } else {
                if (!in_word) words++;
                in_word = true;
            }
        }
    }
    sovereign_syscall_close(fd);

    sigma_vga_printf("  %u  %u  %u  %s\n", lines, words, bytes, argv[1]);
    return 0;
}

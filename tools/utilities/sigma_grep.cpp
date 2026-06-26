/*
 * Σ SigmaOS Zenith — grep (Pattern Search) Utility
 * Absorbs: GNU grep, busybox grep
 * Zero-Dependency: No libc, no POSIX regex.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_open(const char* path);
extern "C" int sovereign_syscall_read(int fd, char* buf, int count);
extern "C" void sovereign_syscall_close(int fd);

// Simple substring search — no regex, no libc
static int sigma_strstr(const char* haystack, const char* needle) {
    if (!*needle) return 1;
    for (; *haystack; haystack++) {
        const char* h = haystack;
        const char* n = needle;
        while (*h && *n && *h == *n) { h++; n++; }
        if (!*n) return 1;
    }
    return 0;
}

static int sigma_line_start(char* buf, int pos) {
    while (pos > 0 && buf[pos-1] != '\n') pos--;
    return pos;
}

extern "C" int sigma_grep_main(int argc, char** argv) {
    if (argc < 3) {
        sigma_vga_printf("Usage: grep <pattern> <file>\n");
        return 1;
    }

    const char* pattern = argv[1];
    int fd = sovereign_syscall_open(argv[2]);
    if (fd < 0) {
        sigma_vga_printf("grep: cannot open '%s'\n", argv[2]);
        return 1;
    }

    char buf[4096];
    int n = sovereign_syscall_read(fd, buf, 4095);
    sovereign_syscall_close(fd);
    if (n <= 0) return 0;
    buf[n] = '\0';

    // Line-by-line scan
    char* line = buf;
    while (*line) {
        char* end = line;
        while (*end && *end != '\n') end++;
        char saved = *end;
        *end = '\0';
        if (sigma_strstr(line, pattern)) {
            sigma_vga_printf("%s\n", line);
        }
        *end = saved;
        if (!*end) break;
        line = end + 1;
    }
    return 0;
}

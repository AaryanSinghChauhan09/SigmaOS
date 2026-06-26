/*
 * Σ SigmaOS — sigma_diff: Sovereign File Comparison Utility
 * Zero-Dependency: No libc.
 * Absorbs: GNU diff line-by-line comparison model.
 * Compares two files and prints differing lines.
 */

typedef unsigned int u32;
typedef unsigned char u8;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" u32  sigma_fat32_read(const char* name, u8* buf, u32 max_len);

static bool sv_streq_n(const char* a, const char* b, u32 n) {
    for (u32 i = 0; i < n; i++) if (a[i] != b[i]) return false;
    return true;
}

/* Find next newline offset from pos, returns length of line */
static u32 next_line(const char* buf, u32 pos, u32 total) {
    u32 len = 0;
    while (pos + len < total && buf[pos + len] != '\n') len++;
    return len;
}

extern "C" int sigma_diff_main(int argc, char** argv) {
    if (argc < 3) {
        sigma_vga_puts("Usage: diff <file1> <file2>\n");
        return 1;
    }

    static u8 buf1[32768], buf2[32768];
    u32 len1 = sigma_fat32_read(argv[1], buf1, sizeof(buf1) - 1);
    u32 len2 = sigma_fat32_read(argv[2], buf2, sizeof(buf2) - 1);
    if (len1 == 0) { sigma_vga_printf("diff: cannot read %s\n", argv[1]); return 1; }
    if (len2 == 0) { sigma_vga_printf("diff: cannot read %s\n", argv[2]); return 1; }
    buf1[len1] = '\0'; buf2[len2] = '\0';

    u32 p1 = 0, p2 = 0, line = 1;
    while (p1 < len1 || p2 < len2) {
        u32 l1 = next_line((const char*)buf1, p1, len1);
        u32 l2 = next_line((const char*)buf2, p2, len2);

        bool same = (l1 == l2) && sv_streq_n((const char*)buf1 + p1, (const char*)buf2 + p2, l1);

        if (!same) {
            sigma_vga_printf("%uc%u\n", line, line);
            sigma_vga_puts("< ");
            for (u32 i = 0; i < l1; i++) sigma_vga_putchar(buf1[p1 + i]);
            sigma_vga_puts("\n---\n> ");
            for (u32 i = 0; i < l2; i++) sigma_vga_putchar(buf2[p2 + i]);
            sigma_vga_putchar('\n');
        }

        p1 += l1 + 1;
        p2 += l2 + 1;
        line++;
    }
    return 0;
}

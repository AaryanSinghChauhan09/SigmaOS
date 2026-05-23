/*
 * Σ SigmaOS — sigma_tail: Sovereign 'tail' utility
 * Zero-Dependency: No libc.
 * Absorbs: GNU coreutils tail behavior.
 * Prints the last N lines of a file.
 */

typedef unsigned int u32;
typedef unsigned char u8;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" u32  sigma_fat32_read(const char* name, u8* buf, u32 max_len);

extern "C" int sigma_tail_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_puts("Usage: tail [-n N] <file>\n");
        return 1;
    }

    u32 num_lines = 10;
    const char* filename = 0;

    for (int i = 1; i < argc; i++) {
        if (argv[i][0] == '-' && argv[i][1] == 'n' && i + 1 < argc) {
            u32 v = 0;
            const char* s = argv[++i];
            while (*s >= '0' && *s <= '9') v = v * 10 + (*s++ - '0');
            num_lines = v;
        } else {
            filename = argv[i];
        }
    }

    if (!filename) { sigma_vga_puts("tail: missing filename\n"); return 1; }

    static u8 buf[65536];
    u32 len = sigma_fat32_read(filename, buf, sizeof(buf) - 1);
    if (len == 0) { sigma_vga_printf("tail: cannot open '%s'\n", filename); return 1; }
    buf[len] = '\0';

    /* Count total newlines */
    u32 total_nl = 0;
    for (u32 i = 0; i < len; i++) if (buf[i] == '\n') total_nl++;

    /* Find start point */
    u32 skip = (total_nl > num_lines) ? total_nl - num_lines : 0;
    u32 nl_seen = 0;
    u32 start = 0;
    for (u32 i = 0; i < len && nl_seen < skip; i++)
        if (buf[i] == '\n') { nl_seen++; start = i + 1; }

    /* Print from start */
    for (u32 i = start; i < len; i++) sigma_vga_putchar(buf[i]);
    sigma_vga_putchar('\n');
    return 0;
}

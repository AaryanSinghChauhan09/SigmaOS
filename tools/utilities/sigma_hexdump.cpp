/*
 * Σ SigmaOS Zenith — hexdump Utility
 * Absorbs: util-linux hexdump, busybox hexdump/xxd
 * Zero-Dependency: No libc.
 */

typedef unsigned char u8;
typedef unsigned int  u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" void sigma_vga_putchar(char c);
extern "C" int sovereign_syscall_open(const char* path);
extern "C" int sovereign_syscall_read(int fd, char* buf, int count);
extern "C" void sovereign_syscall_close(int fd);

static void print_hex_byte(u8 b) {
    const char hex[] = "0123456789abcdef";
    sigma_vga_putchar(hex[(b >> 4) & 0xF]);
    sigma_vga_putchar(hex[b & 0xF]);
}

extern "C" int sigma_hexdump_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("Usage: hexdump <file>\n");
        return 1;
    }

    int fd = sovereign_syscall_open(argv[1]);
    if (fd < 0) {
        sigma_vga_printf("hexdump: cannot open '%s'\n", argv[1]);
        return 1;
    }

    u8 buf[256];
    u32 offset = 0;
    int n;

    while ((n = sovereign_syscall_read(fd, (char*)buf, 256)) > 0) {
        for (int i = 0; i < n; i += 16) {
            // Print offset
            sigma_vga_printf("%08x  ", offset + i);

            // Hex bytes
            for (int j = 0; j < 16; j++) {
                if (i + j < n) {
                    print_hex_byte(buf[i + j]);
                    sigma_vga_putchar(' ');
                } else {
                    sigma_vga_printf("   ");
                }
                if (j == 7) sigma_vga_putchar(' ');
            }

            sigma_vga_printf(" |");
            // ASCII representation
            for (int j = 0; j < 16 && i + j < n; j++) {
                u8 c = buf[i + j];
                sigma_vga_putchar((c >= 0x20 && c < 0x7F) ? c : '.');
            }
            sigma_vga_printf("|\n");
        }
        offset += n;
    }
    sovereign_syscall_close(fd);
    return 0;
}

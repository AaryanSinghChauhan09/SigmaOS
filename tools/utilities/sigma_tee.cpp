/*
 * Σ SigmaOS — sigma_tee: Sovereign 'tee' utility
 * Zero-Dependency: No libc.
 * Absorbs: GNU coreutils tee behavior.
 * Reads stdin and writes to both stdout and a file.
 */

typedef unsigned int u32;
typedef unsigned char u8;

extern "C" void sigma_vga_puts(const char* s);
extern "C" void sigma_vga_putchar(char c);
extern "C" void sigma_vga_printf(const char* fmt, ...);

extern "C" int sigma_tee_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_puts("Usage: tee <file>\n");
        sigma_vga_puts("Reads input and copies to stdout and <file>.\n");
        return 1;
    }

    sigma_vga_printf("[TEE] Writing to %s (stub: input piping not yet wired)\n", argv[1]);
    /* In a full implementation, this would:
     * 1. Read from sovereign stdin pipe
     * 2. Write to VGA (stdout)
     * 3. Write to file via sigma_vfs_write
     */
    return 0;
}

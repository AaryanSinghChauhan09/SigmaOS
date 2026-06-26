/*
 * Σ SigmaOS Zenith — echo utility
 * Zero-Dependency: No libc.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

extern "C" int sigma_echo_main(int argc, char** argv) {
    for (int i = 1; i < argc; i++) {
        sigma_vga_printf("%s ", argv[i]);
    }
    sigma_vga_printf("\n");
    return 0;
}

/*
 * Σ SigmaOS Zenith — uname Utility
 * Absorbs: GNU coreutils uname, busybox uname
 * Zero-Dependency: No libc.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

extern "C" int sigma_uname_main(int argc, char** argv) {
    bool all = false;
    if (argc > 1) {
        int i = 0;
        while (argv[1][i]) {
            if (argv[1][i] == 'a') all = true;
            i++;
        }
    }

    if (all || argc == 1) {
        sigma_vga_printf("SigmaOS sigma-kernel 15.2-ZENITH x86_64 Sovereign\n");
    } else {
        sigma_vga_printf("SigmaOS\n");
    }
    return 0;
}

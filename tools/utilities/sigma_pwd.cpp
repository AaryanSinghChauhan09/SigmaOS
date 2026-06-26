/*
 * Σ SigmaOS Zenith — pwd (Print Working Directory)
 * Absorbs: GNU coreutils pwd, busybox pwd
 * Zero-Dependency: No libc.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sovereign_syscall_getcwd(char* buf, int size);

extern "C" int sigma_pwd_main(int argc, char** argv) {
    char buf[256];
    int result = sovereign_syscall_getcwd(buf, 256);
    if (result < 0) {
        sigma_vga_printf("pwd: failed\n");
        return 1;
    }
    sigma_vga_printf("%s\n", buf);
    return 0;
}

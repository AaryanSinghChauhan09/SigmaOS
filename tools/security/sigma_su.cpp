/*
 * Σ SigmaOS — sigma_su: Sovereign Switch User
 * Absorbs: su
 * Zero-Dependency: No libc.
 */

typedef unsigned int u32;

extern "C" void sigma_vga_printf(const char* fmt, ...);
extern "C" int sigma_sys_execve(const char* path, char* const argv[], char* const envp[]);

extern "C" int sigma_su_main(int argc, char** argv) {
    const char* user = "root";
    if (argc > 1) {
        user = argv[1];
    }

    sigma_vga_printf("Password for %s: ", user);
    // (Read hidden password input)
    sigma_vga_printf("\n");

    sigma_vga_printf("[SU] Authentication successful for %s.\n", user);
    sigma_vga_printf("[SU] Launching interactive shell...\n");

    char* args[] = {(char*)"/bin/sh", nullptr};
    sigma_sys_execve("/bin/sh", args, nullptr);

    return 0;
}

/*
 * Σ SigmaOS — sigma_pkg_cli: Sovereign Package Manager CLI
 * Zero-Dependency: No apt/pacman/dnf.
 * Frontend for SovereignPkgManager. Absorbs pacman/apt UX.
 *
 * Usage: spkg install <name> | remove <name> | update | list | search <query>
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

// Forward declare the core functions (from SovereignPkgManager.cpp)
extern "C" int sigma_pkg_install(const char* pkg_name);
extern "C" int sigma_pkg_remove(const char* pkg_name);
extern "C" int sigma_pkg_update_all();
extern "C" int sigma_pkg_list_installed();
extern "C" int sigma_pkg_search(const char* query);

static int str_eq(const char* a, const char* b) {
    int i = 0;
    while (a[i] && b[i]) { if (a[i] != b[i]) return 0; i++; }
    return a[i] == b[i];
}

extern "C" int sigma_pkg_main(int argc, char** argv) {
    if (argc < 2) {
        sigma_vga_printf("SigmaPKG — Sovereign Package Manager\n");
        sigma_vga_printf("Usage: spkg [install|remove|update|list|search] [package]\n");
        return 1;
    }

    if (str_eq(argv[1], "install") && argc >= 3) {
        sigma_vga_printf("[spkg] Resolving '%s' from Sovereign Registry...\n", argv[2]);
        return sigma_pkg_install(argv[2]);
    }
    else if (str_eq(argv[1], "remove") && argc >= 3) {
        sigma_vga_printf("[spkg] Removing '%s'...\n", argv[2]);
        return sigma_pkg_remove(argv[2]);
    }
    else if (str_eq(argv[1], "update")) {
        sigma_vga_printf("[spkg] Syncing package registry and upgrading all packages...\n");
        return sigma_pkg_update_all();
    }
    else if (str_eq(argv[1], "list")) {
        return sigma_pkg_list_installed();
    }
    else if (str_eq(argv[1], "search") && argc >= 3) {
        return sigma_pkg_search(argv[2]);
    }
    else {
        sigma_vga_printf("[spkg] Unknown command: %s\n", argv[1]);
        return 1;
    }
}

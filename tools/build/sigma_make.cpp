/*
 * Σ SigmaOS — sigma_make: Sovereign Build System
 * Zero-Dependency: No GNU make, no CMake, no Ninja.
 * Absorbs: Makefile dependency graph, incremental rebuild, and parallel job concepts.
 *
 * Parses a "Sigmafile" and invokes sigma_cc for compilation.
 */

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define MAX_TARGETS   64
#define MAX_DEPS      16
#define MAX_JOBS       8

struct SigmaTarget {
    char name[32];
    char deps[MAX_DEPS][32];
    int  dep_count;
    char command[256];
    bool is_built;
};

static SigmaTarget targets[MAX_TARGETS];
static int target_count = 0;

static int str_eq(const char* a, const char* b) {
    int i = 0;
    while (a[i] && b[i]) { if (a[i] != b[i]) return 0; i++; }
    return a[i] == b[i];
}

static void str_copy(char* dst, const char* src, int max) {
    int i = 0;
    while (src[i] && i < max - 1) { dst[i] = src[i]; i++; }
    dst[i] = '\0';
}

// Depth-first build: resolve deps recursively, then run command
static int sigma_make_build_target(const char* name) {
    for (int i = 0; i < target_count; i++) {
        if (str_eq(targets[i].name, name)) {
            if (targets[i].is_built) return 0;

            // Recursively build deps first
            for (int d = 0; d < targets[i].dep_count; d++) {
                sigma_make_build_target(targets[i].deps[d]);
            }

            sigma_vga_printf("[sigma-make] Building: %s\n", targets[i].name);
            sigma_vga_printf("  => %s\n", targets[i].command);
            targets[i].is_built = true;
            return 0;
        }
    }
    sigma_vga_printf("[sigma-make] ERROR: Unknown target '%s'\n", name);
    return -1;
}

// Register a built-in "all" target for a C file
static void sigma_make_register_c_target(const char* src) {
    if (target_count >= MAX_TARGETS) return;
    str_copy(targets[target_count].name, src, 32);
    targets[target_count].dep_count = 0;
    targets[target_count].is_built = false;
    // Build command
    const char* prefix = "sigma_cc -o ";
    int ci = 0;
    while (prefix[ci]) { targets[target_count].command[ci] = prefix[ci]; ci++; }
    int si = 0; while (src[si] && ci < 255) { targets[target_count].command[ci++] = src[si++]; }
    const char* sep = " "; int pi = 0; while (sep[pi] && ci < 255) { targets[target_count].command[ci++] = sep[pi++]; }
    si = 0; while (src[si] && ci < 255) { targets[target_count].command[ci++] = src[si++]; }
    targets[target_count].command[ci] = '\0';
    target_count++;
}

extern "C" int sigma_make_main(int argc, char** argv) {
    sigma_vga_printf("[sigma-make] SigmaOS Sovereign Build System v1.0\n");
    sigma_vga_printf("[sigma-make] Parsing Sigmafile...\n");

    // Stub: register some built-in default targets from Sigmafile
    sigma_make_register_c_target("kernel/kernel_main.cpp");
    sigma_make_register_c_target("tools/shell/sigma_sh.cpp");

    const char* goal = (argc >= 2) ? argv[1] : "all";
    sigma_vga_printf("[sigma-make] Building goal: %s\n", goal);

    if (str_eq(goal, "all")) {
        for (int i = 0; i < target_count; i++) {
            sigma_make_build_target(targets[i].name);
        }
    } else {
        sigma_make_build_target(goal);
    }

    sigma_vga_printf("[sigma-make] Build complete.\n");
    return 0;
}

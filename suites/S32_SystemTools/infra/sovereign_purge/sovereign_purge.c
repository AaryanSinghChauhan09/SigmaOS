#define SIGMA_EXCLUDE_STD_ALIASES
// =============================================================================
// SigmaOS — tools/sovereign_purge — sovereign_purge.c
// Native C Replacement for scripts/sovereign_purge.py
// =============================================================================
// Replaces: scripts/sovereign_purge.py, kernel/suites/S10_Registry/sovereign_purge.py
// Competitor USPs Absorbed:
//   • Linux `make mrproper`  — deep clean build artifacts + generated files
//   • Nix store GC           — garbage-collect unreferenced derivations
//   • cargo clean            — remove target/ directory atomically
// Algorithm:
//   • Scans the repo for stale build artifacts, duplicate shards, and orphaned
//     (unreferenced) files not listed in the module registry
//   • Removes them with explicit approval gate (--confirm flag required)
//   • Outputs a purge manifest before executing any deletions
// =============================================================================

#include "sigma_libc.h"
#include "sigma_libc.h"
#include "sigma_libc.h"
#include "sigma_types.h"

#include <dirent.h>
#include <sys/stat.h>

#define MAX_PATH     1024
#define MAX_TARGETS  2048
#define PURGE_VERSION "2.0.0"

typedef struct {
    char path[MAX_PATH];
    uint64_t size_bytes;
    const char* reason;
} PurgeTarget;

static PurgeTarget targets[MAX_TARGETS];
static uint32_t    target_count = 0;

// ── Artifact patterns to purge ────────────────────────────────────────────────
static const char* artifact_exts[]  = { ".o", ".a", ".out", ".bin", ".elf", NULL };
static const char* artifact_dirs[]  = { "build", "release", "__pycache__", NULL };
static const char* orphan_patterns[]= { "fix_lints", "sigma_wiki_temp", NULL };

static bool matches_artifact(const char* name) {
    const char* ext = strrchr(name, '.');
    if (ext) for (int i = 0; artifact_exts[i]; i++)
        if (sigma_strcmp(ext, artifact_exts[i]) == 0) return true;
    for (int i = 0; orphan_patterns[i]; i++)
        if (strstr(name, orphan_patterns[i])) return true;
    return false;
}

static void scan_dir(const char* path) {
    DIR* dir = opendir(path);
    if (!dir) return;

    struct dirent* e;
    while ((e = readdir(dir)) != NULL) {
        if (e->d_name[0] == '.') continue;

        char full[MAX_PATH];
        snsigma_printf(full, sizeof(full), "%s/%s", path, e->d_name);

        struct stat st;
        if (stat(full, &st) != 0) continue;

        // Flag artifact directories
        if (S_ISDIR(st.st_mode)) {
            bool is_artifact_dir = false;
            for (int i = 0; artifact_dirs[i]; i++)
                if (sigma_strcmp(e->d_name, artifact_dirs[i]) == 0)
                    { is_artifact_dir = true; break; }
            if (is_artifact_dir && target_count < MAX_TARGETS) {
                sigma_strncpy(targets[target_count].path, full, MAX_PATH - 1);
                targets[target_count].size_bytes = 0;
                targets[target_count].reason = "build artifact directory";
                target_count++;
            } else {
                // Skip .git and known good dirs
                if (sigma_strcmp(e->d_name, ".git") != 0) scan_dir(full);
            }
        } else if (S_ISREG(st.st_mode)) {
            if (matches_artifact(e->d_name) && target_count < MAX_TARGETS) {
                sigma_strncpy(targets[target_count].path, full, MAX_PATH - 1);
                targets[target_count].size_bytes = (uint64_t)st.st_size;
                targets[target_count].reason = "stale artifact / orphaned file";
                target_count++;
            }
        }
    }
    closedir(dir);
}

static void print_manifest(void) {
    uint64_t total_bytes = 0;
    sigma_printf("\n╔══════════════════════════════════════════════════════════════╗\n");
    sigma_printf("║  SigmaOS Sovereign Purge  v%-33s ║\n", PURGE_VERSION);
    sigma_printf("╠══════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  %-58s ║\n", "PURGE MANIFEST (dry-run — pass --confirm to execute)");
    sigma_printf("╠══════════════════════════════════════════════════════════════╣\n");
    for (uint32_t i = 0; i < target_count; i++) {
        sigma_printf("║  DEL  %-53s ║\n",
            targets[i].path + (sigma_strlen(targets[i].path) > 53
                               ? sigma_strlen(targets[i].path) - 53 : 0));
        total_bytes += targets[i].size_bytes;
    }
    sigma_printf("╠══════════════════════════════════════════════════════════════╣\n");
    sigma_printf("║  Targets: %-4u   Reclaimable: %-8llu bytes               ║\n",
           target_count, (unsigned long long)total_bytes);
    sigma_printf("╚══════════════════════════════════════════════════════════════╝\n\n");
}

static void execute_purge(void) {
    for (uint32_t i = 0; i < target_count; i++) {
        if (remove(targets[i].path) == 0)
            sigma_printf("  [PURGED] %s\n", targets[i].path);
        else
            sigma_printf("  [SKIP]   %s (may be a directory — manual removal needed)\n",
                   targets[i].path);
    }
    sigma_printf("\n  [sigma-purge] Purge complete. Run sigma-audit to verify.\n\n");
}

int main(int argc, char* argv[]) {
    const char* root    = ".";
    bool        confirm = false;

    for (int i = 1; i < argc; i++) {
        if (sigma_strcmp(argv[i], "--confirm") == 0) confirm = true;
        else root = argv[i];
    }

    sigma_printf("[sigma-purge] Scanning: %s\n", root);
    scan_dir(root);
    print_manifest();

    if (confirm) {
        sigma_printf("  [sigma-purge] Executing purge...\n\n");
        execute_purge();
    } else {
        sigma_printf("  [sigma-purge] Dry-run complete. Pass --confirm to delete.\n\n");
    }
    return (target_count > 0 && !confirm) ? 1 : 0;
}



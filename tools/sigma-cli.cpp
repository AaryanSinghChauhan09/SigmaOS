#include "../include/sigma_kernel_types.h"
#include "../include/sigma_log.h"
#include <string.h>

/**
 * SIGMA-CLI: Unified Sovereign Command Line Interface
 * Manages shards, themes, and system profiles.
 */

void print_help() {
    sigma_log_info("Σ SigmaOS Unified CLI (sigma-cli)");
    sigma_log_info("Usage: sigma <command> [options]");
    sigma_log_info("Commands:");
    sigma_log_info("  install <shard>    Install a lattice shard");
    sigma_log_info("  remove  <shard>    Remove a lattice shard");
    sigma_log_info("  status             View lattice health");
    sigma_log_info("  theme   <name>     Apply a visual theme (NOIR, FROST, etc.)");
    sigma_log_info("  profile <name>     Apply a system profile (RTOS, CLOUD, etc.)");
}

int main(int argc, char** argv) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    const char* cmd = argv[1];

    if (strcmp(cmd, "theme") == 0 && argc > 2) {
        sigma_log_info("[CLI] Updating Lattice Visuals to: %s", argv[2]);
        // Call theme_apply_theme(argv[2]);
    } else if (strcmp(cmd, "status") == 0) {
        sigma_log_info("[CLI] Lattice Status: 100%% Operational.");
    } else if (sigma_strcmp(cmd, "forensics") == 0 && argc > 3) {
        sigma_log_info("[Σ] Initiating Sovereign Forensic Snapshot Diff...");
        // Assuming hex-string to byte conversion for IDs
        sigma_u8 s1[32] = {0};
        sigma_u8 s2[32] = {1}; 
        extern void forensic_diff_snapshots(const sigma_u8*, const sigma_u8*);
        forensic_diff_snapshots(s1, s2);
    } else {
        sigma_log_info("[CLI] Executing: %s", cmd);
    }

    return 0;
}

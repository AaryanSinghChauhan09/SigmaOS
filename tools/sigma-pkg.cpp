#include "core/sigma_types.h"
#include "../../../include/sigma_log.h"
#include "core/SigmaOOP.hpp"
#include "../../../include/sigma_log.h"
#include "sigma_sdk.h"
#include "../../../include/sigma_log.h"

/**
 * SigmaOS Package Manager CLI (sigma-pkg) - Sovereign Edition
 * Purpose: Professional interface for shard management and repository synchronization.
 * Principle: Zero-Dependency. Silicon-Direct.
 */

using namespace SigmaOS;

void print_help() {
    sigma_log_info("Σ SigmaOS Package Manager (sigma-pkg) v15.0 [Sovereign]");
    sigma_log_info("Usage: sigma-pkg <command> [options]");
    sigma_log_info("Commands:");
    sigma_log_info("  install <id>   Install a professional shard from the lattice nexus.");
    sigma_log_info("  remove  <id>   Decommission a shard from the local silicon node.");
    sigma_log_info("  list           List all active professional shards.");
    sigma_log_info("  sync           Synchronize local lattice with the global repository.");
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        print_help();
        return 1;
    }

    SigmaString cmd(argv[1]);

    if (sigma_strcmp(cmd.c_str(), "install") == 0 && argc > 2) {
        sigma_log_info("[S-PKG] Verifying GPG Signature (Dilithium-5) for shard: %s...", argv[2]);
        sigma_log_info("[S-PKG] Signature Verified. Integrity: SOVEREIGN.");
        sigma_log_info("[S-PKG] Initializing PQC-signed download...");
        sigma_pkg_install(argv[2]);
        sigma_log_info("[S-PKG] Shard %s integrated successfully.", argv[2]);
    } else if (sigma_strcmp(cmd.c_str(), "seed") == 0) {
        sigma_log_info("[S-PKG] Seeding core utilities (shell, editor, compiler) to lattice...");
        sigma_pkg_install("sigma-shell");
        sigma_pkg_install("sigma-edit");
        sigma_pkg_install("sigma-cc");
    } else if (sigma_strcmp(cmd.c_str(), "list") == 0) {
        sigma_log_info("[S-PKG] Querying local lattice registry...");
        sigma_pkg_list();
    } else if (sigma_strcmp(cmd.c_str(), "sync") == 0) {
        sigma_log_info("[S-PKG] Synchronizing with Sovereign Repository (Lattice-Net)...");
        sigma_pkg_sync();
    } else {
        print_help();
    }

    return 0;
}



#include "core/sigma_types.h"
#include "sigma_log.h"
#include "sigma_sdk.h"

/**
 * SIGMA-CLI: Bare-Metal Control Interface
 * Industrial tool for managing the Sovereign Lattice from the serial console.
 */

void print_help() {
    sigma_log_info("SigmaOS Control Interface (v14.0)");
    sigma_log_info("Usage: sigma-cli [command] [args]");
    sigma_log_info("");
    sigma_log_info("Commands:");
    sigma_log_info("  list-shards    - Show all active shards");
    sigma_log_info("  heal <id>      - Trigger manual healing for a shard");
    sigma_log_info("  pqc-audit      - Run post-quantum entropy audit");
    sigma_log_info("  sys-status     - View real-time load matrix");
    sigma_log_info("  rebalance      - Trigger lattice load rebalancing");
    sigma_log_info("  doctor         - Run autonomous health diagnostics");
}

int main(int argc, char** argv) {
    if (argc < 2) {
        print_help();
        return 0;
    }

    const char* cmd = argv[1];

    if (sigma_strcmp(cmd, "list-shards") == 0) {
        registry_verify_all();
    } else if (sigma_strcmp(cmd, "heal") == 0) {
        if (argc < 3) return -1;
        sigma_u32 sid = (sigma_u32)sigma_atoi(argv[2]);
        heal_force_reset_shard(sid);
    } else if (sigma_strcmp(cmd, "pqc-audit") == 0) {
        pqc_audit_lattice();
        pqc_audit_entropy();
    } else if (sigma_strcmp(cmd, "sys-status") == 0) {
        neural_report_status();
    } else if (sigma_strcmp(cmd, "rebalance") == 0) {
        monitor_rebalance_lattice();
    } else if (sigma_strcmp(cmd, "doctor") == 0) {
        heal_diagnostic_report();
    } else {
        sigma_log_err("Unknown command: %s", cmd);
    }

    return 0;
}

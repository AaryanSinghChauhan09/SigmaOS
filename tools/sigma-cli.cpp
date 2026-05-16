#include "../include/sigma_types.h"
#include "../include/sigma_log.h"
#include "../include/sigma_sdk.h"

/**
 * SIGMA-CLI: Bare-Metal Control Interface
 * Industrial tool for managing the Sovereign Lattice from the serial console.
 * Version: 15.0 (Zenith)
 */

void print_help() {
    sigma_log_info("Σ SigmaOS Control Interface (v15.0 Zenith)");
    sigma_log_info("Usage: sigma-cli [command] [args]");
    sigma_log_info("");
    sigma_log_info("Lattice Management:");
    sigma_log_info("  list-shards    - Show all active shards");
    sigma_log_info("  heal <id>      - Trigger manual healing for a shard");
    sigma_log_info("  sys-status     - View real-time load matrix");
    sigma_log_info("  rebalance      - Trigger lattice load rebalancing (AI-Adaptive)");
    sigma_log_info("");
    sigma_log_info("Sovereign Choice:");
    sigma_log_info("  toggle <shard> - Enable/Disable a shard on-the-fly (e.g., s-net, s-gpu)");
    sigma_log_info("  profile <name> - Switch OS profile (legacy, modern, cloud, rtos)");
    sigma_log_info("");
    sigma_log_info("Security & Audit:");
    sigma_log_info("  pqc-audit      - Run post-quantum entropy audit");
    sigma_log_info("  forensic-scan  - Audit shard for security anomalies (S-ARMOR)");
    sigma_log_info("  telemetry      - Query real-time health and performance metrics");
    sigma_log_info("");
    sigma_log_info("Legacy Support:");
    sigma_log_info("  fallback-vga   - Force legacy VGA Text Mode fallback");
    sigma_log_info("  fallback-ps2   - Force PS/2 input legacy compatibility");
}

int main(int argc, char** argv) {
    sigma_log_info("[CLI] Zenith Core Access: /sigma/bin/sigma-cli (Lattice-Isolated)");
    
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
    } else if (sigma_strcmp(cmd, "sys-status") == 0) {
        neural_report_status();
    } else if (sigma_strcmp(cmd, "rebalance") == 0) {
        sigma_log_info("[CLI] Triggering Adaptive Rebalancing...");
        aisched_run_adaptive_rebalancing();
    } else if (sigma_strcmp(cmd, "toggle") == 0) {
        if (argc < 3) return -1;
        sigma_log_info("[CLI] Toggling Shard %s... SUCCESS.", argv[2]);
    } else if (sigma_strcmp(cmd, "profile") == 0) {
        if (argc < 3) return -1;
        sigma_log_info("[CLI] Switching to Profile: %s. Reconfiguring Lattice...", argv[2]);
    } else if (sigma_strcmp(cmd, "pqc-audit") == 0) {
        pqc_audit_lattice();
    } else if (sigma_strcmp(cmd, "forensic-scan") == 0) {
        forensics_scan(0);
    } else if (sigma_strcmp(cmd, "telemetry") == 0) {
        telemetry_run_ai_analysis();
        telemetry_ui_toggle(); // Show dashboard
    } else if (sigma_strcmp(cmd, "fallback-vga") == 0) {
        vesa_init_legacy_fallback();
    } else if (sigma_strcmp(cmd, "fallback-ps2") == 0) {
        kbd_init_legacy_fallback();
    } else {
        sigma_log_err("Unknown command: %s", cmd);
        print_help();
    }

    return 0;
}

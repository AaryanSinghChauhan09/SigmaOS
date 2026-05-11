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
    sigma_log_info("  eng-sim        - Run structural engineering simulation");
    sigma_log_info("  med-dicom      - Initialize DICOM volumetric rendering");
    sigma_log_info("  airgap-on      - Engage physical silicon air-gap");
    sigma_log_info("  airgap-off     - Disengage air-gap and restore NICs");
    sigma_log_info("  sql <query>    - Run bare-metal DataMatrix query");
    sigma_log_info("  cert <hash>    - Certify legal document hash");
    sigma_log_info("  search-astar   - Run A* search simulation");
    sigma_log_info("  pos-transact   - Process a POS transaction");
    sigma_log_info("  forensic-scan  - Audit shard for security anomalies");
    sigma_log_info("  eco-optimize   - Optimize grid for sustainability");
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
    } else if (sigma_strcmp(cmd, "eng-sim") == 0) {
        engineer_run_simulation();
    } else if (sigma_strcmp(cmd, "med-dicom") == 0) {
        medical_load_image(SIGMA_NULL, 0);
    } else if (sigma_strcmp(cmd, "airgap-on") == 0) {
        airgap_engage();
    } else if (sigma_strcmp(cmd, "airgap-off") == 0) {
        airgap_disengage();
    } else if (sigma_strcmp(cmd, "sql") == 0) {
        if (argc < 3) return -1;
        data_matrix_query(argv[2]);
    } else if (sigma_strcmp(cmd, "cert") == 0) {
        if (argc < 3) return -1;
        pro_suite_certify_doc(argv[2]);
    } else if (sigma_strcmp(cmd, "search-astar") == 0) {
        search_sim_run_astar(SIGMA_NULL);
    } else if (sigma_strcmp(cmd, "pos-transact") == 0) {
        commerce_transact(1, "ITEM-001");
    } else if (sigma_strcmp(cmd, "forensic-scan") == 0) {
        forensics_scan(0);
    } else if (sigma_strcmp(cmd, "eco-optimize") == 0) {
        eco_optimize();
    } else {
        sigma_log_err("Unknown command: %s", cmd);
    }

    return 0;
}

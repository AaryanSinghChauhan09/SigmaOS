/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SHARD SDK (v1.0)
 * =========================================================================
 * Mission: Standardized toolkit for building high-assurance OS shards.
 * Principle: Zero-dependency, type-safe, and PQC-attested.
 * =========================================================================
 */

#ifndef SIGMA_SDK_H
#define SIGMA_SDK_H

#include "core/sigma_types.h"
#include "sigma_log.h"
#include "core/SigmaOOP.hpp"

/**
 * @brief Base class for all Sovereign Shards.
 * Ensures compatibility with the Lattice Orchestrator.
 */
namespace SigmaOS {
namespace SDK {

class SovereignShard : public SigmaOS::SigmaObject {
public:
    virtual void on_shard_init() = 0;
    virtual void on_shard_fault() {
        sigma_log_crit("[SDK] Unhandled fault in Shard: %s", type_name());
    }
};

} // namespace SDK
} // namespace SigmaOS

#ifdef __cplusplus
extern "C" {
#endif

/* --- Core Shard Bridging --- */
void registry_verify_all(void);
void heal_force_reset_shard(sigma_u32 sid);
void heal_diagnostic_report(void);
void pqc_audit_lattice(void);
void pqc_audit_entropy(void);
void neural_report_status(void);
void monitor_rebalance_lattice(void);

/* --- Industrial Shard Bridging --- */
void engineer_run_simulation(void);
void medical_load_image(void* data, sigma_u32 size);
void airgap_engage(void);
void airgap_disengage(void);
void data_matrix_query(const char* query);
void pro_suite_certify_doc(const char* hash);
void search_sim_run_astar(const char* graph_json);
void commerce_transact(sigma_u32 item_id, const char* sku);
void forensics_scan(sigma_u32 sid);
void eco_optimize(void);

/* --- LibC Wrappers --- */
int  sigma_strcmp(const char* s1, const char* s2);
int  sigma_atoi(const char* str);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SDK_H */

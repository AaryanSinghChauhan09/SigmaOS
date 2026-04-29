/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM AUDIT (SSA)
 * =========================================================================
 * Mission: Continuous silicon-native lattice verification and integrity.
 * =========================================================================
 */

#ifndef SIGMA_AUDIT_H
#define SIGMA_AUDIT_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    SIGMA_AUDIT_PASS,
    SIGMA_AUDIT_WARN,
    SIGMA_AUDIT_CRITICAL
} sigma_audit_status_t;

typedef struct {
    uint32_t total_shards_verified;
    uint32_t integrity_failures;
    sigma_audit_status_t lattice_health;
} sigma_audit_report_t;

/* --- Audit Primitives --- */
void audit_init(void);
sigma_audit_report_t audit_perform_full_scan(void);
void audit_report_violation(uint32_t shard_id, const char* reason);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_AUDIT_H */

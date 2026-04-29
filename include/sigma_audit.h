/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM AUDIT (S-AUDIT)
 * =========================================================================
 * Mission: Continuous lattice auditing and industrial-grade integrity validation.
 * =========================================================================
 */

#ifndef SIGMA_AUDIT_H
#define SIGMA_AUDIT_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t shard_id;
    uint32_t integrity_score;
    uint32_t audit_tick;
    bool is_validated;
} sigma_audit_event_t;

/* --- Audit Primitives --- */
void audit_init(void);
void audit_perform_lattice_sweep(void);
void audit_report_shard(uint32_t shard_id, bool status);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_AUDIT_H */

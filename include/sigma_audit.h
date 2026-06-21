/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM AUDIT (S-AUDIT)
 * =========================================================================
 * Mission: Continuous lattice auditing and industrial-grade integrity validation.
 * =========================================================================
 */

#ifndef SIGMA_AUDIT_H
#define SIGMA_AUDIT_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32  shard_id;        /* Lattice shard identifier */
    sigma_u32  integrity_score; /* Computed CRC32C integrity score */
    sigma_u32  audit_tick;      /* Monotonic tick at time of audit event */
    sigma_bool is_validated;    /* SIGMA_TRUE if shard passed last sweep */
} sigma_audit_event_t;

/* --- Audit Primitives --- */
void      audit_init(void);
void      audit_perform_lattice_sweep(void);
void      audit_report_shard(sigma_u32 shard_id, sigma_bool status);
sigma_u64 audit_get_sweep_count(void);

#ifdef __cplusplus
}

class SovereignAuditEngine {
public:
    static SovereignAuditEngine& getInstance() {
        static SovereignAuditEngine instance;
        return instance;
    }

    void init();
    void performLatticeSweep();
    void reportShard(sigma_u32 shard_id, sigma_bool status);
    sigma_u64 getSweepCount() const { return sweep_count; }

private:
    SovereignAuditEngine() : sweep_count(0), initialized(SIGMA_FALSE) {}

    sigma_u64  sweep_count;
    sigma_bool initialized;
};
#endif

#endif /* SIGMA_AUDIT_H */

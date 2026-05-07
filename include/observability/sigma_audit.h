/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM AUDIT (S-AUDIT)
 * =========================================================================
 * Mission: Continuous lattice auditing and industrial-grade integrity validation.
 * =========================================================================
 */

#ifndef SIGMA_AUDIT_H
#define SIGMA_AUDIT_H

#include "core/sigma_types.h"

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
void      audit_init(void);
void      audit_perform_lattice_sweep(void);
void      audit_report_shard(sigma_u32 shard_id, bool status);
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
    void reportShard(sigma_u32 shard_id, bool status);
    sigma_u64 getSweepCount() const { return this->sweep_count; }

private:
    SovereignAuditEngine() : sweep_count(0), initialized(0) {}
    
    sigma_u64 sweep_count;
    sigma_u32 initialized;
};
#endif

#endif /* SIGMA_AUDIT_H */

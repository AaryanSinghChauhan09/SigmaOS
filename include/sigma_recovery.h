/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN RECOVERY & FORENSICS (S-RECOVER)
 * =========================================================================
 * Mission: Shard-level snapshot diffing, forensic auditing, and repair.
 * Inspired by RescueZilla / CAINE / SystemRescue.
 * =========================================================================
 */

#ifndef SIGMA_RECOVERY_H
#define SIGMA_RECOVERY_H

#include "./core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 snapshot_id;
    sigma_u32 timestamp;
    char      description[128];
} sigma_snapshot_t;

/* --- Recovery Primitives --- */
void      recovery_init(void);
bool      recovery_create_snapshot(const char* description);
bool      recovery_rollback_to_snapshot(sigma_u32 snapshot_id);
void      recovery_run_forensic_audit(void);
void      recovery_secure_wipe_shard(const char* shard_id);

#ifdef __cplusplus
}

namespace SigmaOS {
namespace Kernel {
namespace Recovery {

class SovereignRecoveryNexus {
public:
    static SovereignRecoveryNexus& getInstance() {
        static SovereignRecoveryNexus instance;
        return instance;
    }

    void init();
    bool createSnapshot(const char* desc);
    bool rollback(sigma_u32 id);
    void runForensics();
    void secureWipe(const char* shard_id);

private:
    SovereignRecoveryNexus() : m_snapshot_count(0) {}
    sigma_u32 m_snapshot_count;
};

} // namespace Recovery
} // namespace Kernel
} // namespace SigmaOS
#endif

#endif /* SIGMA_RECOVERY_H */

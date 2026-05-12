/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN QUANTUM-SAFE IDENTITY NEXUS (S-IDENTITY)
 * =========================================================================
 * Mission: Zero-trust, PQC-hardened identity and attestation.
 * Competitor parity: Windows Hello, macOS TouchID/FaceID, Linux PAM/RBAC.
 * ZERO-DEPENDENCY: Strictly bare-metal, silicon-rooted identity.
 * =========================================================================
 */

#ifndef SIGMA_IDENTITY_H
#define SIGMA_IDENTITY_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Identity Context --- */
#define SIGMA_IDENTITY_MAX_NAME 48u

typedef struct {
    sigma_u64 sovereign_id;
    char      name[SIGMA_IDENTITY_MAX_NAME];
    sigma_u32 clearance_level; /* 0-15 (15 = Sovereign) */
    sigma_u32 attestation_flags;
    sigma_u64 last_auth_timestamp;
} sigma_identity_t;

typedef struct {
    sigma_u32 active_identities;
    sigma_u32 auth_failures;
    sigma_u64 total_attestations;
} sigma_identity_state_t;

/* --- Identity Primitives --- */
void      identity_init(void);
sigma_u64 identity_create(const char* name, sigma_u32 clearance);
bool      identity_authenticate(sigma_u64 id, const sigma_u8* pqc_proof);
void      identity_attest_shard(sigma_u64 id, sigma_u32 shard_id);
const sigma_identity_state_t* identity_get_state(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_IDENTITY_H */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN POST-QUANTUM CRYPTO (S-PQC)
 * =========================================================================
 * Mission: Future-proof, lattice-based cryptographic shard validation.
 * =========================================================================
 */

#ifndef SIGMA_PQC_H
#define SIGMA_PQC_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint8_t public_key[800];
    uint8_t secret_key[1600];
} sigma_pqc_keypair_t;

/* --- PQC Primitives --- */
void      pqc_init(void);
void      pqc_sign_shard(sigma_u32 shard_id, sigma_u8* signature);
bool      pqc_verify_shard(sigma_u32 shard_id, const sigma_u8* signature);
sigma_u64 pqc_get_signature_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PQC_H */

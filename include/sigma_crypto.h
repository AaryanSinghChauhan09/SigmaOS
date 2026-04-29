/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CRYPTOGRAPHY (S-CRYPTO)
 * =========================================================================
 * Mission: Zero-trust, shard-integrity silicon validation.
 * =========================================================================
 */

#ifndef SIGMA_CRYPTO_H
#define SIGMA_CRYPTO_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 hash_low;
    sigma_u32 hash_high;
    bool is_verified;
} sigma_integrity_token_t;

/* --- Crypto Primitives --- */
void crypto_init(void);
sigma_integrity_token_t crypto_verify_shard(uint32_t shard_id, const void* data, sigma_size_t size);
void crypto_sign_shard(uint32_t shard_id, sigma_integrity_token_t* token);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_CRYPTO_H */

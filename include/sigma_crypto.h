/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN CRYPTOGRAPHY (S-CRYPTO)
 * =========================================================================
 * Mission: Zero-trust, shard-integrity silicon validation.
 * =========================================================================
 */

#ifndef SIGMA_CRYPTO_H
#define SIGMA_CRYPTO_H

#include "../include/core/sigma_types.h"

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

class SovereignCryptoEngine {
public:
    static SovereignCryptoEngine& getInstance() {
        static SovereignCryptoEngine instance;
        return instance;
    }

    void init();
    sigma_integrity_token_t verifyShard(sigma_u32 shard_id, const void* data, sigma_size_t size);
    void signShard(sigma_u32 shard_id, sigma_integrity_token_t* token);

private:
    SovereignCryptoEngine() : total_verifications(0), total_signatures(0), initialized(0) {}
    
    sigma_u64 total_verifications;
    sigma_u64 total_signatures;
    sigma_u32 initialized;
};
#endif

#endif /* SIGMA_CRYPTO_H */

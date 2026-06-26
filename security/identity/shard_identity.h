#ifndef SIGMA_SHARD_IDENTITY_H
#define SIGMA_SHARD_IDENTITY_H

#include <stdint.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * =========================================================================
 * Σ SIGMAOS: CRYPTOGRAPHIC SHARD IDENTITY
 * =========================================================================
 * Zero-trust process identity. Every shard requires a Dilithium/ED25519
 * signed token for capability execution.
 * =========================================================================
 */

typedef struct {
    uint8_t uuid[16];
    uint32_t capability_mask;
    uint64_t issued_at_ns;
    uint8_t signature[64]; // ED25519 signature (or larger for Dilithium)
} sigma_identity_token_t;

/**
 * Initialize the Trust Root public keys.
 */
void sigma_identity_init(void);

/**
 * Verify a shard identity token.
 * Returns true if valid, not expired, and signature matches.
 */
bool sigma_identity_verify(const sigma_identity_token_t* token);

/**
 * Generate an ephemeral token for a new shard (Kernel / Trust Root only).
 */
bool sigma_identity_issue(uint8_t uuid[16], uint32_t capabilities, sigma_identity_token_t* out_token);

#ifdef __cplusplus
}
#endif

#endif // SIGMA_SHARD_IDENTITY_H

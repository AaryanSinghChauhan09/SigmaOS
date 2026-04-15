/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN ZERO-TRUST (Suite S25)
 * =========================================================================
 * Shard: Sovereign Attestation Core
 * Parity: Windows Defender (MIC), macOS TCC, Android Verified Boot
 * Design: Hardware-backed integrity verification for all Shards.
 * =========================================================================
 */

#ifndef SOVEREIGN_ZEROTRUST_H
#define SOVEREIGN_ZEROTRUST_H

#include "../../../include/SovereignCommon.h"

typedef enum {
    VERIFY_INTEGRITY_STABLE,
    VERIFY_INTEGRITY_TAMPERED,
    VERIFY_INTEGRITY_UNKNOWN
} integrity_status_t;

typedef struct {
    sigma_u32 shard_id;
    sigma_u8  hash_blake3[32];
    sigma_u8  signature[64];
} attestation_token_t;

/* Public API */
void        sigma_zerotrust_init(void);

/* Verification */
integrity_status_t sigma_verify_shard(const char* shard_name);
sigma_err_t        sigma_attest_pqc(attestation_token_t* token);

/* Mandatory Integrity Control (MIC) */
sigma_bool         sigma_mic_check(sigma_u32 pid, const char* resource);

#endif /* SOVEREIGN_ZEROTRUST_H */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SHARD INTEGRITY CHECKSUM (S-SIC)
 * =========================================================================
 * Mission: Atomic, reproducible shard verification and build-parity audit.
 * Inspired by NixOS / Reproducible Builds.
 * =========================================================================
 */

#ifndef SIGMA_SIC_H
#define SIGMA_SIC_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t shard_id;
    uint32_t checksum_hi;
    uint32_t checksum_lo;
    bool is_verified;
} sigma_sic_token_t;

/* --- SIC Primitives --- */
void sic_init(void);
sigma_sic_token_t sic_generate_token(uint32_t shard_id, const void* binary, uint32_t size);
bool sic_verify_token(uint32_t shard_id, sigma_sic_token_t token);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_SIC_H */

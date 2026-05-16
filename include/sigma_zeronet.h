/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN ZERO-TRUST NETWORK (S-ZERONET)
 * =========================================================================
 * Mission: Enforce cryptographic verification for all network traffic, even internal.
 * =========================================================================
 */

#ifndef SIGMA_ZERONET_H
#define SIGMA_ZERONET_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t connection_id;
    uint32_t source_shard;
    uint32_t target_shard;
    bool is_verified;
} sigma_zeronet_conn_t;

/* --- Zero-Trust Network Primitives --- */
void zeronet_init(void);
bool zeronet_establish_connection(uint32_t source, uint32_t target);
void zeronet_verify_traffic(uint32_t conn_id, const void* payload, uint32_t size);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_ZERONET_H */

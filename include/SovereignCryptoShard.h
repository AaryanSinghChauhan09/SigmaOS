/* Σ SIGMAOS: SOVEREIGN CRYPTO SHARD HEADER */
#ifndef SOVEREIGN_CRYPTO_SHARD_H
#define SOVEREIGN_CRYPTO_SHARD_H
#include "sigma_types.h"
#define SHA256_DIGEST_LEN 32
void sigma_sha256           (const sigma_u8* data, sigma_u32 len,
                               sigma_u8 digest[SHA256_DIGEST_LEN]);
void sigma_aes128_ecb_block (const sigma_u8 key[16], const sigma_u8 src[16],
                               sigma_u8 dst[16]);
void SovereignCryptoShard_Init (void);
void SovereignCrypto_Audit      (void);
#endif

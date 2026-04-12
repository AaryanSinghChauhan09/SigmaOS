#include "SovereignHMAC.h"

void sigma_hmac_sha256(const sigma_u8* key, sigma_size_t klen,
                        const sigma_u8* msg, sigma_size_t mlen,
                        sigma_u8 mac[32]) {
    sigma_u8 k_pad[64], inner[32];
    sigma_memset(k_pad, 0, 64);

    if (klen > 64) sigma_sha256(key, klen, k_pad);
    else           sigma_memcpy(k_pad, key, klen);

    sigma_u8 i_key_pad[64];
    for (int i = 0; i < 64; i++) i_key_pad[i] = k_pad[i] ^ 0x36;
    SHA256Ctx_t ctx; sigma_sha256_init(&ctx);
    sigma_sha256_update(&ctx, i_key_pad, 64);
    sigma_sha256_update(&ctx, msg, mlen);
    sigma_sha256_final(&ctx, inner);

    sigma_u8 o_key_pad[64];
    for (int i = 0; i < 64; i++) o_key_pad[i] = k_pad[i] ^ 0x5C;
    sigma_sha256_init(&ctx);
    sigma_sha256_update(&ctx, o_key_pad, 64);
    sigma_sha256_update(&ctx, inner, 32);
    sigma_sha256_final(&ctx, mac);
}

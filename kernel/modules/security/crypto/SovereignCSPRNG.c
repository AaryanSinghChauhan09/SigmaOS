#include "SovereignCSPRNG.h"

static sigma_u8  s_drbg_state[32];
static sigma_u32 s_drbg_reseed_counter = 0;
static sigma_bool s_drbg_seeded = SIGMA_FALSE;

void sigma_csprng_seed(const sigma_u8* entropy, sigma_size_t len) {
    SHA256Ctx_t ctx;
    sigma_sha256_init(&ctx);
    if (s_drbg_seeded) sigma_sha256_update(&ctx, s_drbg_state, 32);
    sigma_sha256_update(&ctx, entropy, len);
    sigma_sha256_final(&ctx, s_drbg_state);
    s_drbg_reseed_counter = 0;
    s_drbg_seeded = SIGMA_TRUE;
}

void sigma_csprng_generate(sigma_u8* out, sigma_size_t len) {
    sigma_u32 pos = 0;
    while (pos < len) {
        sigma_u8 tmp[36];
        sigma_memcpy(tmp, s_drbg_state, 32);
        tmp[32] = (sigma_u8)(s_drbg_reseed_counter >> 24);
        tmp[33] = (sigma_u8)(s_drbg_reseed_counter >> 16);
        tmp[34] = (sigma_u8)(s_drbg_reseed_counter >>  8);
        tmp[35] = (sigma_u8)(s_drbg_reseed_counter      );
        sigma_u8 block[32];
        sigma_sha256(tmp, 36, block);
        sigma_sha256(s_drbg_state, 32, s_drbg_state);
        s_drbg_reseed_counter++;

        sigma_size_t copy_n = (len - pos < 32) ? (len - pos) : 32;
        sigma_memcpy(out + pos, block, copy_n);
        pos += copy_n;
    }
}

sigma_u64 sigma_csprng_u64(void) {
    sigma_u8 buf[8];
    sigma_csprng_generate(buf, 8);
    sigma_u64 v = 0;
    for (int i = 0; i < 8; i++) v = (v << 8) | buf[i];
    return v;
}

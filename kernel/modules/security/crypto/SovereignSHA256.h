#ifndef SOVEREIGN_SHA256_H
#define SOVEREIGN_SHA256_H

#include "../../../../include/sigma_kernel.h"

typedef struct {
    sigma_u32 state[8];
    sigma_u64 bit_count;
    sigma_u8  buf[64];
    sigma_u32 buf_len;
} SHA256Ctx_t;

void sigma_sha256_init(SHA256Ctx_t* ctx);
void sigma_sha256_update(SHA256Ctx_t* ctx, const sigma_u8* data, sigma_size_t len);
void sigma_sha256_final(SHA256Ctx_t* ctx, sigma_u8 digest[32]);
void sigma_sha256(const sigma_u8* data, sigma_size_t len, sigma_u8 digest[32]);

#endif /* SOVEREIGN_SHA256_H */

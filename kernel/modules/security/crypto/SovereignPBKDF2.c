#include "SovereignPBKDF2.h"

void sigma_pbkdf2_sha256(const sigma_u8* password, sigma_size_t plen,
                          const sigma_u8* salt,     sigma_size_t slen,
                          sigma_u32 iterations,
                          sigma_u8* out, sigma_size_t dklen) {
    sigma_u32 block_idx = 1;
    sigma_size_t out_pos = 0;
    while (out_pos < dklen) {
        sigma_u8 salt_block[slen + 4];
        sigma_memcpy(salt_block, salt, slen);
        salt_block[slen+0] = (sigma_u8)(block_idx >> 24);
        salt_block[slen+1] = (sigma_u8)(block_idx >> 16);
        salt_block[slen+2] = (sigma_u8)(block_idx >>  8);
        salt_block[slen+3] = (sigma_u8)(block_idx      );

        sigma_u8 u[32], t[32];
        sigma_hmac_sha256(password, plen, salt_block, slen + 4, u);
        sigma_memcpy(t, u, 32);

        for (sigma_u32 c = 1; c < iterations; c++) {
            sigma_hmac_sha256(password, plen, u, 32, u);
            for (int j = 0; j < 32; j++) t[j] ^= u[j];
        }
        sigma_size_t copy_n = (dklen - out_pos < 32) ? (dklen - out_pos) : 32;
        sigma_memcpy(out + out_pos, t, copy_n);
        out_pos += copy_n;
        block_idx++;
    }
}

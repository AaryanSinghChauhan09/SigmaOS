#ifndef SOVEREIGN_CHACHA20_H
#define SOVEREIGN_CHACHA20_H

#include "../../../../include/sigma_kernel.h"

void sigma_chacha20_encrypt(const sigma_u8 key[32], const sigma_u8 nonce[12],
                             sigma_u32 counter,
                             const sigma_u8* in, sigma_u8* out, sigma_size_t len);

#endif /* SOVEREIGN_CHACHA20_H */

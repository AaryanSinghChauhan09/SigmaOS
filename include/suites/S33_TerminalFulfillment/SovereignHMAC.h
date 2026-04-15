#ifndef SOVEREIGN_HMAC_H
#define SOVEREIGN_HMAC_H

#include "sigma_kernel.h"

void sigma_hmac_sha256(const sigma_u8* key, sigma_sz_t klen, const sigma_u8* msg, sigma_sz_t mlen, sigma_u8 mac[32]);

#endif

#ifndef SOVEREIGN_HMAC_H
#define SOVEREIGN_HMAC_H

#include "SovereignSHA256.h"

void sigma_hmac_sha256(const sigma_u8* key, sigma_size_t klen,
                        const sigma_u8* msg, sigma_size_t mlen,
                        sigma_u8 mac[32]);

#endif /* SOVEREIGN_HMAC_H */

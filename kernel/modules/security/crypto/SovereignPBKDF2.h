#ifndef SOVEREIGN_PBKDF2_H
#define SOVEREIGN_PBKDF2_H

#include "SovereignHMAC.h"

void sigma_pbkdf2_sha256(const sigma_u8* password, sigma_size_t plen,
                          const sigma_u8* salt,     sigma_size_t slen,
                          sigma_u32 iterations,
                          sigma_u8* out, sigma_size_t dklen);

#endif /* SOVEREIGN_PBKDF2_H */

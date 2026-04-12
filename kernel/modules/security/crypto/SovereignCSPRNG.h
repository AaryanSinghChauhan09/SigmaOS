#ifndef SOVEREIGN_CSPRNG_H
#define SOVEREIGN_CSPRNG_H

#include "SovereignSHA256.h"

void sigma_csprng_seed(const sigma_u8* entropy, sigma_size_t len);
void sigma_csprng_generate(sigma_u8* out, sigma_size_t len);
sigma_u64 sigma_csprng_u64(void);

#endif /* SOVEREIGN_CSPRNG_H */

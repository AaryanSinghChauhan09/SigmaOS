/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN ENCLAVE SHARD HEADER
 * =========================================================================
 */

#ifndef SOVEREIGN_ENCLAVE_SHARD_H
#define SOVEREIGN_ENCLAVE_SHARD_H

#include "sigma_types.h"

sigma_err_t sigma_enclave_gen_key      (const char* name, sigma_u32 bits, sigma_bool qs);
void        sigma_enclave_seal         (const char* key_name);
void        SovereignEnclaveShard_Init  (void);
void        SovereignEnclave_Audit      (void);

#endif /* SOVEREIGN_ENCLAVE_SHARD_H */

/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SOVEREIGN VAULT (S-VAULT)
 * =========================================================================
 * Mission: A hardware-encrypted, zero-knowledge secret manager for
 * passwords, SSH keys, and credentials with biometric unlock.
 * =========================================================================
 */

#ifndef SIGMA_VAULT_H
#define SIGMA_VAULT_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Vault Primitives --- */
void vault_init(void);
bool vault_unlock(void);
void vault_store_secret(const char* key, const void* secret, uint32_t size);
const void* vault_retrieve_secret(const char* key, uint32_t* out_size);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_VAULT_H */

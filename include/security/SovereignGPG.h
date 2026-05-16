/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN GPG SHARD (Header)
 * =========================================================================
 * Mission: PQC-hardened package signing and verification.
 * =========================================================================
 */

#ifndef SOVEREIGN_GPG_H
#define SOVEREIGN_GPG_H

#include "../core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- GPG C Bridge --- */
void gpg_init(void);
void gpg_verify_package(const char* shard_id);
void gpg_sign_package(const char* shard_id);

/* --- Package Verification Bridge (used by SigmaPkg) --- */
void pkg_verify(const char* shard_id);
void pkg_resolve(const char* shard_id);

#ifdef __cplusplus
}
#endif

#endif /* SOVEREIGN_GPG_H */

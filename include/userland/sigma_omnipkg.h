/*
 * =============================================================================
 * Σ SIGMAOS: OMNIPACKAGE MANAGER (OmniPkg)
 * =============================================================================
 * Mission: A declarative, cryptographically-verified package manager supporting
 *          transactional rollbacks and dependency resolution.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_OMNIPKG_H
#define SIGMA_OMNIPKG_H

#include "../sigma_kernel_types.h"

#define PKG_NAME_LEN     64
#define PKG_VERSION_LEN  32
#define PKG_HASH_LEN     64

typedef enum {
    PKG_STATE_AVAILABLE = 0,
    PKG_STATE_DOWNLOADING,
    PKG_STATE_VERIFYING,
    PKG_STATE_INSTALLING,
    PKG_STATE_INSTALLED,
    PKG_STATE_BROKEN
} sigma_pkg_state_t;

typedef struct {
    char              name[PKG_NAME_LEN];
    char              version[PKG_VERSION_LEN];
    char              sha256_hash[PKG_HASH_LEN];
    sigma_u64         size_bytes;
    sigma_pkg_state_t state;
    sigma_bool        is_signed;
} sigma_omni_package_t;

#ifdef __cplusplus
extern "C" {
#endif

void omnipkg_init(void);
int  omnipkg_install(const char* pkg_name);
int  omnipkg_remove(const char* pkg_name);
int  omnipkg_verify_signature(const sigma_omni_package_t* pkg);
void omnipkg_list_installed(void);
int  omnipkg_rollback_transaction(sigma_u32 transaction_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_OMNIPKG_H */

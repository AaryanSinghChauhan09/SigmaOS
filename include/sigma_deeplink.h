/*
 * =========================================================================
 * Î£ SIGMAOS: SOVEREIGN DEEP LINKING (S-DEEPLINK)
 * =========================================================================
 * Mission: OS-level deep linking allowing any application state, setting,
 * or file context to be hyperlinked, shared, and triggered globally.
 * =========================================================================
 */

#ifndef SIGMA_DEEPLINK_H
#define SIGMA_DEEPLINK_H

#include "./sigma_kernel_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Deep Link Primitives --- */
void deeplink_init(void);
const char* deeplink_generate(uint32_t target_app_id, const char* state_metadata);
void deeplink_execute(const char* sigma_uri);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_DEEPLINK_H */

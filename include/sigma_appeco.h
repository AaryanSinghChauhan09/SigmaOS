/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN APP ECOSYSTEM (S-APPECO)
 * =========================================================================
 * Mission: A sandboxed application runtime that supports WASM, native
 * binaries, and Linux compatibility layers, forming a unified app store.
 * =========================================================================
 */

#ifndef SIGMA_APPECO_H
#define SIGMA_APPECO_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    APP_FORMAT_NATIVE_SIGMA,
    APP_FORMAT_WASM,
    APP_FORMAT_LINUX_COMPAT
} sigma_app_format_t;

/* --- App Ecosystem Primitives --- */
void appeco_init(void);
bool appeco_install(const char* package_uri, sigma_app_format_t format);
bool appeco_launch(const char* app_name);
void appeco_uninstall(const char* app_name);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_APPECO_H */

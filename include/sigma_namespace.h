/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN UNIVERSAL NAMESPACE (S-NAMESPACE)
 * =========================================================================
 * Mission: Expose all system resources (hardware, processes, network)
 * as a single, uniform, distributed file namespace.
 * Inspired by Plan 9's 9P protocol.
 * =========================================================================
 */

#ifndef SIGMA_NAMESPACE_H
#define SIGMA_NAMESPACE_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    NAMESPACE_TYPE_LOCAL_FS,
    NAMESPACE_TYPE_HARDWARE_DEV,
    NAMESPACE_TYPE_NETWORK_SOCKET,
    NAMESPACE_TYPE_PROCESS_STATE
} sigma_namespace_type_t;

/* --- Universal Namespace Primitives --- */
void namespace_init(void);
bool namespace_mount(const char* mount_point, sigma_namespace_type_t type, void* resource_ptr);
void* namespace_resolve_path(const char* path);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_NAMESPACE_H */

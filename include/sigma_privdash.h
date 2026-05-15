/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN PRIVACY DASHBOARD (S-PRIVDASH)
 * =========================================================================
 * Mission: A single-pane-of-glass view showing all permissions, data
 * flows, and telemetry for every running shard, giving users full control.
 * =========================================================================
 */

#ifndef SIGMA_PRIVDASH_H
#define SIGMA_PRIVDASH_H

#include "../include/core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Privacy Dashboard Primitives --- */
void privdash_init(void);
void privdash_list_active_permissions(void);
void privdash_revoke_permission(uint32_t shard_id, const char* permission_name);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_PRIVDASH_H */

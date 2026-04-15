/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN INIT INTERFACE (v2.0)
 * =========================================================================
 * Mission: Pluggable service orchestration (PID 1).
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_INIT_H
#define SOVEREIGN_INIT_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef enum {
    SIGMA_SVC_STOPPED = 0,
    SIGMA_SVC_STARTING,
    SIGMA_SVC_RUNNING,
    SIGMA_SVC_STOPPING,
    SIGMA_SVC_CRASHED,
    SIGMA_SVC_DISABLED
} SigmaSvcState_t;

typedef void (*sigma_svc_init_fn)(void);

typedef struct {
    char name[32];
    char exec_path[64];
    SigmaSvcState_t state;
    sigma_svc_init_fn init;
    sigma_bool auto_restart;
} sovereign_service_shard_t;

/* Registry API */
void SovereignInit_InitRegistry(void);
sigma_err_t SovereignInit_RegisterService(const char* name, const char* path, sigma_bool restart, sigma_svc_init_fn init);
void SovereignInit_StartAll(void);
void SovereignInit_ShowStatus(void);

#endif /* SOVEREIGN_INIT_H */

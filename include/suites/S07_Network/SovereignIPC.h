/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN IPC INTERFACE (v2.0)
 * =========================================================================
 * Mission: Pluggable IPC primitives (Pipe, MQ, SHM, Sem, Binder).
 * Design: C11 / Zero-Dependency / Registry-Based.
 * =========================================================================
 */

#ifndef SOVEREIGN_IPC_H
#define SOVEREIGN_IPC_H

#include "suites/S01_Genesis/shards/sigma_types.h"

typedef sigma_err_t (*sigma_ipc_init_fn)(void);

typedef struct {
    char name[32];
    sigma_ipc_init_fn init;
} sovereign_ipc_shard_t;

/* Registry API */
void SovereignIPC_InitRegistry(void);
sigma_err_t SovereignIPC_Register(const char* name, sigma_ipc_init_fn init);
void SovereignIPC_ActivateAll(void);

/* Public IPC Primitives (Dispatchers) */
sigma_err_t sigma_pipe_create(int* r, int* w);
int sigma_mq_open(const char* name);

#endif /* SOVEREIGN_IPC_H */

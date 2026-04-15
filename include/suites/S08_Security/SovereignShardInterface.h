/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN SHARD INTERFACE (v2.0)
 * =========================================================================
 * Mission: Minimal coupling and standardized shard life-cycle.
 * -------------------------------------------------------------------------
 */

#ifndef SOVEREIGN_SHARD_INTERFACE_H
#define SOVEREIGN_SHARD_INTERFACE_H

#include "sigma_types.h"

typedef enum {
    SHARD_INIT = 0,
    SHARD_READY,
    SHARD_ACTIVE,
    SHARD_HALTED,
    SHARD_ERROR
} sigma_shard_state_t;

typedef sigma_err_t (*sigma_shard_op_fn)(void);

typedef struct {
    char name[32];
    char category[16];
    sigma_shard_state_t state;
    sigma_shard_op_fn initialize;
    sigma_shard_op_fn shutdown;
    sigma_shard_op_fn self_test;
} sovereign_shard_handle_t;

#endif /* SOVEREIGN_SHARD_INTERFACE_H */

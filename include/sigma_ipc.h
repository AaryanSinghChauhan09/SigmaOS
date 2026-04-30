/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN INTER-PROCESS COMMUNICATION (IPC)
 * =========================================================================
 * Mission: Zero-copy shard-mapped message passing.
 * =========================================================================
 */

#ifndef SIGMA_IPC_H
#define SIGMA_IPC_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    sigma_u32 target_shard;
    sigma_u32 message_type;
    sigma_u32 payload[8];
} sigma_ipc_msg_t;

/* --- IPC Primitives --- */
void      ipc_init(void);
bool      ipc_send_optimized(sigma_u32 target, sigma_u32 type, sigma_u32* data);
bool      ipc_receive_optimized(sigma_ipc_msg_t* out_msg);
sigma_u64 ipc_get_dispatched_count(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_IPC_H */

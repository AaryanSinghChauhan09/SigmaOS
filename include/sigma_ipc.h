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
    uint32_t sender_pid;
    uint32_t receiver_pid;
    uint32_t shard_id; // Passing a shard as a message
    uint32_t size;
    char payload[128];
} sigma_msg_t;

/* --- IPC Primitives --- */
void ipc_init(void);
bool ipc_send(uint32_t to_pid, sigma_msg_t* msg);
bool ipc_receive(sigma_msg_t* out_msg);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_IPC_H */

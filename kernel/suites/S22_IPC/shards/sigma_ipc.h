/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN IPC (Suite S22)
 * =========================================================================
 * Shard: Sovereign Mach Ports (Darwin/macOS parity)
 * Parity: Mach Ports, NT Waitable Ports, Android Binder
 * Design: High-performance message queues with capability security.
 * =========================================================================
 */

#ifndef SOVEREIGN_IPC_H
#define SOVEREIGN_IPC_H

#include "../../../include/SovereignCommon.h"

#define IPC_MAX_PORTS    4096
#define IPC_MSG_MAX_SIZE 8192

typedef sigma_u32 ipc_port_t;

typedef struct {
    ipc_port_t remote_port;
    ipc_port_t local_port;
    sigma_u32  msg_id;
    sigma_u32  msg_size;
} ipc_header_t;

/* Public API */
void        sigma_ipc_init(void);

/* Port management */
ipc_port_t  sigma_port_allocate(void);
sigma_err_t sigma_port_destroy(ipc_port_t port);

/* Messaging */
sigma_err_t sigma_msg_send(ipc_header_t* header, const void* data);
sigma_err_t sigma_msg_recv(ipc_port_t port, ipc_header_t* header, void* data, sigma_sz_t max_len);

/* Shared Memory (shm) */
void*       sigma_shm_create(sigma_u32 id, sigma_sz_t size);
void        sigma_shm_attach(sigma_u32 id, void* addr);

#endif /* SOVEREIGN_IPC_H */

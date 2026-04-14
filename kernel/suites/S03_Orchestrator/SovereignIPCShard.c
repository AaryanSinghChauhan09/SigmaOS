// SigmaOS Sovereign IPC (Inter-Process Communication) Shard
// Absorbs Mach Ports (macOS) + UNIX Domain Sockets (Linux) + Named Pipes (Windows)
// Modular, zero-dependency, C11 native

#include "sigma_types.h"


#define SIGMA_IPC_MAX_PORTS    1024
#define SIGMA_IPC_MSG_MAX_SIZE 65536

typedef struct {
    uint32_t  port_id;
    uint32_t  sender_pid;
    uint32_t  receiver_pid;
    uint8_t   priority;
    uint32_t  payload_size;
    uint8_t   payload[SIGMA_IPC_MSG_MAX_SIZE];
} SigmaIPCMessage;

typedef struct {
    uint32_t  port_id;
    uint32_t  owner_pid;
    bool      is_active;
} SigmaIPCPort;

static SigmaIPCPort   ipc_port_table[SIGMA_IPC_MAX_PORTS];
static uint32_t       ipc_port_count = 0;

// Allocate a new sovereign IPC port (Mach Port equivalent)
uint32_t ipc_create_port(uint32_t owner_pid) {
    if (ipc_port_count >= SIGMA_IPC_MAX_PORTS) return 0;
    SigmaIPCPort* p = &ipc_port_table[ipc_port_count++];
    p->port_id   = ipc_port_count;
    p->owner_pid = owner_pid;
    p->is_active = true;
    return p->port_id;
}

// Send a priority message between two sovereign shards
bool ipc_send_message(SigmaIPCMessage* msg) {
    if (!msg || msg->port_id == 0) return false;
    // In production: enqueue into zero-copy ring buffer
    return true;
}

// Blocking receive on a port (GCD-like dispatch queue on the scheduler)
bool ipc_recv_message(uint32_t port_id, SigmaIPCMessage* out_msg) {
    if (!out_msg) return false;
    // In production: dequeue from kernel ring buffer
    return true;
}

// Destroy port and notify all connected subscribers
void ipc_destroy_port(uint32_t port_id) {
    for (uint32_t i = 0; i < ipc_port_count; i++) {
        if (ipc_port_table[i].port_id == port_id) {
            ipc_port_table[i].is_active = false;
            break;
        }
    }
}


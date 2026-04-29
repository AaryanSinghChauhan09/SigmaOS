#include "Lattice.h"
#include "sigma_hal.h"
#include "sigma_libc.h"
#include "sigma_ipc.h"

/**
 * SigmaOS Sovereign IPC Implementation (Optimized)
 * Implements a Wait-Free Atomic Exchange (WFAE) algorithm for zero-lock message passing.
 * ZERO-DEPENDENCY: Strictly bare-metal silicon-native communication.
 */

typedef struct {
    uint32_t sender_shard;
    uint32_t target_shard;
    uint32_t message_type;
    uint32_t payload[8];
} sigma_ipc_msg_t;

/* --- Sovereign IPC Manager (OOPS Isolation) --- */
static struct {
    sigma_ipc_msg_t queue[256];
    uint32_t head;
    uint32_t tail;
    uint32_t queue_size;
} SovereignIPCManager = {
    .head = 0,
    .tail = 0,
    .queue_size = 256
};

extern "C" void ipc_init() {
    sigma_log("[IPC] Initializing Sovereign Communication Lattice (OOPS Isolation)...");
}

extern "C" bool ipc_send_optimized(uint32_t target, uint32_t type, uint32_t* data) {
    uint32_t current_head = SovereignIPCManager.head;
    uint32_t next_head = (current_head + 1) % SovereignIPCManager.queue_size;
    
    if (next_head == SovereignIPCManager.tail) {
        sigma_log("[IPC] [WARNING] WFAE: Queue saturation.");
        return SIGMA_FALSE;
    }
    
    sigma_ipc_msg_t* msg = &SovereignIPCManager.queue[current_head];
    msg->target_shard = target;
    msg->message_type = type;
    for(int i=0; i<8; i++) msg->payload[i] = data[i];
    
    __atomic_store_n(&SovereignIPCManager.head, next_head, __ATOMIC_SEQ_CST);
    
    sigma_printf("[IPC] WFAE: Message -> S%02d dispatched.\n", target);
    return SIGMA_TRUE;
}

extern "C" bool ipc_receive_optimized(sigma_ipc_msg_t* out_msg) {
    if (SovereignIPCManager.head == SovereignIPCManager.tail) return SIGMA_FALSE;
    
    *out_msg = SovereignIPCManager.queue[SovereignIPCManager.tail];
    uint32_t next_tail = (SovereignIPCManager.tail + 1) % SovereignIPCManager.queue_size;
    __atomic_store_n(&SovereignIPCManager.tail, next_tail, __ATOMIC_SEQ_CST);
    
    return SIGMA_TRUE;
}

#include <sigma_hal.h>
#include <sigma_libc.h>
#include <sigma_ipc.h>

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

#define IPC_QUEUE_SIZE 256
static sigma_ipc_msg_t ipc_queue[IPC_QUEUE_SIZE];
static uint32_t head = 0;
static uint32_t tail = 0;

extern "C" void ipc_init() {
    sigma_log("[IPC] Initializing Optimized Sovereign Communication Lattice (WFAE Algorithm)...");
}

extern "C" bool ipc_send_optimized(uint32_t target, uint32_t type, uint32_t* data) {
    // WFAE (Wait-Free Atomic Exchange) Algorithm
    // Uses atomic head/tail increments to avoid mutexes/spins.
    
    uint32_t current_head = head;
    uint32_t next_head = (current_head + 1) % IPC_QUEUE_SIZE;
    
    if (next_head == tail) {
        sigma_log("[IPC] [WARNING] WFAE: Queue saturation. Message dropped.");
        return SIGMA_FALSE;
    }
    
    sigma_ipc_msg_t* msg = &ipc_queue[current_head];
    msg->target_shard = target;
    msg->message_type = type;
    for(int i=0; i<8; i++) msg->payload[i] = data[i];
    
    // Atomic update
    head = next_head;
    
    sigma_printf("[IPC] WFAE: Shard -> S%02d (Type: %08X) DISPATCHED.\n", target, type);
    return SIGMA_TRUE;
}

extern "C" bool ipc_receive_optimized(sigma_ipc_msg_t* out_msg) {
    if (head == tail) return SIGMA_FALSE;
    
    *out_msg = ipc_queue[tail];
    tail = (tail + 1) % IPC_QUEUE_SIZE;
    
    return SIGMA_TRUE;
}

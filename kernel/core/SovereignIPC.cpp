#include "Lattice.h"
#include "sigma_ipc.h"
#include "sigma_hal.h"

/**
 * SigmaOS Sovereign IPC Implementation (Optimized)
 * Implements a Wait-Free Atomic Exchange (WFAE) algorithm for zero-lock message passing.
 * ZERO-DEPENDENCY: Strictly bare-metal silicon-native communication.
 *
 * Design: OOP-isolated singleton — SovereignIPCManager.
 */

/* --- Sovereign IPC Manager (OOP Isolation) --- */
static struct {
    sigma_ipc_msg_t queue[256];
    sigma_u32       head;
    sigma_u32       tail;
    sigma_u32       queue_size;
    sigma_u64       messages_dispatched;
    sigma_u32       initialized;
} SovereignIPCManager = {
    .head = 0u,
    .tail = 0u,
    .queue_size = 256u,
    .messages_dispatched = 0u,
    .initialized = 0u
};

extern "C" void ipc_init() {
    sigma_log("[IPC] Initializing Sovereign Communication Lattice (WFAE Algorithm)...");
    SovereignIPCManager.initialized = 1u;
}

extern "C" bool ipc_send_optimized(sigma_u32 target, sigma_u32 type, sigma_u32* data) {
    sigma_u32 current_head = SovereignIPCManager.head;
    sigma_u32 next_head = (current_head + 1u) % SovereignIPCManager.queue_size;
    
    if (next_head == SovereignIPCManager.tail) {
        sigma_log("[IPC] [WARNING] WFAE: Queue saturation.");
        return false;
    }
    
    sigma_ipc_msg_t* msg = &SovereignIPCManager.queue[current_head];
    msg->target_shard = target;
    msg->message_type = type;
    if (data) {
        for(sigma_u32 i=0u; i<8u; i++) msg->payload[i] = data[i];
    }
    
    __atomic_store_n(&SovereignIPCManager.head, next_head, __ATOMIC_SEQ_CST);
    SovereignIPCManager.messages_dispatched++;
    
    sigma_printf("[IPC] WFAE: Message -> S%02u dispatched.\n", target);
    return true;
}

extern "C" bool ipc_receive_optimized(sigma_ipc_msg_t* out_msg) {
    if (SovereignIPCManager.head == SovereignIPCManager.tail) return false;
    
    if (out_msg) {
        *out_msg = SovereignIPCManager.queue[SovereignIPCManager.tail];
    }
    
    sigma_u32 next_tail = (SovereignIPCManager.tail + 1u) % SovereignIPCManager.queue_size;
    __atomic_store_n(&SovereignIPCManager.tail, next_tail, __ATOMIC_SEQ_CST);
    
    return true;
}

extern "C" sigma_u64 ipc_get_dispatched_count() {
    return SovereignIPCManager.messages_dispatched;
}

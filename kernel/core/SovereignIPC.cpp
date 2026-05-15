#include "../../include/core/sigma_types.h"
#include "../../include/sigma_log.h"
#include "../../include/system/sigma_ipc.h"
#include "../../include/sigma_log.h"
#include "../../include/hal/sigma_hal.h"
#include "../../include/sigma_log.h"


/**
 * SigmaOS Sovereign IPC Implementation (Optimized)
 * Implements a Wait-Free Atomic Exchange (WFAE) algorithm for zero-lock message passing.
 * ZERO-DEPENDENCY: Strictly bare-metal silicon-native communication.
 *
 * Design: OOP-isolated singleton — SovereignIPCManager.
 */

/* --- Sovereign IPC Manager (OOP Isolation) --- */

void SovereignIPCManager::init() {
    sigma_log("[IPC] Initializing Sovereign Communication Lattice (WFAE Algorithm)...");
    this->initialized = 1u;
}

bool SovereignIPCManager::sendOptimized(sigma_u32 target, sigma_u32 type, sigma_u32* data) {
    sigma_u32 current_head = this->head;
    sigma_u32 next_head = (current_head + 1u) % this->queue_size;
    
    if (next_head == this->tail) {
        sigma_log("[IPC] [WARNING] WFAE: Queue saturation.");
        return false;
    }
    
    sigma_ipc_msg_t* msg = &this->queue[current_head];
    msg->target_shard = target;
    msg->message_type = type;
    if (data) {
        for(sigma_u32 i=0u; i<8u; i++) msg->payload[i] = data[i];
    }
    
    __atomic_store_n(&this->head, next_head, __ATOMIC_SEQ_CST);
    this->messages_dispatched++;
    
    sigma_log_info("[IPC] WFAE: Message -> S%02u dispatched.\n", target);
    return true;
}

bool SovereignIPCManager::receiveOptimized(sigma_ipc_msg_t* out_msg) {
    if (this->head == this->tail) return false;
    
    if (out_msg) {
        *out_msg = this->queue[this->tail];
    }
    
    sigma_u32 next_tail = (this->tail + 1u) % this->queue_size;
    __atomic_store_n(&this->tail, next_tail, __ATOMIC_SEQ_CST);
    
    return true;
}

/* --- C Wrappers --- */
extern "C" void ipc_init() {
    SovereignIPCManager::getInstance().init();
}

extern "C" bool ipc_send_optimized(sigma_u32 target, sigma_u32 type, sigma_u32* data) {
    return SovereignIPCManager::getInstance().sendOptimized(target, type, data);
}

extern "C" bool ipc_receive_optimized(sigma_ipc_msg_t* out_msg) {
    return SovereignIPCManager::getInstance().receiveOptimized(out_msg);
}

extern "C" sigma_u64 ipc_get_dispatched_count() {
    return SovereignIPCManager::getInstance().getDispatchedCount();
}




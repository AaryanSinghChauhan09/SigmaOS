#include "../../../include/SovereignLibC.h""
#include "../../../include/sigma_types.h""
#include "sigma_ipc.h"
#include "../../../include/sigma_hal.h""

/**
 * SigmaOS Sovereign IPC Implementation (Optimized)
 * Implements a Wait-Free Atomic Exchange (WFAE) algorithm for zero-lock message passing.
 * ZERO-DEPENDENCY: Strictly bare-metal silicon-native communication.
 *
 * Design: OOP-isolated singleton — SovereignIPCManager.
 */

namespace SigmaOS {
namespace Kernel {
namespace IPC {

void SovereignIPCManager::init() {
    sigma_log("[IPC] Initializing Sovereign Communication Lattice (WFAE Algorithm)...");
    this->m_initialized = 1u;
}

bool SovereignIPCManager::sendOptimized(sigma_u32 target, sigma_u32 type, sigma_u32* data) {
    sigma_u32 current_head = this->m_head;
    sigma_u32 next_head = (current_head + 1u) % this->m_queue_size;
    
    if (next_head == this->m_tail) {
        sigma_log("[IPC] [WARNING] WFAE: Queue saturation.");
        return false;
    }
    
    sigma_ipc_msg_t* msg = &this->m_queue[current_head];
    msg->target_shard = target;
    msg->message_type = type;
    if (data) {
        for(sigma_u32 i=0u; i<8u; i++) msg->payload[i] = data[i];
    }
    
    __atomic_store_n(&this->m_head, next_head, __ATOMIC_SEQ_CST);
    this->m_messages_dispatched++;
    
    sigma_printf("[IPC] WFAE: Message -> S%02u dispatched.\n", target);
    return true;
}

bool SovereignIPCManager::receiveOptimized(sigma_ipc_msg_t* out_msg) {
    if (this->m_head == this->m_tail) return false;
    
    if (out_msg) {
        *out_msg = this->m_queue[this->m_tail];
    }
    
    sigma_u32 next_tail = (this->m_tail + 1u) % this->m_queue_size;
    __atomic_store_n(&this->m_tail, next_tail, __ATOMIC_SEQ_CST);
    
    return true;
}

} // namespace IPC
} // namespace Kernel
} // namespace SigmaOS

/* --- C Wrappers --- */
extern "C" void ipc_init() {
    SigmaOS::Kernel::IPC::SovereignIPCManager::getInstance().init();
}

extern "C" bool ipc_send_optimized(sigma_u32 target, sigma_u32 type, sigma_u32* data) {
    return SigmaOS::Kernel::IPC::SovereignIPCManager::getInstance().sendOptimized(target, type, data);
}

extern "C" bool ipc_receive_optimized(sigma_ipc_msg_t* out_msg) {
    return SigmaOS::Kernel::IPC::SovereignIPCManager::getInstance().receiveOptimized(out_msg);
}

extern "C" sigma_u64 ipc_get_dispatched_count() {
    return SigmaOS::Kernel::IPC::SovereignIPCManager::getInstance().getDispatchedCount();
}




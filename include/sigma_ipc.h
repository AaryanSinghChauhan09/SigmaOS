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

class SovereignIPCManager {
public:
    static SovereignIPCManager& getInstance() {
        static SovereignIPCManager instance;
        return instance;
    }

    void init();
    bool sendOptimized(sigma_u32 target, sigma_u32 type, sigma_u32* data);
    bool receiveOptimized(sigma_ipc_msg_t* out_msg);
    sigma_u64 getDispatchedCount() const { return this->messages_dispatched; }

private:
    SovereignIPCManager() : head(0), tail(0), queue_size(256), messages_dispatched(0), initialized(0) {}
    
    sigma_ipc_msg_t queue[256];
    sigma_u32       head;
    sigma_u32       tail;
    sigma_u32       queue_size;
    sigma_u64       messages_dispatched;
    sigma_u32       initialized;
};
#endif

#endif /* SIGMA_IPC_H */

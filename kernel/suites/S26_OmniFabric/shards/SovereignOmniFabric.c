/*
 * =========================================================================
 * Σ SIGMAOS: S26_OMNIFABRIC — SovereignOmniFabric.c
 * =========================================================================
 * Mission: High-Performance, Zero-Leakage Inter-Shard Communication Bus.
 * Design: Ring-Buffer based, Lock-Free Asynchronous Message Dispatch.
 * =========================================================================
 */

#include "sigma_base.h"
#include "SovereignLatticeRegistry.h"

#define OMNIFABRIC_QUEUE_SIZE 4096

typedef struct {
    sigma_u32 sender_id;
    sigma_u32 receiver_id;
    sigma_u32 msg_type;
    sigma_u64 payload[4];
} OmniMessage;

static OmniMessage g_msg_ring[OMNIFABRIC_QUEUE_SIZE];
static volatile sigma_u32 g_head = 0;
static volatile sigma_u32 g_tail = 0;

void OmniFabric_Init(void) {
    g_head = 0;
    g_tail = 0;
    sigma_printf("Σ [OMNIFABRIC]: Sovereign Message Bus active. 4096-node capacity.\n");
}

sigma_err_t OmniFabric_Send(sigma_u32 sender, sigma_u32 receiver, sigma_u32 type, sigma_u64* data) {
    sigma_u32 next_tail = (g_tail + 1) % OMNIFABRIC_QUEUE_SIZE;
    
    if (next_tail == g_head) {
        return SIGMA_ERROR; // Queue full
    }
    
    OmniMessage* msg = &g_msg_ring[g_tail];
    msg->sender_id = sender;
    msg->receiver_id = receiver;
    msg->msg_type = type;
    if (data) {
        sigma_memcpy(msg->payload, data, sizeof(msg->payload));
    }
    
    g_tail = next_tail;
    return SIGMA_OK;
}

sigma_bool OmniFabric_Poll(OmniMessage* out_msg) {
    if (g_head == g_tail) return SIGMA_FALSE;
    
    sigma_memcpy(out_msg, &g_msg_ring[g_head], sizeof(OmniMessage));
    g_head = (g_head + 1) % OMNIFABRIC_QUEUE_SIZE;
    
    return SIGMA_TRUE;
}

void S26_OmniFabric_Register(void) {
    OmniFabric_Init();
    SovereignRegistry_Register("S26_OmniFabric", 0, NULL);
}

#include "../../../include/sigma_kernel_types.h"
/*
 * =============================================================================
 * Î£ SIGMAOS KERNEL: SOVEREIGN IPC FRAMEWORK (v1.0)
 * =============================================================================
 * Principles: Asynchronous Message Passing & Shard-Isolation Integrity.
 * =============================================================================
 */
#include "../../../include/sigma_kernel_types.h"

extern sigma_u32 cpu_get_id(void);

typedef struct Message {
    sigma_u64     sender_id;
    sigma_u64     receiver_id;
    sigma_u32     type;
    sigma_u8      data[256];
} msg_t;

#define MAX_PENDING_MSGS 256
static msg_t msg_queue[MAX_PENDING_MSGS];
static sigma_u32 msg_count = 0;

/* Send a sovereign message between shards */
int ipc_send(sigma_u64 receiver, sigma_u32 type, void* data, sigma_u32 size) {
    if (msg_count >= MAX_PENDING_MSGS) return -1;

    msg_t* m = &msg_queue[msg_count++];
    m->sender_id = cpu_get_id(); /* Contextual sender */
    m->receiver_id = receiver;
    m->type = type;
    sigma_memcpy(m->data, data, (size < 256) ? size : 256);

    return 0;
}

/* Retrieve the next message for the current task */
int ipc_recv(sigma_u64 receiver, msg_t* out_msg) {
    for (sigma_u32 i = 0; i < msg_count; i++) {
        if (msg_queue[i].receiver_id == receiver) {
            sigma_memcpy(out_msg, &msg_queue[i], sizeof(msg_t));
            
            /* Shift queue (Simple implementation) */
            for (sigma_u32 j = i; j < msg_count - 1; j++) {
                msg_queue[j] = msg_queue[j + 1];
            }
            msg_count--;
            return 0;
        }
    }
    return -1;
}

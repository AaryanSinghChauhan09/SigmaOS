/*
 * Σ SigmaOS — sigma_ipc: Sovereign Inter-Process Communication
 * Zero-Dependency: No POSIX pipes, no Unix sockets, no semaphores.
 * Absorbs: L4 microkernel message passing, Plan 9 from Bell Labs channel model.
 * Implements: Sovereign message queues with lock-free ring buffers.
 */

typedef unsigned int   u32;
typedef unsigned long long u64;
typedef unsigned char  u8;

extern "C" void sigma_vga_printf(const char* fmt, ...);

#define SIGMA_IPC_MSG_SIZE   256
#define SIGMA_IPC_QUEUE_DEPTH 64
#define SIGMA_IPC_MAX_QUEUES  32

/* Sovereign Message */
struct SigmaMessage {
    u32 sender_pid;
    u32 receiver_pid;
    u32 msg_type;
    u32 payload_len;
    u8  payload[SIGMA_IPC_MSG_SIZE];
};

/* Lock-Free Ring Buffer Queue (absorbs LMAX Disruptor concepts) */
struct SigmaIpcQueue {
    SigmaMessage messages[SIGMA_IPC_QUEUE_DEPTH];
    volatile u32 head;
    volatile u32 tail;
    u32 owner_pid;
    bool active;
};

static SigmaIpcQueue ipc_queues[SIGMA_IPC_MAX_QUEUES];

/* Create a message queue bound to a PID */
extern "C" int sigma_ipc_create_queue(u32 pid) {
    for (int i = 0; i < SIGMA_IPC_MAX_QUEUES; i++) {
        if (!ipc_queues[i].active) {
            ipc_queues[i].head = 0;
            ipc_queues[i].tail = 0;
            ipc_queues[i].owner_pid = pid;
            ipc_queues[i].active = true;
            return i;
        }
    }
    return -1;
}

/* Send a message to a PID's queue */
extern "C" int sigma_ipc_send(u32 dest_pid, u32 msg_type, const u8* payload, u32 len) {
    /* Find queue for dest */
    for (int i = 0; i < SIGMA_IPC_MAX_QUEUES; i++) {
        if (ipc_queues[i].active && ipc_queues[i].owner_pid == dest_pid) {
            u32 next_tail = (ipc_queues[i].tail + 1) % SIGMA_IPC_QUEUE_DEPTH;
            if (next_tail == ipc_queues[i].head) return -1; /* Full */

            SigmaMessage* msg = &ipc_queues[i].messages[ipc_queues[i].tail];
            msg->receiver_pid = dest_pid;
            msg->msg_type = msg_type;
            msg->payload_len = (len < SIGMA_IPC_MSG_SIZE) ? len : SIGMA_IPC_MSG_SIZE;
            for (u32 j = 0; j < msg->payload_len; j++) msg->payload[j] = payload[j];

            ipc_queues[i].tail = next_tail;
            return 0;
        }
    }
    return -2; /* No queue for PID */
}

/* Receive from own queue (non-blocking) */
extern "C" int sigma_ipc_recv(u32 my_pid, SigmaMessage* out) {
    for (int i = 0; i < SIGMA_IPC_MAX_QUEUES; i++) {
        if (ipc_queues[i].active && ipc_queues[i].owner_pid == my_pid) {
            if (ipc_queues[i].head == ipc_queues[i].tail) return -1; /* Empty */

            SigmaMessage* msg = &ipc_queues[i].messages[ipc_queues[i].head];
            for (u32 j = 0; j < sizeof(SigmaMessage); j++)
                ((u8*)out)[j] = ((u8*)msg)[j];

            ipc_queues[i].head = (ipc_queues[i].head + 1) % SIGMA_IPC_QUEUE_DEPTH;
            return 0;
        }
    }
    return -2;
}

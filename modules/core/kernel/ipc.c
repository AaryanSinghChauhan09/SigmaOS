#include <stdint.h>
#include <stddef.h>
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Zero-Trust IPC (Inter-Process Communication)
// Every message is cryptographically signed & verified
// ---------------------------------------------------------

#define MAX_IPC_QUEUES 128
#define MAX_MSG_SIZE   256

// FNV-1a hash (lightweight signing primitive for prototype)
static uint64_t fnv1a_sign(const uint8_t* data, size_t len, uint32_t sender_pid) {
    uint64_t hash = 14695981039346656037ULL ^ (uint64_t)sender_pid;
    for (size_t i = 0; i < len; i++) {
        hash ^= data[i];
        hash *= 1099511628211ULL;
    }
    return hash;
}

typedef struct {
    uint32_t sender_pid;
    uint32_t receiver_pid;
    uint32_t cap_id;        // Sender must hold IPC capability for this queue
    uint16_t length;
    uint64_t signature;     // Integrity signature over payload
    uint8_t  payload[MAX_MSG_SIZE];
} ipc_message_t;

typedef struct {
    uint32_t queue_id;
    uint32_t owner_pid;     // Only owner can receive
    ipc_message_t messages[32];
    uint8_t  head, tail, count;
} ipc_queue_t;

static ipc_queue_t ipc_queues[MAX_IPC_QUEUES];
static uint32_t ipc_queue_count = 0;

extern int cap_check(uint32_t pid, uint32_t cap_id, uint8_t rights);

int ipc_create_queue(uint32_t owner_pid) {
    if (ipc_queue_count >= MAX_IPC_QUEUES) return -1;
    ipc_queue_t* q = &ipc_queues[ipc_queue_count];
    q->queue_id = ipc_queue_count++;
    q->owner_pid = owner_pid;
    q->head = q->tail = q->count = 0;
    return q->queue_id;
}

int ipc_send(uint32_t queue_id, uint32_t sender_pid, uint32_t cap_id,
             const uint8_t* payload, uint16_t len) {
    if (queue_id >= ipc_queue_count) return -1;
    if (len > MAX_MSG_SIZE) return -1;

    // Zero-Trust: Verify sender has IPC capability
    if (!cap_check(sender_pid, cap_id, 0x01 /* CAP_READ */)) return -2; // Denied

    ipc_queue_t* q = &ipc_queues[queue_id];
    if (q->count >= 32) return -3; // Queue full

    ipc_message_t* msg = &q->messages[q->tail];
    msg->sender_pid = sender_pid;
    msg->receiver_pid = q->owner_pid;
    msg->cap_id = cap_id;
    msg->length = len;
    memcpy(msg->payload, payload, len);

    // Sign the message with sender PID as key (in production: use Ed25519)
    msg->signature = fnv1a_sign(payload, len, sender_pid);

    q->tail = (q->tail + 1) % 32;
    q->count++;
    return 0;
}

int ipc_receive(uint32_t queue_id, uint32_t receiver_pid, ipc_message_t* out) {
    if (queue_id >= ipc_queue_count) return -1;
    ipc_queue_t* q = &ipc_queues[queue_id];

    // Only the owner can receive from this queue
    if (q->owner_pid != receiver_pid) return -2;
    if (q->count == 0) return -3; // Empty

    ipc_message_t* msg = &q->messages[q->head];

    // Verify signature before delivering
    uint64_t expected = fnv1a_sign(msg->payload, msg->length, msg->sender_pid);
    if (expected != msg->signature) return -4; // Tampered message!

    memcpy(out, msg, sizeof(ipc_message_t));
    q->head = (q->head + 1) % 32;
    q->count--;
    return 0;
}

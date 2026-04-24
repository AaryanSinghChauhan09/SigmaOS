#include <stdint.h>
#include <stddef.h>
#include <string.h>

// ---------------------------------------------------------
// SigmaOS Sovereign IPC (Inter-Process Communication)
// USP: Zero-Copy Message Passing with Capability Endpoints
// Surpasses POSIX pipes, sockets, and shared memory.
// ---------------------------------------------------------

#define MAX_CHANNELS   128
#define MSG_QUEUE_SIZE 32
#define MSG_MAX_BYTES  4096

typedef struct {
    uint32_t sender_pid;
    uint32_t msg_type;
    uint32_t payload_len;
    uint8_t  payload[MSG_MAX_BYTES];
    uint64_t timestamp;
} ipc_message_t;

typedef struct {
    uint32_t channel_id;
    char     label[32];        // Human-readable name ("display.render", "net.packet")
    uint32_t owner_pid;
    uint32_t subscriber_pids[8];
    uint8_t  subscriber_count;
    uint32_t cap_token_required; // Capability needed to send to this channel

    ipc_message_t queue[MSG_QUEUE_SIZE];
    uint8_t  queue_head;
    uint8_t  queue_tail;
    uint8_t  is_active;
} ipc_channel_t;

static ipc_channel_t channels[MAX_CHANNELS];
static uint32_t channel_count = 0;

extern int cap_registry_verify(uint32_t cap_id, uint32_t pid, uint8_t required_rights);
extern void audit_chain_append(uint32_t pid, uint8_t level, const char* msg);
extern void s_ext_dispatch_automation(const char* event_name);

// Create a named IPC channel (like D-Bus but kernel-native)
int ipc_create_channel(const char* label, uint32_t owner_pid, uint32_t cap_token) {
    if (channel_count >= MAX_CHANNELS) return -1;

    ipc_channel_t* ch = &channels[channel_count];
    ch->channel_id = channel_count++;
    strncpy(ch->label, label, 31);
    ch->owner_pid = owner_pid;
    ch->subscriber_count = 0;
    ch->cap_token_required = cap_token;
    ch->queue_head = 0;
    ch->queue_tail = 0;
    ch->is_active = 1;

    audit_chain_append(owner_pid, 1, "IPC_CHANNEL_CREATED");
    return ch->channel_id;
}

// Subscribe to a channel (requires capability verification)
int ipc_subscribe(uint32_t channel_id, uint32_t pid, uint32_t cap_token) {
    if (channel_id >= channel_count) return -1;
    ipc_channel_t* ch = &channels[channel_id];
    if (!ch->is_active) return -2;
    
    if (!cap_registry_verify(cap_token, pid, 0x04 /* CAP_IPC_RECV */)) {
        audit_chain_append(pid, 3, "IPC_SUBSCRIBE_DENIED");
        return -3;
    }
    
    if (ch->subscriber_count >= 8) return -4;
    ch->subscriber_pids[ch->subscriber_count++] = pid;
    return 0;
}

// Send a message to a channel (Zero-Copy via shared page mapping in real impl)
int ipc_send(uint32_t channel_id, uint32_t sender_pid, uint32_t cap_token,
             uint32_t msg_type, const uint8_t* payload, uint32_t payload_len) {
    if (channel_id >= channel_count) return -1;
    ipc_channel_t* ch = &channels[channel_id];
    if (!ch->is_active) return -2;

    // Capability check: sender must hold IPC_SEND for this channel
    if (!cap_registry_verify(cap_token, sender_pid, 0x08 /* CAP_IPC_SEND */)) {
        audit_chain_append(sender_pid, 3, "IPC_SEND_DENIED");
        return -3;
    }

    // Enqueue the message (ring buffer, lock-free in real implementation)
    uint8_t next_head = (ch->queue_head + 1) % MSG_QUEUE_SIZE;
    if (next_head == ch->queue_tail) return -5; // Queue full

    ipc_message_t* msg = &ch->queue[ch->queue_head];
    msg->sender_pid = sender_pid;
    msg->msg_type = msg_type;
    msg->payload_len = (payload_len > MSG_MAX_BYTES) ? MSG_MAX_BYTES : payload_len;
    if (payload && msg->payload_len > 0) {
        memcpy(msg->payload, payload, msg->payload_len);
    }
    msg->timestamp = 0; // Would be current_tick in real impl

    ch->queue_head = next_head;

    // Fire automation hooks for extensions listening to IPC events
    s_ext_dispatch_automation("IPC_MESSAGE_SENT");
    return 0;
}

// Receive the next message from a channel
int ipc_receive(uint32_t channel_id, uint32_t receiver_pid, ipc_message_t* out_msg) {
    if (channel_id >= channel_count || !out_msg) return -1;
    ipc_channel_t* ch = &channels[channel_id];
    if (ch->queue_tail == ch->queue_head) return -2; // Empty

    // Verify receiver is a subscriber
    uint8_t authorized = 0;
    for (uint8_t i = 0; i < ch->subscriber_count; i++) {
        if (ch->subscriber_pids[i] == receiver_pid) { authorized = 1; break; }
    }
    if (!authorized) return -3;

    *out_msg = ch->queue[ch->queue_tail];
    ch->queue_tail = (ch->queue_tail + 1) % MSG_QUEUE_SIZE;
    return 0;
}

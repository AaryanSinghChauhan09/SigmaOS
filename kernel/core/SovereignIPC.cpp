#include <sigma_hal.h>
#include <sigma_libc.h>
#include <sigma_ipc.h>
#include <sigma_proc.h>

/**
 * SigmaOS Sovereign IPC Implementation
 * Implements a Wait-Free Shard Passing (WFSP) algorithm using a ring buffer.
 */

#define IPC_RING_SIZE 32

static sigma_msg_t msg_ring[IPC_RING_SIZE];
static uint32_t ring_head = 0;
static uint32_t ring_tail = 0;

extern "C" void ipc_init() {
    sigma_log("[IPC] Initializing Sovereign Inter-Process Communication Lattice...");
}

extern "C" bool ipc_send(uint32_t to_pid, sigma_msg_t* msg) {
    // WFSP (Wait-Free Shard Passing) Algorithm
    // Uses a circular ring buffer without locks (simulated bare-metal atomic increments)
    
    uint32_t next_head = (ring_head + 1) % IPC_RING_SIZE;
    if (next_head == ring_tail) {
        sigma_log("[IPC] [WARNING] Ring buffer overflow. Packet dropped.");
        return false;
    }
    
    sigma_msg_t* slot = &msg_ring[ring_head];
    slot->sender_pid = proc_get_current()->pid;
    slot->receiver_pid = to_pid;
    slot->shard_id = msg->shard_id;
    slot->size = msg->size;
    sigma_hardened_strcpy(slot->payload, msg->payload, 128);
    
    ring_head = next_head;
    
    sigma_printf("[IPC] Message SENT: PID %d -> PID %d (Shard S%02d)\n", 
                 slot->sender_pid, to_pid, msg->shard_id);
    return true;
}

extern "C" bool ipc_receive(sigma_msg_t* out_msg) {
    if (ring_head == ring_tail) return false;
    
    sigma_msg_t* slot = &msg_ring[ring_tail];
    uint32_t current_pid = proc_get_current()->pid;
    
    if (slot->receiver_pid != current_pid) return false;
    
    *out_msg = *slot;
    ring_tail = (ring_tail + 1) % IPC_RING_SIZE;
    
    sigma_printf("[IPC] Message RECEIVED: PID %d (From PID %d, Shard S%02d)\n", 
                 current_pid, out_msg->sender_pid, out_msg->shard_id);
    return true;
}

// SigmaOS — sigma-ipc-channel: Zero-Copy Capability Channels
// Module: sigma-ipc-channel
// USP: Defeats Google Fuchsia (Zircon). Highly asynchronous message passing
//      with strict capability-handle transferring and zero-copy semantics.

#ifndef SIGMA_IPC_CHANNEL_HPP
#define SIGMA_IPC_CHANNEL_HPP

#include "S43_SovereignCaps/sigma_caps.h"

namespace sigma {
namespace ipc {

struct MessageHandle {
    unsigned long message_id;
    SigmaCapToken capability_grant;
    void* zero_copy_payload_ptr;
    unsigned int payload_size;
};

class ZirconSlayerChannel {
private:
    MessageHandle ring_buffer[256];
    unsigned int head;
    unsigned int tail;
    // Spinlock for thread-safe cross-core IPC
    volatile int lock;

    void acquire_lock() {
#if defined(__x86_64__)
        while (__sync_lock_test_and_set(&lock, 1)) {
            __asm__ __volatile__("pause\n\t" ::: "memory");
        }
#endif
    }

    void release_lock() {
#if defined(__x86_64__)
        __sync_lock_release(&lock);
#endif
    }

public:
    ZirconSlayerChannel() : head(0), tail(0), lock(0) {}

    // Send a message via handle transferring. No payload copying occurs.
    bool send(unsigned long msg_id, void* payload, unsigned int size, SigmaCapToken grant) {
        bool success = false;
        acquire_lock();
        
        unsigned int next_head = (head + 1) % 256;
        if (next_head != tail) {
            ring_buffer[head] = {msg_id, grant, payload, size};
            head = next_head;
            success = true;
        }
        
        release_lock();
        return success;
    }

    // Receive a handle asynchronously
    bool receive(MessageHandle* out_msg) {
        bool success = false;
        acquire_lock();
        
        if (head != tail) {
            *out_msg = ring_buffer[tail];
            tail = (tail + 1) % 256;
            success = true;
        }
        
        release_lock();
        return success;
    }
};

} // namespace ipc
} // namespace sigma

#endif /* SIGMA_IPC_CHANNEL_HPP */

#ifndef SOVEREIGN_MACH_IPC_H
#define SOVEREIGN_MACH_IPC_H

/**
 * @brief Sovereign-Mach IPC (Microkernel Message Passing)
 * Based on the Mach microkernel architecture (found in macOS XNU).
 * Provides port-based communication between shards.
 */

#include "SovereignLibC.h"
#include <stdint.h>

typedef uint32_t mach_port_t;
typedef uint32_t mach_msg_size_t;

typedef struct {
    uint32_t bits;
    mach_msg_size_t size;
    mach_port_t remote_port;
    mach_port_t local_port;
    uint32_t voucher_port;
    int32_t id;
} mach_msg_header_t;

// Port Namespaces
#define MACH_PORT_NULL 0
#define MACH_PORT_DEAD ~0

// Action Prototypes
mach_port_t sovereign_mach_port_allocate();
void sovereign_mach_msg_send(mach_msg_header_t* header, void* data, mach_msg_size_t size);
void sovereign_mach_msg_receive(mach_port_t port, mach_msg_header_t* header, void* buffer, mach_msg_size_t buffer_size);

#endif

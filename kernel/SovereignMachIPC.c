#include "SovereignMachIPC.h"
#include "libc/SovereignLibC.h"

static sigma_u32 current_port_id = 1000;

mach_port_t sovereign_mach_port_allocate() {
    sigma_u32 allocated = current_port_id++;
    sigma_printf("[MACH] Allocated Port: %u\n", allocated);
    return (mach_port_t)allocated;
}

void sovereign_mach_msg_send(mach_msg_header_t* header, void* data, mach_msg_size_t size) {
    if (header->remote_port == MACH_PORT_NULL) {
        sigma_printf("[MACH] Error: Sending to NULL port.\n");
        return;
    }
    sigma_printf("[MACH] Sending Message ID: %d to Port %u (Size: %u bytes)\n", header->id, header->remote_port, size);
    // Implementing basic silicon-direct copying (simulated)
    // sigma_memcpy(target_buffer, data, size);
}

void sovereign_mach_msg_receive(mach_port_t port, mach_msg_header_t* header, void* buffer, mach_msg_size_t buffer_size) {
    if (port == MACH_PORT_NULL) {
        sigma_printf("[MACH] Error: Receiving from NULL port.\n");
        return;
    }
    sigma_printf("[MACH] Receiving message from Port %u (Current Buffer: %u bytes)\n", port, buffer_size);
    // Mimicking blocking receive
    // wait_for_data(port);
}

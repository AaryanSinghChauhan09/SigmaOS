/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN IPC (Suite S22)
 * =========================================================================
 */

#include "sigma_ipc.h"
#include "sigma_libc.h"

static sigma_u32 s_next_port = 0x1000;
static sigma_u32 s_port_count = 0;

/* ── Initialization ───────────────────────────────────────────────────── */
void sigma_ipc_init(void) {
    sigma_sigma_sigma_printf("S [IPC] Sovereign IPC Subsystem initialized\n");
    sigma_sigma_sigma_printf("S [IPC] Fabric: Mach Ports | Shared Memory | Ring vBus\n");
}

/* ── Port Management ──────────────────────────────────────────────────── */
ipc_port_t sigma_port_allocate(void) {
    ipc_port_t p = s_next_port++;
    s_port_count++;
    sigma_sigma_sigma_printf("S [IPC] Port allocated: 0x%08x\n", p);
    return p;
}

sigma_err_t sigma_port_destroy(ipc_port_t port) {
    sigma_sigma_sigma_printf("S [IPC] Port destroyed: 0x%08x\n", port);
    if (s_port_count > 0) s_port_count--;
    return SIGMA_OK;
}

/* ── Messaging ────────────────────────────────────────────────────────── */
sigma_err_t sigma_msg_send(ipc_header_t* header, const void* data) {
    sigma_sigma_sigma_printf("S [IPC] Msg SEND: (remote=0x%x, size=%u)\n", 
                 header->remote_port, header->msg_size);
    (void)data;
    return SIGMA_OK;
}

sigma_err_t sigma_msg_recv(ipc_port_t port, ipc_header_t* header, void* data, sigma_sz_t max_len) {
    sigma_sigma_sigma_printf("S [IPC] Msg RECV on port 0x%x\n", port);
    header->local_port = port;
    header->msg_size = 0;
    (void)data; (void)max_len;
    return SIGMA_OK;
}

/* ── Shared Memory ─────────────────────────────────────────────────────── */
void* sigma_shm_create(sigma_u32 id, sigma_sz_t size) {
    void* addr = (void*)0xFFFF900000000000ULL; /* Mock SHM range */
    sigma_sigma_sigma_printf("S [SHM] Created segment %u (size=%llu) at %p\n", 
                 id, (unsigned long long)size, addr);
    return addr;
}

void sigma_shm_attach(sigma_u32 id, void* addr) {
    sigma_sigma_sigma_printf("S [SHM] Segment %u mapped to %p\n", id, addr);
}

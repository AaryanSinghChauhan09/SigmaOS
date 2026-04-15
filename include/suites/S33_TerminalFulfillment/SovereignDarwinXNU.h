/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN DARWIN XNU (v1.0 — C11)
 * =========================================================================
 * Absorbed USPs from: Apple/Darwin XNU
 *   https://github.com/apple/darwin-xnu
 *
 * Features implemented:
 *   ✓ Mach Ports (Capability-based IPC endpoints)
 *   ✓ Mach Messages (Out-of-line data, port rights transfer)
 *   ✓ Grand Central Dispatch (GCD) / libdispatch parity (Dispatch Queues)
 * =========================================================================
 */

#ifndef SOVEREIGN_DARWIN_XNU_H
#define SOVEREIGN_DARWIN_XNU_H

#include "suites/S01_Genesis/shards/sigma_types.h"

/* --- MACH PORTS --- */
typedef sigma_u32 mach_port_t;
#define MACH_PORT_NULL 0

typedef struct {
    sigma_u32 msgh_bits;
    sigma_u32 msgh_size;
    mach_port_t msgh_remote_port;
    mach_port_t msgh_local_port;
    sigma_u32 msgh_id;
} MachMsgHeader_t;

typedef struct {
    MachMsgHeader_t header;
    sigma_u8 body[256];
} MachMsg_t;

sigma_err_t sigma_mach_port_allocate(mach_port_t *out_port);
sigma_err_t sigma_mach_msg_send(MachMsg_t *msg);
sigma_err_t sigma_mach_msg_receive(MachMsg_t *msg);

/* --- GRAND CENTRAL DISPATCH (GCD) --- */
typedef void (*dispatch_block_t)(void *context);

typedef struct {
    const char *label;
    sigma_bool serial; 
    /* Queue state underneath */
} DispatchQueue_t;

DispatchQueue_t* sigma_dispatch_queue_create(const char *label, sigma_bool serial);
void sigma_dispatch_async(DispatchQueue_t *queue, dispatch_block_t block, void *context);
void sigma_dispatch_sync(DispatchQueue_t *queue, dispatch_block_t block, void *context);

void SovereignDarwinXNU_Init(void);

#endif /* SOVEREIGN_DARWIN_XNU_H */

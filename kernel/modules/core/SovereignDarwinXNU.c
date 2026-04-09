/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DARWIN XNU — IMPL (v1.0 — C11)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignDarwinXNU.h"

static sigma_u32 s_next_mach_port = 1;

static DispatchQueue_t s_main_queue = { "com.apple.main-thread", SIGMA_TRUE };

sigma_err_t sigma_mach_port_allocate(mach_port_t *out_port) {
    *out_port = s_next_mach_port++;
    sigma_printf("Σ [XNU]: Allocated Mach Port #%u\n", *out_port);
    return SIGMA_OK;
}

sigma_err_t sigma_mach_msg_send(MachMsg_t *msg) {
    sigma_printf("Σ [XNU]: Mach Msg Send: id=%u -> port=%u (size=%u)\n", msg->header.msgh_id, msg->header.msgh_remote_port, msg->header.msgh_size);
    return SIGMA_OK;
}

sigma_err_t sigma_mach_msg_receive(MachMsg_t *msg) {
    sigma_printf("Σ [XNU]: Mach Msg Receive: polling port=%u\n", msg->header.msgh_local_port);
    return SIGMA_OK;
}

DispatchQueue_t* sigma_dispatch_queue_create(const char *label, sigma_bool serial) {
    static DispatchQueue_t queue;
    queue.label = label;
    queue.serial = serial;
    sigma_printf("Σ [GCD]: Created libdispatch queue '%s' (serial=%s)\n", label, serial ? "YES" : "NO");
    return &queue;
}

void sigma_dispatch_async(DispatchQueue_t *queue, dispatch_block_t block, void *context) {
    sigma_printf("Σ [GCD]: Queueing async block on '%s'...\n", queue->label);
    if(block) block(context); /* Simulated immediate run */
}

void sigma_dispatch_sync(DispatchQueue_t *queue, dispatch_block_t block, void *context) {
    sigma_printf("Σ [GCD]: Queueing sync block on '%s'...\n", queue->label);
    if(block) block(context);
}

static void sample_gcd_worker(void *ctx) {
    (void)ctx;
    sigma_printf("Σ [GCD]: Executing Grand Central Dispatch block payload.\n");
}

void SovereignDarwinXNU_Init(void) {
    sigma_printf("Σ [XNU]: Initialising Sovereign Darwin XNU parity (Mach/GCD)...\n");

    mach_port_t my_port;
    sigma_mach_port_allocate(&my_port);
    
    MachMsg_t msg;
    sigma_memset(&msg, 0, sizeof(msg));
    msg.header.msgh_id = 100;
    msg.header.msgh_local_port = my_port;
    msg.header.msgh_remote_port = 0;
    msg.header.msgh_size = sizeof(MachMsg_t);
    sigma_mach_msg_send(&msg);

    DispatchQueue_t *my_q = sigma_dispatch_queue_create("sigma.xnu.worker", SIGMA_TRUE);
    sigma_dispatch_async(my_q, sample_gcd_worker, SIGMA_NULL);
    sigma_dispatch_async(&s_main_queue, sample_gcd_worker, SIGMA_NULL);
}

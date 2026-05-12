/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN LAZY-LOAD ACTIVATION (S-LAZYLOAD)
 * =========================================================================
 * Mission: On-demand service execution based on socket/event triggers,
 * drastically reducing boot times and memory overhead (systemd USP).
 * =========================================================================
 */

#ifndef SIGMA_LAZYLOAD_H
#define SIGMA_LAZYLOAD_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    TRIGGER_TYPE_SOCKET,
    TRIGGER_TYPE_IPC_CALL,
    TRIGGER_TYPE_HARDWARE_INTERRUPT
} sigma_trigger_type_t;

/* --- Lazy-Load Primitives --- */
void lazyload_init(void);
void lazyload_register_service(uint32_t shard_id, sigma_trigger_type_t trigger);
void lazyload_trigger_event(sigma_trigger_type_t trigger, uint32_t context_id);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LAZYLOAD_H */

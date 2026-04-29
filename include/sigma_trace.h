/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM CALL TRACING (S-TRACE)
 * =========================================================================
 * Mission: Real-time syscall observability and predictive interception.
 * =========================================================================
 */

#ifndef SIGMA_TRACE_H
#define SIGMA_TRACE_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t syscall_id;
    uint32_t caller_shard_id;
    uint32_t timestamp_ms;
    bool is_intercepted;
} sigma_trace_event_t;

/* --- Trace Primitives --- */
void trace_init(void);
void trace_log_syscall(uint32_t id, uint32_t shard_id);
void trace_set_interceptor(uint32_t syscall_id, bool active);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_TRACE_H */

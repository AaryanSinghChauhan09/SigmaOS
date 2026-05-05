/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM LOGGING (S-LOG)
 * =========================================================================
 * Mission: Wait-free, circular shard logging and machine-state tracing.
 * =========================================================================
 */

#ifndef SIGMA_LOG_H
#define SIGMA_LOG_H

#include "sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

#define LOG_DEBUG    0u
#define LOG_INFO     1u
#define LOG_WARN     2u
#define LOG_ERROR    3u
#define LOG_CRITICAL 4u

typedef struct {
    uint32_t timestamp;
    uint32_t severity;
    char message[128];
} sigma_log_entry_t;

/* --- Log Primitives --- */
void      log_init(void);
void      log_emit(sigma_u32 severity, const char* message);
void      log_dump_lattice(void);
sigma_u64 log_get_total_emitted(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LOG_H */

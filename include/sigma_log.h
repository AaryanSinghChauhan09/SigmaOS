/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM LOGGING (S-LOG)
 * =========================================================================
 * Mission: Wait-free, circular shard logging and machine-state tracing.
 * =========================================================================
 */

#ifndef SIGMA_LOG_H
#define SIGMA_LOG_H

#include <sigma_types.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t timestamp;
    uint32_t severity;
    char message[128];
} sigma_log_entry_t;

/* --- Log Primitives --- */
void log_init(void);
void log_emit(uint32_t severity, const char* message);
void log_dump_lattice(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LOG_H */

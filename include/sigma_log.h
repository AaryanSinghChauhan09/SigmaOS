/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN SYSTEM LOGGING (S-LOG) v2.0
 * =========================================================================
 * Mission: Wait-free, circular shard logging and machine-state tracing.
 * =========================================================================
 */

#ifndef SIGMA_LOG_H
#define SIGMA_LOG_H

#include "core/sigma_types.h"

#ifdef __cplusplus
extern "C" {
#endif

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

extern int sigma_printf(const char* format, ...);

/* --- Severity-gated log macros --- */
#define sigma_log(msg)          sigma_printf("%s", msg)
#define sigma_log_info(...)     sigma_printf(__VA_ARGS__)
#define sigma_log_warn(...)     sigma_printf(__VA_ARGS__)
#define sigma_log_err(...)      sigma_printf(__VA_ARGS__)
#define sigma_log_error(...)    sigma_printf(__VA_ARGS__)
#define sigma_log_crit(...)     sigma_printf(__VA_ARGS__)

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LOG_H */

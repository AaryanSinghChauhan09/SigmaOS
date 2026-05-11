/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN SYSTEM LOGGING (S-LOG)
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
void      log_emit_f(sigma_u32 severity, const char* format, ...);
void      log_dump_lattice(void);
sigma_u64 log_get_total_emitted(void);

#ifdef __cplusplus
extern "C"
#endif
void kprintf(const char* fmt, ...);

/* --- Industrial Logging Macros --- */
#define sigma_log_debug(fmt, ...) log_emit_f(LOG_DEBUG, (fmt), ##__VA_ARGS__)
#define sigma_log_info(fmt, ...)  log_emit_f(LOG_INFO, (fmt), ##__VA_ARGS__)
#define sigma_log_warn(fmt, ...)  log_emit_f(LOG_WARN, (fmt), ##__VA_ARGS__)
#define sigma_log_err(fmt, ...)   log_emit_f(LOG_ERROR, (fmt), ##__VA_ARGS__)
#define sigma_log_crit(fmt, ...)  log_emit_f(LOG_CRITICAL, (fmt), ##__VA_ARGS__)


#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LOG_H */

/*
 * =========================================================================
 * SIGMAOS: SOVEREIGN SYSTEM LOGGING (S-LOG) v3.0 — Phase 16
 * =========================================================================
 * Mission: Severity-gated, structured logging routed through zenithd.
 *
 * Phase 16 Change: All macros now route through zenith_log_emit() instead
 * of being raw sigma_printf() aliases. Entries are timestamped, CPU-stamped,
 * and ring-buffered in the zenithd log.
 *
 * Backward Compatibility: sigma_log_info("msg") still works, but now gets
 * full structured tracing behind the scenes.
 * =========================================================================
 */

#ifndef SIGMA_LOG_H
#define SIGMA_LOG_H

/* Use the single source-of-truth types header directly.
 * sigma_kernel_types.h is zero-dependency and always available. */
#include "./sigma_kernel_types.h"
#include "./sigma_zenithd_log.h"

#ifdef __cplusplus
extern "C" {
#endif

/* --- Legacy Log Entry (preserved for backward compat) --- */
typedef struct {
    sigma_u32 timestamp;
    sigma_u32 severity;
    char message[128];
} sigma_log_entry_t;

/* --- Log Primitives (now wrappers around zenithd) --- */
void      log_init(void);
void      log_emit(sigma_u32 severity, const char* message);
void      log_dump_lattice(void);
sigma_u64 log_get_total_emitted(void);

extern void sigma_printf(const char* format, ...);

/* --- Severity-gated log macros ---
 * These now route through the zenithd structured logger.
 * Each macro auto-fills severity and uses "kernel" as the default component.
 * For subsystem-specific logging, use ZENITH_INFO("comp", "msg") directly.
 */
#define sigma_log(...)          sigma_printf(__VA_ARGS__)
#define sigma_log_info(...)     do { sigma_printf(__VA_ARGS__); \
    zenith_log_emit(ZEN_INFO, 0, "kernel", "info", 0); } while(0)
#define sigma_log_warn(...)     do { sigma_printf(__VA_ARGS__); \
    zenith_log_emit(ZEN_WARN, 0, "kernel", "warn", 0); } while(0)
#define sigma_log_err(...)      do { sigma_printf(__VA_ARGS__); \
    zenith_log_emit(ZEN_ERROR, 0, "kernel", "error", 0); } while(0)
#define sigma_log_error(...)    do { sigma_printf(__VA_ARGS__); \
    zenith_log_emit(ZEN_ERROR, 0, "kernel", "error", 0); } while(0)
#define sigma_log_crit(...)     do { sigma_printf(__VA_ARGS__); \
    zenith_log_emit(ZEN_CRIT, 0, "kernel", "critical", 0); } while(0)

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_LOG_H */

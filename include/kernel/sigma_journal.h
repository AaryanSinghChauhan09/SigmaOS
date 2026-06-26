/*
 * =============================================================================
 * Σ SIGMAOS: SOVEREIGN JOURNAL (sigma-journal v1.0)
 * =============================================================================
 * Mission: Persistent, structured binary logging with ring-buffer storage,
 *          severity filtering, and queryable boot/service log history.
 * Absorbs: systemd-journald structured logging, syslog severity levels.
 * Standard: C11/C++17 — Zero external dependencies.
 * =============================================================================
 */

#ifndef SIGMA_JOURNAL_H
#define SIGMA_JOURNAL_H

#include "../sigma_kernel_types.h"

/* --- Journal Configuration --- */
#define JOURNAL_MAX_ENTRIES      4096
#define JOURNAL_MSG_LEN           192
#define JOURNAL_SOURCE_LEN         48
#define JOURNAL_PERSIST_PATH  "/var/log/sigma-journal.bin"

/* --- Severity Levels (syslog-compatible) --- */
typedef enum {
    JOURNAL_EMERG   = 0,   /* System is unusable                     */
    JOURNAL_ALERT   = 1,   /* Action must be taken immediately       */
    JOURNAL_CRIT    = 2,   /* Critical conditions                    */
    JOURNAL_ERR     = 3,   /* Error conditions                       */
    JOURNAL_WARNING = 4,   /* Warning conditions                     */
    JOURNAL_NOTICE  = 5,   /* Normal but significant condition       */
    JOURNAL_INFO    = 6,   /* Informational messages                 */
    JOURNAL_DEBUG   = 7    /* Debug-level messages                   */
} sigma_journal_severity_t;

/* --- Journal Entry (fixed-size for ring buffer) --- */
typedef struct {
    sigma_u64                seq;                        /* monotonic sequence number   */
    sigma_u64                timestamp_tsc;              /* TSC at time of emit         */
    sigma_u64                boot_id;                    /* boot generation counter     */
    sigma_journal_severity_t severity;                   /* message severity            */
    sigma_u32                pid;                        /* emitting process ID         */
    char                     source[JOURNAL_SOURCE_LEN]; /* service/subsystem name      */
    char                     message[JOURNAL_MSG_LEN];   /* log message                 */
} sigma_journal_entry_t;

/* --- Query Filter --- */
typedef struct {
    sigma_journal_severity_t min_severity;   /* minimum severity to include          */
    const char*              source_filter;  /* NULL = all, else exact match         */
    sigma_u64                since_seq;      /* entries with seq > this (0 = all)    */
    sigma_u64                boot_id;        /* 0 = current boot, else specific boot */
    sigma_u32                max_results;    /* 0 = unlimited                        */
} sigma_journal_query_t;

/* --- Query Result --- */
typedef struct {
    sigma_journal_entry_t*   entries;        /* pointer to result array              */
    sigma_u32                count;          /* number of results returned           */
    sigma_u64                newest_seq;     /* highest sequence number in results   */
} sigma_journal_result_t;

#ifdef __cplusplus
extern "C" {
#endif

/* --- Lifecycle --- */
void journal_init(void);
void journal_shutdown(void);

/* --- Emit --- */
void journal_emit(sigma_journal_severity_t severity, const char* source,
                  const char* fmt, ...);

/* Convenience macros for common severity levels */
#define journal_info(src, ...)  journal_emit(JOURNAL_INFO,    (src), __VA_ARGS__)
#define journal_warn(src, ...)  journal_emit(JOURNAL_WARNING, (src), __VA_ARGS__)
#define journal_err(src, ...)   journal_emit(JOURNAL_ERR,     (src), __VA_ARGS__)
#define journal_crit(src, ...)  journal_emit(JOURNAL_CRIT,    (src), __VA_ARGS__)
#define journal_debug(src, ...) journal_emit(JOURNAL_DEBUG,   (src), __VA_ARGS__)

/* --- Query --- */
sigma_u32 journal_query(const sigma_journal_query_t* filter,
                        sigma_journal_entry_t* out_buf, sigma_u32 buf_capacity);

/* --- Persistence --- */
int  journal_flush_to_disk(void);
int  journal_load_from_disk(void);

/* --- Statistics --- */
sigma_u64 journal_get_total_emitted(void);
sigma_u64 journal_get_current_seq(void);
sigma_u32 journal_get_entry_count(void);

/* --- Display --- */
void journal_print_recent(sigma_u32 count);
void journal_print_boot_log(void);

#ifdef __cplusplus
}
#endif

#endif /* SIGMA_JOURNAL_H */

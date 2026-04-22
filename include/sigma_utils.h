/*
 * ╔══════════════════════════════════════════════════════════════════════╗
 * ║  sigma_utils.h — SigmaOS Shared Utility Library                    ║
 * ║  Common logging, config, networking — zero duplication across shards║
 * ║  Include this instead of duplicating functionality in every suite   ║
 * ╚══════════════════════════════════════════════════════════════════════╝
 */
#ifndef SIGMA_UTILS_H
#define SIGMA_UTILS_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

/* ── Log Levels ──────────────────────────────────────────────────────────── */
typedef enum {
    SIGMA_LOG_DEBUG = 0,
    SIGMA_LOG_INFO  = 1,
    SIGMA_LOG_WARN  = 2,
    SIGMA_LOG_ERROR = 3,
    SIGMA_LOG_FATAL = 4,
} SigmaLogLevel;

/* ── Config Entry ─────────────────────────────────────────────────────────── */
#define SIGMA_CFG_MAX_ENTRIES 256
#define SIGMA_CFG_KEY_LEN      64
#define SIGMA_CFG_VAL_LEN     256

typedef struct {
    char key[SIGMA_CFG_KEY_LEN];
    char value[SIGMA_CFG_VAL_LEN];
} SigmaCfgEntry;

typedef struct {
    SigmaCfgEntry entries[SIGMA_CFG_MAX_ENTRIES];
    uint32_t      count;
} SigmaConfig;

/* ── Network Address ──────────────────────────────────────────────────────── */
typedef struct {
    uint8_t  ip[4];
    uint16_t port;
    uint8_t  protocol; /* 0=TCP, 1=UDP */
} SigmaNetAddr;

/* ═══════════════════════════════════════════════════════════════════════════
 * LOGGING API
 * ═══════════════════════════════════════════════════════════════════════════ */

/**
 * sigma_log - Emit a log entry to the Sovereign Audit Terminal
 * @level:   Log severity level
 * @module:  Short name of the emitting shard (e.g. "S05_MEMORY")
 * @msg:     Message string (no format specifiers — call sigma_log_fmt for that)
 */
void sigma_log(SigmaLogLevel level, const char *module, const char *msg);

/**
 * sigma_log_fmt - Format-safe log entry (wraps sigma_log with snprintf)
 * Usage: sigma_log_fmt(SIGMA_LOG_INFO, "S07_NET", "Connected to %s:%d", host, port);
 */
void sigma_log_fmt(SigmaLogLevel level, const char *module, const char *fmt, ...);

/* Convenience macros for each level */
#define SIGMA_DEBUG(mod, msg)  sigma_log(SIGMA_LOG_DEBUG, mod, msg)
#define SIGMA_INFO(mod, msg)   sigma_log(SIGMA_LOG_INFO,  mod, msg)
#define SIGMA_WARN(mod, msg)   sigma_log(SIGMA_LOG_WARN,  mod, msg)
#define SIGMA_ERROR(mod, msg)  sigma_log(SIGMA_LOG_ERROR, mod, msg)
#define SIGMA_FATAL(mod, msg)  sigma_log(SIGMA_LOG_FATAL, mod, msg)

/* ═══════════════════════════════════════════════════════════════════════════
 * CONFIG API
 * ═══════════════════════════════════════════════════════════════════════════ */

/**
 * sigma_cfg_init   - Zero-initialize a SigmaConfig structure
 * sigma_cfg_load   - Parse a flat key=value config file into SigmaConfig
 * sigma_cfg_get    - Retrieve a value by key (returns NULL if not found)
 * sigma_cfg_set    - Set or update a key-value pair
 * sigma_cfg_save   - Serialize SigmaConfig back to disk
 */
void        sigma_cfg_init(SigmaConfig *cfg);
int         sigma_cfg_load(SigmaConfig *cfg, const char *path);
const char *sigma_cfg_get (const SigmaConfig *cfg, const char *key);
int         sigma_cfg_set (SigmaConfig *cfg, const char *key, const char *value);
int         sigma_cfg_save(const SigmaConfig *cfg, const char *path);

/* ═══════════════════════════════════════════════════════════════════════════
 * NETWORKING API (minimal, no libc sockets — uses syscall wrappers)
 * ═══════════════════════════════════════════════════════════════════════════ */

/**
 * sigma_net_connect  - Open a TCP connection to addr, returns fd or -1
 * sigma_net_send     - Send raw bytes over fd
 * sigma_net_recv     - Receive bytes from fd into buf
 * sigma_net_close    - Close the file descriptor
 */
int  sigma_net_connect(const SigmaNetAddr *addr);
int  sigma_net_send   (int fd, const void *buf, size_t len);
int  sigma_net_recv   (int fd, void *buf, size_t max_len);
void sigma_net_close  (int fd);

/* ═══════════════════════════════════════════════════════════════════════════
 * UTILITY HELPERS
 * ═══════════════════════════════════════════════════════════════════════════ */

/** sigma_utils_version - Return version string of this library */
const char *sigma_utils_version(void);

/** sigma_assert - Panic with a message if condition is false (kernel-safe) */
#define sigma_assert(cond, msg) \
    do { if (!(cond)) { SIGMA_FATAL("UTILS", "ASSERT FAILED: " msg); while(1){} } } while(0)

#endif /* SIGMA_UTILS_H */

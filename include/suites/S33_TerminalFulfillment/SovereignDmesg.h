/*
 * =========================================================================
 * S SIGMAOS: SOVEREIGN DMESG — KERNEL RING BUFFER (v1.0 — PURE C11)
 * =========================================================================
 * Mission: Persistent kernel message log, printk-level filtering, dmesg CLI.
 * Inspired By: Linux printk / dmesg, FreeBSD dmesg, macOS OSLog / kern.msgbuf.
 * Principle: Lock-free ring buffer. Zero-allocation. Power-of-2 wrap.
 * =========================================================================
 */

#ifndef SOVEREIGN_DMESG_H
#define SOVEREIGN_DMESG_H

#include "sigma_types.h"

/* -------------------------------------------------------------------------
 * Log levels (mirrors Linux KERN_EMERG … KERN_DEBUG)
 * ---------------------------------------------------------------------- */
typedef enum {
    SIGMA_LOG_EMERG   = 0,  /* System is unusable               */
    SIGMA_LOG_ALERT   = 1,  /* Action must be taken immediately */
    SIGMA_LOG_CRIT    = 2,  /* Critical condition               */
    SIGMA_LOG_ERR     = 3,  /* Error condition                  */
    SIGMA_LOG_WARN    = 4,  /* Warning condition                */
    SIGMA_LOG_NOTICE  = 5,  /* Normal but significant           */
    SIGMA_LOG_INFO    = 6,  /* Informational                    */
    SIGMA_LOG_DEBUG   = 7,  /* Debug-level                      */
} SigmaLogLevel_t;

/* -------------------------------------------------------------------------
 * Ring buffer constants — must be power-of-2
 * ---------------------------------------------------------------------- */
#define SIGMA_DMESG_BUF_SIZE   (1u << 14)   /* 16 KiB ring */
#define SIGMA_DMESG_MSG_MAX    256
#define SIGMA_DMESG_MAX_MSGS   128

/* -------------------------------------------------------------------------
 * Message record
 * ---------------------------------------------------------------------- */
typedef struct {
    sigma_u64     timestamp_us;               /* µsec since boot       */
    SigmaLogLevel_t level;
    char          text[SIGMA_DMESG_MSG_MAX];
} SigmaDmesgMsg_t;

/* -------------------------------------------------------------------------
 * Ring buffer context
 * ---------------------------------------------------------------------- */
typedef struct {
    SigmaDmesgMsg_t msgs[SIGMA_DMESG_MAX_MSGS];
    sigma_u32       head;       /* Oldest message index  */
    sigma_u32       tail;       /* Next write index      */
    sigma_u32       count;      /* Messages present      */
    SigmaLogLevel_t min_level;  /* Filter: only show >= min_level */
} SigmaDmesgCtx_t;

/* -------------------------------------------------------------------------
 * Public API
 * ---------------------------------------------------------------------- */
void  sigma_dmesg_init    (SigmaDmesgCtx_t *ctx);
void  sigma_printk        (SigmaLogLevel_t level, const char *fmt, ...);
void  sigma_dmesg_dump    (const SigmaDmesgCtx_t *ctx);   /* dmesg */
void  sigma_dmesg_clear   (SigmaDmesgCtx_t *ctx);          /* dmesg -c */
void  sigma_dmesg_set_level(SigmaDmesgCtx_t *ctx, SigmaLogLevel_t lvl);

/* Global kernel ring buffer */
extern SigmaDmesgCtx_t g_sigma_dmesg;

/* Convenience macros (mirrors KERN_ERR, KERN_INFO …) */
#define SIGMA_KERN_EMERG(fmt,  ...) sigma_printk(SIGMA_LOG_EMERG,  fmt, ##__VA_ARGS__)
#define SIGMA_KERN_ERR(fmt,    ...) sigma_printk(SIGMA_LOG_ERR,    fmt, ##__VA_ARGS__)
#define SIGMA_KERN_WARN(fmt,   ...) sigma_printk(SIGMA_LOG_WARN,   fmt, ##__VA_ARGS__)
#define SIGMA_KERN_INFO(fmt,   ...) sigma_printk(SIGMA_LOG_INFO,   fmt, ##__VA_ARGS__)
#define SIGMA_KERN_DEBUG(fmt,  ...) sigma_printk(SIGMA_LOG_DEBUG,  fmt, ##__VA_ARGS__)

void SovereignDmesg_Init(void);

#endif /* SOVEREIGN_DMESG_H */

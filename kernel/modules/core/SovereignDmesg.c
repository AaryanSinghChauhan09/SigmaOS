/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN DMESG — KERNEL RING BUFFER IMPLEMENTATION (v1.0)
 * =========================================================================
 */

#include "../../../include/sigma_kernel.h"
#include "../../../include/SovereignDmesg.h"

/* Global kernel ring buffer */
SigmaDmesgCtx_t g_sigma_dmesg;

/* -------------------------------------------------------------------------
 * sigma_dmesg_init
 * ---------------------------------------------------------------------- */
void sigma_dmesg_init(SigmaDmesgCtx_t *ctx) {
    sigma_memset(ctx, 0, sizeof(*ctx));
    ctx->min_level = SIGMA_LOG_DEBUG;   /* Show everything by default */
}

/* -------------------------------------------------------------------------
 * Level prefix strings (mirrors Linux [  0.000000] style)
 * ---------------------------------------------------------------------- */
static const char *level_prefix(SigmaLogLevel_t lvl) {
    switch (lvl) {
        case SIGMA_LOG_EMERG:  return "EMERG  ";
        case SIGMA_LOG_ALERT:  return "ALERT  ";
        case SIGMA_LOG_CRIT:   return "CRIT   ";
        case SIGMA_LOG_ERR:    return "ERR    ";
        case SIGMA_LOG_WARN:   return "WARN   ";
        case SIGMA_LOG_NOTICE: return "NOTICE ";
        case SIGMA_LOG_INFO:   return "INFO   ";
        case SIGMA_LOG_DEBUG:  return "DEBUG  ";
        default:               return "?????  ";
    }
}

/* -------------------------------------------------------------------------
 * sigma_printk — core kernel logger (mirrors Linux printk)
 * ---------------------------------------------------------------------- */
void sigma_printk(SigmaLogLevel_t level, const char *fmt, ...) {
    SigmaDmesgCtx_t *ctx = &g_sigma_dmesg;
    if (level > ctx->min_level) return;     /* Filtered out */

    SigmaDmesgMsg_t *msg;

    if (ctx->count < SIGMA_DMESG_MAX_MSGS) {
        msg = &ctx->msgs[ctx->tail];
        ctx->tail  = (ctx->tail + 1) % SIGMA_DMESG_MAX_MSGS;
        ctx->count++;
    } else {
        /* Ring is full — overwrite oldest (head advances) */
        msg = &ctx->msgs[ctx->head];
        ctx->head  = (ctx->head + 1) % SIGMA_DMESG_MAX_MSGS;
        ctx->tail  = ctx->head;            /* tail follows head in full ring */
    }

    msg->level        = level;
    msg->timestamp_us = 0;   /* Real: read HPET / TSC; simulated = 0 */

    /* Format the message */
    sigma_va_list ap;
    sigma_va_start(ap, fmt);
    sigma_snprintf(msg->text, SIGMA_DMESG_MSG_MAX, fmt, ap);
    sigma_va_end(ap);

    /* Also echo to serial / VGA console immediately */
    sigma_printf("[%s] %s", level_prefix(level), msg->text);
}

/* -------------------------------------------------------------------------
 * sigma_dmesg_dump — dmesg (print all buffered messages)
 * ---------------------------------------------------------------------- */
void sigma_dmesg_dump(const SigmaDmesgCtx_t *ctx) {
    sigma_printf("Σ [DMESG]: Kernel ring buffer (%u messages):\n", ctx->count);
    sigma_u32 idx = ctx->head;
    for (sigma_u32 i = 0; i < ctx->count; i++) {
        const SigmaDmesgMsg_t *m = &ctx->msgs[idx];
        sigma_printf("  [%07llu.%06llu] [%s] %s",
                     (unsigned long long)(m->timestamp_us / 1000000ULL),
                     (unsigned long long)(m->timestamp_us % 1000000ULL),
                     level_prefix(m->level),
                     m->text);
        idx = (idx + 1) % SIGMA_DMESG_MAX_MSGS;
    }
}

/* -------------------------------------------------------------------------
 * sigma_dmesg_clear — dmesg -c
 * ---------------------------------------------------------------------- */
void sigma_dmesg_clear(SigmaDmesgCtx_t *ctx) {
    sigma_memset(ctx->msgs, 0, sizeof(ctx->msgs));
    ctx->head  = 0;
    ctx->tail  = 0;
    ctx->count = 0;
    sigma_printf("Σ [DMESG]: Ring buffer cleared.\n");
}

/* -------------------------------------------------------------------------
 * sigma_dmesg_set_level — dmesg -n <level>
 * ---------------------------------------------------------------------- */
void sigma_dmesg_set_level(SigmaDmesgCtx_t *ctx, SigmaLogLevel_t lvl) {
    ctx->min_level = lvl;
    sigma_printf("Σ [DMESG]: Console log level set to %s.\n",
                 level_prefix(lvl));
}

/* -------------------------------------------------------------------------
 * SovereignDmesg_Init — Seed ring buffer with boot messages
 * ---------------------------------------------------------------------- */
void SovereignDmesg_Init(void) {
    sigma_dmesg_init(&g_sigma_dmesg);
    sigma_printf("Σ [DMESG]: Kernel ring buffer online.\n");

    sigma_printk(SIGMA_LOG_INFO,   "SigmaOS " "v1.0 kernel booting...\n");
    sigma_printk(SIGMA_LOG_INFO,   "CPU: x86_64, 4 cores, APIC online.\n");
    sigma_printk(SIGMA_LOG_INFO,   "Memory: 8192 MB physical RAM detected.\n");
    sigma_printk(SIGMA_LOG_INFO,   "VFS: ext4 root mounted on /dev/nvme0n1p1.\n");
    sigma_printk(SIGMA_LOG_INFO,   "NET: sigma-netd driver loaded.\n");
    sigma_printk(SIGMA_LOG_NOTICE, "INIT: Transitioning to default runlevel.\n");
    sigma_printk(SIGMA_LOG_WARN,   "ACPI: Battery not found — assuming AC power.\n");

    sigma_dmesg_dump(&g_sigma_dmesg);
    sigma_printf("Σ [DMESG]: Ring buffer sovereignty achieved.\n");
}

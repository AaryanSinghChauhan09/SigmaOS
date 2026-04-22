/*
 * core/logging.c — Lightweight C logging (no libc fprintf dependency in kernel mode)
 * In hosted mode: writes to stderr via write() syscall.
 * In bare-metal mode: writes to VGA ring buffer.
 */

#include <stdint.h>
#include <stddef.h>
#include <stdarg.h>

/* ── Log levels ──────────────────────────────────────────────────────────────── */
typedef enum { LOG_DEBUG=0, LOG_INFO, LOG_WARN, LOG_ERROR, LOG_FATAL } LogLevel;
static const char* LEVEL_PREFIX[] = { "DBG", "INF", "WRN", "ERR", "!!!" };

/* ── Tiny itoa (no sprintf dependency) ──────────────────────────────────────── */
static void _u32_to_str(uint32_t n, char *buf, int *pos) {
    if (n == 0) { buf[(*pos)++] = '0'; return; }
    char tmp[10]; int t = 0;
    while (n) { tmp[t++] = '0' + (n % 10); n /= 10; }
    while (t--) buf[(*pos)++] = tmp[t+1];
}

/* ── Output sink (swap for VGA/ring-buffer in bare-metal) ──────────────────── */
static void _sink_write(const char *buf, size_t len) {
#ifdef SIGMA_HOSTED_BUILD
    /* POSIX write(2) — no buffered stdio dependency */
    extern long write(int, const void*, long);
    write(2, buf, (long)len);
#else
    /* Bare-metal: write to serial port 0x3F8 */
    for (size_t i = 0; i < len; i++) {
        while (!(*((volatile uint8_t*)0x3F8 + 5) & 0x20)); /* wait for TX ready */
        *((volatile uint8_t*)0x3F8) = (uint8_t)buf[i];
    }
#endif
}

/* ── Main log function ──────────────────────────────────────────────────────── */
void sigma_log_c(LogLevel level, const char *module, const char *msg) {
    char buf[256];
    int pos = 0;

    /* [LVL][MODULE] msg\n */
    buf[pos++] = '[';
    const char *pfx = LEVEL_PREFIX[level < 5 ? level : 4];
    buf[pos++] = pfx[0]; buf[pos++] = pfx[1]; buf[pos++] = pfx[2];
    buf[pos++] = ']'; buf[pos++] = '[';
    for (int i = 0; module[i] && pos < 240; i++) buf[pos++] = module[i];
    buf[pos++] = ']'; buf[pos++] = ' ';
    for (int i = 0; msg[i] && pos < 253; i++) buf[pos++] = msg[i];
    buf[pos++] = '\n';

    _sink_write(buf, (size_t)pos);
}

/* ── Convenience macros ─────────────────────────────────────────────────────── */
#define LOG_INFO(mod, msg)  sigma_log_c(LOG_INFO,  mod, msg)
#define LOG_WARN(mod, msg)  sigma_log_c(LOG_WARN,  mod, msg)
#define LOG_ERROR(mod, msg) sigma_log_c(LOG_ERROR, mod, msg)
#define LOG_FATAL(mod, msg) sigma_log_c(LOG_FATAL, mod, msg)

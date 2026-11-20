/* SigmaOS ELF32 user program: printf — basic format output.
 * Usage: exec PRINTF.ELF <fmt> [args...]
 * Supports %s %d %i %u %x %c %% — SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

static int atoi_(const char* s) {
    int neg = 0, v = 0;
    if (*s == '-') { neg = 1; s++; }
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return neg ? -v : v;
}
static void putx(unsigned int v) {
    char b[9]; int i = 8; b[i] = 0;
    if (!v) { uputs("0"); return; }
    while (v) { int d = v & 15; b[--i] = d < 10 ? '0' + d : 'a' + d - 10; v >>= 4; }
    uputs(b + i);
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* fmt = (argc > 1) ? (const char*)sp[2] : 0;
    if (!fmt) { uputs("usage: printf <fmt> [args]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    u32 ai = 3;
    for (const char* p = fmt; *p; p++) {
        if (*p != '%') { sc3(SYS_WRITE_LEN, (u32)p, 1, 0); continue; }
        p++;
        if (!*p) break;
        if (*p == '%') { sc3(SYS_WRITE_LEN, (u32)"%", 1, 0); continue; }
        if (ai >= argc) break;
        const char* arg = (const char*)sp[ai++];
        switch (*p) {
            case 's': uputs(arg); break;
            case 'd': case 'i': uputn((u32)atoi_(arg)); break;
            case 'u': uputn((u32)atoi_(arg)); break;
            case 'x': putx((unsigned int)atoi_(arg)); break;
            case 'c': sc3(SYS_WRITE_LEN, (u32)arg, 1, 0); break;
            default: sc3(SYS_WRITE_LEN, (u32)p - 1, 2, 0); break;
        }
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

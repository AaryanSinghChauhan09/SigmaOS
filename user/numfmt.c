/* SigmaOS ELF32 user program: numfmt — human-readable numbers (u32 only).
 * Usage: exec NUMFMT.ELF <number> [K|M|G]
 * Prints e.g. "1.5M"; raw print when < 1024 or no suffix. */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void outn(u32 v) { uputn(v); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: numfmt <number> [K|M|G]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    u32 v = 0;
    for (const char* p = (const char*)sp[2]; *p >= '0' && *p <= '9'; p++) {
        u32 nv = v * 10 + (*p - '0');
        if (nv < v) { v = 0xFFFFFFFFu; break; }  /* saturate */
        v = nv;
    }
    int mul = 1;
    if (argc > 2) {
        const char* m = (const char*)sp[4];
        if (m[0] == 'K' || m[0] == 'k') mul = 1024;
        else if (m[0] == 'M' || m[0] == 'm') mul = 1024 * 1024;
        else if (m[0] == 'G' || m[0] == 'g') mul = 1024 * 1024 * 1024;
    }
    /* saturating multiply */
    u32 t = v;
    if (mul > 1) {
        if (v > 0xFFFFFFFFu / (u32)mul) t = 0xFFFFFFFFu;
        else t = v * (u32)mul;
    }
    const char* units[5] = {"", "K", "M", "G", "T"};
    int u = 0;
    while (u < 4 && t >= 1024u * (u32[]){1,1024,1048576,1073741824,0}[u+1] && u < 3) u++;
    /* simpler: divide down */
    u = 0;
    u32 d = 1;
    while (u < 4 && t / d >= 1024 && d <= 0x40000000u) { d *= 1024; u++; }
    if (argc <= 2 || u == 0) { outn(t); outs("\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    u32 ip = t / d;
    u32 fp = (t % d) * 10 / d;
    outn(ip);
    if (fp) { outs("."); outn(fp); }
    outs(units[u]); outs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

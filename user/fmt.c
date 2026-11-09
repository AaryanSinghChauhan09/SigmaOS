/* SigmaOS ELF32 user program: fmt — simple paragraph text wrapper.
 * Usage: exec FMT.ELF [width] <file>
 * Greedy word wrap at width (default 72). Long words split hard.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 ; EXIT=0 */
#include "lib.h"
static unsigned char buf[32768];
static void out(const char* s, u32 n) { sc3(SYS_WRITE_LEN, (u32)s, n, 0); }
static void outs(const char* s) { out(s, u32strlen(s)); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    u32 width = 72;
    const char* path;
    if (argc > 2) {
        /* first arg = width */
        width = 0;
        for (const char* p = (const char*)sp[2]; *p >= '0' && *p <= '9'; p++) width = width * 10 + *p - '0';
        path = (const char*)sp[4];
    } else {
        path = (argc > 1) ? (const char*)sp[2] : 0;
    }
    if (!path || !width) { uputs("usage: fmt [width] <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("fmt: cannot open "); uputs(path); uputs("\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    int total = 0;
    for (;;) {
        int r = sc3(SYS_READ, fd, (u32)(buf + total), sizeof buf - total);
        if (r <= 0) break;
        total += r;
        if (total >= (int)sizeof buf || (u32)r < sizeof buf - (u32)(total - r)) break;
        if ((u32)r < sizeof buf) break;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    /* wrap: tokens separated by whitespace; newlines = hard break */
    u32 col = 0;
    u32 i = 0;
    while (i < (u32)total) {
        /* skip whitespace (spaces/tabs) */
        while (i < (u32)total && (buf[i] == ' ' || buf[i] == '\t')) i++;
        if (i >= (u32)total) break;
        if (buf[i] == '\n') { outs("\n"); col = 0; i++; continue; }
        /* find word end */
        u32 ws = i;
        while (i < (u32)total && buf[i] != ' ' && buf[i] != '\t' && buf[i] != '\n') i++;
        u32 wl = i - ws;
        if (col && col + 1 + wl > width) { outs("\n"); col = 0; }
        else if (col) { outs(" "); col++; }
        /* emit word (split if longer than width) */
        u32 k = 0;
        while (k < wl) {
            u32 chunk = wl - k;
            if (col + chunk > width) chunk = width - col;
            if (chunk == 0) { outs("\n"); col = 0; continue; }
            out((const char*)(buf + ws + k), chunk);
            col += chunk; k += chunk;
        }
    }
    if (col) outs("\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

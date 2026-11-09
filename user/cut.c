/* SigmaOS ELF32 user program: cut — print char range of each line.
 * Usage: exec CUT.ELF <file> <start> <end>   (1-based, inclusive)
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 1024
static char line[MAXL + 1];
static int ll;
static int cstart = 1, cend = 0;

static void flush_line(void) {
    if (ll > 0) {
        for (int i = cstart - 1; i < cend && i < ll; i++)
            if (i >= 0) sc3(SYS_WRITE_LEN, (u32)&line[i], 1, 0);
    }
    sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0);
    ll = 0;
}

static int atoi_(const char* s) {
    int v = 0;
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return v;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (argc > 2) cstart = atoi_((const char*)sp[3]);
    if (argc > 3) cend = atoi_((const char*)sp[4]);
    if (!path || cend <= 0) {
        uputs("usage: cut <file> <start> <end>\n");
        sc3(SYS_EXIT,0,0,0); __builtin_unreachable();
    }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("cut: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            if (buf[i] == '\n') flush_line();
            else if (ll < MAXL) line[ll++] = buf[i];
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (ll > 0) flush_line();
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

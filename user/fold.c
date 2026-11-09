/* SigmaOS ELF32 user program: fold — wrap lines at width.
 * Usage: exec FOLD.ELF <file> [width]   (default 80)
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 2048
static char line[MAXL];
static int width = 80;

static void emit(const char* s, int n) { sc3(SYS_WRITE_LEN, (u32)s, n, 0); }

static void wrap_line(void) {
    int n = 0; while (n < MAXL && line[n]) n++;
    int col = 0;
    for (int i = 0; i < n; i++) {
        if (col >= width) { emit("\n", 1); col = 0; }
        emit(&line[i], 1);
        col++;
    }
    emit("\n", 1);
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (argc > 2) {
        const char* a = (const char*)sp[3];
        int v = 0;
        while (*a >= '0' && *a <= '9') { v = v * 10 + (*a - '0'); a++; }
        if (v > 0) width = v;
    }
    if (!path) { uputs("usage: fold <file> [width]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("fold: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    int ll = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            if (buf[i] == '\n') {
                line[ll] = 0; wrap_line(); ll = 0;
            } else if (ll < MAXL - 1) line[ll++] = buf[i];
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (ll > 0) { line[ll] = 0; wrap_line(); }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

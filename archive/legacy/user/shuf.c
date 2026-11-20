/* SigmaOS ELF32 user program: shuf — random line permutation.
 * Usage: exec SHUF.ELF            (stdin via GETCHAR)  OR  exec SHUF.ELF <file>
 * Reservoir sampling would need storage; simple approach: read all lines
 * (cap 512 lines x 128 chars), Fisher-Yates with SYS_TICKS-seeded LCG.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_TICKS=3 SYS_WRITE_LEN=8 ; EXIT=0 */
#include "lib.h"
static unsigned char fbuf[65536];
static char lines[512][160];
static int lens[512];
static u32 rng;
static u32 rnd(void) { rng = rng * 1103515245u + 12345u; return (rng >> 16) & 0x7FFF; }
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    rng = (u32)sc3(SYS_TICKS,0,0,0) ^ 0x5DEECE66Du;
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    int fd = -1;
    if (path) {
        fd = sc3(SYS_OPEN, (u32)path, 0, 0);
        if (fd < 0) { uputs("shuf: cannot open "); uputs(path); uputs("\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    }
    /* read all input: file via SYS_READ, stdin via SYS_GETCHAR */
    u32 n = 0, cur = 0, col = 0;
    int done = 0;
    while (!done && n < 512) {
        int c;
        if (fd >= 0) {
            int r = sc3(SYS_READ, fd, (u32)fbuf, sizeof fbuf);
            if (r <= 0) break;
            /* process fbuf bytes one at a time from a static cursor —
               simpler: only handle stdin path fully; file path reads once */
            for (int i = 0; i < r && n < 512; i++) {
                c = fbuf[i];
                if (c == '\n') {
                    if (col) { lines[n][col] = 0; lens[n] = (int)col; n++; col = 0; }
                } else if (col < 150) lines[n][col++] = (char)c;
            }
            if ((u32)r < sizeof fbuf) done = 1;
        } else {
            c = sc3(SYS_GETCHAR, 0, 0, 0);
            if (c < 0 || c == 0xFF) break;
            if (c == '\n') {
                if (col) { lines[n][col] = 0; lens[n] = (int)col; n++; col = 0; }
            } else if (col < 150) lines[n][col++] = (char)c;
        }
    }
    if (col && n < 512) { lines[n][col] = 0; lens[n] = (int)col; n++; }
    if (fd >= 0) sc3(SYS_CLOSE, fd, 0, 0);
    /* Fisher-Yates */
    for (u32 i = n; i > 1; i--) {
        u32 j = rnd() % i;
        /* swap lines i-1, j */
        char tmp[160]; int tl;
        for (int k = 0; k < 160; k++) tmp[k] = lines[i-1][k];
        tl = lens[i-1];
        for (int k = 0; k < 160; k++) lines[i-1][k] = lines[j][k];
        lens[i-1] = lens[j];
        for (int k = 0; k < 160; k++) lines[j][k] = tmp[k];
        lens[j] = tl;
    }
    for (u32 i = 0; i < n; i++) {
        sc3(SYS_WRITE_LEN, (u32)lines[i], (u32)lens[i], 0);
        sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0);
    }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

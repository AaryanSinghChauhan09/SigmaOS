/* SigmaOS ELF32 user program: paste — merge files line-by-line with tabs.
 * Usage: exec PASTE.ELF <file> [file...]
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 ; EXIT=0
 * Reads each file fully into a static buffer (64KiB cap each, 4 files max). */
#include "lib.h"
#define MAXF 4
static unsigned char data[MAXF][65536];
static int fsz[MAXF];
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 2) { uputs("usage: paste <file> [file...]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int nf = (int)argc - 1;
    if (nf > MAXF) nf = MAXF;
    int fds[MAXF];
    for (int i = 0; i < nf; i++) {
        fds[i] = sc3(SYS_OPEN, sp[2 + i], 0, 0);
        if (fds[i] < 0) { uputs("paste: cannot open "); uputs((const char*)sp[2 + i]); uputs("\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
        int total = 0;
        for (;;) {
            int r = sc3(SYS_READ, fds[i], (u32)(data[i] + total), 65536 - total);
            if (r <= 0) break;
            total += r;
            if (total >= 65536 || (u32)r < 65536 - (u32)(total - r)) break;
            if ((u32)r < 65536) break;
        }
        fsz[i] = total;
    }
    /* per-line merge: track position per file */
    int pos[MAXF] = {0};
    int active = 1;
    while (active) {
        active = 0;
        for (int i = 0; i < nf; i++) {
            if (i) sc3(SYS_WRITE_LEN, (u32)"\t", 1, 0);
            int start = pos[i];
            int end = start;
            while (end < fsz[i] && data[i][end] != '\n') end++;
            if (end > start) {
                sc3(SYS_WRITE_LEN, (u32)(data[i] + start), (u32)(end - start), 0);
                active = 1;
            }
            pos[i] = (end < fsz[i]) ? end + 1 : fsz[i];
        }
        if (active) sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0);
        /* if all pos reached end and nothing printed, stop */
        int all_done = 1;
        for (int i = 0; i < nf; i++) if (pos[i] < fsz[i]) all_done = 0;
        if (all_done) break;
    }
    for (int i = 0; i < nf; i++) sc3(SYS_CLOSE, fds[i], 0, 0);
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

/* SigmaOS ELF32 user program: tail — print last N lines of a file.
 * Usage: exec TAIL.ELF <file> [n]
 * SYS_OPEN=4 SYS_FSTAT=15 SYS_SEEK=7 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 256
static char lines[MAXL][257];
static int nlines;

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    int want = 10;
    if (argc > 2) {
        const char* a = (const char*)sp[3];
        int v = 0;
        while (*a >= '0' && *a <= '9') { v = v * 10 + (*a - '0'); a++; }
        if (v > 0) want = v;
    }
    if (!path) { uputs("usage: tail <file> [n]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("tail: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int size = sc3(SYS_FSTAT, fd, 0, 0);
    if (size < 0) size = 0;
    /* read whole file (cap 256KB), keep last MAXL lines */
    static char buf[65536];
    int got = 0;
    if (size > 0 && size <= (int)sizeof buf) {
        sc3(SYS_SEEK, fd, 0, 0);
        got = sc3(SYS_READ, fd, (u32)buf, size);
        if (got < 0) got = 0;
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    /* split into lines, keep last MAXL */
    static char cur[256];
    int cl = 0;
    for (int i = 0; i < got; i++) {
        if (buf[i] == '\n') {
            if (nlines == MAXL) {
                /* shift down */
                for (int r = 0; r < MAXL - 1; r++) {
                    int l = 0; while (lines[r+1][l]) { lines[r][l] = lines[r+1][l]; l++; }
                    lines[r][l] = 0;
                }
                nlines--;
            }
            if (nlines < MAXL) {
                cur[cl < 255 ? cl : 255] = 0;
                int l = 0; while (cur[l] && l < 256) { lines[nlines][l] = cur[l]; l++; }
                lines[nlines][l] = 0; nlines++;
            }
            cl = 0;
        } else if (cl < 255) cur[cl++] = buf[i];
    }
    if (cl > 0 && nlines < MAXL) {
        cur[cl] = 0;
        int l = 0; while (cur[l] && l < 256) { lines[nlines][l] = cur[l]; l++; }
        lines[nlines][l] = 0; nlines++;
    }
    int start = nlines - want; if (start < 0) start = 0;
    for (int i = start; i < nlines; i++) { uputs(lines[i]); uputs("\n"); }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

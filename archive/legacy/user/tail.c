/* SigmaOS ELF32 user program: tail — print last N lines/bytes of a file.
 * Usage: exec TAIL.ELF <file> [n] | tail -c <n> <file>
 * SYS_OPEN=4 SYS_FSTAT=15 SYS_SEEK=7 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 256
static char lines[MAXL][257];
static int nlines;

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path;
    int want = 10, bytes_mode = 0;
    if (argc > 1) {
        const char* a1 = (const char*)sp[2];
        if (a1[0] == '-' && a1[1] == 'c') {
            bytes_mode = 1;
            want = 0;
            const char* a = (argc > 2) ? (const char*)sp[3] : 0;
            while (a && *a >= '0' && *a <= '9') { want = want * 10 + (*a - '0'); a++; }
            if (want <= 0) want = 10;
            path = (argc > 3) ? (const char*)sp[4] : 0;
        } else {
            path = a1;
            if (argc > 2) {
                const char* a = (const char*)sp[3];
                int v = 0;
                while (*a >= '0' && *a <= '9') { v = v * 10 + (*a - '0'); a++; }
                if (v > 0) want = v;
            }
        }
    } else path = 0;
    if (!path) { uputs("usage: tail [-c n] <file> [n]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("tail: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int size = sc3(SYS_FSTAT, fd, 0, 0);
    if (size < 0) size = 0;
    if (bytes_mode) {
        /* seek to max(0, size - want), dump rest in chunks */
        int start = size - want;
        if (start < 0) start = 0;
        sc3(SYS_SEEK, fd, start, 0);
        static char cbuf[4096];
        int left = size - start;
        while (left > 0) {
            int w = left < (int)sizeof cbuf ? left : (int)sizeof cbuf;
            int n = sc3(SYS_READ, fd, (u32)cbuf, w);
            if (n <= 0) break;
            sc3(SYS_WRITE_LEN, (u32)cbuf, n, 0);
            left -= n;
        }
        sc3(SYS_CLOSE, fd, 0, 0);
        sc3(SYS_EXIT, 0, 0, 0);
        __builtin_unreachable();
    }
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

/* SigmaOS ELF32 user program: uniq — drop adjacent duplicate lines.
 * Usage: exec UNIQ.ELF <file>
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 512
static char lines[MAXL][257];
static int nlines;

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { uputs("usage: uniq <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("uniq: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    static char cur[256];
    int cl = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            if (buf[i] == '\n') {
                if (nlines < MAXL) {
                    cur[cl < 255 ? cl : 255] = 0;
                    int l = 0; while (cur[l] && l < 256) { lines[nlines][l] = cur[l]; l++; }
                    lines[nlines][l] = 0; nlines++;
                }
                cl = 0;
            } else if (cl < 255) cur[cl++] = buf[i];
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (cl > 0 && nlines < MAXL) { cur[cl] = 0; int l = 0; while (cur[l] && l < 256) { lines[nlines][l] = cur[l]; l++; } lines[nlines][l] = 0; nlines++; }
    for (int i = 0; i < nlines; i++) {
        if (i > 0) {
            int same = 1, k = 0;
            while (lines[i][k] && lines[i-1][k]) {
                if (ulower(lines[i][k]) != ulower(lines[i-1][k])) { same = 0; break; }
                k++;
            }
            if (lines[i][k] != lines[i-1][k]) same = 0;
            if (same) continue;
        }
        uputs(lines[i]); uputs("\n");
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

/* SigmaOS ELF32 user program: sort — sort lines of a file.
 * Usage: exec SORT.ELF <file>
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 512
#define MAXLL 256
static char lines[MAXL][MAXLL + 1];
static int nlines;

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : 0;
    if (!path) { uputs("usage: sort <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("sort: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    static char cur[MAXLL];
    int cl = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            char c = buf[i];
            if (c == '\n') {
                if (nlines < MAXL && cl < MAXLL) {
                    cur[cl] = 0;
                    int l = 0; while (cur[l] && l < MAXLL) { lines[nlines][l] = cur[l]; l++; }
                    lines[nlines][l] = 0;
                    nlines++;
                }
                cl = 0;
            } else if (cl < MAXLL - 1) cur[cl++] = c;
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (cl > 0 && nlines < MAXL) { cur[cl] = 0; int l = 0; while (cur[l] && l < MAXLL) { lines[nlines][l] = cur[l]; l++; } lines[nlines][l] = 0; nlines++; }
    /* insertion sort */
    for (int i = 1; i < nlines; i++) {
        char tmp[MAXLL + 1];
        int l = 0; while (lines[i][l] && l <= MAXLL) { tmp[l] = lines[i][l]; l++; } tmp[l] = 0;
        int j = i - 1;
        while (j >= 0) {
            int k = 0; int cmp = 0;
            while (lines[j][k] && tmp[k]) { if (ulower(lines[j][k]) != ulower(tmp[k])) { cmp = ulower(lines[j][k]) - ulower(tmp[k]); break; } k++; }
            if (!cmp) cmp = (lines[j][k] ? 1 : (tmp[k] ? -1 : 0));
            if (cmp <= 0) break;
            int m = 0; while (lines[j][m] && m <= MAXLL) { lines[j+1][m] = lines[j][m]; m++; } lines[j+1][m] = 0;
            j--;
        }
        int m = 0; while (tmp[m] && m <= MAXLL) { lines[j+1][m] = tmp[m]; m++; } lines[j+1][m] = 0;
    }
    for (int i = 0; i < nlines; i++) { uputs(lines[i]); uputs("\n"); }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

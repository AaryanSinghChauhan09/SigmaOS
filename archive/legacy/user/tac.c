/* SigmaOS ELF32 user program: tac — print lines in reverse order.
 * Usage: exec TAC.ELF <file>
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
    if (!path) { uputs("usage: tac <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("tac: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    static char cur[MAXLL];
    int cl = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            if (buf[i] == '\n') {
                if (nlines < MAXL) {
                    cur[cl < MAXLL ? cl : MAXLL] = 0;
                    int l = 0; while (cur[l] && l < MAXLL) { lines[nlines][l] = cur[l]; l++; }
                    lines[nlines][l] = 0; nlines++;
                }
                cl = 0;
            } else if (cl < MAXLL - 1) cur[cl++] = buf[i];
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (cl > 0 && nlines < MAXL) {
        cur[cl] = 0;
        int l = 0; while (cur[l] && l < MAXLL) { lines[nlines][l] = cur[l]; l++; }
        lines[nlines][l] = 0; nlines++;
    }
    for (int i = nlines - 1; i >= 0; i--) { uputs(lines[i]); uputs("\n"); }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

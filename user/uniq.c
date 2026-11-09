/* SigmaOS ELF32 user program: uniq — drop adjacent duplicate lines.
 * Usage: exec UNIQ.ELF [-c] [-d] <file>
 * -c prefix counts, -d print only duplicated lines.
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 512
static char lines[MAXL][257];
static int nlines;

static void uitoa(unsigned int v, char* o) {
    char t[12]; int i = 0;
    do { t[i++] = (char)('0' + v % 10); v /= 10; } while (v);
    while (i) *o++ = t[--i];
    *o = 0;
}
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    int want_c = 0, want_d = 0;
    u32 ai = 2;
    while (ai <= argc) {
        const char* a = (const char*)sp[ai];
        if (!a || a[0] != '-') break;
        if (a[1] == 'c' && !a[2]) want_c = 1;
        else if (a[1] == 'd' && !a[2]) want_d = 1;
        else break;
        ai++;
    }
    const char* path = (ai <= argc) ? (const char*)sp[ai] : 0;
    if (!path) { uputs("usage: uniq [-cd] <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
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
    /* group runs of equal lines, then emit per flags */
    for (int i = 0; i < nlines;) {
        int j = i + 1;
        while (j < nlines) {
            int same = 1, k = 0;
            while (lines[j][k] && lines[i][k]) {
                if (ulower(lines[j][k]) != ulower(lines[i][k])) { same = 0; break; }
                k++;
            }
            if (lines[j][k] != lines[i][k]) same = 0;
            if (!same) break;
            j++;
        }
        int cnt = j - i;
        if (!(want_d && cnt < 2)) {
            if (want_c) { char nb[12]; uitoa(cnt, nb); uputs(nb); uputs(" "); }
            uputs(lines[i]); uputs("\n");
        }
        i = j;
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

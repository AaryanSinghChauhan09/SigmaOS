/* SigmaOS ELF32 user program: sort — line sort with -n/-r/-u.
 * Usage: exec SORT.ELF [-n] [-r] [-u] <file>
 * -n numeric (leading integer), -r reverse, -u unique. */
#include "lib.h"
#define MAXL 512
#define MAXLL 200
static char lines[MAXL][MAXLL + 1];
static int nlines;
static int mode_n, mode_r, mode_u;

static int cmp_lines(const char* a, const char* b) {
    int c;
    if (mode_n) {
        /* parse leading integers (optional sign) */
        const char *pa = a, *pb = b;
        while (*pa == ' ' || *pa == '\t') pa++;
        while (*pb == ' ' || *pb == '\t') pb++;
        int sa = 1, sb = 1;
        if (*pa == '-') { sa = -1; pa++; } else if (*pa == '+') pa++;
        if (*pb == '-') { sb = -1; pb++; } else if (*pb == '+') pb++;
        long va = 0, vb = 0;
        while (*pa >= '0' && *pa <= '9') { va = va * 10 + (*pa - '0'); pa++; }
        while (*pb >= '0' && *pb <= '9') { vb = vb * 10 + (*pb - '0'); pb++; }
        va *= sa; vb *= sb;
        c = (va < vb) ? -1 : (va > vb);
        if (c) return mode_r ? -c : c;
        /* tie: compare rest as strings */
    }
    const char *x = a, *y = b;
    while (*x && ulower(*x) == ulower(*y)) { x++; y++; }
    c = ulower(*x) - ulower(*y);
    if (!c) c = (int)(unsigned char)*x - (int)(unsigned char)*y;
    return mode_r ? -c : c;
}
static int eq_lines(const char* a, const char* b) {
    while (*a && ulower(*a) == ulower(*b)) { a++; b++; }
    return !*a && !*b;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    u32 ai = 2; /* argv[1] */
    while (ai <= argc) {
        const char* a = (const char*)sp[ai];
        if (!a || a[0] != '-') break;
        if (a[1] == 'n' && !a[2]) mode_n = 1;
        else if (a[1] == 'r' && !a[2]) mode_r = 1;
        else if (a[1] == 'u' && !a[2]) mode_u = 1;
        else break;
        ai++;
    }
    if (ai > argc) { uputs("usage: sort [-nru] <file>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    const char* path = (const char*)sp[ai];
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("sort: open failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
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
        while (j >= 0 && cmp_lines(lines[j], tmp) > 0) {
            int m = 0; while (lines[j][m] && m <= MAXLL) { lines[j+1][m] = lines[j][m]; m++; } lines[j+1][m] = 0;
            j--;
        }
        int m = 0; while (tmp[m] && m <= MAXLL) { lines[j+1][m] = tmp[m]; m++; } lines[j+1][m] = 0;
    }
    /* output with optional dedup */
    int prev_valid = 0;
    char prev[MAXLL + 1]; prev[0] = 0;
    for (int i = 0; i < nlines; i++) {
        if (mode_u && prev_valid && eq_lines(prev, lines[i])) continue;
        uputs(lines[i]); uputs("\n");
        int l = 0; while (lines[i][l] && l <= MAXLL) { prev[l] = lines[i][l]; l++; } prev[l] = 0;
        prev_valid = 1;
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

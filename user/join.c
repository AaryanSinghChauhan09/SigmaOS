/* SigmaOS ELF32 user program: join — equijoin two sorted files on field 1.
 * Usage: exec JOIN.ELF <file1> <file2>
 * Lines "key rest..."; matching keys emit "key f1rest f2rest".
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 ; EXIT=0 */
#include "lib.h"
static char la[1024][160], lb[1024][160];
static int na = 0, nb = 0;

static int load(const char* path, char lines[][160], int* cnt) {
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) return -1;
    static unsigned char big[65536];
    int total = 0;
    for (;;) {
        int r = sc3(SYS_READ, fd, (u32)(big + total), 65536 - total);
        if (r <= 0) break;
        total += r;
        if (total >= 65536 || (u32)r < 65536) { if ((u32)r < 65536) break; }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    int n = 0, col = 0;
    for (int i = 0; i < total && n < 1024; i++) {
        if (big[i] == '\n') { if (col) { lines[n][col] = 0; n++; col = 0; } }
        else if (col < 150) lines[n][col++] = (char)big[i];
    }
    if (col && n < 1024) { lines[n][col] = 0; n++; }
    *cnt = n;
    return 0;
}

/* compare only up to first space/tab (the key) */
static int keycmp(const char* a, const char* b) {
    while (*a && *a != ' ' && *a != '\t' && *b && *b != ' ' && *b != '\t') {
        if (*a != *b) return *a - *b;
        a++; b++;
    }
    int ea = (*a == ' ' || *a == '\t' || *a == 0);
    int eb = (*b == ' ' || *b == '\t' || *b == 0);
    if (ea && eb) return 0;
    if (ea) return -1;
    return 1;
}
static const char* rest(const char* s) {
    while (*s && *s != ' ' && *s != '\t') s++;
    while (*s == ' ' || *s == '\t') s++;
    return s;
}
static void out(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    if (argc < 3) { uputs("usage: join <file1> <file2>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    if (load((const char*)sp[2], la, &na) != 0 || load((const char*)sp[3], lb, &nb) != 0) {
        uputs("join: cannot open input\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable();
    }
    int i = 0, j = 0;
    while (i < na && j < nb) {
        int c = keycmp(la[i], lb[j]);
        if (c < 0) i++;
        else if (c > 0) j++;
        else {
            /* emit: key + rest1 + rest2 */
            const char* ka = la[i]; const char* kb = lb[j];
            /* print key (up to first space) */
            const char* p = ka;
            while (*p && *p != ' ' && *p != '\t') { char ch[2] = {*p, 0}; out(ch); p++; }
            const char* r1 = rest(ka); const char* r2 = rest(kb);
            if (*r1) { out(" "); out(r1); }
            if (*r2) { out(" "); out(r2); }
            out("\n");
            i++; j++;
        }
    }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

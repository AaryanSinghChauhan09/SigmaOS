/* SigmaOS ELF32 user program: ll — long-format cwd listing.
 * Usage: exec LL.ELF
 * SYS_READDIRX=20 records "d|f name size\n" ; SYS_STAT=16 per-name size;
 * SYS_GETCWD=17 ; SYS_WRITE_LEN=8 ; SYS_EXIT=0 */
#include "lib.h"

static char buf[8192];
static char cwd[256];
static char path[384];

static void putsize(u32 v) {
    char nb[12]; int ni = 11; nb[ni] = 0;
    if (!v) nb[--ni] = '0';
    while (v) { nb[--ni] = '0' + v % 10; v /= 10; }
    for (int i = ni; i < 11; i++) sc3(SYS_WRITE_LEN, (u32)&nb[i], 1, 0);
}

void _start(void) {
    int cl = sc3(SYS_GETCWD, (u32)cwd, sizeof cwd - 1, 0);
    if (cl > 0) cwd[cl < 250 ? cl : 250] = 0;
    int total = sc3(SYS_READDIRX, (u32)buf, sizeof buf, 0);
    if (total <= 0) { uputs("(empty)\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    uputs("type  size        name\n");
    uputs("----  ----------- ----\n");
    int pos = 0;
    while (pos < total) {
        int ls = pos;
        while (pos < total && buf[pos] != '\n') pos++;
        int le = pos; if (pos < total) pos++;
        if (le - ls < 4) continue;
        char type = buf[ls];
        int ns = ls + 2, ne = ns;
        while (ne < le && buf[ne] != ' ') ne++;
        int ss = ne + 1, se = le;
        char nm[128];
        int nl = ne - ns; if (nl > 120) nl = 120;
        for (int i = 0; i < nl; i++) nm[i] = buf[ns + i];
        nm[nl] = 0;
        /* stat for real size */
        int pl = 0;
        for (const char* q = cwd; *q && pl < 370; q++) path[pl++] = *q;
        if (pl && path[pl-1] != '/') path[pl++] = '/';
        for (int i = 0; i < nl && pl < 378; i++) path[pl++] = nm[i];
        path[pl] = 0;
        int sz = sc3(SYS_STAT, (u32)path, 0, 0);
        if (sz < 0) {
            sz = 0;
            for (int i = ss; i < se; i++) if (buf[i] >= '0' && buf[i] <= '9') sz = sz * 10 + buf[i] - '0';
        }
        sc3(SYS_WRITE_LEN, (u32)&type, 1, 0);
        uputs("  ");
        putsize((u32)sz);
        uputs("  ");
        if (type == 'd') uputs("<dir> ");
        uputs(nm);
        uputs("\n");
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

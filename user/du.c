/* SigmaOS ELF32 user program: du — recursive disk usage.
 * Usage: exec DU.ELF [path] [depth]
 * Walks via SYS_CHDIR (kernel-global cwd) + SYS_READDIRX records +
 * SYS_STATX packed type|size. Depth-bounded (default 6).
 * SYS_GETCWD=17 CHDIR=18 READDIRX=20 STATX=23 WRITE_LEN=8 EXIT=0 */
#include "lib.h"

static char buf[8192];
static char cwd[256];
static char sub[384];

static u32 du_rec(int depth) {
    u32 total = 0;
    int n = sc3(SYS_READDIRX, (u32)buf, sizeof buf, 0);
    if (n <= 0) return 0;
    int pos = 0;
    while (pos < n) {
        int ls = pos;
        while (pos < n && buf[pos] != '\n') pos++;
        int le = pos; if (pos < n) pos++;
        if (le - ls < 4) continue;
        char type = buf[ls];
        int ns = ls + 2, ne = ns;
        while (ne < le && buf[ne] != ' ') ne++;
        int nl = ne - ns; if (nl > 120) nl = 120;
        for (int i = 0; i < nl; i++) sub[i] = buf[ns + i];
        sub[nl] = 0;
        if (sub[0] == '.') continue; /* skip . and .. */
        /* STATX resolves the bare name against kernel cwd */
        int sx = sc3(SYS_STATX, (u32)sub, 0, 0);
        if (sx < 0) continue;
        u32 sz = sx & 0x7FFFFFFF;
        total += sz;
        if ((sx & 0x80000000) && depth > 0) {
            /* chdir into subdir, recurse, restore */
            static char save[256];
            sc3(SYS_GETCWD, (u32)save, sizeof save - 1, 0);
            if (sc3(SYS_CHDIR, (u32)sub, 0, 0) == 0) {
                sc3(SYS_GETCWD, (u32)cwd, sizeof cwd - 1, 0);
                if (cwd[0]) cwd[(sizeof cwd) - 2] = 0;
                total += du_rec(depth - 1);
                sc3(SYS_CHDIR, (u32)save, 0, 0);
                sc3(SYS_GETCWD, (u32)cwd, sizeof cwd - 1, 0);
            }
        }
        /* restore cwd buffer anyway (we may not have chdir'd) */
        sc3(SYS_GETCWD, (u32)cwd, sizeof cwd - 1, 0);
    }
    return total;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* path = (argc > 1) ? (const char*)sp[2] : ".";
    int depth = (argc > 2) ? 6 : 6;
    if (argc > 2) { /* crude atoi */
        depth = 0;
        for (const char* p = (const char*)sp[4]; *p >= '0' && *p <= '9'; p++) depth = depth * 10 + *p - '0';
        if (depth > 6) depth = 6;
    }
    sc3(SYS_GETCWD, (u32)cwd, sizeof cwd - 1, 0);
    /* if path != ".", chdir into it first */
    if (!(path[0] == '.' && path[1] == 0)) {
        static char save[256];
        sc3(SYS_GETCWD, (u32)save, sizeof save - 1, 0);
        if (sc3(SYS_CHDIR, (u32)path, 0, 0) != 0) {
            uputs("du: cannot access "); uputs(path); uputs("\n");
            sc3(SYS_EXIT, 1, 0, 0); __builtin_unreachable();
        }
        sc3(SYS_GETCWD, (u32)cwd, sizeof cwd - 1, 0);
        u32 t = du_rec(depth);
        uputs(path); uputs("\t"); uputn(t); uputs(" bytes\n");
        sc3(SYS_CHDIR, (u32)save, 0, 0);
    } else {
        u32 t = du_rec(depth);
        uputs(".\t"); uputn(t); uputs(" bytes\n");
    }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

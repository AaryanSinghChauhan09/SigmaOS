/* SigmaOS ELF32 user program: cut — extract chars or delimited fields.
 * Usage:
 *   exec CUT.ELF <file> <start> <end>        char range (1-based incl)
 *   exec CUT.ELF -d <delim> -f <list> <file> fields; list = N or N-M,
 *                                            comma-separated, max 8
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

#define MAXL 1024
static char line[MAXL + 1];
static int ll;

static int atoi_(const char* s) {
    int v = 0;
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return v;
}
static void outch(char c) { sc3(SYS_WRITE_LEN, (u32)&c, 1, 0); }

/* field mode state */
static char delim = 0;
static int fr_lo[8], fr_hi[8], nfr;

static void parse_list(const char* s) {
    nfr = 0;
    while (*s && nfr < 8) {
        int a = atoi_(s);
        int b = a;
        while (*s >= '0' && *s <= '9') s++;
        if (*s == '-') { s++; b = atoi_(s); while (*s >= '0' && *s <= '9') s++; }
        if (a < 1) a = 1;
        if (b < a) b = a;
        fr_lo[nfr] = a; fr_hi[nfr] = b; nfr++;
        if (*s == ',') s++; else break;
    }
}
static int want_field(int idx) {
    for (int i = 0; i < nfr; i++)
        if (idx >= fr_lo[i] && idx <= fr_hi[i]) return 1;
    return 0;
}

static void flush_fields(void) {
    if (ll <= 0) { sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0); ll = 0; return; }
    int fld = 1, emitted = 0;
    int start = 0;
    for (int i = 0; i <= ll; i++) {
        if (i == ll || line[i] == delim) {
            if (want_field(fld)) {
                if (emitted) outch(delim);
                for (int k = start; k < i; k++) outch(line[k]);
                emitted = 1;
            }
            fld++; start = i + 1;
        }
    }
    sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0);
    ll = 0;
}

static int cstart = 1, cend = 0;
static void flush_chars(void) {
    if (ll > 0)
        for (int i = cstart - 1; i < cend && i < ll; i++)
            if (i >= 0) outch(line[i]);
    sc3(SYS_WRITE_LEN, (u32)"\n", 1, 0);
    ll = 0;
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    u32 ai = 2;
    const char* path = 0;
    int field_mode = 0;
    while (ai <= argc) {
        const char* a = (const char*)sp[ai];
        if (!a || a[0] != '-') break;
        if (a[1] == 'd' && a[2] == 0 && ai + 1 <= argc) { delim = *(const char*)sp[ai + 1]; ai += 2; }
        else if (a[1] == 'f' && a[2] == 0 && ai + 1 <= argc) { parse_list((const char*)sp[ai + 1]); ai += 2; }
        else break;
    }
    if (nfr > 0 && delim) field_mode = 1;
    if (ai <= argc) path = (const char*)sp[ai];
    if (field_mode) {
        /* path is arg after -f list */
    } else {
        path = (argc > 1) ? (const char*)sp[2] : 0;
        if (argc > 2) cstart = atoi_((const char*)sp[3]);
        if (argc > 3) cend = atoi_((const char*)sp[4]);
    }
    if (!path || (!field_mode && cend <= 0)) {
        uputs("usage: cut [-d x -f list] <file> | cut <file> <start> <end>\n");
        sc3(SYS_EXIT,0,0,0); __builtin_unreachable();
    }
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("cut: open failed\n"); sc3(SYS_EXIT,1,0,0); __builtin_unreachable(); }
    static char buf[4096];
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            if (buf[i] == '\n') { if (field_mode) flush_fields(); else flush_chars(); }
            else if (ll < MAXL) line[ll++] = buf[i];
        }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (ll > 0) { if (field_mode) flush_fields(); else flush_chars(); }
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

/* SigmaOS ELF32 user program: grep — print lines containing substring.
 * Usage: exec GREP.ELF <pattern> <file>
 * SYS_OPEN=4 SYS_READ=5 SYS_CLOSE=6 SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

static int lower(int c) { return ulower(c); }
static int contains(const char* h, int hlen, const char* n) {
    if (!*n) return 1;
    for (int i = 0; i <= hlen; i++) {
        int j = 0;
        while (n[j] && i + j < hlen && lower(h[i+j]) == lower(n[j])) j++;
        if (!n[j]) return 1;
    }
    return 0;
}

static void uitoa(unsigned int v, char* o) {
    char t[12]; int i = 0;
    do { t[i++] = (char)('0' + v % 10); v /= 10; } while (v);
    while (i) *o++ = t[--i];
    *o = 0;
}
static int g_count_only;
static void emit(const char* l, int ll, int num, int want_n) {
    if (g_count_only) return;
    if (want_n) { char n[12]; uitoa(num, n); uputs(n); uputs(":"); }
    uputs(l); uputs("\n");
}
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    int want_n = 0, invert = 0, want_c = 0;
    /* argv[k] == sp[k+1]; flags start at argv[1] */
    u32 fi = 1;
    while (fi + 1 < argc) {
        const char* a = (const char*)sp[fi + 1];
        if (!a) break;
        if (a[0] == '-' && a[1] == 'n' && !a[2]) { want_n = 1; fi++; }
        else if (a[0] == '-' && a[1] == 'v' && !a[2]) { invert = 1; fi++; }
        else if (a[0] == '-' && a[1] == 'c' && !a[2]) { want_c = 1; fi++; }
        else break;
    }
    const char* pat = (fi + 1 < argc) ? (const char*)sp[fi + 1] : 0;
    const char* path = (fi + 2 < argc) ? (const char*)sp[fi + 2] : 0;
    if (!pat || !path) {
        uputs("usage: grep [-nvc] <pattern> <file>\n");
        sc3(SYS_EXIT, 0, 0, 0);
        __builtin_unreachable();
    }
    g_count_only = want_c;
    int fd = sc3(SYS_OPEN, (u32)path, 0, 0);
    if (fd < 0) { uputs("grep: open failed\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    static char buf[4096];
    static char line[1024];
    int ll = 0, matched = 0, lineno = 0;
    for (;;) {
        int n = sc3(SYS_READ, fd, (u32)buf, sizeof buf);
        if (n <= 0) break;
        for (int i = 0; i < n; i++) {
            char c = buf[i];
            if (c == '\n' || ll >= 1000) {
                line[ll] = 0;
                lineno++;
                if (contains(line, ll, pat) != invert) {
                    matched++;
                    emit(line, ll, lineno, want_n);
                }
                ll = 0;
            } else line[ll++] = c;
        }
    }
    if (ll > 0) {
        line[ll] = 0;
        lineno++;
        if (contains(line, ll, pat) != invert) { matched++; emit(line, ll, lineno, want_n); }
    }
    sc3(SYS_CLOSE, fd, 0, 0);
    if (want_c) { char nb[12]; uitoa(matched, nb); uputs(nb); uputs("\n"); }
    sc3(SYS_EXIT, matched ? 0 : 1, 0, 0);
    __builtin_unreachable();
}

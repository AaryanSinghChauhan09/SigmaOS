/* SigmaOS ELF32 user program: seq — print number sequences.
 * Usage: exec SEQ.ELF [-s <sep>] [-w] <n> | <first> <last> [step]
 * Supports negative numbers, negative steps, custom separator, -w pad. */
#include "lib.h"

static int atoi_(const char* s) {
    int neg = 0, v = 0;
    if (*s == '-') { neg = 1; s++; } else if (*s == '+') s++;
    while (*s >= '0' && *s <= '9') { v = v * 10 + (*s - '0'); s++; }
    return neg ? -v : v;
}
static void uputs_(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
/* simpler: build string then emit */
static void emit_num(int v, int width, const char* sep) {
    char tmp[16]; int i = 0, neg = 0;
    unsigned int u;
    if (v < 0) { neg = 1; u = (unsigned int)(-(v + 1)) + 1; } else u = (unsigned int)v;
    do { tmp[i++] = (char)('0' + u % 10); u /= 10; } while (u);
    int digits = i + neg;
    int pad = width > digits ? width - digits : 0;
    for (int p = 0; p < pad; p++) uputs_("0");
    if (neg) uputs_("-");
    while (i) { char c[2] = { tmp[--i], 0 }; uputs_(c); }
    uputs_(sep);
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* sep = "\n";
    int eq_width = 0;
    u32 ai = 2;
    while (ai <= argc) {
        const char* a = (const char*)sp[ai];
        if (!a || a[0] != '-') break;
        if (a[1] == 's' && a[2] == 0 && ai + 1 <= argc) { sep = (const char*)sp[ai + 1]; ai += 2; }
        else if (a[1] == 'w' && !a[2]) { eq_width = 1; ai++; }
        else break;
    }
    if (ai > argc) { uputs("usage: seq [-s sep] [-w] <n> | <first> <last> [step]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    int first = 1, last, step = 1;
    int nargs = argc - ai + 1;
    if (nargs == 1) last = atoi_((const char*)sp[ai]);
    else {
        first = atoi_((const char*)sp[ai]);
        last = atoi_((const char*)sp[ai + 1]);
        if (nargs > 2) step = atoi_((const char*)sp[ai + 2]);
        if (step == 0) step = 1;
    }
    /* width = digits of largest-magnitude endpoint */
    int width = 1;
    if (eq_width) {
        int m = first < 0 ? -first : first;
        int l = last < 0 ? -last : last;
        if (l > m) m = l;
        width = 1; while (m >= 10) { m /= 10; width++; }
        if (first < 0 || last < 0) width++; /* sign */
    }
    long i = first;
    int guard = 0;
    while (step > 0 ? i <= last : i >= last) {
        emit_num((int)i, width, sep);
        i += step;
        if (++guard > 1000000) break;
    }
    /* trailing newline if sep wasn't newline */
    if (sep[0] && sep[1] == 0 && sep[0] != '\n') uputs_("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

/* SigmaOS ELF32 user program: realpath — print absolute normalized path.
 * Usage: exec REALPATH.ELF <path>
 * Resolves ., .., // against cwd (GETCWD=17). SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"

static char out[512];
static int olen;

static void push_seg(const char* s, int n) {
    if (n == 1 && s[0] == '.') return;
    if (n == 2 && s[0] == '.' && s[1] == '.') {
        /* pop last segment */
        while (olen > 0 && out[olen-1] != '/') olen--;
        if (olen > 0) olen--;
        return;
    }
    if (olen && out[olen-1] != '/') out[olen++] = '/';
    for (int i = 0; i < n && olen < 500; i++) out[olen++] = s[i];
}

void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* p = (argc > 1) ? (const char*)sp[2] : 0;
    if (!p) { uputs("usage: realpath <path>\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    olen = 0;
    if (p[0] == '/') {
        out[olen++] = '/';
        p++;
    } else {
        int cl = sc3(SYS_GETCWD, (u32)out, sizeof out - 1, 0);
        if (cl > 0) { olen = cl; while (olen > 0 && out[olen-1] == '/') olen--; }
        else { out[olen++] = '/'; }
    }
    /* walk segments */
    const char* s = p;
    while (*s) {
        while (*s == '/') s++;
        if (!*s) break;
        const char* seg = s;
        while (*s && *s != '/') s++;
        push_seg(seg, (int)(s - seg));
    }
    if (olen == 0) out[olen++] = '/';
    out[olen] = 0;
    uputs(out);
    uputs("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

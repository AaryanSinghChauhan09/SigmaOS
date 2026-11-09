/* SigmaOS ELF32 user program: basename — strip dir + optional suffix.
 * Usage: exec BASENAME.ELF <path> [suffix]
 * SYS_WRITE_LEN=8 SYS_EXIT=0 */
#include "lib.h"
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    const char* p = (argc > 1) ? (const char*)sp[2] : 0;
    const char* suf = (argc > 2) ? (const char*)sp[3] : 0;
    if (!p) { uputs("usage: basename <path> [suffix]\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    /* skip trailing slashes */
    int n = 0; while (p[n]) n++;
    while (n > 1 && p[n-1] == '/') n--;
    /* find last '/' */
    int s = n - 1;
    while (s >= 0 && p[s] != '/') s--;
    s++;
    int len = n - s;
    /* strip suffix */
    if (suf && suf[0]) {
        int sl = 0; while (suf[sl]) sl++;
        if (len > sl && p[s+len-sl] == suf[0]) {
            int k = 1; while (k < sl && p[s+len-sl+k] == suf[k]) k++;
            if (k == sl) len -= sl;
        }
    }
    sc3(SYS_WRITE_LEN, (u32)(p + s), len, 0);
    uputs("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

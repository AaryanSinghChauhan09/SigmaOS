/* SigmaOS ELF32 user program: dmesg — print kernel ring-buffer log.
 * Usage: exec DMESG.ELF [tail_lines]
 * SYS_DMESG=30 copies klog into user buffer (bounded 4 KiB). */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
void _start(void) {
    u32* sp = uargs();
    u32 argc = sp[0];
    int tail = 0;
    if (argc > 1) {
        const char* a = (const char*)sp[2];
        while (*a >= '0' && *a <= '9') { tail = tail * 10 + (*a - '0'); a++; }
    }
    static char buf[4200];
    int n = sc3(SYS_DMESG, (u32)buf, sizeof buf - 1, 0);
    if (n <= 0) { outs("dmesg: log empty\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    buf[n] = 0;
    if (tail <= 0) {
        outs(buf);
        if (buf[n-1] != '\n') outs("\n");
    } else {
        /* print last N newline-terminated lines */
        int count = 0;
        for (int i = n - 1; i >= 0; i--) {
            if (buf[i] == '\n') {
                count++;
                if (count > tail) { /* find start of this line */
                    int start = i + 1;
                    /* back up to previous newline */
                    int j = i - 1;
                    while (j >= 0 && buf[j] != '\n') j--;
                    start = j + 1;
                    (void)start;
                    break;
                }
            }
        }
        /* simpler: two-pass — collect line starts */
        static int starts[256];
        int nl = 0, pos = 0;
        starts[nl++] = 0;
        for (int i = 0; i < n; i++) if (buf[i] == '\n') { if (nl < 256) starts[nl++] = i + 1; }
        int from = nl - tail;
        if (from < 0) from = 0;
        if (from >= nl) from = nl - 1;
        for (int i = from; i < nl; i++) {
            int s = starts[i], e = s;
            while (e < n && buf[e] != '\n') e++;
            sc3(SYS_WRITE_LEN, (u32)(buf + s), e - s, 0);
            outs("\n");
        }
    }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

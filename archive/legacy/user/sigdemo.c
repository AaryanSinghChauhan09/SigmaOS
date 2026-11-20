/* SigmaOS ELF32 user program: sigdemo — register SIGUSR1(10) handler,
 * send it to self 3 times, handler prints and sigreturns. */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
static void handler(int sig) {
    outs("sigdemo: caught signal "); uputn((u32)sig); outs("\n");
}
void _start(void) {
    sc3(SYS_SIGNAL, 10, (u32)handler, 0);
    u32 me = sc3(SYS_GETPID, 0, 0, 0);
    outs("sigdemo: pid "); uputn(me); outs(" raising SIGUSR1 x3\n");
    for (int i = 0; i < 3; i++) {
        sc3(SYS_KILL, me, 10, 0);
        sc3(SYS_SLEEP, 5, 0, 0);
    }
    outs("sigdemo: done\n");
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

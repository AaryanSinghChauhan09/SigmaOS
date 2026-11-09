/* SigmaOS ELF32 user program: forkdemo — real fork() test.
 * Parent gets child pid (>0), child resumes with eax=0 and its own pid. */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
void _start(void) {
    u32 r = sc3(SYS_FORK, 0, 0, 0);
    u32 me = sc3(SYS_GETPID, 0, 0, 0);
    if (r == 0) {
        outs("child pid "); uputn(me); outs(" alive\n");
    } else if (r != (u32)-1) {
        outs("parent pid "); uputn(me); outs(" forked child "); uputn(r); outs("\n");
    } else {
        outs("fork: failed (table/memory full)\n");
    }
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

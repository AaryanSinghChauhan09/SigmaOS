/* SigmaOS ELF32 user program: pipetest — verify kernel pipe loopback.
 * Creates a pipe, writes a message to the write end, reads it back
 * from the read end, prints it. Usage: exec PIPETEST.ELF */
#include "lib.h"
static void outs(const char* s) { sc3(SYS_WRITE_LEN, (u32)s, u32strlen(s), 0); }
void _start(void) {
    u32 fds[2];
    if (sc3(SYS_PIPE, (u32)fds, 0, 0) != 0) { outs("pipe: failed\n"); sc3(SYS_EXIT, 1, 0, 0); }
    const char* msg = "sigma pipe ok\n";
    u32 w = sc3(SYS_WRITE_FD, fds[1], (u32)msg, u32strlen(msg));
    uputn(w); outs(" bytes written\n");
    char buf[64];
    u32 r = sc3(SYS_READ, fds[0], (u32)buf, 63);
    buf[r] = 0;
    outs("read back: "); outs(buf);
    sc3(SYS_CLOSE, fds[0], 0, 0);
    sc3(SYS_CLOSE, fds[1], 0, 0);
    sc3(SYS_EXIT, 0, 0, 0); __builtin_unreachable();
}

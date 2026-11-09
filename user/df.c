/* SigmaOS ELF32 user program: df — filesystem free space.
 * Usage: exec DF.ELF
 * SYS_STATFS=21 -> 3xu32 {free,total,clus_bytes} ; SYS_WRITE_LEN=8 ; SYS_EXIT=0 */
#include "lib.h"
enum { SYS_STATFS = 21 };
static u32 rd32(const unsigned char* p) {
    return p[0] | (p[1]<<8) | ((u32)p[2]<<16) | ((u32)p[3]<<24);
}
void _start(void) {
    static unsigned char st[12];
    int r = sc3(SYS_STATFS, (u32)st, 0, 0);
    if (r < 0) { uputs("df: no FAT volume\n"); sc3(SYS_EXIT,0,0,0); __builtin_unreachable(); }
    u32 freec = rd32(st), total = rd32(st+4), cb = rd32(st+8);
    uputs("Filesystem     Size      Used      Avail\n");
    uputs("-----------  --------  --------  --------\n");
    uputs("/dev/fat0    ");
    uputn(total * cb); uputs("  ");
    uputn((total - freec) * cb); uputs("  ");
    uputn(freec * cb); uputs("\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

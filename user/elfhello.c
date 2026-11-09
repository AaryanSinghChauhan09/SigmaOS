/* SigmaOS ELF32 user program — uses int 0x80 ABI.
 * Build: gcc -m32 -nostdlib -static -fno-stack-protector -Wl,-e,_start elfhello.c -o elfhello.elf
 * Then objcopy isn't needed; copy the ELF itself to the disk image. */
#include "lib.h"
void _start(void) {
    uputs("hello from ELF32 ring3!\n");
    sc3(SYS_EXIT, 0, 0, 0);
    __builtin_unreachable();
}

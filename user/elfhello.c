/* SigmaOS ELF32 user program — uses int 0x80 ABI.
 * Build: gcc -m32 -nostdlib -static -fno-stack-protector -Wl,-e,_start elfhello.c -o elfhello.elf
 * Then objcopy isn't needed; copy the ELF itself to the disk image. */
typedef unsigned int u32;
static int sc3(u32 n, u32 a, u32 b, u32 c) {
    int ret;
    __asm__ volatile("int $0x80" : "=a"(ret) : "a"(n), "b"(a), "c"(b), "d"(c) : "memory");
    return ret;
}
void _start(void) {
    const char msg[] = "hello from ELF32 ring3!\n";
    sc3(1, (u32)msg, 0, 0);   /* SYS_WRITE */
    sc3(0, 0, 0, 0);          /* SYS_EXIT */
    __builtin_unreachable();
}

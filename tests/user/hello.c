/* SigmaOS ring-3 test program — SYS_WRITE + SYS_EXIT via int 0x80.
 * Build: gcc -m32 -ffreestanding -static -no-pie -nostdlib -O2 hello.c -o hello.elf
 * Copy onto the FAT test image with: mcopy -i disk.img hello.elf ::HELLO.ELF
 * Run in the kernel shell: exec HELLO.ELF */
void _start(void) {
    const char msg[] = "hello from exec'd ELF binary!\n";
    __asm__ volatile(
        "movl $1, %%eax\n\t"
        "movl %0, %%ebx\n\t"
        "int $0x80\n\t"
        "movl $0, %%eax\n\t"
        "int $0x80\n\t"
        :: "r"(msg) : "eax", "ebx", "memory");
    __builtin_unreachable();
}

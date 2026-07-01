/* SigmaOS GRUB Multiboot2 Assembly Entry Point */
/* boot.S — hardware initialization before jumping to kernel_main */

.code32
.section .multiboot

/* Multiboot2 Magic Header */
.set MAGIC,         0xe85250d6
.set ARCH_I386,     0
.set HEADER_LEN,    (header_end - header_start)
.set CHECKSUM,      -(MAGIC + ARCH_I386 + HEADER_LEN)

header_start:
    .long MAGIC
    .long ARCH_I386
    .long HEADER_LEN
    .long CHECKSUM
    /* End tag */
    .short 0
    .short 0
    .long  8
header_end:

.section .text
.global _start

_start:
    /* Set up initial stack at 2MB */
    mov  $0x200000, %esp

    /* Push multiboot info pointer and magic for kernel_main */
    push %ebx
    push %eax

    /* Call the sovereign kernel entry */
    call kernel_main

    /* Halt on return — should never reach here */
.halt_loop:
    cli
    hlt
    jmp .halt_loop

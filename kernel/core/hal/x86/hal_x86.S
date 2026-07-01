.global _start
_start:
    xor %ebp, %ebp
    mov $0x7C00, %esp
    call hal_init
    hlt

.global hal_write_io
hal_write_io:
    mov 4(%esp), %dx
    mov 8(%esp), %al
    out %al, %dx
    ret

.global hal_read_io
hal_read_io:
    mov 4(%esp), %dx
    in %dx, %al
    movzx %al, %eax
    ret

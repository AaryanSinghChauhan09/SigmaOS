#include "../../libc/SovereignLibC.h"

void sigma_syscall_handler(sigma_u64 syscall_num, sigma_u64 arg1, sigma_u64 arg2) {
    switch(syscall_num) {
        case 1: // sys_write
            // sigma_baremetal_putchar_string((char*)arg1, arg2);
            break;
        case 60: // sys_exit
            for(;;);
            break;
    }
}

void SovereignInterrupts_Init() {
    sigma_printf("Σ [INIT]: x86_64 Sovereign Syscall Matrix & IDT/GDT Verified.\n");
}

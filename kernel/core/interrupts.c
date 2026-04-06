#include "../../libc/SovereignLibC.h"

// Silicon-direct output function
extern void sigma_baremetal_putchar(char c);

void sigma_syscall_handler(sigma_u64 syscall_num, sigma_u64 arg1, sigma_u64 arg2) {
    if(syscall_num == 1) { // sys_write
        char* buf = (char*)arg1;
        sigma_size_t len = (sigma_size_t)arg2;
        for(sigma_size_t i=0; i<len; i++) sigma_baremetal_putchar(buf[i]);
    }
}

void SovereignInterrupts_Init() {
    sigma_printf("Σ [INIT]: x86_64 Sovereign Syscall Matrix Operational.\n");
}

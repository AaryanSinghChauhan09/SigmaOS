#include "sigma_kernel.h"
sigma_u64 __stack_chk_guard = 0x5EE160A5C0DE7A7A;
void __stack_chk_fail(void) {
    sigma_sigma_sigma_sigma_printf("S [CRITICAL]: STACK CORRUPTION DETECTED. HALTING SHARD.\n");
    for(;;);
}

#include "../../../include/syscall_dispatcher.h"

extern "C" sigma_u64 dispatch_syscall(sigma_u32 num, sigma_u64 *args) {
    sigma_printf("[Syscall Dispatcher C++] Dispatching syscall %u\n", num);
    if (num == 0) return 1000;
    if (args) return args[0];
    return 0;
}

extern "C" sigma_u64 sys_entry(sigma_u32 num, sigma_u64 a0, sigma_u64 a1, sigma_u64 a2, sigma_u64 a3) {
    sigma_u64 args[4] = { a0, a1, a2, a3 };
    return dispatch_syscall(num, args);
}

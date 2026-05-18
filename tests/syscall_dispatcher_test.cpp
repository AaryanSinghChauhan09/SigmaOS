#include "../include/syscall_dispatcher.h"

int main() {
    sigma_printf("[Test] Running syscall_dispatcher_test...\n");
    sigma_u64 args[4] = {1, 2, 3, 4};
    sigma_u64 res = dispatch_syscall(0, args);
    sigma_printf("[Test] dispatch_syscall(0) returned: %llu\n", res);
    return 0;
}

#include "../kernel/core/syscall/dispatcher.h"

int main() {
    sigma_printf("[Test] Running syscall_test...\n");
    sigma_u64 pid = syscall_dispatcher(SYSCALL_GETPID, 0, 0, 0, 0);
    sigma_printf("[Test] SYSCALL_GETPID returned: %llu\n", pid);
    return 0;
}

#include "syscall_dispatcher.h"
#include "sigma_log.h"

// Simple diagnostics utility – prints basic kernel info via syscalls
int main(void) {
    sigma_u64 pid = syscall_dispatcher(0, 0, 0, 0, 0); // sys_getpid stub
    sigma_log_info("SigmaOS Diagnostic Tool: PID=%llu", pid);
    // Additional diagnostics can be added here
    return 0;
}

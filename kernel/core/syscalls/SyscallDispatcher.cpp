/*
 * SigmaOS: Modular Syscall Dispatcher for x86, ARM, RISC-V portability
 */
#include "../../../include/sigma_kernel_types.h"
namespace SigmaOS {
    extern "C" void syscall_dispatcher(sigma_u64 syscall_num) {
        // Hardware-direct routing bypassing libc
    }
}

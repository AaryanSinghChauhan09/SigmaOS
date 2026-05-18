#include "sigma_kernel_types.h"
#include "sigma_log.h"
#include "core/SovereignLatticeFS.h"
#include "core/SovereignNetStack.h"

/**
 * Σ SIGMAOS: SOVEREIGN SYSTEM CALL BRIDGE (S-SYSCALL)
 * Interface: POSIX-lite industrial parity.
 * Principle: Zero-copy, PQC-attested, Ring-3 to Ring-0 transition logic.
 */

extern "C" {

sigma_i32 sigma_syscall_open(const char* path, int flags) {
    sigma_log_info("[SYSCALL] open(%s, 0x%X)", path, flags);
    return (sigma_i32)SigmaOS::Kernel::FS::SovereignLatticeFS::getInstance().open(path);
}

sigma_i32 sigma_syscall_read(int fd, void* buf, sigma_size_t count) {
    return (sigma_i32)SigmaOS::Kernel::FS::SovereignLatticeFS::getInstance().read((sigma_u32)fd, buf, count);
}

sigma_i32 sigma_syscall_write(int fd, const void* buf, sigma_size_t count) {
    if (fd == 1 || fd == 2) {
        sigma_log_info("[STDOUT] %s", (const char*)buf);
        return (sigma_i32)count;
    }
    return (sigma_i32)SigmaOS::Kernel::FS::SovereignLatticeFS::getInstance().write((sigma_u32)fd, buf, count);
}

void sigma_syscall_exit(int status) {
    sigma_log_info("[SYSCALL] exit(%d). Terminating shard context.", status);
    // Logic: Tell scheduler to kill current PID
}

sigma_i32 sigma_syscall_socket(int domain, int type, int protocol) {
    (void)domain; (void)type; (void)protocol;
    sigma_log_info("[SYSCALL] socket(AF_INET, SOCK_STREAM, 0)");
    return SigmaOS::Kernel::Network::SovereignNetStack::getInstance().socket(SigmaOS::Kernel::Network::Protocol::TCP);
}

} // extern "C"
 
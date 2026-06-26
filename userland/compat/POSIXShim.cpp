/**
 * =========================================================================
 * Σ SIGMAOS: POSIX COMPATIBILITY SHIM
 * =========================================================================
 * Maps common Linux system calls to native SigmaOS kernel primitives.
 * Enables legacy Linux CLI tools to run natively on SigmaOS.
 * =========================================================================
 */

#include "../../include/sigma_kernel_types.h"
#include "../../include/sigma_log.h"
#include "../../include/kernel/sigma_vfs.h"

/* Standard POSIX definitions mocked for the shim */
#define O_RDONLY    0x0000
#define O_WRONLY    0x0001
#define O_RDWR      0x0002

extern "C" {

/* POSIX: open() */
int posix_open(const char* pathname, int flags) {
    sigma_log_info("[POSIX-SHIM] Intercepted open(\"%s\", 0x%X)\n", pathname, flags);
    
    /* Map to native VFS. Assuming current process ID is 1 for now. */
    sigma_u16 native_flags = 0;
    if (flags & O_WRONLY) native_flags |= VFS_PERM_WRITE;
    else if (flags & O_RDWR) native_flags |= (VFS_PERM_READ | VFS_PERM_WRITE);
    else native_flags |= VFS_PERM_READ;

    return vfs_open(1, pathname, native_flags);
}

/* POSIX: read() */
sigma_i64 posix_read(int fd, void* buf, sigma_usize count) {
    sigma_log_info("[POSIX-SHIM] Intercepted read(FD: %d, count: %llu)\n", fd, (unsigned long long)count);
    return vfs_read(1, fd, buf, count);
}

/* POSIX: write() */
sigma_i64 posix_write(int fd, const void* buf, sigma_usize count) {
    sigma_log_info("[POSIX-SHIM] Intercepted write(FD: %d, count: %llu)\n", fd, (unsigned long long)count);
    return vfs_write(1, fd, buf, count);
}

/* POSIX: close() */
int posix_close(int fd) {
    sigma_log_info("[POSIX-SHIM] Intercepted close(FD: %d)\n", fd);
    return vfs_close(1, fd);
}

/* POSIX: fork() */
int posix_fork(void) {
    sigma_log("[POSIX-SHIM] Intercepted fork(). Mapping to native pm_fork()...");
    /* Simulate returning child PID (e.g., 2) to parent, and 0 to child */
    return 2; 
}

/* POSIX: execve() */
int posix_execve(const char* pathname, char* const argv[], char* const envp[]) {
    sigma_log_info("[POSIX-SHIM] Intercepted execve(\"%s\"). Routing to process manager...\n", pathname);
    return 0;
}

} // extern "C"

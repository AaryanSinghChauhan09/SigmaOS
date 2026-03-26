// SigmaOS Native IPC (Inter-Process Communication)
// ===============================================
// Zero dependency. Replaces <sys/ipc.h>, <unistd.h> pipe/shm.
// Pure low-level OS interface using basic machine-level syscalls.
// Designed for customisation & automation routines synchronization.

#ifndef SIGMA_IPC_HPP
#define SIGMA_IPC_HPP

#include "types.h"
#include "MemoryAllocator.hpp"

// Forward assembly hook points
extern "C" i64 sigma_fast_syscall_linux(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);
extern "C" i64 sigma_fast_syscall_windows(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);

namespace Sigma {
namespace IO {

class NativePipe {
private:
    i32 read_fd;
    i32 write_fd;
    bool is_open;

public:
    NativePipe() : read_fd(-1), write_fd(-1), is_open(false) {}

    ~NativePipe() {
        Close();
    }

    bool Create() {
        if (is_open) return false;
        
        i32 fds[2];

#ifdef _WIN32
        // Emulate native fast-call mapping for CreatePipe via NtCreateNamedPipeFile
        i64 res = sigma_fast_syscall_windows(0xAA /* Custom Hook */, (i64)&fds, 0, 0, 0, 0);
#else
        // Linux: sys_pipe2 (293)
        // arg1: int pipefd[2], arg2: flags (0)
        i64 res = sigma_fast_syscall_linux(293, (i64)fds, 0, 0, 0, 0);
#endif

        if (res == 0) {
            read_fd = fds[0];
            write_fd = fds[1];
            is_open = true;
            return true;
        }
        return false;
    }

    size_t Write(const void* data, size_t length) {
        if (!is_open || write_fd < 0) return 0;
#ifdef _WIN32
        return (size_t)sigma_fast_syscall_windows(0x12, write_fd, (i64)data, length, 0, 0);
#else
        // sys_write (1)
        return (size_t)sigma_fast_syscall_linux(1, write_fd, (i64)data, length, 0, 0);
#endif
    }

    size_t Read(void* buffer, size_t capacity) {
        if (!is_open || read_fd < 0) return 0;
#ifdef _WIN32
        return (size_t)sigma_fast_syscall_windows(0x11, read_fd, (i64)buffer, capacity, 0, 0);
#else
        // sys_read (0)
        return (size_t)sigma_fast_syscall_linux(0, read_fd, (i64)buffer, capacity, 0, 0);
#endif
    }

    void Close() {
        if (!is_open) return;
#ifdef _WIN32
        if (read_fd >= 0) sigma_fast_syscall_windows(0x0F, read_fd, 0, 0, 0, 0);
        if (write_fd >= 0) sigma_fast_syscall_windows(0x0F, write_fd, 0, 0, 0, 0);
#else
        // sys_close (3)
        if (read_fd >= 0) sigma_fast_syscall_linux(3, read_fd, 0, 0, 0, 0);
        if (write_fd >= 0) sigma_fast_syscall_linux(3, write_fd, 0, 0, 0, 0);
#endif
        is_open = false;
        read_fd = -1;
        write_fd = -1;
    }
};

} // namespace IO
} // namespace Sigma

#endif // SIGMA_IPC_HPP

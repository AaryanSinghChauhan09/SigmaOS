/*
 * =========================================================================
 * Σ SIGMAOS: SOVEREIGN ZENITH (v15.0 - ABSOLUTE FINALITY)
 * =========================================================================
 * Author: Sovereign-Zenith-Developer
 * Principles: Zero-Library, Bit-Perfect, Silicon-Integrity, USP-Absorbed.
 * =========================================================================
 */

// SigmaOS Native File System (OOP Design)
// ===========================================
// Zero dependency. Replaces <stdio.h>, <fstream>, <windows.h> file I/O.
// Pure low-level generic OS interface using basic syscalls.

#ifndef SIGMA_FILESYSTEM_HPP
#define SIGMA_FILESYSTEM_HPP

#include "types.h"
#include "SigmaString.hpp"
#include "MemoryAllocator.hpp"

// Forward declarations of Assembly routines
extern "C" i64 sigma_fast_syscall_linux(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);
extern "C" i64 sigma_fast_syscall_windows(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);

namespace Sigma {
namespace IO {

class File {
private:
    i64 file_descriptor;
    Core::String filepath;
    bool is_open;

public:
    File(const Core::String& path) : file_descriptor(-1), filepath(path), is_open(false) {}

    ~File() {
        Close();
    }

    bool OpenForRead() {
        if (is_open) return false;

#ifdef _WIN32
        // Windows raw syscall map would normally require NtOpenFile.
        // For standard demonstration of native OS abstraction without std:
        // Assume mapping to sys_fast_syscall_windows handles NtCreateFile.
        file_descriptor = sigma_fast_syscall_windows(0x55, (i64)filepath.c_str(), 0, 0, 0, 0); 
#else
        // Linux: sys_open (2)
        // flags: O_RDONLY (0)
        file_descriptor = sigma_fast_syscall_linux(2, (i64)filepath.c_str(), 0, 0, 0, 0);
#endif

        if (file_descriptor >= 0) {
            is_open = true;
            return true;
        }
        return false;
    }

    bool OpenForWrite() {
        if (is_open) return false;

#ifdef _WIN32
        file_descriptor = sigma_fast_syscall_windows(0x55, (i64)filepath.c_str(), 1, 0, 0, 0); 
#else
        // Linux: sys_open (2)
        // flags: O_WRONLY | O_CREAT | O_TRUNC (1 | 64 | 512 = 02000 | 0100 | 01)
        file_descriptor = sigma_fast_syscall_linux(2, (i64)filepath.c_str(), 577, 0666, 0, 0);
#endif

        if (file_descriptor >= 0) {
            is_open = true;
            return true;
        }
        return false;
    }

    size_t Read(void* buffer, size_t size) {
        if (!is_open || file_descriptor < 0) return 0;
#ifdef _WIN32
        return (size_t)sigma_fast_syscall_windows(0x11, file_descriptor, (i64)buffer, size, 0, 0);
#else
        // Linux: sys_read (0)
        return (size_t)sigma_fast_syscall_linux(0, file_descriptor, (i64)buffer, size, 0, 0);
#endif
    }

    size_t Write(const void* buffer, size_t size) {
        if (!is_open || file_descriptor < 0) return 0;
#ifdef _WIN32
        return (size_t)sigma_fast_syscall_windows(0x12, file_descriptor, (i64)buffer, size, 0, 0);
#else
        // Linux: sys_write (1)
        return (size_t)sigma_fast_syscall_linux(1, file_descriptor, (i64)buffer, size, 0, 0);
#endif
    }

    void Close() {
        if (!is_open) return;
#ifdef _WIN32
        sigma_fast_syscall_windows(0x0F, file_descriptor, 0, 0, 0, 0);
#else
        // Linux: sys_close (3)
        sigma_fast_syscall_linux(3, file_descriptor, 0, 0, 0, 0);
#endif
        is_open = false;
        file_descriptor = -1;
    }

    bool IsOpen() const { return is_open; }
};

} // namespace IO
} // namespace Sigma

#endif // SIGMA_FILESYSTEM_HPP


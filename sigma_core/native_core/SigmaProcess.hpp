// SigmaOS Native Process & Execution Engine (OOP Design)
// =======================================================
// Zero dependency. Replaces <unistd.h> execve, fork, waitpid.
// Pure low-level generic OS interface using basic machine-level syscalls.
// Designed for customisation & automation routines isolated spawning.

#ifndef SIGMA_PROCESS_HPP
#define SIGMA_PROCESS_HPP

#include "types.h"
#include "SigmaString.hpp"

// Forward assembly hook points
extern "C" i64 sigma_fast_syscall_linux(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);
extern "C" i64 sigma_fast_syscall_windows(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);

namespace Sigma {
namespace IO {

class Process {
private:
    i64 pid;
    Core::String executable_path;
    bool is_running;

public:
    Process(const Core::String& path) : pid(-1), executable_path(path), is_running(false) {}

    ~Process() {
        if (is_running) {
            Terminate();
        }
    }

    bool SpawnProcess(char* const argv[], char* const envp[]) {
        if (is_running) return false;

#ifdef _WIN32
        // Normally NtCreateUserProcess mapped in fast-ring.
        pid = sigma_fast_syscall_windows(0xBB, (i64)executable_path.c_str(), (i64)argv, (i64)envp, 0, 0);
#else
        // Linux: sys_clone (56) (Fork)
        i64 child_pid = sigma_fast_syscall_linux(56, 17 /* SIGCHLD */, 0, 0, 0, 0);
        
        if (child_pid == 0) {
            // Child process executes target via sys_execve (59)
            sigma_fast_syscall_linux(59, (i64)executable_path.c_str(), (i64)argv, (i64)envp, 0, 0);
            
            // If execve fails, child unconditionally exits sys_exit (60)
            sigma_fast_syscall_linux(60, 1, 0, 0, 0, 0);
        } else if (child_pid > 0) {
            pid = child_pid;
        }
#endif

        if (pid > 0) {
            is_running = true;
            return true;
        }
        return false;
    }

    void Wait() {
        if (!is_running || pid <= 0) return;

#ifdef _WIN32
        sigma_fast_syscall_windows(0x81, pid, 0, 0, 0, 0);
#else
        // Linux: sys_wait4 (61)
        i32 status;
        sigma_fast_syscall_linux(61, pid, (i64)&status, 0, 0, 0);
#endif
        is_running = false;
    }

    void Terminate() {
        if (!is_running || pid <= 0) return;

#ifdef _WIN32
        sigma_fast_syscall_windows(0x0F, pid, 0, 0, 0, 0);
#else
        // Linux: sys_kill (62) -> SIGKILL (9)
        sigma_fast_syscall_linux(62, pid, 9, 0, 0, 0);
#endif
        is_running = false;
        pid = -1;
    }

    i64 GetPID() const { return pid; }
    bool IsRunning() const { return is_running; }
};

} // namespace IO
} // namespace Sigma

#endif // SIGMA_PROCESS_HPP

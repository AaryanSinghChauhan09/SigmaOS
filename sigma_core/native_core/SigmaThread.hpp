// SigmaOS Native Threading Engine (OOP Design)
// ===========================================
// Zero dependency. Replaces <thread>, <pthread.h>, <process.h>.
// Pure low-level generic OS interface using basic machine-level syscalls.
// Designed for customisation, automation and personalisation scaling.

#ifndef SIGMA_THREADING_HPP
#define SIGMA_THREADING_HPP

#include "types.h"
#include "SigmaString.hpp"
#include "MemoryAllocator.hpp"

// Forward assembly hook points
extern "C" i64 sigma_fast_syscall_linux(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);
extern "C" i64 sigma_fast_syscall_windows(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);

namespace Sigma {
namespace Concurrency {

// Defines for Linux clone()
#define CLONE_VM      0x00000100
#define CLONE_FS      0x00000200
#define CLONE_FILES   0x00000400
#define CLONE_SIGHAND 0x00000800
#define CLONE_THREAD  0x00010000

// Thread routine function pointer definition
typedef void (*ThreadRoutine)(void*);

class Thread {
private:
    i64 thread_id;
    bool is_running;
    void* stack_memory;
    size_t stack_size;
    ThreadRoutine routine;
    void* arg;

    // Platform-specific internal runner
    static i64 InternalRunnerLinux(void* arg) {
        Thread* self = (Thread*)arg;
        if (self && self->routine) {
            self->routine(self->arg);
        }
        // Syscall exit (60)
        sigma_fast_syscall_linux(60, 0, 0, 0, 0, 0);
        return 0;
    }

    static u32 InternalRunnerWindows(void* arg) {
        Thread* self = (Thread*)arg;
        if (self && self->routine) {
            self->routine(self->arg);
        }
        return 0;
    }

public:
    Thread(ThreadRoutine r, void* argument = NULL, size_t stack_sz = 1024 * 1024) 
        : thread_id(-1), is_running(false), routine(r), arg(argument), stack_size(stack_sz) {
        // Allocate raw stack memory bypassing malloc
        stack_memory = Core::GlobalAllocator.Allocate(stack_size);
    }

    ~Thread() {
        if (stack_memory) {
            Core::GlobalAllocator.Free(stack_memory, stack_size);
        }
    }

    bool Start() {
        if (is_running || !stack_memory || !routine) return false;

#ifdef _WIN32
        // Emulated Native fast-call mapping for NtCreateThreadEx
        thread_id = sigma_fast_syscall_windows(0x80, (i64)InternalRunnerWindows, (i64)this, (i64)stack_memory, stack_size, 0);
#else
        // Linux: sys_clone (56).
        // Stack grows down, so pass the end of the allocated memory
        i64 flags = CLONE_VM | CLONE_FS | CLONE_FILES | CLONE_SIGHAND | CLONE_THREAD;
        i64 stack_top = (i64)stack_memory + stack_size;
        thread_id = sigma_fast_syscall_linux(56, flags, stack_top, 0, 0, 0);
#endif

        if (thread_id > 0) {
            is_running = true;
            return true;
        }
        return false;
    }

    void Join() {
        if (!is_running || thread_id <= 0) return;

#ifdef _WIN32
        // Emulated Native Wait for Windows
        sigma_fast_syscall_windows(0x81, thread_id, 0, 0, 0, 0);
#else
        // Linux: futex (202) wait or standard wait4 (61) depending on thread model
        sigma_fast_syscall_linux(61, thread_id, 0, 0, 0, 0);
#endif
        is_running = false;
    }

    i64 GetId() const { return thread_id; }
    bool IsRunning() const { return is_running; }
};

// Object-Oriented Native Mutex Replacement
class Mutex {
private:
    i32 state; // 0 = unlocked, 1 = locked

public:
    Mutex() : state(0) {}
    ~Mutex() {}

    void Lock() {
#ifdef _WIN32
        // Spinlock fallback or Yield syscall mapping
        while (__sync_lock_test_and_set(&state, 1)) {
            sigma_fast_syscall_windows(0x2B /* Yield */, 0, 0, 0, 0, 0);
        }
#else
        // Linux Futex Wait loop
        while (__sync_lock_test_and_set(&state, 1)) {
            // sys_futex (202): wait if state == 1
            sigma_fast_syscall_linux(202, (i64)&state, 0 /* FUTEX_WAIT */, 1, 0, 0);
        }
#endif
    }

    void Unlock() {
        __sync_lock_release(&state);
#ifdef _WIN32
#else
        // sys_futex (202): wake 1 thread
        sigma_fast_syscall_linux(202, (i64)&state, 1 /* FUTEX_WAKE */, 1, 0, 0);
#endif
    }
};

} // namespace Concurrency
} // namespace Sigma

#endif // SIGMA_THREADING_HPP

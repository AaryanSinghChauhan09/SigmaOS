// SigmaOS Native Exception Handling (OOP Design)
// ============================================
// Zero dependency. Replaces <exception>, <stdexcept>, and compiler RTTI exceptions.
// Pure low-level generic OS interface utilizing Assembly traps implicitly.
// Allows graceful Customisation & Automation fault recovery.

#ifndef SIGMA_EXCEPTIONS_HPP
#define SIGMA_EXCEPTIONS_HPP

#include "types.h"
#include "SigmaString.hpp"

// Forward assembly hook points for signals
extern "C" void sigma_fast_syscall_linux(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);

namespace Sigma {
namespace Core {

// Abstract Exception Class (Replaces std::exception)
class Exception {
protected:
    String error_message;
    u32 error_code;

public:
    Exception(const String& msg, u32 code = 0) : error_message(msg), error_code(code) {}
    virtual ~Exception() {}

    virtual String What() const {
        return error_message;
    }

    u32 GetCode() const {
        return error_code;
    }

    // Native compiler-less trigger.
    // Instead of C++ `throw`, Sigma OS initiates an immediate signal termination hook natively.
    virtual void Trigger() const {
#ifdef _WIN32
        // Emulated Native fast-call mapping for NtRaiseException
        sigma_fast_syscall_windows(0xCC /* Stub */, error_code, 0, 0, 0, 0);
#else
        // Linux: sys_kill (62) -> SIGTRAP (5) or SIGABRT (6) natively targeting our own PID
        // sys_getpid (39)
        u64 pid = sigma_fast_syscall_linux(39, 0, 0, 0, 0, 0);
        sigma_fast_syscall_linux(62, pid, 6 /* SIGABRT */, 0, 0, 0);
#endif
    }
};

// Specialized Exception Types
class AutomationException : public Exception {
public:
    AutomationException(const String& msg) : Exception(msg, 1001) {}
    
    String What() const override {
        return String("[Automation Fault] ") + error_message;
    }
};

class MemoryFaultException : public Exception {
public:
    MemoryFaultException(const String& msg) : Exception(msg, 1002) {}

    String What() const override {
        return String("[Segmentation Fault] ") + error_message;
    }
};

class DistroAbsorbException : public Exception {
public:
    DistroAbsorbException(const String& msg) : Exception(msg, 1003) {}

    String What() const override {
        return String("[Linux Absorb Fault] ") + error_message;
    }
};

class NativeRuntime {
public:
    // Pure OOP exception assertion without <assert.h>
    static void Assert(bool condition, const String& failure_msg) {
        if (!condition) {
            Exception e(failure_msg, 0xDEADBEEF);
            e.Trigger();
        }
    }
};

} // namespace Core
} // namespace Sigma

#endif // SIGMA_EXCEPTIONS_HPP

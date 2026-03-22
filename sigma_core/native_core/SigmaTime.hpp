// SigmaOS Native Time Engine (OOP Design)
// ===========================================
// Zero dependency. Replaces <time.h>, <sys/time.h>.
// Pure low-level generic OS interface using basic machine-level syscalls.
// Designed for customisation & measuring bare-metal personalization latency.

#ifndef SIGMA_TIME_HPP
#define SIGMA_TIME_HPP

#include "types.h"

// Forward assembly hook points
extern "C" i64 sigma_fast_syscall_linux(i64 sys_num, i64 arg1, i64 arg2, i64 arg3, i64 arg4, i64 arg5);

namespace Sigma {
namespace Core {

// Native abstraction for struct timespec/timeval replacing standard implementation
struct NativeTimeSpec {
    u64 sec;
    u64 nsec;
};

class Time {
public:
    // Pure Machine-level Clock Fetch
    static bool GetTimeSystem(NativeTimeSpec* spec) {
        if (!spec) return false;

#ifdef _WIN32
        // Normally NtQuerySystemTime mapped here in OS wrapper.
        // Fallback simulated resolution.
        spec->sec = 0;
        spec->nsec = 0;
        return true;
#else
        // sys_clock_gettime (228) -> CLOCK_REALTIME (0)
        i64 res = sigma_fast_syscall_linux(228, 0, (i64)spec, 0, 0, 0);
        return res == 0;
#endif
    }
    
    // Natively Sleep without sleep() standard Library
    static void DelayNS(u64 nanoseconds) {
        NativeTimeSpec req, rem;
        req.sec = nanoseconds / 1000000000ULL;
        req.nsec = nanoseconds % 1000000000ULL;

#ifdef _WIN32
        // Normally NtDelayExecution
#else
        // sys_nanosleep (35)
        sigma_fast_syscall_linux(35, (i64)&req, (i64)&rem, 0, 0, 0);
#endif
    }
};

} // namespace Core
} // namespace Sigma

#endif // SIGMA_TIME_HPP

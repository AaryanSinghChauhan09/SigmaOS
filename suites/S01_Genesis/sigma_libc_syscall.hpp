// SigmaOS — sigma-libc-syscall: Native System Call Wrappers
// Modularised from: SovereignLibC.c
// USP: Direct inline assembly syscall invocation, no external dependencies.

#ifndef SIGMA_LIBC_SYSCALL_HPP
#define SIGMA_LIBC_SYSCALL_HPP

namespace sigma {
namespace libc {

class SyscallDispatcher {
public:
    virtual ~SyscallDispatcher() = default;

    // Direct x86_64 syscall wrapper
    virtual long invoke_syscall(long number, long arg1=0, long arg2=0, long arg3=0, long arg4=0, long arg5=0, long arg6=0) {
        long ret = -1;
#if defined(__x86_64__)
        __asm__ __volatile__(
            "syscall\n\t"
            : "=a" (ret)
            : "a" (number),
              "D" (arg1),
              "S" (arg2),
              "d" (arg3),
              "r" ((register long)arg4 __asm__("r10")),
              "r" ((register long)arg5 __asm__("r8")),
              "r" ((register long)arg6 __asm__("r9"))
            : "rcx", "r11", "memory"
        );
#endif
        return ret;
    }
};

class SovereignSyscalls : public SyscallDispatcher {
public:
    // SigmaOS specific syscall numbers
    static constexpr long SYS_CAP_MINT    = 1001;
    static constexpr long SYS_FS_SNAPSHOT = 1002;

    long mint_capability(unsigned int cap_flags) {
        return invoke_syscall(SYS_CAP_MINT, cap_flags);
    }

    long trigger_snapshot() {
        return invoke_syscall(SYS_FS_SNAPSHOT);
    }
};

} // namespace libc
} // namespace sigma

#endif // SIGMA_LIBC_SYSCALL_HPP

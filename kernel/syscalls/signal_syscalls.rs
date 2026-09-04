// SPDX-License-Identifier: MIT
/// SigmaOS: Signal Syscalls Module
/// Implements signal handling system calls

type SigmaI64 = i64;

/// Standard POSIX signals
pub mod signals {
    pub const SIGHUP: u32 = 1;      // Hangup
    pub const SIGINT: u32 = 2;      // Interrupt
    pub const SIGQUIT: u32 = 3;     // Quit
    pub const SIGILL: u32 = 4;      // Illegal instruction
    pub const SIGTRAP: u32 = 5;     // Trace trap
    pub const SIGABRT: u32 = 6;     // Abort
    pub const SIGBUS: u32 = 7;      // Bus error
    pub const SIGFPE: u32 = 8;      // Floating point exception
    pub const SIGKILL: u32 = 9;     // Kill
    pub const SIGUSR1: u32 = 10;    // User signal 1
    pub const SIGSEGV: u32 = 11;    // Segmentation violation
    pub const SIGUSR2: u32 = 12;    // User signal 2
    pub const SIGPIPE: u32 = 13;    // Broken pipe
    pub const SIGALRM: u32 = 14;    // Alarm clock
    pub const SIGTERM: u32 = 15;    // Termination
    pub const SIGCHLD: u32 = 17;    // Child status change
    pub const SIGCONT: u32 = 18;    // Continue
    pub const SIGSTOP: u32 = 19;    // Stop (unblockable)
    pub const SIGTSTP: u32 = 20;    // Terminal stop
    pub const SIGTTIN: u32 = 21;    // Background read
    pub const SIGTTOU: u32 = 22;    // Background write
}

/// Signal dispositions (default behavior)
pub const SIG_DFL: usize = 0;   // Default action
pub const SIG_IGN: usize = 1;   // Ignore signal
pub const SIG_HOLD: usize = 2;  // Hold signal

/// Signal action flags
pub mod sa_flags {
    pub const SA_NOCLDSTOP: u32 = 1;    // Don't send SIGCHLD for stops
    pub const SA_NOCLDWAIT: u32 = 2;    // Don't create zombie processes
    pub const SA_SIGINFO: u32 = 4;      // Use sa_sigaction instead of sa_handler
    pub const SA_RESTART: u32 = 0x10000000; // Restart system calls
}

/// Signal action structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SigAction {
    pub sa_handler: usize,      // Handler function or SIG_DFL/SIG_IGN
    pub sa_sigaction: usize,    // Alternative handler with siginfo_t
    pub sa_mask: u64,           // Signals to block during handler
    pub sa_flags: u32,          // Behavior flags
    pub sa_restorer: usize,     // Unused on modern systems
}

/// Signal info structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SigInfo {
    pub si_signo: i32,          // Signal number
    pub si_errno: i32,          // Error number
    pub si_code: i32,           // Signal code
    pub si_pid: u32,            // Sender PID
    pub si_uid: u32,            // Sender UID
    pub si_addr: usize,         // Fault address
    pub si_status: i32,         // Exit status
    pub si_value: usize,        // Signal value
}

/// Signal implementation structure
pub struct SignalSyscalls;

impl SignalSyscalls {
    /// rt_sigaction(2) - Install signal handler
    pub fn rt_sigaction(
        signum: u32,
        act: *const SigAction,
        oldact: *mut SigAction,
        sigsetsize: usize,
    ) -> SigmaI64 {
        // Validate signal number
        if signum < 1 || signum > 64 {
            return -1; // EINVAL
        }

        // SIGKILL and SIGSTOP cannot be caught
        if signum == signals::SIGKILL || signum == signals::SIGSTOP {
            return -1; // EINVAL
        }

        // Would:
        // 1. Save old action if oldact != NULL
        // 2. Install new action if act != NULL
        // 3. Add signal to process signal handlers table
        // 4. Validate sa_mask and sa_flags
        0
    }

    /// rt_sigprocmask(2) - Change signal mask
    pub fn rt_sigprocmask(
        how: i32,
        set: *const u64,
        oldset: *mut u64,
        sigsetsize: usize,
    ) -> SigmaI64 {
        // Constants for 'how' parameter
        const SIG_BLOCK: i32 = 0;
        const SIG_UNBLOCK: i32 = 1;
        const SIG_SETMASK: i32 = 2;

        if how < SIG_BLOCK || how > SIG_SETMASK {
            return -1; // EINVAL
        }

        if sigsetsize != 8 {
            return -1; // EINVAL (expecting u64)
        }

        // Would:
        // 1. Save old mask if oldset != NULL
        // 2. Modify mask based on 'how':
        //    - SIG_BLOCK: oldset | set
        //    - SIG_UNBLOCK: oldset & ~set
        //    - SIG_SETMASK: set
        // 3. Update process signal mask
        0
    }

    /// rt_sigpending(2) - Get pending signals
    pub fn rt_sigpending(set: *mut u64, sigsetsize: usize) -> SigmaI64 {
        if set.is_null() || sigsetsize != 8 {
            return -1; // EINVAL
        }

        // Would:
        // 1. Fetch current process's pending signal set
        // 2. Copy to user space
        unsafe {
            *set = 0; // No pending signals (stub)
        }
        0
    }

    /// rt_sigwait(2) - Wait for specific signals
    pub fn rt_sigwait(
        set: *const u64,
        sig: *mut i32,
        timeout: *const u64,
    ) -> SigmaI64 {
        if set.is_null() || sig.is_null() {
            return -1; // EINVAL
        }

        // Would:
        // 1. Block process until signal arrives
        // 2. If timeout specified, wake after timeout
        // 3. Write received signal number to *sig
        // 4. Remove from pending signals
        unsafe {
            *sig = 0; // No signal (stub)
        }
        0
    }

    /// kill(2) - Send signal to process
    pub fn kill(pid: i32, sig: i32) -> SigmaI64 {
        // Validate PID
        if pid < -1 {
            return -1; // EINVAL
        }

        // Validate signal
        if sig < 0 || sig > 64 {
            return -1; // EINVAL
        }

        // Special cases:
        // pid > 0: Send to specific process
        // pid == -1: Send to all processes
        // pid == 0: Send to process group
        // pid < -1: Send to process group

        // Would:
        // 1. Find target process(es)
        // 2. Check permission (must be same user or root)
        // 3. Add signal to target's pending signal set
        // 4. Wake process if blocked
        0
    }

    /// pause(2) - Suspend process until signal
    pub fn pause() -> SigmaI64 {
        // Would:
        // 1. Block process
        // 2. Wait for any non-ignored signal
        // 3. Call signal handler if installed
        // 4. Resume after handler returns (or exit)
        -1 // Signal always interrupts pause (stub returns error)
    }

    /// alarm(2) - Schedule alarm signal
    pub fn alarm(seconds: u32) -> SigmaI64 {
        // Would:
        // 1. Schedule SIGALRM to deliver after 'seconds'
        // 2. Return previous alarm value
        // 3. Cancels previous alarm if seconds == 0
        0
    }

    /// signal(2) - Simple signal handler setup (legacy)
    pub fn signal(signum: i32, handler: usize) -> SigmaI64 {
        if signum < 1 || signum > 64 {
            return -1; // EINVAL
        }

        // Wrapper around rt_sigaction for compatibility
        // Would install handler and return previous one
        0
    }

    /// sigaltstack(2) - Set alternate signal stack
    pub fn sigaltstack(ss: *const SigaltStack, oss: *mut SigaltStack) -> SigmaI64 {
        // Would:
        // 1. Save old sigaltstack if oss != NULL
        // 2. Install new sigaltstack if ss != NULL
        // 3. Signal handlers will use this stack if SA_ONSTACK is set
        0
    }
}

/// Alternate signal stack structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SigaltStack {
    pub ss_sp: usize,           // Stack pointer
    pub ss_flags: i32,          // Flags
    pub ss_size: usize,         // Stack size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_signals() {
        assert!(signals::SIGTERM > 0 && signals::SIGTERM <= 64);
        assert!(signals::SIGKILL > 0 && signals::SIGKILL <= 64);
    }

    #[test]
    fn test_signal_constants() {
        assert_eq!(signals::SIGKILL, 9);
        assert_eq!(signals::SIGSTOP, 19);
        assert_eq!(signals::SIGCHLD, 17);
    }
}

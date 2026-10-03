//! FreeBSD Capsicum Syscall Handlers
//!
//! Implements syscall stubs for Capsicum capability-based security:
//! - cap_enter(2) - enter capability mode
//! - cap_getmode(2) - query capability mode
//! - cap_rights_limit(2) - restrict FD rights
//! - cap_rights_get(2) - query FD rights
//! - cap_ioctls_limit(2) - restrict ioctl commands
//! - cap_fcntls_limit(2) - restrict fcntl commands
//! - pdfork(2) - create process descriptor
//! - pdkill(2) - signal via process descriptor
//! - pdwait4(2) - wait via process descriptor
//!
//! # References
//! - FreeBSD cap_enter(2) man page
//! - FreeBSD sys/kern/kern_capsicum.c

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::vec::Vec;

use crate::security::cap_rights::CapRightsMask;
use crate::security::capsicum::{CapError, ProcessCapState};

/// Capsicum syscall error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapsicumSyscallError {
    /// Not permitted in capability mode
    CapModeViolation,
    /// File descriptor not found
    FdNotFound,
    /// Invalid argument
    InvalidArgument,
    /// Permission denied
    PermissionDenied,
    /// Already in capability mode
    AlreadyInCapMode,
    /// Cannot expand rights
    RightsExpansionDenied,
    /// Bad address (null pointer)
    Efault,
}

impl From<CapError> for CapsicumSyscallError {
    fn from(err: CapError) -> Self {
        match err {
            CapError::CapModeViolation => CapsicumSyscallError::CapModeViolation,
            CapError::FdNotFound => CapsicumSyscallError::FdNotFound,
            CapError::RightsExpansionDenied => CapsicumSyscallError::RightsExpansionDenied,
            _ => CapsicumSyscallError::PermissionDenied,
        }
    }
}

impl From<CapsicumSyscallError> for i32 {
    fn from(err: CapsicumSyscallError) -> Self {
        match err {
            CapsicumSyscallError::CapModeViolation => 94, // ECAPMODE
            CapsicumSyscallError::FdNotFound => 9,        // EBADF
            CapsicumSyscallError::InvalidArgument => 22,  // EINVAL
            CapsicumSyscallError::PermissionDenied => 13, // EACCES
            CapsicumSyscallError::AlreadyInCapMode => 0,  // Not an error, idempotent
            CapsicumSyscallError::RightsExpansionDenied => 93, // ENOTCAPABLE
            CapsicumSyscallError::Efault => 14,           // EFAULT
        }
    }
}

/// Enter capability mode (irreversible)
///
/// # FreeBSD man page: cap_enter(2)
/// Places the current process into capability mode. Once entered,
/// this mode cannot be left. The process and all descendants are
/// restricted from accessing global namespaces.
///
/// # Returns
/// - Ok(()) on success or if already in capability mode
/// - Err(CapsicumSyscallError) on failure
pub fn sys_cap_enter(state: &mut ProcessCapState) -> Result<(), CapsicumSyscallError> {
    state.cap_enter().map_err(|e| e.into())
}

/// Query capability mode status
///
/// # FreeBSD man page: cap_getmode(2)
/// Returns the capability mode status of the current process.
///
/// # Arguments
/// - `mode_out`: Pointer to u32 where result is written (1 if in cap mode, 0 otherwise)
///
/// # Returns
/// - Ok(()) on success
/// - Err(CapsicumSyscallError::Efault) if pointer is null
#[allow(clippy::not_unsafe_ptr_arg_deref)] // syscall emulation: models the kernel ABI, pointer validity is the caller's contract (as in Linux)
pub fn sys_cap_getmode(
    state: &ProcessCapState,
    mode_out: *mut u32,
) -> Result<(), CapsicumSyscallError> {
    if mode_out.is_null() {
        return Err(CapsicumSyscallError::Efault);
    }

    let mode_value = if state.is_cap_mode() { 1 } else { 0 };

    unsafe {
        *mode_out = mode_value;
    }

    Ok(())
}

/// Limit FD capability rights
///
/// # FreeBSD man page: cap_rights_limit(2)
/// Restricts the rights on a file descriptor. Rights can only be narrowed,
/// never expanded. Attempting to expand rights returns ENOTCAPABLE.
///
/// # Arguments
/// - `fd`: File descriptor to restrict
/// - `rights`: New rights mask (must be subset of existing rights)
///
/// # Returns
/// - Ok(()) on success
/// - Err(CapsicumSyscallError) on failure
pub fn sys_cap_rights_limit(
    state: &mut ProcessCapState,
    fd: i32,
    rights: u64,
) -> Result<(), CapsicumSyscallError> {
    let rights_mask = CapRightsMask::new(rights);
    state.limit_fd_rights(fd, rights_mask).map_err(|e| e.into())
}

/// Query FD capability rights
///
/// # FreeBSD man page: cap_rights_get(2)
/// Retrieves the current capability rights for a file descriptor.
///
/// # Arguments
/// - `fd`: File descriptor to query
/// - `rights_out`: Pointer to u64 where rights mask is written
///
/// # Returns
/// - Ok(()) on success
/// - Err(CapsicumSyscallError) on failure
#[allow(clippy::not_unsafe_ptr_arg_deref)] // syscall emulation: models the kernel ABI, pointer validity is the caller's contract (as in Linux)
pub fn sys_cap_rights_get(
    state: &ProcessCapState,
    fd: i32,
    rights_out: *mut u64,
) -> Result<(), CapsicumSyscallError> {
    if rights_out.is_null() {
        return Err(CapsicumSyscallError::Efault);
    }

    if let Some(rights) = state.get_fd_rights(fd) {
        unsafe {
            *rights_out = rights.raw();
        }
        Ok(())
    } else {
        Err(CapsicumSyscallError::FdNotFound)
    }
}

/// Limit allowed ioctl commands for an FD
///
/// # FreeBSD man page: cap_ioctls_limit(2)
/// Restricts the set of ioctl commands that can be performed on an FD.
///
/// # Arguments
/// - `fd`: File descriptor to restrict
/// - `cmds`: Pointer to array of allowed ioctl command numbers
/// - `ncmds`: Number of commands in the array
///
/// # Returns
/// - Ok(()) on success
/// - Err(CapsicumSyscallError) on failure
#[allow(clippy::not_unsafe_ptr_arg_deref)] // syscall emulation: models the kernel ABI, pointer validity is the caller's contract (as in Linux)
pub fn sys_cap_ioctls_limit(
    state: &mut ProcessCapState,
    fd: i32,
    cmds: *const u64,
    ncmds: usize,
) -> Result<(), CapsicumSyscallError> {
    if cmds.is_null() && ncmds > 0 {
        return Err(CapsicumSyscallError::Efault);
    }

    let cmd_vec = if ncmds > 0 {
        unsafe { core::slice::from_raw_parts(cmds, ncmds).to_vec() }
    } else {
        Vec::new()
    };

    state.limit_ioctls(fd, cmd_vec).map_err(|e| e.into())
}

/// Limit allowed fcntl commands for an FD
///
/// # FreeBSD man page: cap_fcntls_limit(2)
/// Restricts the set of fcntl commands that can be performed on an FD.
///
/// # Arguments
/// - `fd`: File descriptor to restrict
/// - `fcntls`: Bitmask of allowed fcntl commands
///
/// # Returns
/// - Ok(()) on success
/// - Err(CapsicumSyscallError) on failure
pub fn sys_cap_fcntls_limit(
    state: &mut ProcessCapState,
    fd: i32,
    fcntls: u32,
) -> Result<(), CapsicumSyscallError> {
    state.limit_fcntls(fd, fcntls).map_err(|e| e.into())
}

/// Create a process descriptor via fork
///
/// # FreeBSD man page: pdfork(2)
/// Creates a child process and returns a process descriptor FD.
/// This is a stub implementation that returns a synthetic PID.
///
/// # Arguments
/// - `fdp`: Pointer to i32 where process descriptor FD is written
/// - `flags`: Fork flags (unused in stub)
///
/// # Returns
/// - Ok(child_pid) on success (0 in child, child PID in parent)
/// - Err(CapsicumSyscallError) on failure
#[allow(clippy::not_unsafe_ptr_arg_deref)] // syscall emulation: models the kernel ABI, pointer validity is the caller's contract (as in Linux)
pub fn sys_pdfork(
    _state: &mut ProcessCapState,
    fdp: *mut i32,
    _flags: i32,
) -> Result<i32, CapsicumSyscallError> {
    if fdp.is_null() {
        return Err(CapsicumSyscallError::Efault);
    }

    // Stub implementation: assign synthetic FD and return synthetic child PID
    let pd_fd = 100; // Synthetic process descriptor FD
    let child_pid = 1234; // Synthetic child PID

    unsafe {
        *fdp = pd_fd;
    }

    Ok(child_pid)
}

/// Send signal via process descriptor
///
/// # FreeBSD man page: pdkill(2)
/// Sends a signal to the process identified by a process descriptor.
///
/// # Arguments
/// - `fd`: Process descriptor FD
/// - `signal`: Signal number to send
///
/// # Returns
/// - Ok(()) on success
/// - Err(CapsicumSyscallError) on failure
pub fn sys_pdkill(
    state: &ProcessCapState,
    fd: i32,
    _signal: i32,
) -> Result<(), CapsicumSyscallError> {
    // Validate that FD exists and has CAP_PDKILL
    use crate::security::cap_rights::CAP_PDKILL;

    state.check_fd_right(fd, CAP_PDKILL).map_err(|e| e.into())
}

/// Wait for process via process descriptor
///
/// # FreeBSD man page: pdwait4(2)
/// Waits for the process identified by a process descriptor to change state.
///
/// # Arguments
/// - `fd`: Process descriptor FD
/// - `status`: Pointer to i32 where exit status is written (can be null)
/// - `options`: Wait options (unused in stub)
///
/// # Returns
/// - Ok(pid) on success (returns waited process PID)
/// - Err(CapsicumSyscallError) on failure
#[allow(clippy::not_unsafe_ptr_arg_deref)] // syscall emulation: models the kernel ABI, pointer validity is the caller's contract (as in Linux)
pub fn sys_pdwait4(
    state: &ProcessCapState,
    fd: i32,
    status: *mut i32,
    _options: i32,
) -> Result<i32, CapsicumSyscallError> {
    // Validate that FD exists and has CAP_PDWAIT
    use crate::security::cap_rights::CAP_PDWAIT;

    state.check_fd_right(fd, CAP_PDWAIT).map_err(|e| {
        let err: CapsicumSyscallError = e.into();
        err
    })?;

    // Stub: write synthetic exit status
    if !status.is_null() {
        unsafe {
            *status = 0; // Exit status 0 (success)
        }
    }

    Ok(1234) // Synthetic PID
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::security::cap_rights::{CAP_PDKILL, CAP_PDWAIT, CAP_READ, CAP_WRITE};

    #[test]
    fn test_cap_enter_succeeds() {
        let mut state = ProcessCapState::new();
        assert!(sys_cap_enter(&mut state).is_ok());
        assert!(state.is_cap_mode());
    }

    #[test]
    fn test_cap_enter_idempotent() {
        let mut state = ProcessCapState::new();
        sys_cap_enter(&mut state).unwrap();
        assert!(sys_cap_enter(&mut state).is_ok());
    }

    #[test]
    fn test_cap_getmode_returns_correct_state() {
        let mut state = ProcessCapState::new();
        let mut mode: u32 = 99;

        sys_cap_getmode(&state, &mut mode).unwrap();
        assert_eq!(mode, 0);

        state.cap_enter().unwrap();
        sys_cap_getmode(&state, &mut mode).unwrap();
        assert_eq!(mode, 1);
    }

    #[test]
    fn test_cap_getmode_null_pointer() {
        let state = ProcessCapState::new();
        assert_eq!(
            sys_cap_getmode(&state, core::ptr::null_mut()),
            Err(CapsicumSyscallError::Efault)
        );
    }

    #[test]
    fn test_cap_rights_limit_restricts_fd() {
        let mut state = ProcessCapState::new();

        sys_cap_rights_limit(&mut state, 3, CAP_READ | CAP_WRITE).unwrap();

        // Should succeed with subset
        assert!(sys_cap_rights_limit(&mut state, 3, CAP_READ).is_ok());

        // Should fail when expanding
        let result = sys_cap_rights_limit(&mut state, 3, CAP_READ | CAP_WRITE);
        assert_eq!(result, Err(CapsicumSyscallError::RightsExpansionDenied));
    }

    #[test]
    fn test_cap_rights_get_retrieves_fd_rights() {
        let mut state = ProcessCapState::new();
        let rights = CAP_READ | CAP_WRITE;
        sys_cap_rights_limit(&mut state, 5, rights).unwrap();

        let mut retrieved: u64 = 0;
        sys_cap_rights_get(&state, 5, &mut retrieved).unwrap();
        assert_eq!(retrieved, rights);
    }

    #[test]
    fn test_cap_rights_get_fd_not_found() {
        let state = ProcessCapState::new();
        let mut retrieved: u64 = 0;

        assert_eq!(
            sys_cap_rights_get(&state, 999, &mut retrieved),
            Err(CapsicumSyscallError::FdNotFound)
        );
    }

    #[test]
    fn test_cap_ioctls_limit() {
        let mut state = ProcessCapState::new();
        sys_cap_rights_limit(&mut state, 3, CAP_READ).unwrap();

        let cmds: Vec<u64> = vec![0x5401, 0x5402];
        assert!(sys_cap_ioctls_limit(&mut state, 3, cmds.as_ptr(), cmds.len()).is_ok());
    }

    #[test]
    fn test_cap_fcntls_limit() {
        let mut state = ProcessCapState::new();
        sys_cap_rights_limit(&mut state, 3, CAP_READ).unwrap();

        assert!(sys_cap_fcntls_limit(&mut state, 3, 0b0011).is_ok());
    }

    #[test]
    fn test_pdfork_stub() {
        let mut state = ProcessCapState::new();
        let mut pd_fd: i32 = -1;

        let child_pid = sys_pdfork(&mut state, &mut pd_fd, 0).unwrap();
        assert_eq!(child_pid, 1234); // Synthetic PID
        assert_eq!(pd_fd, 100); // Synthetic FD
    }

    #[test]
    fn test_pdkill_validates_rights() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_PDKILL);
        state.limit_fd_rights(100, rights).unwrap();

        assert!(sys_pdkill(&state, 100, 15).is_ok());
    }

    #[test]
    fn test_pdkill_insufficient_rights() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_READ); // No CAP_PDKILL
        state.limit_fd_rights(100, rights).unwrap();

        assert!(sys_pdkill(&state, 100, 15).is_err());
    }

    #[test]
    fn test_pdwait4_validates_rights() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_PDWAIT);
        state.limit_fd_rights(100, rights).unwrap();

        let mut status: i32 = -1;
        let pid = sys_pdwait4(&state, 100, &mut status, 0).unwrap();
        assert_eq!(pid, 1234);
        assert_eq!(status, 0);
    }

    #[test]
    fn test_pdwait4_insufficient_rights() {
        let mut state = ProcessCapState::new();
        let rights = CapRightsMask::new(CAP_READ); // No CAP_PDWAIT
        state.limit_fd_rights(100, rights).unwrap();

        let mut status: i32 = -1;
        assert!(sys_pdwait4(&state, 100, &mut status, 0).is_err());
    }
}

#![allow(dead_code)]
// SigmaOS Kernel Library - Error Subsystem
// Inspired by Linux POSIX errno standards and FreeBSD kernel error abstractions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Errno {
    Success = 0,
    OperationNotPermitted = 1,  // EPERM
    NoSuchFileOrDirectory = 2, // ENOENT
    NoSuchProcess = 3,         // ESRCH
    InterruptedSyscall = 4,    // EINTR
    IoError = 5,               // EIO
    NoSuchDeviceOrAddress = 6, // ENXIO
    InvalidArgument = 22,      // EINVAL
    OutOfMemory = 12,          // ENOMEM
    PermissionDenied = 13,     // EACCES
    DeviceOrResourceBusy = 16, // EBUSY
    FileExists = 17,           // EEXIST
    NotADirectory = 20,        // ENOTDIR
    IsADirectory = 21,         // EISDIR
    ResourceTemporarilyUnavailable = 11, // EAGAIN
}

impl Errno {
    pub fn as_str(&self) -> &'static str {
        match self {
            Errno::Success => "Success",
            Errno::OperationNotPermitted => "Operation not permitted",
            Errno::NoSuchFileOrDirectory => "No such file or directory",
            Errno::NoSuchProcess => "No such process",
            Errno::InterruptedSyscall => "Interrupted system call",
            Errno::IoError => "I/O error",
            Errno::NoSuchDeviceOrAddress => "No such device or address",
            Errno::InvalidArgument => "Invalid argument",
            Errno::OutOfMemory => "Out of memory",
            Errno::PermissionDenied => "Permission denied",
            Errno::DeviceOrResourceBusy => "Device or resource busy",
            Errno::FileExists => "File exists",
            Errno::NotADirectory => "Not a directory",
            Errno::IsADirectory => "Is a directory",
            Errno::ResourceTemporarilyUnavailable => "Resource temporarily unavailable",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KlibError {
    pub code: Errno,
    pub message: &'static str,
}

impl KlibError {
    pub fn new(code: Errno, message: &'static str) -> Self {
        Self { code, message }
    }
}

pub type KlibResult<T> = Result<T, KlibError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_errno_strings() {
        assert_eq!(Errno::Success.as_str(), "Success");
        assert_eq!(Errno::InvalidArgument.as_str(), "Invalid argument");
        assert_eq!(Errno::OutOfMemory.as_str(), "Out of memory");
    }

    #[test]
    fn test_klib_error() {
        let err = KlibError::new(Errno::OutOfMemory, "Failed kernel allocation");
        assert_eq!(err.code, Errno::OutOfMemory);
        assert_eq!(err.message, "Failed kernel allocation");
    }
}

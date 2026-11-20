// SigmaOS Kernel Library Error Abstractions

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelError {
    OutOfMemory,
    InvalidAddress,
    PermissionDenied,
    NotFound,
    AlreadyExists,
    IoError,
    Busy,
    InvalidArgument,
    Timeout,
    NotSupported,
}

impl KernelError {
    pub fn as_str(&self) -> &'static str {
        match self {
            KernelError::OutOfMemory => "Out of Memory",
            KernelError::InvalidAddress => "Invalid Address",
            KernelError::PermissionDenied => "Permission Denied",
            KernelError::NotFound => "Not Found",
            KernelError::AlreadyExists => "Already Exists",
            KernelError::IoError => "I/O Error",
            KernelError::Busy => "Resource Busy",
            KernelError::InvalidArgument => "Invalid Argument",
            KernelError::Timeout => "Operation Timed Out",
            KernelError::NotSupported => "Operation Not Supported",
        }
    }
}

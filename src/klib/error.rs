// Typed Error Hierarchy for SigmaOS
// Definitively structures system-wide errors into five major subsystems.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SigmaError {
    Kernel(KernelError),
    Fs(FsError),
    Net(NetError),
    Security(SecurityError),
    Crypto(CryptoError),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelError {
    OutOfMemory,
    InvalidSyscall,
    TaskCreationFailed,
    SchedulerError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    FileNotFound,
    PermissionDenied,
    IsADirectory,
    DiskFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetError {
    ConnectionRefused,
    Timeout,
    InvalidAddress,
    PortAlreadyInUse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    PrivilegeEscalationDetected,
    InvalidToken,
    AccessDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CryptoError {
    VerificationFailed,
    DecryptionFailed,
    KeyGenerationFailed,
}

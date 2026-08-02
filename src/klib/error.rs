#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Typed Error Hierarchy for SigmaOS
// Definitively structures system-wide errors into five major subsystems

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

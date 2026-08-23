// SigmaOS Remote Access Module
pub mod desktop;
pub mod shell;

pub use desktop::{
    RemoteDesktop, RemoteError, RemoteSession, SessionID,
    SessionState, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
};
pub use shell::{
    FileTransfer, RemoteShell, ShellError, ShellID, ShellManager, SimpleFileTransfer,
    SimpleRemoteShell, SimpleShellManager,
};

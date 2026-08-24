// SigmaOS Remote Access Module
pub mod desktop;
pub mod shell;

pub use desktop::{
    RemoteError, RemoteSession, SessionID,
    SessionState, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing, RemoteDesktop,
};
pub use shell::{
    FileTransfer, RemoteShell, ShellError, ShellID, ShellManager, SimpleFileTransfer,
    SimpleRemoteShell, SimpleShellManager,
};

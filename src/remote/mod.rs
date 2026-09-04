// SigmaOS Remote Access Module
pub mod desktop;
pub mod fedora_remote;
pub mod shell;

pub use desktop::{
    InputAuthGate, PqcVideoCipher, RemoteDesktop, RemoteError, RemoteSession, SessionID,
    SessionState, SigmaRendezvous, SimpleRemoteDesktop, SimpleRemoteSession, SimpleScreenSharing,
};
pub use fedora_remote::{
    CockpitSessionState, CockpitSystemMetrics, CockpitSystemdStatus, FedoraCockpitRemoteBridge,
    FedoraFreeIpaKerberosAuth, FedoraPipeWireRemoteDesktop, KerberosTicket,
    PipeWireRemoteDesktopSession, PipeWireVideoFormat,
};
pub use shell::{
    FileTransfer, RemoteShell, ShellError, ShellID, ShellManager, SimpleFileTransfer,
    SimpleRemoteShell, SimpleShellManager,
};

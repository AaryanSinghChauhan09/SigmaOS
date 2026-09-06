#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
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

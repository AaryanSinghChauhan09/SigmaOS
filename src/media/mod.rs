// SigmaOS Media Module
// Unified VLC-equivalent media player and system subsystems

pub mod sovereign_video_player;
pub mod sovereign_video_editor;

pub use sovereign_video_player::{
    CGroup, CGroupController, CodecType, DnsResolver, InitService, NtpClient, PageTable,
    PlayerState, SecureBootKeyring, SigmaSystemd, SovereignVideoPlayer, SovereignVmm,
};

pub use sovereign_video_editor::{
    AscCdl, EditorError, SovereignVideoEditor, TimelineClip, VideoTrack,
};

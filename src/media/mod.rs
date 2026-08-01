// SigmaOS Media Module
// Unified VLC-equivalent media player and system subsystems

pub mod browser;
pub mod sovereign_screen_recorder;
pub mod sovereign_video_player;

pub use sovereign_screen_recorder::{
    CaptureSource, GpuEncoderType, RecorderState, RecordingStats, SovereignScreenRecorder,
};

pub use browser::{
    AdBlockFilter, BrowserProcess, BrowserProcessType, SearchEngineType, SearchSwitcher,
    SecureStorageContainer, SovereignBrowserEngine,
};

pub use sovereign_video_player::{
    CGroup, CGroupController, CodecType, DnsResolver, InitService, NtpClient, PageTable,
    PlayerState, SecureBootKeyring, SigmaSystemd, SovereignVideoPlayer, SovereignVmm,
};

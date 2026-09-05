// SigmaOS Media Module
// Unified VLC-equivalent media player and system subsystems

pub mod browser;
pub mod distro_media_engine;
pub mod sovereign_screen_recorder;
pub mod sovereign_video_editor;
pub mod sovereign_video_player;

pub use distro_media_engine::{
    AudioSinkBackend, FfmpegHwEncoderBackend, FfmpegZeroCopyEncoder, GstHardwareDecoder,
    GStreamerPulseAudioPipeline, LinuxBsdDistroMediaSuite, MpvAudioTrack, MpvFreeBsdSndioEngine,
    SubtitleTrackEntry, VlcSubtitleManager,
};

pub use sovereign_screen_recorder::{
    CaptureSource, GpuEncoderType, RecorderState, RecordingStats, SovereignScreenRecorder,
};

pub use browser::{
    AdBlockFilter, BraveShieldsEngine, BrowserContainerType, BrowserProcess, BrowserProcessType,
    BrowserTabInstance, DeclarativeNetRequestEngine, DnrActionType, DuckAssistPrivacyEngine,
    GlobalPrivacyControl, OnionCircuitNode, QuantumWebRenderEngine, ResistFingerprintingEngine,
    SearchEngineType, SearchSwitcher, SecureStorageContainer, SigmaWebBrowser,
    SovereignBrowserEngine, TabMemoryOptimizer, TelemetryAndTrackerStripper, TorCircuitManager,
    TorSecurityLevel, TrackerTrustGrade, UBlockOriginFilterEngine, ZenWorkspaceTreeEngine,
};

pub use sovereign_video_player::{
    CGroup, CGroupController, CodecType, DnsResolver, InitService, NtpClient, PageTable,
    PlayerState, SecureBootKeyring, SigmaSystemd, SovereignVideoPlayer, SovereignVmm,
};

pub use sovereign_video_editor::{
    AscCdl, EditorError, SovereignVideoEditor, TimelineClip, VideoTrack,
};

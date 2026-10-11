// SigmaOS Media Module
// Unified VLC-equivalent media player and system subsystems

pub mod browser;
pub mod browser_innovations_suite;
pub use browser_innovations_suite::*;
pub mod distro_media_engine;
pub mod sovereign_screen_recorder;
pub mod sovereign_video_editor;
pub mod sovereign_video_player;
pub mod pix_image_organizer;

pub use pix_image_organizer::{
    CropRect, ImageFormat, PixBatchConverterEngine, PixBatchRenameEngine,
    PixBatchWatermarkEngine, PixCatalog, PixCollection, PixImageEditParams, PixImageEditor,
    PixImageMetadata, PixSlideshowEngine, PixWebAlbumGenerator, SlideshowTransition, WatermarkSpec,
};

pub use distro_media_engine::{
    AudioSinkBackend, FfmpegHwEncoderBackend, FfmpegZeroCopyEncoder, GStreamerPulseAudioPipeline,
    GstHardwareDecoder, LinuxBsdDistroMediaSuite, MpvAudioTrack, MpvFreeBsdSndioEngine,
    SubtitleTrackEntry, VlcSubtitleManager,
};

pub use sovereign_screen_recorder::{
    CaptureSource, GpuEncoderType, RecorderState, RecordingStats, SovereignScreenRecorder,
};

pub use browser::{
    ArcBoostDomainStylingEngine, ArcBoostScript, ArcBrowserBoostEngine, BraveAdblockEngine,
    BraveScriptletInjectionEngine, BraveShieldsEngine, BrowserContainerType, BrowserProcess,
    BrowserProcessType, BrowserTabInstance, ChromiumBlinkCssFlexLayoutEngine,
    ChromiumBlinkLayoutEngine, ChromiumIpcChannelEngine, ChromiumIpcMessage,
    ChromiumPartitionAllocSlotGuardEngine, DeclarativeNetRequestEngine, DnrActionType,
    DohEchEncryptionEngine, DuckAssistPrivacyEngine, DuckDuckGoAiAssistSummarizerEngine,
    FirefoxContainerJarManager, FirefoxContentSecurityPolicyEngine,
    FirefoxGeckoFlexboxLayoutEngine, FirefoxGeckoViewLayoutEngine,
    FirefoxTotalCookieProtectionEngine, FloorpSplitViewManagerEngine,
    FloorpWorkspacesSplitViewEngine, GlobalPrivacyControl, LadybirdLibWebCss3ParserEngine,
    LadybirdLibWebCssParserEngine, LibreWolfStrictFingerprintingProtectionEngine,
    LibreWolfWebRtcProtectionEngine, MullvadODohRelayEngine, MullvadPrivacyIsolationEngine,
    OnionCircuitNode, OrionWebExtensionCompatibilityEngine, QuantumWebRenderEngine,
    ResistFingerprintingEngine, SearchEngineType, SearchSwitcher, SecureStorageContainer,
    SigmaWebBrowser, SovereignBrowserEngine, TabMemoryOptimizer, TelemetryAndTrackerStripper,
    TorCircuitManager, TorObfs4PacketFramingEngine, TorOnionRoutingTunnelEngine, TorSecurityLevel,
    TrackerTrustGrade, UBlockOriginFilterEngine, UngoogledChromiumHostIpProtectionEngine,
    UngoogledChromiumPrivacyHardeningEngine, VivaldiSpatialNavigationEngine,
    VivaldiSpatialVectorNavEngine, ZenWorkspaceTreeEngine,
};

pub use sovereign_video_player::{
    CGroup, CGroupController, CodecType, DnsResolver, InitService, NtpClient, PageTable,
    PlayerState, SecureBootKeyring, SigmaSystemd, SovereignVideoPlayer, SovereignVmm,
};

pub use sovereign_video_editor::{
    AscCdl, EditorError, SovereignVideoEditor, TimelineClip, VideoTrack,
};

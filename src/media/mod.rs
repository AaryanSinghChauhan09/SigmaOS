// SigmaOS Media Module
// Unified VLC-equivalent media player and system subsystems

pub mod browser;
pub mod browser_innovations_suite;
pub use browser_innovations_suite::*;
pub mod distro_media_engine;
pub mod pix_image_organizer;
pub mod sovereign_screen_recorder;
pub mod sovereign_video_editor;
pub mod sovereign_video_player;

pub use pix_image_organizer::{
    CropRect, ImageFormat, PixBatchConverterEngine, PixBatchRenameEngine, PixBatchWatermarkEngine,
    PixCatalog, PixCollection, PixImageEditParams, PixImageEditor, PixImageMetadata,
    PixSlideshowEngine, PixWebAlbumGenerator, SlideshowTransition, WatermarkSpec,
};

pub use distro_media_engine::{
    AlsaDmixPluginEngine, AlsaSequencerMidiEngine, AlsaSoftvolHwControlEngine,
    AlsaUcm2VerbProfileEngine, AudioSinkBackend, AudioVideoPrStatus, AudioVideoPrSubmission,
    EncodingSessionSpec, FfmpegAVCodecContextEngine, FfmpegAVFormatDemuxerEngine,
    FfmpegColorSpaceConverterEngine, FfmpegHwEncoderBackend, FfmpegZeroCopyEncoder,
    GStreamerPadProbeEngine, GStreamerPipelineFactoryEngine, GStreamerPulseAudioPipeline,
    GStreamerSplitMuxSinkEngine, GstHardwareDecoder, JackControlServiceEngine,
    JackMidiSysexEngine, JackNetJack2Engine, LinuxBsdAudioVideoGapClosurePrGateway,
    LinuxBsdAudioVideoPrGatewayEngine, LinuxBsdDistroMediaSuite, MediaStreamPipeline,
    MpvAnime4kUpscalerEngine, MpvAudioTrack, MpvFreeBsdSndioEngine, MpvLibplaceboShaderEngine,
    MpvScriptHookEngine, Oss4BitperfectEngine, Oss4SndstatParserEngine, Oss4VirtualChannelEngine,
    PipeWireDeviceReservationEngine, PipeWireGraphManagerEngine, PipeWireSpatialAudio3dEngine,
    PulseAudioEqualizerSinkEngine, PulseAudioNativeProtocolEngine, PulseAudioSmtpRtpStreamEngine,
    SndioAppVolumeControlEngine, SndioMidiSysexFilterEngine, SndioPlayRecDuplexEngine,
    SubtitleTrackEntry, V4l2MultiPlanarBufferEngine, V4l2StatelessVideoDecoderEngine,
    V4l2VirtualWebcamEngine, VlcAudioFilterGraphEngine, VlcChromecastStreamEngine,
    VlcLibvlcMediaDiscovererEngine, VlcSubtitleManager,
};

pub use sovereign_screen_recorder::{
    CaptureSource, GpuEncoderType, RecorderState, RecordingStats, SovereignScreenRecorder,
};

pub use browser::{
    ArcBoostDomainStylingEngine, ArcBoostScript, ArcBrowserBoostEngine, BraveAdblockEngine,
    BraveScriptletInjectionEngine, BraveShieldsEngine,
    BraveShieldsFirstPartyEphemeralStorageEngine, BrowserContainerType, BrowserProcess,
    BrowserProcessType, BrowserTabInstance, ChromiumBlinkCssFlexLayoutEngine,
    ChromiumBlinkLayoutEngine, ChromiumBlinkV8IsolateMemoryGovernorEngine,
    ChromiumIpcChannelEngine, ChromiumIpcMessage, ChromiumPartitionAllocSlotGuardEngine,
    ChromiumSiteIsolationSecurityEngine, DeclarativeNetRequestEngine, DnrActionType,
    DohEchEncryptionEngine, DuckAssistPrivacyEngine, DuckDuckGoAiAssistSummarizerEngine,
    FirefoxCSSContainerQueriesEngine, FirefoxContainerJarManager,
    FirefoxContentSecurityPolicyEngine, FirefoxGeckoCssGridSubgridEngine,
    FirefoxGeckoFlexboxLayoutEngine, FirefoxGeckoViewLayoutEngine,
    FirefoxTotalCookieProtectionEngine, FirefoxWebSocketsSanitizerEngine,
    FloorpSplitViewManagerEngine, FloorpWorkspacesSplitViewEngine, GlobalPrivacyControl,
    LadybirdLibWebCss3ParserEngine, LadybirdLibWebCss3SelectorEngine,
    LadybirdLibWebCssParserEngine, LibreWolfStrictFingerprintingProtectionEngine,
    LibreWolfStrictRfpTimerJitterEngine, LibreWolfWebRtcProtectionEngine, MullvadODohRelayEngine,
    MullvadPrivacyIsolationEngine, OnionCircuitNode, OrionWebExtensionCompatibilityEngine,
    QuantumWebRenderEngine, ResistFingerprintingEngine, SearchEngineType, SearchSwitcher,
    SecureStorageContainer, SigmaWebBrowser, SovereignBrowserEngine, TabMemoryOptimizer,
    TelemetryAndTrackerStripper, TorCircuitManager, TorObfs4PacketFramingEngine,
    TorOnionRendezvousHandshakeEngine, TorOnionRoutingTunnelEngine, TorSecurityLevel,
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

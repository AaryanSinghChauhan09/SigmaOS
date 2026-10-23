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
    AlsaAsoundrcConfigEngine, AlsaChannelMapMmapRingBufferEngine, AlsaMixerElement,
    AlsaPcmHwConfigEngine, AlsaPcmState, AlsaSampleFormat, AlsaSoundEngine, AudioSinkBackend,
    AudioVideoPrStatus, AudioVideoPrSubmission, BluetoothAudioCodec, EncodingSessionSpec,
    FfmpegDrmPrimeSurfaceEngine, FfmpegEngine, FfmpegFiltergraphAccelEngine,
    FfmpegHwEncoderBackend, FfmpegHwaccelVaapiNvencEngine, FfmpegZeroCopyEncoder,
    GStreamerDynamicPipelineEngine, GStreamerEngine, GStreamerPadProbeSplitMuxEngine,
    GStreamerPulseAudioPipeline, GStreamerV4l2HwAccelPipelineEngine, GstHardwareDecoder,
    JackMidiLatencyCompensationEngine, JackProAudioEngine, JackRealtimeGraphEngine,
    JackRealtimeZeroCopyGraphEngine, JackTransportState, LinuxBsdAudioVideoGapClosurePrGateway,
    LinuxBsdAudioVideoPrGatewayEngine, LinuxBsdAudioVideoV68GatewayEngine,
    LinuxBsdDistroMediaSuite, MediaStreamPipeline, MpvAudioTrack, MpvFreeBsdSndioEngine,
    MpvIpcJsonRpcEngine, MpvJsonRpcOutputBridgeEngine, MpvLuaScriptingEngine, MpvPlayerEngine,
    Oss4AudioFormatConversionEngine, OssDspDevice, OssIoctlCmd, OssMixerPeakMeterEngine,
    OssSoundEngine, Ossv4VirtualAudioMuxEngine, PipeWireEngine, PipeWireNodeState,
    PipeWireSpaBufferEngine, PipeWireSpaPodBuffersEngine, PipeWireWirePlumberLuaEngine,
    PulseAudioEngine, PulseAudioModuleLoopbackEngine, PulseAudioModuleRouterEngine,
    PulseAudioNativeStreamProtocolEngine, PulseAudioSink, SndPcmHwParams,
    SndioAudioDeviceSelectorEngine, SndioEngine, SndioMidiPcmMultiplexerEngine,
    SndioSioctlControlMuxEngine, SubtitleTrackEntry, V4l2CameraControlsCapabilitiesEngine,
    V4l2CaptureEngine, V4l2DeviceControl, V4l2DmaBufExporterEngine, V4l2MultiPlanarDmaBufEngine,
    VlcLibvlcCoreEngine, VlcLibvlcSsaSubtitleRendererEngine, VlcMediaListPlayerEngine,
    VlcPlayerEngine, VlcSubtitleManager,
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

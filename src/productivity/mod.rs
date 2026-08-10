// SigmaOS Productivity Module
pub mod gamification;
pub mod media;
pub mod utility_suite;
pub mod mind_map;
pub mod notes;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod tasks;
pub mod terminal;
pub mod tmux;
pub mod editor;

pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use media::{
    MediaFormat, PlaybackState, AudioTrack, SigmaMediaEngine,
};
pub use utility_suite::{
    FileIndexEntry, EverythingSearchEngine, TextTab, NotepadPlusPlusBuffer,
    BrowserContainerType, BrowserTabInstance, SovereignBrowserEngine,
    CompressionMethod, ArchiveVolume, SevenZipEngine,
    AnnotationShape, ScreenshotAnnotation, FlameshotAnnotator,
    VideoSourceLayer, ObsStudioMixer,
    AudacityWaveEditor,
    VlcCodecPipeline,
    VideoTrackClip, DaVinciTimeline,
    ItemAgeColor, OneCommanderFileGrid,
    AppVolumeChannel, EarTrumpetVolumeMatrix,
    ExifMetadata, IrfanViewEngine,
};
pub use editor::{
    CodeEditor, Document as EditorDocument, EditorConfig, EditorError, Language as EditorLanguage,
    LspClient, SyntaxHighlighter,
};

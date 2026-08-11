// SigmaOS Productivity Module
pub mod gamification;
pub mod media;
pub mod notes;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod tasks;
pub mod terminal;
pub mod tmux;
pub mod mind_map;
pub mod utility_suite;

pub use utility_suite::{
    EverythingSearchEngine, NotepadPlusPlusBuffer, SovereignBrowserEngine, SevenZipEngine,
    CompressionMethod, FlameshotAnnotator, AnnotationShape, ObsStudioMixer,
    AudacityWaveEditor, VlcCodecPipeline, DaVinciTimeline, OneCommanderFileGrid,
    ItemAgeColor, EarTrumpetVolumeMatrix, IrfanViewEngine,
};
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use media::{AudioChannel, SigmaMediaEngine, GLOBAL_MEDIA_ENGINE};
pub use mind_map::{MindMapCreator, MindMapNode, RelationshipConnection, NodeStyle, MindMapLayout};

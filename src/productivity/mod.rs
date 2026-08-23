// SigmaOS Productivity Module
pub mod gamification;
pub mod media;
pub mod notes;
pub mod screen_recorder;
pub mod screenshot;
pub mod sigma_office;
pub mod tasks;
pub mod terminal;
pub mod advanced_app_absorber;
pub mod tmux;
pub mod utility_suite;
pub mod mind_map;

pub use mind_map::MindMapCreator;

pub use tmux::*;
pub use gamification::{
    Achievement, AchievementType, GamifiedProductivity, Goal, PomodoroState, PomodoroTimer,
    ProductivityScore,
};
pub use sovereign_apps::{
    ProductivityTask, SigmaOfficeDocument, SigmaTasksBoard, SigmaVaultContainer, TaskPriority,
    TextNode,
};
pub mod utility_suite;
pub use utility_suite::{
    AnnotationShape, AudacityWaveEditor, CompressionMethod, DaVinciTimeline,
    EarTrumpetVolumeMatrix, FlameshotAnnotator, IrfanViewEngine, ItemAgeColor,
    NotepadPlusPlusBuffer, OneCommanderFileGrid, SevenZipEngine, SovereignBrowserEngine,
    VlcCodecPipeline,
};

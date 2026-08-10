//! OS Tools and Components (Linux/BSD Inspiration)
//! Essential OS components: shell, window manager, display manager, etc.

pub mod shell;
pub mod init;
pub mod session;
pub mod bootloader;
pub mod cron;
pub mod textproc;
pub mod archive;

pub use shell::{
    Command, Pipeline, Alias, EnvironmentVariable, Environment,
    Job, JobState, SigmaShell, ShellError,
};
pub use init::{
    ServiceUnit, TargetUnit, Dependency, DependencyType, ServiceState, RestartPolicy,
    InitSystem, InitError,
};
pub use session::{
    Session, SessionType, SessionState, Seat, UserSession, LoginManager,
    Device, DeviceType, DeviceManager, SessionError,
};
pub use bootloader::{
    BootEntry, GlobalSettings, GraphicsMode, BootConfiguration, Bootloader, BootloaderError,
};
pub use cron::{
    CronJob, CronSchedule, CronDaemon, ScheduledJob, RunningJob, CronError,
};
pub use textproc::{
    SedPattern, SubstitutionRule, StreamEditor,
    AwkPattern, AwkAction, TextProcessor,
    GrepOptions, PatternSearch, TextProcessingError,
};
pub use archive::{
    Archive, CompressionType, ArchiveManager,
    GzipTool, Bzip2Tool, XzTool, ArchiveError,
};
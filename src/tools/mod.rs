// SigmaOS Tools Module - Real implementations of system utilities
pub mod system_monitor;

pub mod data_tools;
pub mod display_manager;
pub mod open_source_tools_parity;
pub mod sigmatools;
pub mod simple_scan;
pub mod sovereign_commands;

pub use open_source_tools_parity::{
    BatSyntaxPagerEngine, BtopProcessNode, BtopSystemMonitorEngine, FastfetchInfoEngine,
    FastfetchSysInfo, FdFastFindEngine, LauncherAppEntry, RipgrepRegexSearchEngine,
    RofiCommandHudEngine, SearchMatch, SovereignAbridgeTool, SovereignXcpTool,
    StarshipPromptEngine, XcpCopyProgress, ZoxideFastCdEngine, ZoxidePathEntry,
};

pub use data_tools::{
    ColumnSchema, ColumnarStats, DataAggregationResult, DataFieldType, DataFrame,
    DataPipelineEtlEngine, DataQueryEngine, DataValue, ParquetArrowDataEngine,
    DataVisualizationEngine,
};

pub use simple_scan::{
    SaneScanOptions, SaneScannerDevice, ScanColorMode, ScanExportFormat, ScanSource, ScannedPage,
    SovereignSimpleScanEngine,
};

pub use display_manager::{DMError, DisplayManager, Session, SessionType, User};
pub use sigmatools::{
    AccessibilityFeature, ClusterNode, NodeState, SigmaAccess, SigmaCluster, SigmaDeploy,
    SigmaIdentity, SigmaToolError, SovereignAptDuo, SovereignDpkgEtcher, SovereignImageToDataUri,
    SovereignImeConvertCase, SovereignIsWebsiteDown, SovereignKeyboardTester,
    SovereignTableConverter, SovereignTextFixer, SovereignWordCounter, UserIdentity,
};
pub use sovereign_commands::{
    FilesystemSpaceInfo, KernelDmesgEntry, ProcessTaskMetrics, SovereignBsdSysctl,
    SovereignDevDmesg, SovereignDfDu, SovereignGccToolchain, SovereignInitramfsSystemd,
    SovereignOpenBsdDoas, SovereignSudo, SovereignTopHtop,
};

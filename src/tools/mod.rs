// SigmaOS Tools Module - Real implementations of system utilities
pub mod system_monitor;

pub mod data_tools;
pub mod display_manager;
pub mod sigmatools;
pub mod simple_scan;
pub mod sovereign_commands;
pub mod open_source_tools_engine;
pub mod open_source_cli_tools_suite;
pub mod open_source_tools_parity;

pub use open_source_tools_engine::{
    BatSyntaxHighlighterEngine, FzfFuzzyFinderEngine, FzfSearchResult, HtopProcessEntry,
    HtopProcessMonitorEngine, HtopSortField, RsyncBlockChecksum, RsyncDeltaOp, RsyncDeltaSyncEngine,
};

pub use open_source_cli_tools_suite::{
    EzaFormattedEntry, EzaLsEngine, FdFileEntry, FdFileSearchEngine, RipgrepMatchResult,
    RipgrepSearchEngine, ZoxideCdEngine, ZoxideEntry,
};

pub use open_source_tools_parity::{
    DufDeviceUsage, DufDiskUsageEngine, DustDirectoryTreeEngine, DustNode, FastfetchSysinfo,
    ItsFossFastfetchSysinfoEngine, SimpleJqJsonQueryEngine,
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
pub mod dependency_reduction;
pub use dependency_reduction::*;

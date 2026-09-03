#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
// SPDX-License-Identifier: MIT

// SigmaOS Storage Module
// Database engines, block storage systems, and data management

pub mod block;
pub mod block_device_extended;
pub mod cinder;
pub mod geom;
pub mod nosql_engine;
pub mod search;
pub mod sovereign_disk_manager;
pub mod sql_engine;
pub mod volume;

pub use block::{
    BlockError, BlockKind, BlockOpCode, BlockOperationEngine, BlockOrientedDevice,
    BlockingStrategy, DeviceClass, NvmeBlockDevice, RecordBlockingEngine, SimpleBlockDevice,
    SovereignBlockClassifier, SsdBlockDevice, SystemBlockDiagramEngine,
};

pub use cinder::{
    CinderBackup, CinderBlockStorageEngine, CinderProvisioningType, CinderQosPolicy,
    CinderSnapshot, CinderVolume, CinderVolumeState, CinderVolumeType,
};

pub use geom::{
    BioCmd, BioRequest, GeomClassType, GeomConsumer, GeomEliConfig, GeomProvider, GeomTopology,
    PartitionEntry,
};

pub use sql_engine::{
    Column, QueryResult, SqlEngine, SqlType, SqlValue, Table, Transaction, TransactionState,
};

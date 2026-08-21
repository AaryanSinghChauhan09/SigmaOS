#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

pub mod alpc;
pub mod ipc;
pub mod mechanism;
pub mod message;
pub mod unix_socket;
pub mod signals;
pub mod async_io;

pub use alpc::{
    AlpcFacility, AlpcFacilityServer, AlpcManager, AlpcMessage, AlpcMessageHeader, AlpcPort,
    AlpcPortType, AlpcSectionHandle, alpc_flags,
};

pub use ipc::{
    IPCEndpoint, IPCError, IPCType, IPCInfo, IPCCapability,
    Pipe, MessageQueue, SharedMemory, IPCManager,
    SerenityIpcMessage, SerenitySharedBackingStore, SerenityIpcSandboxEnforcer,
};

pub use unix_socket::{
    UnixSocketType, UnixSocketAddress, UnixSocketState, UnixSocket, UnixSocketManager,
};

pub use signals::{
    SignalType, SignalDisposition, PendingSignal, ProcessSignalState, SignalDeliverySystem,
};

pub use async_io::{
    AsyncIoRingEngine, CompletionQueueEntry, IoOpCode, SubmissionQueueEntry,
};

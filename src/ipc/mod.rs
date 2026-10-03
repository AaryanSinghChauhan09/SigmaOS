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

pub mod async_io;
pub mod bus;
pub mod helenos_async;
pub mod ipc;
pub mod ipc_namespace;
pub mod mechanism;
pub mod message;
pub mod pipe;
pub mod signals;
pub mod sovereign_async_procedure_call;
pub mod std_streams;
pub mod unix_socket;
pub use bus::{BusEndpoint, BusName, IpcBus, IpcMessage, MessageType};

pub use ipc::{
    IPCCapability, IPCEndpoint, IPCError, IPCInfo, IPCManager, IPCType, MessageQueue, Pipe,
    SerenityIpcMessage, SerenityIpcSandboxEnforcer, SerenitySharedBackingStore, SharedMemory,
};

pub use unix_socket::{
    UnixSocket, UnixSocketAddress, UnixSocketManager, UnixSocketState, UnixSocketType,
};

pub use signals::{
    PendingSignal, ProcessSignalState, SignalDeliverySystem, SignalDisposition, SignalType,
};

pub use async_io::{
    AsyncIoRingEngine, CompletionQueueEntry, IoOpCode, KqueueAioFilter,
    LinuxBsdUniversalIoSubsystemEngine, OpenBsdIoPledgeRights, PosixAioControlBlock,
    SubmissionQueueEntry, IORING_SETUP_CQSIZE, IORING_SETUP_IOPOLL, IORING_SETUP_SQPOLL,
    IORING_SETUP_SQ_AFF,
};

pub use std_streams::{
    StandardStreamController, StandardStreamHandle, StreamBufferMode, StreamTeeSpliceRouter,
    STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO,
};

pub use ipc_namespace::{
    IpcNamespace, IpcNamespaceStats, IpcObjectId, IpcObjectRegistry, IpcObjectType, MessageQueueId,
    MessageQueueObject, SemaphoreId, SemaphoreObject, SharedMemoryId, SharedMemoryObject,
};

pub use sovereign_async_procedure_call::SovereignAsyncProcedureCallEngine;

pub use helenos_async::{HelenAsyncSystem, HelenIpcError, HelenMessage};

pub mod dbus_sovereign;
pub use dbus_sovereign::{
    DbusMatchRule, DbusMessage, DbusMessageType, DbusName, DbusService, DbusValue, SovereignDbusBus,
};

pub mod tech_media_std_streams_synthesis;
pub use tech_media_std_streams_synthesis::{
    AnsiStreamColorizerEngine, LogLevel, PqcEncryptedStreamMultiplexerEngine,
    SovereignTechMediaStdStreamsSuite, StreamColorLevel, StructuredJsonLogStreamFormatter,
    ZeroCopySpliceTeeStreamEngine,
};
pub mod binder_ring_buffer;
pub mod zircon_channels;
pub mod plan9_9p2000;
pub mod posix_mq;
pub mod sysv_ipc;

pub use posix_mq::{MessageQueue as PosixMessageQueue, MqAttr, MqError, MqRegistry};
pub use sysv_ipc::{
    IpcPerm, IpcError, ShmRegistry, ShmSegment, ShmidDs,
    SemRegistry, SemSet, SemidDs, SemBuf,
    MsgRegistry, MsgQueue as SysvMsgQueue, MsqidDs,
    IPC_CREAT, IPC_EXCL, IPC_NOWAIT, IPC_PRIVATE,
};

#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
pub mod hal;
pub mod portability;
pub mod cpu_sys;
pub mod cpu_features;
pub mod comprehensive;
pub mod arm_bx_formats;

pub use hal::*;
pub use cpu_sys::{SegmentType, GdtDescriptor, IdtGate, VirtualMemoryRegion, ProcessorInitSuite, FastSyscallDispatcher};
pub use cpu_features::{
    CpuInstructionExtension, SovereignCompilerOptimizer, SovereignRegisterSet,
    SovereignCpuRegisters, SovereignX86Topology, SovereignXcr0State,
};
pub use comprehensive::{
    PageTableEntry, MultiLevelPaging, ArmExceptionLevel as CompArmExceptionLevel, ArmV8ProcessorState,
    NtMajorFunction, IoStatusBlock, IoRequestPacket, ObjectType as NtObjectType,
    ObjectHeader as NtObjectHeader, ObjectManager as NtObjectManager,
    TaskState as LinuxTaskState, TaskStruct as LinuxTaskStruct, RcuSynchronizer as LinuxRcuSynchronizer,
    KqueueFilter as BsdKqueueFilter, Kevent as BsdKevent, KqueueMultiplexer as BsdKqueueMultiplexer,
    SysctlNode as BsdSysctlNode, SysctlRegistry as BsdSysctlRegistry,
    SovereignIsaArchitecture, X86Registers, X64Registers, AArch64Registers, Riscv64Registers,
    LoongArch64Registers, SovereignRegisterContext,
};

pub use arm_bx_formats::{
    ArmBxBranchExchangeDecoder, ArmExecutionState, BranchExchangeType, DecodedBxInstruction,
    DataEndianness, SovereignBytecodeEncryptor, SovereignDataWordFormatter, WordWidth,
};

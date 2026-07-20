// SigmaOS Kernel Module — Phase K: IPv4 stack, TCP, Block I/O, Page Cache, Crypto
#![allow(dead_code, unused_imports, clippy::all)]

// ── Core kernel primitives ─────────────────────────────────────────────────
pub mod ipc;
pub mod memory;
pub mod roundrobin;
pub mod scheduler;

// ── Phase J: subsystem registry & legacy device drivers ───────────────────
pub mod drivers;
pub mod subsystems;

// ── Phase J: process management (fork/exec/signals/namespaces/cgroups) ────
pub mod proc;

// ── Phase J: advanced memory management (SLAB/vmalloc/HugePages/OOM/NUMA) ─
pub mod mm;

// ── Phase J: kernel filesystems (proc_fs / sysfs / devtmpfs) ─────────────
pub mod fs;

// ── Phase J: interrupt infrastructure (IRQ domain / softirq / workqueue) ──
pub mod irq;

// ── Phase J: power management (CPUfreq / thermal / suspend-resume) ─────────
pub mod power;

// ── Phase J: network socket layer / netfilter / traffic control ────────────
pub mod net;

// ── Phase K: block device I/O layer (bio, elevator, blk-mq) ───────────────
pub mod block_dev;

// ── Phase K: crypto subsystem (SHA-256, HMAC, AES, PBKDF2, CSPRNG) ────────
pub mod crypto;

// ── Phase K: syscall table (POSIX-300 + SigmaOS extensions) ─────────────
pub mod syscall;

// ── Core re-exports ────────────────────────────────────────────────────────
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};

// ── Phase J + K consolidated re-exports ────────────────────────────────────
pub use proc::{
    CgroupManager, Namespace, NamespaceManager, NamespaceType, ProcessLifecycleManager,
    ResourceLimits, Signal, SignalHandler, SignalManager,
};
// mm: single export covering both Phase J and Phase K additions
pub use fs::{DevTmpFs, DeviceClass, ProcFileSystem, SysfsTree};
pub use irq::{
    ControllerType, IRQController, IRQHandler, IrqDomain, SoftirqEngine, SoftirqType, Work,
    Workqueue,
};
pub use mm::{
    CachedPage, HugePageManager, HugePageSize, NumaNode, NumaTopologyManager, OomKiller, PageCache,
    PageStatus, SlabAllocator, VmallocManager,
};
pub use power::{
    CpufreqGovernor, CpufreqManager, CpufreqPolicy, PowerStateManager, SleepState, ThermalManager,
    ThermalZone,
};
// net: single export covering Phase J (socket/netfilter/tc) + Phase K (IPv4/TCP)
pub use block_dev::{
    Bio, BioOp, BlockDeviceManager, DeadlineScheduler, RamDisk, BLOCK_SIZE, SECTOR_SIZE,
};
pub use crypto::{
    hmac_sha256, pbkdf2_hmac_sha256, sha256, Aes128, CryptoAlgorithm, CryptoEngine, SigmaCsprng,
};
pub use net::{
    AddressFamily, ArpTable, CongestionAlgorithm, Ipv4Header, Ipv4Stack, NetfilterTable,
    NfHookpoint, NfRule, NfVerdict, Pfifo, PfifoFast, Protocol, QPacket, Route, RoutingTable, Sfq,
    SockAddrIn, SocketLayer, SocketType, Tbf, TcpConnection, TcpSegment, TcpState,
};
pub use syscall::{SyscallArgs, SyscallError, SyscallNr, SyscallResult, SyscallTable};

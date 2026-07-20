// SigmaOS Kernel Module — Phase K: IPv4 stack, TCP, Block I/O, Page Cache, Crypto
#![allow(dead_code, unused_imports, clippy::all)]

// ── Core kernel primitives ─────────────────────────────────────────────────
pub mod ipc;
pub mod memory;
pub mod numa_allocator;
pub mod roundrobin;
pub mod scheduler;
pub mod secure_free;
pub mod slab_allocator;

// ── Phase J: subsystem registry & legacy device drivers ───────────────────
pub mod subsystems;
pub mod drivers;

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
pub use numa_allocator::{AllocationPolicy, NumaAllocator, NumaNode, NodeState};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};
pub use secure_free::{SanitizationLevel, SecureFreeDetector, SecureFreeStats};
pub use slab_allocator::{SlabAllocator, SlabCache, SlabCacheStats, SlabState};

// ── Phase J + K consolidated re-exports ────────────────────────────────────
pub use proc::{ProcessLifecycleManager, Signal, SignalHandler, SignalManager,
               Namespace, NamespaceType, NamespaceManager,
               ResourceLimits, CgroupManager};
// mm: single export covering both Phase J and Phase K additions
pub use mm::{SlabAllocator, VmallocManager, HugePageManager, HugePageSize,
             OomKiller, NumaTopologyManager, NumaNode,
             PageCache, CachedPage, PageStatus};
pub use fs::{ProcFileSystem, SysfsTree, DevTmpFs, DeviceClass};
pub use irq::{IRQController, IRQHandler, ControllerType, IrqDomain,
              SoftirqEngine, SoftirqType, Workqueue, Work};
pub use power::{CpufreqPolicy, CpufreqGovernor, CpufreqManager,
                ThermalZone, ThermalManager, PowerStateManager, SleepState};
// net: single export covering Phase J (socket/netfilter/tc) + Phase K (IPv4/TCP)
pub use net::{SocketLayer, SockAddrIn, AddressFamily, SocketType, Protocol,
              NetfilterTable, NfRule, NfVerdict, NfHookpoint,
              Pfifo, PfifoFast, Sfq, Tbf, QPacket,
              Ipv4Stack, Ipv4Header, ArpTable, RoutingTable, Route,
              TcpConnection, TcpSegment, TcpState, CongestionAlgorithm};
pub use block_dev::{BlockDeviceManager, RamDisk, Bio, BioOp, DeadlineScheduler,
                    SECTOR_SIZE, BLOCK_SIZE};
pub use crypto::{sha256, hmac_sha256, pbkdf2_hmac_sha256, Aes128,
                 SigmaCsprng, CryptoEngine, CryptoAlgorithm};
pub use syscall::{SyscallTable, SyscallArgs, SyscallResult, SyscallError, SyscallNr};

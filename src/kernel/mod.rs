// SigmaOS Kernel Module — Phase J: Heritage absorption complete
#![allow(dead_code, unused_imports, clippy::all)]

// ── Core kernel primitives ─────────────────────────────────────────────────
pub mod ipc;
pub mod memory;
pub mod roundrobin;
pub mod scheduler;

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

// ── Core re-exports ────────────────────────────────────────────────────────
pub use ipc::{Channel, IpcError, IpcManager, Message};
pub use memory::{BuddyAllocator, MemoryBlock, PAGE_SIZE};
pub use roundrobin::{RoundRobinConfig, RoundRobinScheduler, SchedulerError};
pub use scheduler::{Priority, Process, ProcessState, Scheduler};

// ── Phase J selective re-exports ──────────────────────────────────────────
pub use proc::{ProcessLifecycleManager, Signal, SignalHandler, SignalManager,
               Namespace, NamespaceType, NamespaceManager,
               ResourceLimits, CgroupManager};
pub use mm::{SlabAllocator, VmallocManager, HugePageManager, HugePageSize,
             OomKiller, NumaTopologyManager, NumaNode};
pub use fs::{ProcFileSystem, SysfsTree, DevTmpFs, DeviceClass};
pub use irq::{IRQController, IRQHandler, ControllerType, IrqDomain, SoftirqEngine, SoftirqType, Workqueue, Work};
pub use power::{CpufreqPolicy, CpufreqGovernor, CpufreqManager,
                ThermalZone, ThermalManager, PowerStateManager, SleepState};
pub use net::{SocketLayer, SockAddrIn, AddressFamily, SocketType, Protocol,
              NetfilterTable, NfRule, NfVerdict, NfHookpoint,
              Pfifo, PfifoFast, Sfq, Tbf, QPacket};

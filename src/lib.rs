// SigmaOS Library
// Core library for SigmaOS operating system

pub mod security;
pub mod sigpkg;
pub mod kernel;
pub mod network;
pub mod filesystem;
pub mod drivers;
pub mod shell;

pub use security::{CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise};
pub use sigpkg::{SatSolver, ContentAddressedStore, CryptoVerifier, Transaction, PackageRecipe, BuildSystem, RecipeManager, RecipeError};
pub use kernel::{Scheduler, Process, Priority, ProcessState, BuddyAllocator, MemoryBlock, PAGE_SIZE, IpcManager, Channel, Message, IpcError, RoundRobinScheduler, RoundRobinConfig, SchedulerError};
pub use network::{TcpStack, TcpConnection, TcpSegment, TcpState, TcpError};
pub use filesystem::{VirtualFilesystem, Inode, FileDescriptor, FileType, FilePermissions, FsError};
pub use drivers::{GpuDriver, GpuCommand, GpuError, StorageDriver, StorageCommand, StorageType, StorageError, NetworkDriver, NetworkCommand, NetworkType, NetworkError, InputDriver, InputEvent, InputType, UsbHidDriver, HidKeyboardEvent, HidReportType, HidError, VesaDriver, VesaModeInfo, VesaError};
pub use shell::{ShellRepl, ShellCommand};

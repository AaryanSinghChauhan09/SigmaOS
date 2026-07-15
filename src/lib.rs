// SigmaOS Library
// Core library for SigmaOS operating system

pub mod security;
pub mod sigpkg;
pub mod kernel;
pub mod network;
pub mod filesystem;
pub mod drivers;
pub mod shell;
pub mod dashboard;
pub mod accessibility;
pub mod customization;
pub mod automation;
pub mod resilience;
pub mod productivity;
pub mod orchestration;

pub use security::{CapabilityGate, CapabilityToken, Permission, PledgeManager, PledgePromise};
pub use sigpkg::{SatSolver, ContentAddressedStore, CryptoVerifier, Transaction, PackageRecipe, BuildSystem, RecipeManager, RecipeError};
pub use kernel::{Scheduler, Process, Priority, ProcessState, BuddyAllocator, MemoryBlock, PAGE_SIZE, IpcManager, Channel, Message, IpcError, RoundRobinScheduler, RoundRobinConfig, SchedulerError};
pub use network::{TcpStack, TcpConnection, TcpSegment, TcpState, TcpError};
pub use filesystem::{VirtualFilesystem, Inode, FileDescriptor, FileType, FilePermissions, FsError};
pub use drivers::{GpuDriver, GpuCommand, GpuError, StorageDriver, StorageCommand, StorageType, StorageError, NetworkDriver, NetworkCommand, NetworkType, NetworkError, InputDriver, InputEvent, InputType, UsbHidDriver, HidKeyboardEvent, HidReportType, HidError, VesaDriver, VesaModeInfo, VesaError};
pub use shell::{ShellRepl, ShellCommand};
pub use dashboard::{UnifiedDashboard, DashboardWidget, MetricData, MetricType, WidgetType, SystemMonitor};
pub use accessibility::{AccessibilityFramework, AccessibilityProfile, AccessibilitySetting, AccessibilityCategory, AccessibilityFeature, AccessibilityError};
pub use customization::{CustomizationEngine, Routine, Condition, Action, Theme, TriggerType, CustomizationError};
pub use automation::{AiOptimizer, OptimizationRecommendation, SystemState, OptimizationCategory, OptimizationError};
pub use resilience::{SelfHealingModule, RecoveryRule, RecoveryAction, SystemSnapshot, RecoveryEventType, ResilienceError};
pub use productivity::{GamifiedProductivity, Achievement, Goal, PomodoroTimer, ProductivityScore, AchievementType, PomodoroState};
pub use orchestration::{CrossDeviceOrchestrator, ConnectedDevice, SmartHomeDevice, AutomationRule, DeviceType, ConnectionStatus, DeviceCapability, AutomationTrigger, CrossDeviceAction, OrchestrationError};

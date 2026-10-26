# SigmaOS Component Boundaries and Dependency Contracts

## Overview
This document defines clear boundaries between SigmaOS repositories and establishes contracts for inter-repository communication. Following **INTERFACE SEGREGATION**, **DEPENDENCY INVERSION**, and **LOOSE COUPLING** principles.

## Core Principles

### 1. **INTERFACE SEGREGATION**
- Each repository exposes minimal, focused interfaces
- No repository depends on internal implementation details of another
- Public APIs are versioned and stable

### 2. **DEPENDENCY INVERSION**
- High-level repositories depend on abstractions (traits), not concretions
- Kernel provides trait definitions that userland implements
- Dependencies flow toward stable, foundational components

### 3. **LOOSE COUPLING**
- Communication via well-defined contracts (traits, protocols)
- Event-driven architecture where possible
- Minimal direct imports between repositories

### 4. **CLEAR ABSTRACTION**
- Each component has a single, well-defined responsibility
- Abstractions match domain concepts, not implementation details
- Hide complexity behind simple interfaces

## Repository Hierarchy

```
Tier 0: Foundation (No dependencies)
├── SigmaOS-kernel
├── SigmaOS-klib
└── SigmaOS-wiki

Tier 1: Core Services (Depend on Tier 0)
├── SigmaOS-iso
├── SigmaOS-package
├── SigmaOS-drivers
├── SigmaOS-security
├── SigmaOS-network
└── SigmaOS-filesystem

Tier 2: Desktop Platform (Depend on Tier 0-1)
├── SigmaOS-desktop-environment
├── SigmaOS-window-management-library
└── SigmaOS-theme-desktop-environment

Tier 3: User Applications (Depend on Tier 0-2)
├── SigmaOS-software-manager
├── SigmaOS-file-manager-system
├── SigmaOS-terminal
├── SigmaOS-screen-recorder
├── SigmaOS-screen-shot
├── SigmaOS-player
├── SigmaOS-photo-viewer
└── SigmaOS-draw

Tier 4: System Utilities (Depend on Tier 0-2)
├── SigmaOS-update
├── SigmaOS-backup-tool
├── SigmaOS-stick
└── SigmaOS-drivers

Tier 5: Specialized Components (Depend on Tier 0-3)
├── SigmaOS-accessibility
├── SigmaOS-office
├── SigmaOS-dev-tools
├── SigmaOS-term
├── SigmaOS-theme-icon
├── SigmaOS-theme-cursor
└── SigmaOS-wallpaper-engine

Tier 6: Governance (Orchestrates all)
└── SigmaOS (main)
```

## Contract Definitions

### Kernel Contracts (SigmaOS-kernel)

#### Syscall Interface
```rust
// SigmaOS-kernel/src/syscall/interface.rs
pub trait SyscallDispatcher {
    /// Dispatch a system call with given number and arguments
    fn dispatch(&self, num: u64, args: &[u64]) -> Result<u64, SyscallError>;

    /// Register a syscall handler
    fn register_handler(&mut self, num: u64, handler: Box<dyn SyscallHandler>);
}

pub trait SyscallHandler {
    fn handle(&self, args: &[u64]) -> Result<u64, SyscallError>;
}
```

#### Memory Management Interface
```rust
// SigmaOS-kernel/src/memory/interface.rs
pub trait PhysicalMemoryManager {
    /// Allocate a physical page frame
    fn alloc_frame(&mut self) -> Result<usize, MemoryError>;

    /// Free a physical page frame
    fn free_frame(&mut self, frame: usize) -> Result<(), MemoryError>;

    /// Query available memory
    fn available_frames(&self) -> usize;
}

pub trait VirtualMemoryManager {
    /// Map a virtual page to a physical frame
    fn map_page(&mut self, virt: usize, phys: usize, flags: PageFlags)
        -> Result<(), MemoryError>;

    /// Unmap a virtual page
    fn unmap_page(&mut self, virt: usize) -> Result<(), MemoryError>;
}
```

#### Process Management Interface
```rust
// SigmaOS-kernel/src/process/interface.rs
pub trait ProcessManager {
    /// Create a new process
    fn create_process(&mut self, binary: &str) -> Result<Pid, ProcessError>;

    /// Schedule a process for execution
    fn schedule(&mut self, pid: Pid) -> Result<(), ProcessError>;

    /// Terminate a process
    fn terminate(&mut self, pid: Pid) -> Result<(), ProcessError>;

    /// Query process state
    fn get_state(&self, pid: Pid) -> Option<ProcessState>;
}
```

#### Scheduler Interface
```rust
// SigmaOS-kernel/src/scheduler/interface.rs
pub trait Scheduler {
    /// Add a task to the run queue
    fn enqueue(&mut self, task: Task) -> Result<(), SchedulerError>;

    /// Pick the next task to run
    fn pick_next(&mut self) -> Option<Task>;

    /// Preempt the current task
    fn preempt(&mut self, task: Task) -> Result<(), SchedulerError>;

    /// Get scheduler statistics
    fn stats(&self) -> SchedulerStats;
}
```

#### Interrupt Handling Interface
```rust
// SigmaOS-kernel/src/interrupt/interface.rs
pub trait InterruptController {
    /// Register an interrupt handler
    fn register_handler(&mut self, vector: u8, handler: Box<dyn InterruptHandler>);

    /// Enable an interrupt line
    fn enable(&mut self, vector: u8);

    /// Disable an interrupt line
    fn disable(&mut self, vector: u8);

    /// Send end-of-interrupt signal
    fn eoi(&mut self, vector: u8);
}

pub trait InterruptHandler {
    fn handle(&mut self, frame: &InterruptFrame);
}
```

### Package Management Contracts (SigmaOS-package)

#### Package Backend Interface
```rust
// SigmaOS-package/src/backend/interface.rs
pub trait PackageBackend {
    /// Search for packages matching query
    fn search(&self, query: &str) -> Vec<PackageInfo>;

    /// Install a package
    fn install(&self, pkg: &PackageInfo) -> Result<(), PackageError>;

    /// Remove a package
    fn remove(&self, pkg_name: &str) -> Result<(), PackageError>;

    /// Check if package is installed
    fn is_installed(&self, pkg_name: &str) -> bool;

    /// List installed packages
    fn list_installed(&self) -> Vec<PackageInfo>;

    /// Update package database
    fn update(&self) -> Result<(), PackageError>;

    /// Upgrade all packages
    fn upgrade(&self) -> Result<(), PackageError>;
}
```

#### Dependency Resolution Interface
```rust
// SigmaOS-package/src/resolver/interface.rs
pub trait DependencyResolver {
    /// Resolve dependencies for a package
    fn resolve(&self, pkg: &PackageInfo) -> Result<Vec<PackageInfo>, ResolveError>;

    /// Check for conflicts
    fn check_conflicts(&self, packages: &[PackageInfo]) -> Result<(), ResolveError>;

    /// Find installation order
    fn install_order(&self, packages: &[PackageInfo]) -> Vec<PackageInfo>;
}
```

### Desktop Environment Contracts (SigmaOS-desktop-environment)

#### Window Management Interface
```rust
// SigmaOS-desktop-environment/src/window/interface.rs
pub trait WindowManager {
    /// Create a new window
    fn create_window(&mut self, props: WindowProperties) -> Result<WindowId, WMError>;

    /// Destroy a window
    fn destroy_window(&mut self, id: WindowId) -> Result<(), WMError>;

    /// Focus a window
    fn focus_window(&mut self, id: WindowId) -> Result<(), WMError>;

    /// Move a window
    fn move_window(&mut self, id: WindowId, x: i32, y: i32) -> Result<(), WMError>;

    /// Resize a window
    fn resize_window(&mut self, id: WindowId, w: u32, h: u32) -> Result<(), WMError>;

    /// Minimize a window
    fn minimize_window(&mut self, id: WindowId) -> Result<(), WMError>;

    /// Maximize a window
    fn maximize_window(&mut self, id: WindowId) -> Result<(), WMError>;

    /// Get window list
    fn list_windows(&self) -> Vec<WindowInfo>;
}
```

#### Panel/Dock Interface
```rust
// SigmaOS-desktop-environment/src/panel/interface.rs
pub trait Panel {
    /// Add an applet to the panel
    fn add_applet(&mut self, applet: Box<dyn Applet>) -> Result<(), PanelError>;

    /// Remove an applet from the panel
    fn remove_applet(&mut self, id: AppletId) -> Result<(), PanelError>;

    /// Set panel position
    fn set_position(&mut self, position: PanelPosition) -> Result<(), PanelError>;

    /// Set panel size
    fn set_size(&mut self, size: u32) -> Result<(), PanelError>;
}

pub trait Applet {
    fn render(&self) -> AppletRender;
    fn handle_event(&mut self, event: AppletEvent);
}
```

### Screen Subsystem Contracts

#### Screenshot Interface
```rust
// SigmaOS-screen-shot/src/interface.rs
pub trait ScreenshotCapture {
    /// Capture fullscreen
    fn capture_fullscreen(&self) -> Result<Image, CaptureError>;

    /// Capture window
    fn capture_window(&self, window_id: WindowId) -> Result<Image, CaptureError>;

    /// Capture region
    fn capture_region(&self, x: u32, y: u32, w: u32, h: u32)
        -> Result<Image, CaptureError>;

    /// Save image to file
    fn save(&self, image: &Image, path: &str, format: ImageFormat)
        -> Result<(), CaptureError>;
}
```

#### Screen Recorder Interface
```rust
// SigmaOS-screen-recorder/src/interface.rs
pub trait ScreenRecorder {
    /// Start recording
    fn start(&mut self, config: RecordConfig) -> Result<(), RecordError>;

    /// Stop recording
    fn stop(&mut self) -> Result<String, RecordError>;

    /// Pause recording
    fn pause(&mut self) -> Result<(), RecordError>;

    /// Resume recording
    fn resume(&mut self) -> Result<(), RecordError>;

    /// Get recording state
    fn state(&self) -> RecorderState;
}
```

### ISO Builder Contracts

#### ISO Build Interface
```rust
// SigmaOS-iso/src/builder/interface.rs
pub trait IsoBuilder {
    /// Configure ISO build
    fn configure(&mut self, config: IsoConfig) -> Result<(), IsoError>;

    /// Validate configuration
    fn validate(&self) -> Result<(), IsoError>;

    /// Build ISO image
    fn build(&mut self) -> Result<PathBuf, IsoError>;

    /// Estimate ISO size
    fn estimate_size(&self) -> u64;

    /// Get build plan
    fn build_plan(&self) -> Vec<BuildStep>;
}
```

### File System Contracts

#### VFS Interface
```rust
// SigmaOS-filesystem/src/vfs/interface.rs
pub trait FileSystem {
    /// Open a file
    fn open(&self, path: &str, flags: OpenFlags) -> Result<FileHandle, FsError>;

    /// Close a file
    fn close(&self, handle: FileHandle) -> Result<(), FsError>;

    /// Read from file
    fn read(&self, handle: FileHandle, buf: &mut [u8]) -> Result<usize, FsError>;

    /// Write to file
    fn write(&self, handle: FileHandle, buf: &[u8]) -> Result<usize, FsError>;

    /// Create directory
    fn mkdir(&self, path: &str) -> Result<(), FsError>;

    /// Remove file/directory
    fn remove(&self, path: &str) -> Result<(), FsError>;

    /// List directory
    fn list(&self, path: &str) -> Result<Vec<DirEntry>, FsError>;

    /// Get file metadata
    fn metadata(&self, path: &str) -> Result<FileMetadata, FsError>;
}
```

### Network Contracts

#### Network Interface
```rust
// SigmaOS-network/src/interface.rs
pub trait NetworkInterface {
    /// Send packet
    fn send(&self, packet: &[u8]) -> Result<(), NetworkError>;

    /// Receive packet
    fn receive(&self) -> Result<Vec<u8>, NetworkError>;

    /// Get interface status
    fn status(&self) -> InterfaceStatus;

    /// Configure interface
    fn configure(&mut self, config: InterfaceConfig) -> Result<(), NetworkError>;
}
```

### Security Contracts

#### Access Control Interface
```rust
// SigmaOS-security/src/interface.rs
pub trait AccessController {
    /// Check if operation is permitted
    fn check_permission(&self, subject: &Subject, operation: &Operation, object: &Object)
        -> bool;

    /// Grant permission
    fn grant(&mut self, subject: &Subject, permission: Permission)
        -> Result<(), SecurityError>;

    /// Revoke permission
    fn revoke(&mut self, subject: &Subject, permission: Permission)
        -> Result<(), SecurityError>;
}
```

## Dependency Resolution Strategy

### Version Semantics
- **Major.Minor.Patch** (Semantic Versioning)
- Major: Breaking changes to public API
- Minor: Additions to public API (backward compatible)
- Patch: Bug fixes (backward compatible)

### Dependency Declaration
```toml
# In Cargo.toml for dependent repositories
[dependencies]
sigmaos-kernel = { version = "0.1", path = "../SigmaOS-kernel" }
sigmaos-klib = { version = "0.1", path = "../SigmaOS-klib" }
```

### API Stability Guarantees
- **Tier 0 (Foundation)**: APIs stable for 12 months
- **Tier 1 (Core Services)**: APIs stable for 6 months
- **Tier 2 (Desktop Platform)**: APIs stable for 3 months
- **Tier 3+ (Applications)**: APIs may change frequently

### Breaking Change Process
1. Deprecate old API (with warning) for 1 release cycle
2. Introduce new API alongside old API
3. Remove old API in next major version
4. Update dependent repositories
5. Coordinate release across all affected repos

## Communication Patterns

### Synchronous Communication
- Direct function calls via trait interfaces
- Used for: kernel syscalls, package operations, file I/O
- Pros: Simple, predictable
- Cons: Can block, tight coupling

### Asynchronous Communication
- Event-based messaging, channels
- Used for: desktop events, network I/O, notifications
- Pros: Non-blocking, loose coupling
- Cons: More complex, harder to debug

### RPC-style Communication
- Remote procedure calls between processes
- Used for: D-Bus services, cross-process communication
- Pros: Process isolation, language-agnostic
- Cons: Overhead, serialization

## Data Flow Between Repositories

### Boot Flow
```
SigmaOS-iso (builds ISO)
  → SigmaOS-kernel (bootstraps)
    → SigmaOS-drivers (load hardware)
      → SigmaOS-filesystem (mount root)
        → SigmaOS-package (install userland)
          → SigmaOS-desktop-environment (start GUI)
```

### Desktop Application Launch
```
SigmaOS-desktop-environment (launcher)
  → SigmaOS-file-manager-system (file dialog)
    → SigmaOS-filesystem (read files)
  → SigmaOS-software-manager (install)
    → SigmaOS-package (package operations)
```

### System Update Flow
```
SigmaOS-update (trigger)
  → SigmaOS-package (check updates)
    → SigmaOS-kernel (kernel updates)
    → SigmaOS-drivers (driver updates)
  → SigmaOS-backup-tool (pre-update snapshot)
```

## Testing Strategy

### Unit Tests (Per Repository)
- Test internal logic without external dependencies
- Mock external interfaces
- Fast execution (< 1 second per test)

### Integration Tests (Cross-Repository)
- Test contracts between repositories
- Use real implementations where feasible
- Slower execution (1-10 seconds per test)

### Contract Tests
- Verify API compatibility across versions
- Test that implementations adhere to trait contracts
- Run on every PR to API-defining repositories

### End-to-End Tests
- Test complete user workflows
- Run in SigmaOS (main governance repo)
- Slowest execution (10-60 seconds per test)

## Error Handling Strategy

### Error Type Hierarchy
```rust
// Base error trait for all SigmaOS errors
pub trait SigmaError: std::error::Error + Send + Sync {
    fn source_repository(&self) -> &'static str;
    fn is_recoverable(&self) -> bool;
}

// Repository-specific errors
pub struct KernelError { /* ... */ }
pub struct PackageError { /* ... */ }
pub struct DesktopError { /* ... */ }
```

### Error Propagation
- Convert lower-level errors to higher-level abstractions
- Preserve context (which repository, which operation)
- Use `anyhow` or `eyre` for error chains in application code

### Error Recovery
- Define recoverable vs unrecoverable errors
- Provide retry mechanisms for transient failures
- Graceful degradation when possible

## Performance Considerations

### Zero-Copy Interfaces
- Use shared memory where possible
- Pass references instead of copies
- Buffer pooling for high-throughput paths

### Lazy Initialization
- Delay expensive operations until needed
- Use `once_cell` or `lazy_static` for singletons
- Just-in-time compilation/linking

### Caching Strategy
- Cache frequently accessed data
- Invalidate cache on data changes
- Use LRU eviction for memory-constrained contexts

## Security Considerations

### Input Validation
- Validate all inputs at repository boundaries
- Use types to enforce invariants (newtype pattern)
- Sanitize untrusted data

### Privilege Separation
- Kernel runs with highest privileges
- Userland runs with minimal privileges
- Use capability-based security where possible

### Sandboxing
- Isolate untrusted code
- Use SigmaOS-sandbox for process confinement
- Restrict filesystem/network access

## Governance

### API Review Process
1. Proposed API change documented in RFC
2. Community review (2 weeks minimum)
3. SIG (Special Interest Group) approval
4. Implementation
5. Testing and validation
6. Release

### Backward Compatibility Policy
- Public APIs cannot break without major version bump
- Internal APIs may break at any time
- Document deprecated APIs with removal timeline

### Repository Ownership
- Each repository has designated maintainers
- Maintainers approve PRs to their repository
- Cross-repo changes require approval from all affected maintainers

## Success Metrics

### Code Quality
- Each repository has > 80% test coverage
- Zero high-severity security vulnerabilities
- Code complexity metrics within thresholds

### Build Performance
- Full build of all repositories < 30 minutes
- Incremental build < 2 minutes
- CI/CD pipeline < 10 minutes per PR

### Developer Experience
- Clear documentation for all public APIs
- Easy to add new repositories
- Minimal boilerplate for new components

### Release Cadence
- Kernel: Monthly LTS releases
- Core services: Bi-weekly releases
- Applications: Weekly releases
- Governance: As needed

## References

- SOLID Principles: https://en.wikipedia.org/wiki/SOLID
- Interface Segregation: https://en.wikipedia.org/wiki/Interface_segregation_principle
- Dependency Inversion: https://en.wikipedia.org/wiki/Dependency_inversion_principle
- Semantic Versioning: https://semver.org
- Rust API Guidelines: https://rust-lang.github.io/api-guidelines/

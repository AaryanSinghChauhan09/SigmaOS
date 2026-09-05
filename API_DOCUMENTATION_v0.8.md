# SigmaOS v0.8 API Documentation

Complete API reference for all 5 Tier 1 features.

---

## Table of Contents

1. [Namespaces API](#namespaces-api)
2. [File Monitoring API](#file-monitoring-api)
3. [Resource Limits API](#resource-limits-api)
4. [Security API](#security-api)
5. [Event System API](#event-system-api)
6. [Error Handling](#error-handling)

---

## Namespaces API

### Types

```rust
pub type NamespaceId = u64;
pub type ProcessId = u32;

pub enum NamespaceType {
    Pid,
    Ipc,
    Mount,
}
```

### Structures

```rust
pub struct Namespace {
    pub id: NamespaceId,
    pub ns_type: NamespaceType,
    pub parent_id: Option<NamespaceId>,
}

pub struct ProcessNamespaceContext {
    pub pid_ns: NamespaceId,
    pub ipc_ns: NamespaceId,
    pub mount_ns: NamespaceId,
}
```

### Functions

#### Create Namespace
```rust
fn create_pid_namespace(parent_id: Option<NamespaceId>) -> Result<NamespaceId, String>;
fn create_ipc_namespace(parent_id: Option<NamespaceId>) -> Result<NamespaceId, String>;
fn create_mount_namespace(parent_id: Option<NamespaceId>) -> Result<NamespaceId, String>;
```

#### Process Operations
```rust
fn sys_clone(flags: u32, fn_ptr: *mut c_void, child_stack: *mut c_void, arg: *mut c_void, ptid: *mut i32) -> Result<i32, String>;
fn sys_unshare(flags: u32) -> Result<(), String>;
fn sys_setns(fd: i32, flags: u32) -> Result<(), String>;
```

### Syscall Flags

```rust
const CLONE_NEWPID: u32 = 0x20000000;
const CLONE_NEWIPC: u32 = 0x08000000;
const CLONE_NEWNS: u32 = 0x00020000;
const CLONE_NEWUTS: u32 = 0x04000000;
const CLONE_NEWNET: u32 = 0x40000000;
const CLONE_NEWUSER: u32 = 0x10000000;
```

### Examples

```rust
// Create isolated PID namespace
let ns_id = create_pid_namespace(None)?;

// Clone process into namespace
let pid = sys_clone(CLONE_NEWPID, ...)?;

// Join existing namespace
sys_setns(fd, CLONE_NEWPID)?;

// Unshare (split from current namespace)
sys_unshare(CLONE_NEWPID)?;
```

---

## File Monitoring API

### Types

```rust
pub type WatchId = u64;
pub type EventId = u64;

#[repr(u32)]
pub enum FileEventType {
    Create = 1,
    Delete = 2,
    Modify = 3,
    Rename = 4,
    Close = 5,
    Open = 6,
    Move = 7,
}
```

### Structures

```rust
pub struct FileEvent {
    pub id: EventId,
    pub event_type: FileEventType,
    pub path: PathBuf,
    pub related_path: Option<PathBuf>,
    pub timestamp: u64,
    pub watch_id: WatchId,
}

pub struct WatchConfig {
    pub max_queue_size: usize,
    pub recursive: bool,
    pub filter: EventFilter,
}

pub struct EventFilter {
    pub event_types: Vec<FileEventType>,
    pub path_patterns: Vec<String>,
}
```

### Functions

#### inotify Syscalls
```rust
fn sys_inotify_init1(flags: u32) -> Result<i32, String>;
fn sys_inotify_add_watch(fd: i32, pathname: &str, mask: u32) -> Result<i32, String>;
fn sys_inotify_rm_watch(fd: i32, wd: i32) -> Result<(), String>;
fn read_inotify_events(fd: i32, buf: &mut [u8]) -> Result<usize, String>;
```

#### Watch Management
```rust
impl WatchManager {
    pub fn register_watch(&self, path: PathBuf, config: WatchConfig) -> Result<WatchId, String>;
    pub fn deregister_watch(&self, watch_id: WatchId) -> Result<bool, String>;
    pub fn add_event(&self, watch_id: WatchId, event_type: FileEventType, path: PathBuf) -> Result<EventId, String>;
    pub fn get_event(&self, watch_id: WatchId) -> Result<Option<FileEvent>, String>;
    pub fn peek_event(&self, watch_id: WatchId) -> Result<Option<FileEvent>, String>;
}
```

### inotify Flags

```rust
const IN_ACCESS: u32 = 1;
const IN_MODIFY: u32 = 2;
const IN_ATTRIB: u32 = 4;
const IN_CLOSE_WRITE: u32 = 8;
const IN_CLOSE_NOWRITE: u32 = 16;
const IN_OPEN: u32 = 32;
const IN_MOVED_FROM: u32 = 64;
const IN_MOVED_TO: u32 = 128;
const IN_CREATE: u32 = 256;
const IN_DELETE: u32 = 512;
const IN_DELETE_SELF: u32 = 1024;
const IN_MOVE_SELF: u32 = 2048;
const IN_UNMOUNT: u32 = 0x2000;
const IN_Q_OVERFLOW: u32 = 0x4000;
const IN_IGNORED: u32 = 0x8000;
```

### Examples

```rust
// Create inotify instance
let fd = sys_inotify_init1(IN_NONBLOCK)?;

// Add watch
let wd = sys_inotify_add_watch(fd, "/app/data", IN_MODIFY | IN_CREATE)?;

// Read events
let mut buf = vec![0u8; 4096];
let len = read_inotify_events(fd, &mut buf)?;

// Remove watch
sys_inotify_rm_watch(fd, wd)?;
```

---

## Resource Limits API

### Types

```rust
pub type CgroupId = u64;

pub enum ControllerType {
    Cpu = 1,
    Memory = 2,
    Io = 3,
    Pids = 4,
}

pub enum OomPolicy {
    Kill = 1,
    Signal = 2,
    Block = 3,
    Handler = 4,
}
```

### Structures

```rust
pub struct CgroupHierarchy { ... }

pub struct MemoryController {
    pub memory_limit: u64,
    pub memory_soft_limit: u64,
    pub memory_high: u64,
}

pub struct MemoryStats {
    pub rss: u64,
    pub vms: u64,
    pub page_cache: u64,
    pub swap: u64,
}
```

### Functions

#### Cgroup Management
```rust
impl CgroupHierarchy {
    pub fn create_cgroup(&self, path: PathBuf, parent_id: Option<CgroupId>) -> Result<CgroupId, String>;
    pub fn remove_cgroup(&self, cgroup_id: CgroupId) -> Result<(), String>;
    pub fn add_process_to_cgroup(&self, cgroup_id: CgroupId, process_id: u32) -> Result<(), String>;
    pub fn remove_process_from_cgroup(&self, cgroup_id: CgroupId, process_id: u32) -> Result<(), String>;
}
```

#### Resource Control
```rust
impl CgroupHierarchy {
    pub fn set_memory_limit(&self, cgroup_id: CgroupId, limit: u64) -> Result<(), String>;
    pub fn set_cpu_limit(&self, cgroup_id: CgroupId, quota_us: u64, period_us: u64) -> Result<(), String>;
    pub fn set_pids_limit(&self, cgroup_id: CgroupId, limit: u64) -> Result<(), String>;
}
```

#### Memory Tracking
```rust
impl MemoryController {
    pub fn register_process(&self, process_id: u32) -> Result<(), String>;
    pub fn set_process_limit(&self, process_id: u32, limit: u64) -> Result<(), String>;
    pub fn allocate_memory(&self, process_id: u32, size: u64) -> Result<(), String>;
    pub fn deallocate_memory(&self, process_id: u32, size: u64) -> Result<(), String>;
    pub fn check_oom(&self, process_id: u32) -> Result<bool, String>;
}
```

### Examples

```rust
// Create cgroup
let hierarchy = CgroupHierarchy::new()?;
let cg_id = hierarchy.create_cgroup(PathBuf::from("/app"), None)?;

// Set limits
hierarchy.set_memory_limit(cg_id, 512 * 1024 * 1024)?; // 512MB
hierarchy.set_cpu_limit(cg_id, 500_000, 100_000)?; // 500ms per 100ms period

// Add process
hierarchy.add_process_to_cgroup(cg_id, 1234)?;

// Monitor memory
let stats = memory_controller.get_process_stats(1234)?;
println!("RSS: {} KB", stats.rss / 1024);
```

---

## Security API

### Types

```rust
pub enum SeccompAction {
    Kill = 0,
    Trap = 1,
    Abort = 2,
    Errno = 3,
    Trace = 4,
    Allow = 5,
}

pub enum CompareOp {
    Equal = 0,
    NotEqual = 1,
    LessThan = 2,
    GreaterThan = 4,
    MaskedEqual = 6,
}

pub enum FilterType {
    Whitelist = 0,
    Blacklist = 1,
}
```

### Structures

```rust
pub struct FilterRule {
    pub syscall_nr: SyscallNumber,
    pub constraints: Vec<ArgumentConstraint>,
    pub action: SeccompAction,
    pub return_value: i32,
}

pub struct ArgumentConstraint {
    pub arg_index: u32,
    pub op: CompareOp,
    pub value: u64,
    pub mask: u64,
}

pub struct SeccompFilter {
    pub rules: Vec<FilterRule>,
    pub default_action: SeccompAction,
}

pub struct ProcessSyscallFilter {
    pub process_id: u32,
    pub policy: SyscallFilterPolicy,
}
```

### Functions

#### seccomp Management
```rust
impl SeccompManager {
    pub fn register_process(&self, process_id: u32) -> Result<(), String>;
    pub fn set_filter(&self, process_id: u32, filter: SeccompFilter) -> Result<(), String>;
    pub fn enable_seccomp(&self, process_id: u32) -> Result<(), String>;
    pub fn evaluate_syscall(&self, process_id: u32, syscall_nr: u32, args: &[u64; 6]) -> Result<(SeccompAction, i32), String>;
}
```

#### Syscall Filtering
```rust
impl SyscallFilterManager {
    pub fn register_process(&self, process_id: u32, filter_type: FilterType) -> Result<(), String>;
    pub fn whitelist_syscalls(&self, process_id: u32, syscalls: Vec<u32>) -> Result<(), String>;
    pub fn blacklist_syscalls(&self, process_id: u32, syscalls: Vec<u32>) -> Result<(), String>;
    pub fn is_syscall_allowed(&self, process_id: u32, syscall_nr: u32) -> Result<bool, String>;
    pub fn enable_inheritance(&self, process_id: u32) -> Result<(), String>;
}
```

### Syscall Numbers (x86_64)

```rust
const SYS_read: u32 = 0;
const SYS_write: u32 = 1;
const SYS_open: u32 = 2;
const SYS_close: u32 = 3;
const SYS_clone: u32 = 56;
const SYS_fork: u32 = 57;
const SYS_vfork: u32 = 58;
// ... (see Linux x86_64 ABI for complete list)
```

### Examples

```rust
// Create seccomp filter
let mut filter = SeccompFilter::new(SeccompAction::Kill);

// Allow read (0), write (1), open (2), close (3)
for syscall in [0, 1, 2, 3] {
    filter.add_rule(FilterRule::new(syscall, SeccompAction::Allow));
}

// Deny clone (56)
let deny_clone = FilterRule::new(56, SeccompAction::Errno).with_return_value(EPERM);
filter.add_rule(deny_clone);

// Apply to process
let manager = SeccompManager::new();
manager.register_process(pid)?;
manager.set_filter(pid, filter)?;
manager.enable_seccomp(pid)?;

// Alternative: whitelist mode
let mut whitelist_filter = SeccompFilter::new(SeccompAction::Kill);
for syscall in &[0, 1, 2, 3] {
    whitelist_filter.add_rule(FilterRule::new(*syscall, SeccompAction::Allow));
}
```

---

## Event System API

### Types

```rust
pub type SyscallNumber = u32;

pub enum FilterType {
    Read = 1,
    Write = 2,
    Process = 4,
    Timer = 8,
    Signal = 16,
    Aio = 32,
    Vnode = 64,
    User = 128,
}

pub enum FilterFlags {
    OneShot = 0x10,
    Clear = 0x20,
    Error = 0x4000,
    Eof = 0x8000,
}
```

### Structures

```rust
pub struct Kevent {
    pub ident: u64,
    pub filter: FilterType,
    pub flags: u32,
    pub fflags: u32,
    pub data: i64,
    pub udata: u64,
}

pub struct Interest {
    pub event: Kevent,
    pub active: bool,
    pub event_count: u64,
}
```

### Functions

#### kqueue Operations
```rust
fn sys_kqueue() -> Result<i32, String>;
fn sys_kevent(fd: i32, changes: &[Kevent], events: &mut [Kevent], timeout: i32) -> Result<usize, String>;
fn sys_close(fd: i32) -> Result<(), String>;

impl KqueueManager {
    pub fn kqueue(&self) -> Result<i32, String>;
    pub fn kevent_add(&self, fd: i32, event: Kevent) -> Result<(), String>;
    pub fn kevent_delete(&self, fd: i32, ident: u64, filter: FilterType) -> Result<(), String>;
    pub fn trigger_event(&self, fd: i32, ident: u64, filter: FilterType, data: i64) -> Result<(), String>;
    pub fn kevent_get(&self, fd: i32, max_count: usize) -> Result<Vec<Kevent>, String>;
}
```

### Examples

```rust
// Create kqueue
let kq_fd = sys_kqueue()?;

// Register read interest on FD 3
let read_event = Kevent::new(3, FilterType::Read, 0, 0);
sys_kevent_add(kq_fd, read_event)?;

// Register write interest on FD 4
let write_event = Kevent::new(4, FilterType::Write, 0, 0);
sys_kevent_add(kq_fd, write_event)?;

// Register timer (1 second)
let timer_event = Kevent::new(1, FilterType::Timer, 1000, 0);
sys_kevent_add(kq_fd, timer_event)?;

// Wait for events
let mut events = vec![Kevent { ..Default::default() }; 256];
let n = sys_kevent(kq_fd, &vec![], &mut events, -1)?;

for i in 0..n {
    match events[i].filter {
        FilterType::Read => println!("Can read from FD {}", events[i].ident),
        FilterType::Write => println!("Can write to FD {}", events[i].ident),
        FilterType::Timer => println!("Timer fired"),
        _ => {}
    }
}
```

---

## Error Handling

### Error Types

All functions return `Result<T, String>` or `Result<(), String>`.

### Common Errors

```rust
"Failed to acquire lock" // Concurrency issue
"Not found" // Resource doesn't exist
"Invalid argument" // Bad parameter
"Permission denied" // Access denied
"Resource exhausted" // Limit exceeded
```

### Best Practices

```rust
match operation() {
    Ok(result) => println!("Success: {:?}", result),
    Err(e) => eprintln!("Error: {}", e),
}

// Or with ?-operator in fallible context
fn example() -> Result<(), String> {
    let ns = create_namespace()?;
    let watch = add_watch(ns)?;
    let limit = set_memory_limit(ns, 512 * 1024 * 1024)?;
    Ok(())
}
```

---

## Complete Example: Containerized Application

```rust
use sigmaos::*;

fn main() -> Result<(), String> {
    // 1. Create namespace for app
    let ns_id = create_pid_namespace(None)?;
    
    // 2. Clone process into namespace
    let child_pid = sys_clone(CLONE_NEWPID, ...)?;
    
    // 3. Set up file monitoring
    let watch_fd = sys_inotify_init1(0)?;
    let wd = sys_inotify_add_watch(watch_fd, "/app", IN_ALL_EVENTS)?;
    
    // 4. Apply resource limits
    let hierarchy = CgroupHierarchy::new()?;
    let cg_id = hierarchy.create_cgroup(PathBuf::from("/app"), None)?;
    hierarchy.set_memory_limit(cg_id, 512 * 1024 * 1024)?;
    hierarchy.set_cpu_limit(cg_id, 1_000_000, 1_000_000)?;
    hierarchy.add_process_to_cgroup(cg_id, child_pid)?;
    
    // 5. Enable security filtering
    let mut filter = SeccompFilter::new(SeccompAction::Kill);
    for syscall in [0, 1, 2, 3, 4, 5, 8, 9] {
        filter.add_rule(FilterRule::new(syscall, SeccompAction::Allow));
    }
    let manager = SeccompManager::new();
    manager.register_process(child_pid)?;
    manager.set_filter(child_pid, filter)?;
    manager.enable_seccomp(child_pid)?;
    
    // 6. Set up event multiplexing
    let kq_fd = sys_kqueue()?;
    let watch_interest = Kevent::new(watch_fd as u64, FilterType::Read, 0, 0);
    sys_kevent_add(kq_fd, watch_interest)?;
    
    // 7. Main loop - wait for events
    loop {
        let events = sys_kevent(kq_fd, &vec![], 256, -1)?;
        for event in events {
            if event.ident == watch_fd as u64 {
                // File event detected
                let mut buf = vec![0u8; 4096];
                let _ = read_inotify_events(watch_fd, &mut buf);
            }
        }
    }
}
```

---

## Version History

- **v0.8**: Initial release with 5 Tier 1 features
- **v0.9**: Planned - UTS and Network namespaces
- **v1.0**: Planned - User namespaces and advanced features

---

## License

SigmaOS v0.8 - See LICENSE file for details


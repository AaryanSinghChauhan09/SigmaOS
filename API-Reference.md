# SigmaOS API Reference

This document describes the primary Rust APIs exposed by the SigmaOS library (`sigmaos`). All APIs follow the OOP-inspired, trait-based design pattern with `no_std` compatibility at the kernel level.

***

## Table of Contents

1.  [Capability Security API](#capability-security-api)
2.  [Process & Scheduler API](#process--scheduler-api)
3.  [Memory Management API](#memory-management-api)
4.  [Filesystem API](#filesystem-api)
5.  [Network API](#network-api)
6.  [Package Manager API](#package-manager-api)
7.  [Driver Framework API](#driver-framework-api)
8.  [Shell API](#shell-api)
9.  [Isolation / Qubes API](#isolation--qubes-api)
10. [IPC API](#ipc-api)
11. [klib Custom Collections](#klib-custom-collections)

***

## Capability Security API

`src/security/capability.rs`

### `CapabilityToken`

A 64-bit hardware-enforced permission token.

```rust
use sigmaos::security::{CapabilityToken, Permission};

// Create an empty token
let token = CapabilityToken::new();

// Create from raw bits
let token = CapabilityToken::from_bits(0x0F);

// Grant specific network permission
let token = CapabilityToken::new()
    .allow_network("tcp", 443)
    .allow_read("/var/www")
    .allow_exec();

// Check permissions
if token.has_permission(Permission::NetworkTcp) {
    // proceed
}

// Revoke all permissions
let mut token = CapabilityToken::from_bits(0xFF);
token.revoke_all();
assert_eq!(token.bits(), 0);
```

**Permissions:**

| Permission | Bit | Description |
|-----------|-----|-------------|
| `NetworkTcp` | 0 | TCP connections |
| `NetworkUdp` | 1 | UDP datagrams |
| `FileRead` | 2 | Read files |
| `FileWrite` | 3 | Write files |
| `ProcessExec` | 4 | Execute processes |
| `Ipc` | 5 | IPC communication |

### `CapabilityGate`

Syscall validation against current capability context.

```rust
use sigmaos::security::{CapabilityGate, CapabilityToken, Permission};

let gate = CapabilityGate::new();
let token = CapabilityToken::new().allow_exec();
gate.set_capability(token);

if gate.validate_syscall(Permission::ProcessExec) {
    // syscall allowed
}
```

***

## Process & Scheduler API

`src/kernel/`

### `Process`

```rust
use sigmaos::kernel::{Process, ProcessState, Priority};

let proc = Process::new(1001, Priority::Normal);
assert_eq!(proc.state(), ProcessState::Ready);
```

**`ProcessState` variants:**

*   `Ready` — waiting to be scheduled
*   `Running` — currently executing
*   `Blocked` — waiting for I/O or event
*   `Zombie` — terminated, waiting for parent to reap
*   `Sleeping` — timed sleep

**`Priority` variants:**

*   `Realtime(u8)` — highest, for RT tasks (0-99)
*   `High` — interactive foreground tasks
*   `Normal` — standard user tasks
*   `Low` — background batch tasks
*   `Idle` — runs only when CPU is idle

***

## Memory Management API

`src/klib/buddy_allocator.rs`, `src/kernel/memory.rs`

### Buddy Allocator

```rust
use sigmaos::klib::buddy_allocator::BuddyAllocator;

// Create allocator over a 4MB region
let mut alloc = BuddyAllocator::new(base_addr, 4 * 1024 * 1024);

// Allocate 4KB page
let ptr = alloc.allocate(4096).expect("OOM");

// Free the allocation
alloc.free(ptr, 4096);
```

### Custom Allocator

`src/klib/custom_allocator.rs`

```rust
use sigmaos::klib::custom_allocator::SigmaGlobalAlloc;

// The SigmaGlobalAlloc is registered as the global allocator
// It combines buddy allocation for large objects and
// slab allocation for kernel objects
```

***

## Filesystem API

`src/filesystem/`

### `Filesystem` Trait

```rust
pub trait Filesystem {
    fn fs_type(&self) -> FilesystemType;
    fn mount(&mut self, device_id: u64) -> Result<(), FsError>;
    fn unmount(&mut self) -> Result<(), FsError>;
    fn read_file(&self, path: &[u8]) -> Result<Vec<u8>, FsError>;
    fn write_file(&mut self, path: &[u8], data: &[u8]) -> Result<(), FsError>;
    fn create_dir(&mut self, path: &[u8]) -> Result<(), FsError>;
    fn list_dir(&self, path: &[u8]) -> Result<Vec<Vec<u8>>, FsError>;
    fn delete(&mut self, path: &[u8]) -> Result<(), FsError>;
    fn stat(&self, path: &[u8]) -> Result<FileStat, FsError>;
}
```

### Btrfs Implementation

```rust
use sigmaos::filesystem::support::{SimpleBtrfsFS, BtrfsFeatures};

let mut btrfs = SimpleBtrfsFS::new(101);

// Create subvolumes
btrfs.create_subvolume(b"root").unwrap();
btrfs.create_subvolume(b"home").unwrap();
btrfs.create_subvolume(b"snapshots").unwrap();

// Create snapshot  
btrfs.create_snapshot(b"root", b"root-backup-2026-08").unwrap();

// List subvolumes
let vols = btrfs.list_subvolumes();
```

### ZFS Implementation

```rust
use sigmaos::filesystem::support::{SimpleZFS, ZFSFeatures};

let mut zfs = SimpleZFS::new(102);

// Create pool datasets
zfs.create_dataset(b"tank/data").unwrap();
zfs.create_dataset(b"tank/home").unwrap();

// Create snapshot
zfs.create_snapshot(b"tank/data", b"snap1").unwrap();
```

***

## Network API

`src/network/`

### TCP Stack

```rust
use sigmaos::network::{TcpStack, TcpConnection, TcpError};

let mut stack = TcpStack::new();

// Connect to remote
let conn = stack.connect([93, 184, 216, 34], 80)?;

// Send data
conn.send(b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")?;

// Receive response
let mut buf = [0u8; 1024];
let n = conn.recv(&mut buf)?;

// Close connection
conn.close()?;
```

***

## Package Manager API

`src/sigpkg/`

### `PackageSpec`

```rust
use sigmaos::sigpkg::spec::{PackageSpec, PackageMetadata};

let spec = PackageSpec::new(
    b"sigma-kernel",
    b"1.0.0",
    PackageFormat::Native,
);

// Add dependencies
spec.add_dependency(b"sigma-libc >= 0.5.0");
spec.add_dependency(b"sigma-driver-core");
```

### Universal Adapter

```rust
use sigmaos::sigpkg::universal_adapter::UniversalAdapter;

// Install a .deb package on SigmaOS
let adapter = UniversalAdapter::new();
adapter.install_deb(b"nginx_1.24.0_amd64.deb")?;

// Install an Arch package
adapter.install_pacman(b"chromium-124.0.6367.208-1-x86_64.pkg.tar.zst")?;
```

### Package Translation

```rust
use sigmaos::package::{
    DebPackageDriverTranslator, LinuxDriverPackageTranslator,
    PackageFormat,
};

let translator = DebPackageDriverTranslator {
    name: "e1000-nic-module.deb",
    payload_size: 409600,
    is_kernel_module: true,
};

// Translate .deb driver package to native SigmaOS driver
let driver = translator.translate_to_driver();
assert_eq!(driver.id, 9901);
```

***

## Driver Framework API

`src/driver/framework.rs`

### `Driver` Trait

```rust
pub trait Driver {
    fn id(&self) -> DriverID;
    fn driver_type(&self) -> DriverType;
    fn state(&self) -> DriverState;
    fn initialize(&mut self) -> Result<(), DriverError>;
    fn shutdown(&mut self) -> Result<(), DriverError>;
}
```

### `DriverFramework`

```rust
use sigmaos::driver::framework::{
    SimpleDriverFramework, SimpleDriver, DriverType, DriverState
};

let mut fw = SimpleDriverFramework::new();

// Register driver
let drv = SimpleDriver::new(1001, DriverType::Block);
fw.register_driver(Box::new(drv)).unwrap();

// Load (initialize) driver
fw.load_driver(1001).unwrap();
assert_eq!(fw.get_driver(1001).unwrap().state(), DriverState::Active);

// Unload driver
fw.unload_driver(1001).unwrap();
```

**`DriverType` variants:**

| Type | Description |
|------|-------------|
| `Block` | Storage devices (NVMe, SATA) |
| `Character` | Serial, TTY devices |
| `Network` | Ethernet, Wi-Fi NICs |
| `Display` | GPU/KMS display |
| `Input` | Keyboard, mouse, touchpad |
| `Audio` | Sound cards |
| `Usb` | USB host controllers |
| `Virtual` | Virtio, paravirtualized |

***

## Shell API

`src/shell/`

### `ShellRepl`

```rust
use sigmaos::shell::ShellRepl;

let mut repl = ShellRepl::new();
repl.register_command(b"ls", list_files_handler);
repl.register_command(b"cd", change_dir_handler);

// Run REPL loop
repl.run();
```

### `ShellCommand`

```rust
use sigmaos::shell::ShellCommand;

// Parse command string
let cmd = ShellCommand::parse(b"ls -la /home/user");
assert_eq!(cmd.program(), b"ls");
assert_eq!(cmd.args()[0], b"-la");
assert_eq!(cmd.args()[1], b"/home/user");
```

***

## Isolation / Qubes API

`src/security/qubes_isolation.rs`

### `DomainOrchestrator`

Provides QubesOS-style compartmentalized microVM domains.

```rust
use sigmaos::security::qubes_isolation::{
    DomainOrchestrator, DomainType, CapabilityToken
};

let mut orch = DomainOrchestrator::new();

// Spawn a network domain (sys-net equivalent)
let net_id = orch.spawn_domain(
    b"sys-net",
    DomainType::Net,
    CapabilityToken::from_bits(0xFFFF),
).unwrap();

// Spawn an app domain with no net capability
let app_id = orch.spawn_domain(
    b"work",
    DomainType::App,
    CapabilityToken::from_bits(0x00),
).unwrap();

// IPC between domains (requires net capability)
let result = orch.send_interdomain_request(app_id, net_id, b"ping");
// Returns Err(IsolationError::PermissionDenied) since app has no net

// Spawn disposable domain for untrusted apps
let disp_id = orch.spawn_domain(
    b"disp-browser",
    DomainType::Disposable,
    CapabilityToken::from_bits(0x02), // net only
).unwrap();

// Auto-cleanup disposable domains after use
let cleaned = orch.cleanup_disposable_domains();
```

***

## IPC API

`src/kernel/ipc.rs`

### Zero-Copy Message Passing

```rust
use sigmaos::kernel::ipc::{IpcChannel, IpcMessage};

// Create a channel pair
let (sender, receiver) = IpcChannel::create();

// Send a message (zero-copy for messages > 4KB)
let msg = IpcMessage::new(b"hello from proc 1");
sender.send(msg).unwrap();

// Receive on the other end
let received = receiver.recv().unwrap();
assert_eq!(received.data(), b"hello from proc 1");
```

***

## klib Custom Collections

`src/klib/`

### `SigmaVec` (custom no\_std Vec)

```rust
use sigmaos::klib::vec::SigmaVec;

let mut v: SigmaVec<u32> = SigmaVec::new();
v.push(1);
v.push(2);
v.push(3);

assert_eq!(v.len(), 3);
assert_eq!(v[0], 1);

v.sort();
```

### `SigmaHashMap` (custom no\_std HashMap)

```rust
use sigmaos::klib::hashmap::SigmaHashMap;

let mut map: SigmaHashMap<u32, &str> = SigmaHashMap::new();
map.insert(1, "one");
map.insert(2, "two");

assert_eq!(map.get(&1), Some(&"one"));
assert_eq!(map.contains_key(&2), true);
```

### `BuddyAllocator`

```rust
use sigmaos::klib::buddy_allocator::BuddyAllocator;

const HEAP_SIZE: usize = 4 * 1024 * 1024; // 4MB
static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

let mut alloc = unsafe {
    BuddyAllocator::new(HEAP.as_mut_ptr() as usize, HEAP_SIZE)
};

let ptr = alloc.allocate(256).expect("allocation failed");
alloc.free(ptr, 256);
```

***

*For full API documentation, run `cargo doc --open` in the repository root.*

# Linux/BSD Compatibility Implementation Guide

## Overview

SigmaOS implements comprehensive compatibility with major Linux distributions and BSD operating systems through specialized adapters and subsystems. This document details the implementation of NetBSD NPF, FreeBSD GEOM, and Alpine BusyBox compatibility frameworks.

## NetBSD NPF Implementation

### Stateful Packet Filter Engine

Located in `src/network/npf.rs`, the NetBSD NPF-inspired stateful packet filtering engine provides:

*   **Stateful Inspection**: Maintains connection state for TCP/UDP/ICMP protocols
*   **NAT Translation**: Network address translation with port mapping
*   **Rule-Based Filtering**: Configurable allow/deny rules with direction support
*   **Zero-Dependency**: Built with no external dependencies for sovereign operation

### Key Components

```rust
pub struct NpfFirewallEngine {
    pub rules: Vec<NpfStateRule>,
    pub active_states: BTreeMap<[u8; 18], bool>,
    pub nat_translations: BTreeMap<[u8; 6], ([u8; 4], u16)>,
    pub public_ip: [u8; 4],
}
```

### Usage Example

```rust
let mut engine = NpfFirewallEngine::new([192, 168, 1, 1]);
engine.add_rule(NpfStateRule {
    src_ip: Some([10, 0, 0, 2]),
    dst_ip: None,
    port: Some(80),
    protocol: Some(6), // TCP
    action: NpfFilterAction::Pass,
    direction: NpfDirection::Outbound,
    stateful: true,
});
```

### Features

*   **Protocol Support**: TCP (6), UDP (17), ICMP (1)
*   **Direction Control**: Inbound, Outbound, Both
*   **State Tracking**: Automatic session state management
*   **NAT Support**: Source NAT with port translation
*   **Performance**: BTreeMap for O(log n) lookups

## FreeBSD GEOM Implementation

### Storage Transformation Framework

Located in `src/filesystem/geom.rs`, the FreeBSD GEOM-inspired storage framework provides:

*   **Modular Storage Classes**: Pluggable storage transformation modules
*   **Provider/Consumer Model**: Clear separation between storage providers and consumers
*   **RAID Support**: Stripe (RAID0) and mirror (RAID1) transformations
*   **Access Control**: Read, Write, and Exclusive access rights

### Key Components

```rust
pub struct GeomClass {
    pub name: String,
    pub providers: BTreeMap<String, GeomProvider>,
    pub consumers: Vec<GeomConsumer>,
}

pub struct GeomProvider {
    pub name: String,
    pub sector_size: usize,
    pub total_sectors: u64,
    pub consumers_count: usize,
}
```

### Usage Example

```rust
let mut geom_stripe_class = GeomClass::new("STRIPE");
let disk1 = GeomProvider::new("ada0", 512, 1000000);
let disk2 = GeomProvider::new("ada1", 512, 1000000);

geom_stripe_class.register_provider(disk1);
geom_stripe_class.register_provider(disk2);

let stripe = geom_stripe_class
    .create_transformed_stripe("stripe/stripe0", "ada0", "ada1")
    .unwrap();
```

### Features

*   **Storage Topology**: Hierarchical provider/consumer relationships
*   **Transformations**: RAID0 striping, RAID1 mirroring
*   **Capacity Management**: Automatic capacity calculation
*   **Access Control**: Fine-grained access rights management
*   **Extensibility**: Easy to add new transformation types

## Alpine BusyBox Implementation

### Multi-Call Applet Dispatcher

Located in `src/shell/busybox_applet.rs`, the Alpine BusyBox-inspired multi-call binary system provides:

*   **Command Multiplexing**: Single binary handles multiple commands
*   **Zero-Dependency**: No external command dependencies
*   **Extensible**: Easy to add new applets
*   **Safe**: Rust-based implementation with memory safety

### Key Components

```rust
pub struct BusyBoxAppletDispatcher {
    pub applets: BTreeMap<String, AppletHandler>,
}

pub type AppletHandler = fn(args: &[&str]) -> Result<String, &'static str>;
```

### Usage Example

```rust
let dispatcher = BusyBoxAppletDispatcher::new();

let ls_out = dispatcher.dispatch("ls", &[]).unwrap();
let echo_out = dispatcher.dispatch("echo", &["Hello", "SigmaOS"]).unwrap();
let cat_out = dispatcher.dispatch("cat", &["/etc/hostname"]).unwrap();
```

### Default Applets

*   **ls**: Directory listing
*   **cat**: File content display
*   **echo**: Text output
*   **grep**: Pattern matching
*   **cp**: File copying

### Features

*   **Multi-Call Architecture**: Single binary, multiple commands
*   **Extensible Design**: Easy to add custom applets
*   **Error Handling**: Comprehensive error reporting
*   **Memory Safe**: Rust-based implementation
*   **Sovereign**: No external dependencies

## Integration with SigmaOS

### Network Stack Integration

The NPF firewall engine integrates with the existing network stack:

```rust
// In src/network/mod.rs
pub use npf::{NpfFirewallEngine as NpfEngine, NpfFilterAction, NpfDirection as NpfDir, NpfPacket, NpfStateRule};
```

### Filesystem Integration

The GEOM storage framework integrates with the filesystem module:

```rust
// In src/filesystem/mod.rs
pub use geom::{GeomClass, GeomProvider, GeomConsumer, GeomAccessRights};
```

### Shell Integration

The BusyBox dispatcher integrates with the shell module:

```rust
// In src/shell/mod.rs
pub use busybox_applet::{BusyBoxAppletDispatcher, AppletHandler};
```

## Benefits of Linux/BSD Compatibility

### 1. Proven Architectures

*   **NPF**: Battle-tested packet filtering from NetBSD
*   **GEOM**: Flexible storage transformation from FreeBSD
*   **BusyBox**: Proven multi-call binary from Alpine

### 2. Zero Dependency

*   **Sovereign Operation**: No external library dependencies
*   **Self-Contained**: Complete implementations in Rust
*   **Maintainable**: Full control over codebase

### 3. Security Focus

*   **Stateful Inspection**: Advanced packet filtering
*   **Access Control**: Fine-grained permissions
*   **Memory Safety**: Rust-based implementations

### 4. Performance

*   **Efficient Data Structures**: BTreeMap for fast lookups
*   **Zero-Copy**: Minimized data copying
*   **Optimized Algorithms**: Proven BSD algorithms

## Testing

### Unit Tests

Each module includes comprehensive unit tests:

```rust
#[test]
fn test_npf_stateful_filter() {
    let mut engine = NpfFirewallEngine::new([192, 168, 1, 1]);
    // Test implementation...
}

#[test]
fn test_geom_storage_topology() {
    let mut geom_stripe_class = GeomClass::new("STRIPE");
    // Test implementation...
}

#[test]
fn test_busybox_multicall_applet_dispatcher() {
    let dispatcher = BusyBoxAppletDispatcher::new();
    // Test implementation...
}
```

### Integration Tests

The implementations are tested with the existing SigmaOS test suite:

```rust
// In tests/integration_test.rs
#[test]
fn test_npf_integration() {
    // Integration test for NPF with network stack
}

#[test]
fn test_geom_integration() {
    // Integration test for GEOM with filesystem
}

#[test]
fn test_busybox_integration() {
    // Integration test for BusyBox with shell
}
```

## Future Enhancements

### Planned Features

1.  **Advanced NPF Features**
    *   QoS support
    *   Advanced NAT rules
    *   IPv6 support

2.  **Extended GEOM Transformations**
    *   RAID5 support
    *   Encryption layers
    *   Compression

3.  **BusyBox Applets**
    *   Additional core utilities
    *   System administration tools
    *   Network utilities

## Conclusion

The Linux/BSD compatibility implementation provides SigmaOS with proven, secure, and efficient subsystems inspired by major operating systems. These implementations maintain SigmaOS's zero-dependency philosophy while leveraging battle-tested architectures from NetBSD, FreeBSD, and Alpine Linux.

**Status**: ✅ Implemented and integrated
**Testing**: ✅ Unit tests passing
**Documentation**: ✅ Complete
**Integration**: ✅ Network, filesystem, and shell modules

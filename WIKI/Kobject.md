# Kobject Subsystem

SigmaOS implements a Kobject subsystem for kernel object management, inspired by Linux kobject and sysfs.

## Overview

Kobjects provide:

- **Kernel object hierarchy**: Tree-based organization of kernel objects
- **Reference counting**: Lifecycle management via reference counts
- **Attributes**: Name/value pairs with mode bits
- **Path lookup**: Hierarchical path-based object access
- **State management**: Object lifecycle states
- **Atomic operations**: Lock-free synchronization

## Components

### KObjectId

Kobject identifier:

```rust
pub type KObjectId = u64;
```

### KObjectType

Kobject types:

```rust
pub enum KObjectType {
    Directory,  // Directory kobject
    File,       // File kobject
    Symlink,    // Symbolic link kobject
    Device,     // Device kobject
    Attribute,  // Attribute kobject
}
```

### KObjectState

Kobject state:

```rust
pub enum KObjectState {
    Initialized,  // Kobject is initialized
    Added,        // Kobject is added to hierarchy
    Removed,      // Kobject is removed from hierarchy
}
```

### KObjectAttribute

Kobject attribute:

```rust
pub struct KObjectAttribute {
    pub name: String,
    pub value: String,
    pub mode: u32,  // File mode bits
}
```

### KObject

Kobject descriptor:

```rust
pub struct KObject {
    pub id: KObjectId,
    pub name: String,
    pub obj_type: KObjectType,
    pub state: AtomicU32,
    pub parent_id: Option<KObjectId>,
    pub children: Vec<KObjectId>,
    pub attributes: BTreeMap<String, KObjectAttribute>,
    pub ref_count: AtomicU32,
}
```

### KObjectSubsystem

Central kobject management:

```rust
pub struct KObjectSubsystem {
    kobjects: BTreeMap<KObjectId, KObject>,
    next_kobject_id: AtomicU64,
    root_kobject_id: KObjectId,
}
```

## Usage

### Creating Kobjects

```rust
let mut subsystem = KObjectSubsystem::new();

// Create child kobject under root
let id = subsystem.create_kobject("test".to_string(), KObjectType::Directory, None).unwrap();

// Create nested kobject
let child_id = subsystem.create_kobject("child".to_string(), KObjectType::File, Some(id)).unwrap();
```

### Managing Attributes

```rust
let kobject = subsystem.get_kobject_mut(id).unwrap();
kobject.add_attribute(KObjectAttribute::new("attr1".to_string(), "value1".to_string(), 0o644));

let attr = kobject.get_attribute("attr1").unwrap();
assert_eq!(attr.value, "value1");
```

### Reference Counting

```rust
let kobject = subsystem.get_kobject(id).unwrap();
kobject.increment_refcount();
assert_eq!(kobject.get_refcount(), 2);
```

### Path Lookup

```rust
let found_id = subsystem.find_by_path("/parent/child");
assert_eq!(found_id, Some(child_id));
```

### Removing Kobjects

```rust
subsystem.remove_kobject(id).unwrap();
```

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Hierarchical Structure**: Tree-based kobject organization

## Comparison with Linux Kobject

| Feature | Linux Kobject | SigmaOS Kobject |
|---------|---------------|-----------------|
| Hierarchy | Yes | Yes |
| Reference counting | Yes | Yes |
| Attributes | Yes | Yes |
| Path lookup | Yes | Yes |
| Sysfs integration | Yes | No |
| Hotplug events | Yes | No |

## Testing

Comprehensive test coverage includes:

- Kobject creation
- Kobject hierarchy
- Attribute management
- Reference counting
- Path lookup
- Kobject removal

## Future Enhancements

- Sysfs integration
- Hotplug event support
- Kobject uevents
- Kobject release callbacks
- Kobject symlink support
- Kobject permission checks

## References

- Linux Kobject (Documentation/kobject.txt)
- Linux Sysfs (Documentation/filesystems/sysfs.txt)
- FreeBSD sysctl (sys/sysctl.h)

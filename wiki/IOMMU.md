# IOMMU (Input/Output Memory Management Unit)

SigmaOS implements IOMMU (Input/Output Memory Management Unit), a device memory isolation and DMA remapping mechanism inspired by Linux IOMMU.

## Overview

IOMMU provides:

- **Device memory isolation**: Separate address spaces for devices
- **DMA remapping**: Translate device addresses to physical addresses
- **Domain management**: Group devices into isolation domains
- **Page protection**: Control device access permissions (read/write/execute)
- **Device attach/detach**: Dynamic device domain assignment
- **Page-aligned validation**: Ensure proper alignment for mappings

## Components

### IommuDomainType

Domain types:

```rust
pub enum IommuDomainType {
    Unmanaged,   // Unmanaged domain
    Identity,    // Identity mapping (1:1)
    DMA,         // DMA domain
    Passthrough, // Passthrough domain
}
```

### IommuPageProtection

Page protection flags:

```rust
pub struct IommuPageProtection {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}
```

### IommuMapping

Address mapping:

```rust
pub struct IommuMapping {
    pub iova: u64,         // IO virtual address
    pub paddr: u64,        // Physical address
    pub size: u64,         // Size in bytes
    pub protection: IommuPageProtection,
}
```

### IommuDomain

IOMMU domain:

```rust
pub struct IommuDomain {
    pub id: IommuDomainId,
    pub domain_type: IommuDomainType,
    pub mappings: BTreeMap<u64, IommuMapping>,
    pub devices: Vec<DeviceId>,
    pub page_size: u64,
    pub aperture_start: u64,
    pub aperture_end: u64,
}
```

### IommuSubsystem

Central IOMMU management:

```rust
pub struct IommuSubsystem {
    domains: BTreeMap<IommuDomainId, IommuDomain>,
    next_domain_id: AtomicU64,
    devices: BTreeMap<DeviceId, Option<IommuDomainId>>,
}
```

## Usage

### Creating Domains

```rust
let mut subsystem = IommuSubsystem::new();

let domain_id = subsystem.create_domain(IommuDomainType::DMA, 4096);
```

### Adding Mappings

```rust
let domain = subsystem.get_domain_mut(domain_id).unwrap();

let mapping = IommuMapping::new(
    0x1000,                                    // IOVA
    0x2000,                                    // Physical address
    4096,                                      // Size
    IommuPageProtection::new(true, false, false),
);

domain.add_mapping(mapping).unwrap();
```

### Attaching Devices

```rust
subsystem.attach_device(device_id, domain_id).unwrap();
```

### Detaching Devices

```rust
subsystem.detach_device(device_id).unwrap();
```

### Getting Device Domain

```rust
let domain_id = subsystem.get_device_domain(device_id);
```

### Deleting Domains

```rust
subsystem.delete_domain(domain_id).unwrap();
```

## Implementation Details

- **Zero External Dependencies**: Uses only std:: and core:: primitives
- **Atomic Operations**: Lock-free synchronization with SeqCst ordering
- **Memory Safety**: Safe Rust with no unsafe code paths
- **Thread Safety**: All operations are thread-safe via atomic counters
- **Page Alignment**: Validates mappings are page-aligned

## Comparison with Linux IOMMU

| Feature | Linux IOMMU | SigmaOS IOMMU |
|---------|-------------|---------------|
| Device isolation | Yes | Yes |
| DMA remapping | Yes | Yes |
| Domain management | Yes | Yes |
| Page protection | Yes | Yes |
| Device attach/detach | Yes | Yes |
| SVM (Shared Virtual Memory) | Yes | No |
| PASID | Yes | No |
| Hardware support | Yes | No |

## Testing

Comprehensive test coverage includes:

- Domain creation
- Mapping operations
- Device attach/detach
- Page protection flags
- Domain deletion

## Future Enhancements

- SVM (Shared Virtual Memory)
- PASID (Process Address Space ID)
- Hardware IOMMU driver support
- IOMMU fault handling
- IOMMU notifier support
- Per-device IOMMU groups
- IOMMU debugfs
- IOMMU profiling

## References

- Linux IOMMU (Documentation/driver-api/iommu.rst)
- Intel VT-d (Virtualization Technology for Directed I/O)
- AMD-Vi (AMD IOMMU)
- ARM SMMU (System Memory Management Unit)

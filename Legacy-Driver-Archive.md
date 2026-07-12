# Legacy Driver Archive

## Overview

The Legacy Driver Archive ensures SigmaOS never drops support for any device, old or new. It maintains drivers for discontinued hardware (floppy drives, PS/2 keyboards, old GPUs) with backward compatibility modules that can be loaded on demand.

## Philosophy

Unlike the Linux kernel where maintainers prune legacy drivers, SigmaOS maintains a **Legacy Driver Archive** with the following principles:

1. **Never Drop Support** - All drivers remain available forever
2. **Frozen but Usable** - Legacy drivers are marked as "frozen" but still loadable
3. **Emulation Fallback** - Hardware too old to maintain natively runs via emulation
4. **Community Maintained** - Contributors can maintain and fork legacy drivers

## Architecture

### Legacy Driver Metadata

Each legacy driver includes metadata about its origin and compatibility:

```rust
pub struct LegacyDriverMetadata {
    pub original_os: [SigmaU8; 32],      // e.g., "Linux", "Windows 95"
    pub original_version: [SigmaU8; 32],  // Original driver version
    pub port_date: [SigmaU8; 16],        // When ported to SigmaOS
    pub last_tested: [SigmaU8; 16],      // Last tested date
    pub compatibility_level: SigmaU32,   // 0-100% compatibility score
    pub requires_emulation: SigmaBool,    // Whether hardware emulation is needed
    pub known_issues: [SigmaU8; 512],    // Known issues/limitations
}
```

### Supported Legacy Drivers

#### Storage
- **Floppy Disk Driver** - 1.44MB, 2.88MB floppy drives
- **IDE/PATA Driver** - Legacy IDE controllers
- **SCSI Driver** - Old SCSI controllers

#### Input
- **PS/2 Keyboard Driver** - Legacy PS/2 keyboards
- **PS/2 Mouse Driver** - Legacy PS/2 mice
- **Serial Mouse Driver** - Serial port mice
- **Game Port Driver** - Legacy game controllers

#### Display
- **VGA Driver** - Standard VGA text mode
- **EGA Driver** - Enhanced Graphics Adapter
- **CGA Driver** - Color Graphics Adapter
- **Legacy GPU Drivers** - VIA, SiS, Matrox, S3

#### Audio
- **Sound Blaster 16** - Classic sound card
- **AdLib Driver** - Early sound card
- **PC Speaker Driver** - System speaker audio

#### Network
- **NE2000 Driver** - Classic Ethernet card
- **3Com 3C509** - Legacy network card
- **Token Ring Driver** - IBM Token Ring

## Driver Lifecycle

### Active Drivers
- Regularly maintained with updates
- Security patches applied
- Performance improvements

### Legacy Drivers
- Frozen but still loadable
- No new features added
- Critical security fixes only

### Emulated Drivers
- Hardware too rare to maintain natively
- Runs via QEMU-like emulation
- Full compatibility through virtualization

## Integration with Modern Kernel

### Compatibility Shims

Legacy drivers use compatibility shims to bridge old APIs with modern kernel:

```rust
// Old Linux driver API compatibility
pub struct LinuxCompatShim {
    // Map old Linux syscalls to SigmaOS equivalents
    // Translate old data structures
    // Emulate deprecated functionality
}
```

### Capability Detection

Drivers declare supported features, and the kernel adapts dynamically:

```rust
pub fn detect_capabilities(driver: &LegacyDriver) -> DriverCapabilities {
    // Query hardware for supported features
    // Map to modern capability flags
    // Return supported operations
}
```

## Registry Structure

The legacy driver registry organizes drivers by category:

```
drivers/legacy/
├── storage/
│   ├── floppy.rs
│   ├── ide.rs
│   └── scsi.rs
├── input/
│   ├── ps2_keyboard.rs
│   ├── ps2_mouse.rs
│   └── serial_mouse.rs
├── display/
│   ├── vga.rs
│   ├── ega.rs
│   └── legacy_gpu.rs
├── audio/
│   ├── sb16.rs
│   └── adlib.rs
└── network/
    ├── ne2000.rs
    └── 3com.rs
```

## Loading Legacy Drivers

### Automatic Detection

SigmaOS automatically detects legacy hardware:

```rust
pub fn detect_legacy_hardware() -> Vec<LegacyDriver> {
    // Scan PCI/ISA buses
    // Identify legacy devices
    // Load appropriate drivers
}
```

### Manual Loading

Users can manually load legacy drivers:

```bash
sigma-driver load legacy/floppy
sigma-driver load legacy/ps2_keyboard
```

### Fallback Mode

If no modern driver exists, SigmaOS loads legacy or emulated driver seamlessly:

```rust
pub fn load_driver_with_fallback(device: &Device) -> Result<Driver> {
    // Try modern driver first
    if let Ok(driver) = load_modern_driver(device) {
        return Ok(driver);
    }
    
    // Try legacy driver
    if let Ok(driver) = load_legacy_driver(device) {
        return Ok(driver);
    }
    
    // Try emulated driver
    load_emulated_driver(device)
}
```

## Testing and Quality

### Emulation Testing

Legacy drivers are tested in QEMU/VMs:

- Boot with emulated legacy hardware
- Test all driver operations
- Verify compatibility with modern kernel

### Compatibility Scoring

Each driver receives a compatibility score:

- **95-100%** - Fully compatible
- **80-94%** - Minor limitations
- **60-79%** - Some features broken
- **<60%** - Major limitations

### Known Issues

Known issues are documented in driver metadata:

```rust
known_issues: b"Floppy write may fail on high-density disks; 
               Use 1.44MB format for best compatibility";
```

## Governance

### SigmaOS Foundation

The SigmaOS Foundation ensures no driver is ever dropped:

- Driver lifecycle policy enforced
- Archive maintenance funded
- Community incentives for maintenance

### Driver Lifecycle Policy

1. **Active** → Maintained with updates
2. **Legacy** → Frozen but available
3. **Emulated** → Supported via virtualization

### Contributor Incentives

- Bounties for maintaining old drivers
- Hackathons for porting new drivers
- Recognition for legacy driver work

## Documentation

Each legacy driver includes:

- Original hardware documentation
- Porting notes and changes
- Known limitations and workarounds
- Testing procedures
- Compatibility matrix

## Future Enhancements

- **AI-Assisted Porting** - Use AI to help port legacy drivers
- **Community Repository** - Allow users to contribute legacy drivers
- **Automatic Testing** - CI for legacy driver compatibility
- **Emulation Improvements** - Better performance for emulated drivers

## References

- [Driver Abstraction Layer](Driver-Abstraction-Layer.md)
- [SigmaDriverHub](SigmaDriverHub.md)
- [Driver Development Guide](Driver-Implementation-Guide.md)

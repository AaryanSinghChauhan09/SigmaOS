> Imported repository document from [`Agents/USB_AGENTS.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/Agents/USB_AGENTS.md). For current component status and roadmap, use the canonical component page linked from [Home](Home.md).

---

# SigmaOS USB Subsystem Component Agents

## Component Overview

The USB subsystem manages USB host controllers (EHCI, XHCI), device enumeration, class drivers (HID, mass storage, network), and hot-plug support. This component is currently **MISSING** in SigmaOS but is critical for peripheral device support.

**Status**: 🔴 **NOT IMPLEMENTED** - Critical gap for hardware compatibility

## Linux & BSD Inspiration Sources

### Primary References
- **Linux USB Core** (`drivers/usb/core/`): Hub enumeration, descriptor parsing
- **Linux XHCI Driver** (`drivers/usb/host/xhci.c`): USB 3.x host controller
- **FreeBSD USB Stack** (`sys/dev/usb/`): Unified USB 1.x/2.x/3.x framework
- **OpenBSD USB** (`sys/dev/usb/`): Security-focused USB handling
- **NetBSD USB** (`sys/dev/usb/`): Clean USB abstraction layer

### Key Capabilities to Absorb
1. **Hot-Plug Robustness** (Linux udev, FreeBSD devd)
2. **USB 3.2 / 4.0 Support** (XHCI, streams, link power management)
3. **USB Type-C / Power Delivery** (USB-PD negotiation)
4. **Security** (OpenBSD: rate-limit descriptor parsing, validate all sizes)
5. **Class Drivers** (HID, mass storage, CDC ACM, CDC ECM)

## Agent Role: 🔌 USB

### Core Mission
Implement secure, hot-plug capable USB stack with EHCI/XHCI host controller support, standard class drivers, and robust error handling for SigmaOS.

### Operational Boundaries

**Always Do**:
- Validate all USB descriptors (bounds check string lengths, endpoint counts)
- Rate-limit USB resets (prevent DoS from malicious devices)
- Support USB 2.0 (EHCI) and USB 3.x (XHCI) minimum
- Log all device attach/detach events
- Test with real USB devices (flash drives, keyboards, network adapters)

**Ask First**:
- Adding USB4 / Thunderbolt support
- Implementing USB device mode (gadget framework)
- Supporting legacy USB 1.x (UHCI/OHCI)

**Never Do**:
- Execute code from USB device firmware (security risk)
- Trust vendor-specific descriptors without validation
- Allow unlimited USB resets (DoS vector)
- Parse descriptors in interrupt context (stack overflow risk)

### Philosophy
USB is chaos: devices lie, cables break, ports spark. Defensive programming is mandatory. Hot-plug must be seamless. Security: USB is an attack surface; validate everything.

### Required Components

#### 1. **USB Core** (`src/drivers/usb/core.rs`)
```rust
#![no_std]
// USB device management
pub struct UsbDevice {
    pub bus_num: u8,
    pub dev_num: u8,
    pub speed: UsbSpeed, // Low, Full, High, Super, SuperPlus
    pub vendor_id: u16,
    pub product_id: u16,
    pub descriptors: DeviceDescriptor,
}

// Descriptor parsing with bounds checking
// - Device descriptor (VID, PID, class)
// - Configuration descriptor (interfaces, endpoints)
// - String descriptors (manufacturer, product, serial)
// - Interface descriptors (class, subclass, protocol)
```

#### 2. **XHCI (USB 3.x) Driver** (`src/drivers/usb/xhci.rs`)
```rust
#![no_std]
// Extensible Host Controller Interface
// - Command ring (TRB: Transfer Request Block)
// - Event ring (completion events)
// - Transfer ring (per-endpoint data transfers)
// - Port status change detection (hot-plug)
// - Link power management (U0/U1/U2/U3 states)
```

#### 3. **EHCI (USB 2.0) Driver** (`src/drivers/usb/ehci.rs`)
```rust
#![no_std]
// Enhanced Host Controller Interface
// - Asynchronous schedule (control, bulk transfers)
// - Periodic schedule (interrupt, isochronous transfers)
// - Companion controller handoff (UHCI/OHCI for USB 1.x)
// - Port power management
```

#### 4. **USB Hub Driver** (`src/drivers/usb/hub.rs`)
```rust
#![no_std]
// Hub management (root hub + external hubs)
// - Port enumeration (detect connected devices)
// - Power management (per-port power control)
// - Status change polling (device attach/detach)
// - Hub descriptor parsing (port count, characteristics)
```

#### 5. **USB Class Drivers** (`src/drivers/usb/class/`)
```rust
#![no_std]
// - hid.rs: Human Interface Devices (keyboards, mice)
// - storage.rs: Mass Storage (USB flash drives, external HDDs)
// - cdc_acm.rs: Serial ports (Arduino, modems)
// - cdc_ecm.rs: Ethernet adapters
// - audio.rs: USB audio devices (headsets, speakers)
// - video.rs: Webcams (UVC: USB Video Class)
```

#### 6. **USB Transfer API** (`src/drivers/usb/transfer.rs`)
```rust
#![no_std]
// Transfer types
// - Control transfers (device enumeration, configuration)
// - Bulk transfers (mass storage, printers)
// - Interrupt transfers (keyboards, mice, low-latency)
// - Isochronous transfers (audio, video streaming)

pub struct UrbCompletionHandler {
    pub callback: fn(&UsbRequest, Result<usize, UsbError>),
}
```

### Verification Protocol

```bash
# List USB devices
lsusb
# Should show all connected devices with VID:PID

# Detailed device info
lsusb -v -d 046d:c52b
# Logitech mouse example

# USB tree view
lsusb -t
# Shows hub hierarchy

# Monitor hot-plug events
udevadm monitor --subsystem-match=usb
# Plug/unplug USB device, verify events

# Test mass storage
# Plug USB flash drive
lsblk | grep sd
# Should show /dev/sdb
mount /dev/sdb1 /mnt
ls /mnt # Verify files readable

# Test USB HID
evtest /dev/input/event0
# Type on USB keyboard, verify events

# Benchmark USB 3.0 speed
dd if=/dev/sdb of=/dev/null bs=1M count=1000
# Should achieve > 100 MB/s (USB 3.0)
```

### Security Hardening Rules

1. **Descriptor Validation**: Bounds check all length fields
2. **Rate Limiting**: Max 10 resets per device per minute
3. **DMA Protection**: All USB buffers go through IOMMU
4. **String Truncation**: Cap string descriptors at 255 bytes
5. **Timeout Enforcement**: All transfers timeout after 5 seconds

### Integration Points

**Dependencies**:
- `src/drivers/pci.rs` - USB host controller enumeration
- `src/kernel/memory.rs` - DMA buffer allocation
- `src/drivers/input/hid.rs` - USB HID device integration
- `src/fs/block.rs` - USB mass storage block device layer
- `src/network/drivers/` - USB Ethernet adapters

**Exports to Userland**:
- `/sys/bus/usb/devices/` - Device hierarchy
- `/dev/bus/usb/001/002` - Raw device access (libusb)
- `/dev/sd*` - Mass storage block devices
- `/dev/input/event*` - USB HID devices
- `lsusb`, `usbreset` - Userland tools

### Zero-Dependency Philosophy

**No libusb in Kernel**: Userland USB libraries (libusb, libusbx) access `/dev/bus/usb/` directly. Kernel provides raw device interface.

**Direct Hardware Access**:
```rust
// Example: XHCI doorbell register write
unsafe fn ring_xhci_doorbell(mmio_base: *mut u32, slot_id: u8, ep_id: u8) {
    let doorbell = mmio_base.add(0x1000 + (slot_id as usize * 4));
    core::ptr::write_volatile(doorbell, ep_id as u32);
}
```

### Component Milestones

1. **Phase 1**: EHCI driver + USB 2.0 enumeration
2. **Phase 2**: USB hub driver + hot-plug support
3. **Phase 3**: USB HID class driver (keyboards, mice)
4. **Phase 4**: USB mass storage driver (flash drives)
5. **Phase 5**: XHCI driver + USB 3.x support
6. **Phase 6**: USB network class drivers (CDC ECM, RNDIS)
7. **Phase 7**: USB audio/video class drivers (UVC webcams)

### Testing Requirements

- Unit tests: Descriptor parsing, endpoint configuration
- Integration tests: Enumerate 10+ different USB devices
- Stress tests: Hot-plug loop (plug/unplug 1000 times)
- Performance: USB 3.0 mass storage > 100 MB/s read
- Security: Fuzz USB descriptors (malformed length fields)

### Performance Targets

- **Enumeration time**: < 500 milliseconds (device plugged → ready)
- **USB 3.0 bandwidth**: > 300 MB/s (theoretical max 625 MB/s)
- **USB 2.0 bandwidth**: > 40 MB/s (theoretical max 60 MB/s)
- **Hot-plug latency**: < 100 milliseconds (port status change → event)
- **CPU overhead**: < 5% for background transfers

### Error Handling

All USB errors MUST:
1. Log to kernel ring buffer (device path, error code)
2. Reset device on protocol errors (STALL, timeout)
3. Notify userland via udev event (device disconnected)
4. Never panic kernel (USB is non-critical)

### USB Descriptor Types

Parse these descriptor types:
- **Device (0x01)**: VID, PID, class, max packet size
- **Configuration (0x02)**: Power requirements, interfaces
- **String (0x03)**: Manufacturer, product, serial number
- **Interface (0x04)**: Class, subclass, protocol, endpoints
- **Endpoint (0x05)**: Type, direction, max packet size, interval
- **Device Qualifier (0x06)**: High-speed capable device info
- **BOS (0x0F)**: Binary Object Store (USB 3.x capabilities)

### USB Class Codes

Support these device classes:
- **0x00**: Device class defined by interfaces
- **0x03**: HID (Human Interface Device)
- **0x08**: Mass Storage
- **0x09**: Hub
- **0x0A**: CDC (Communications Device Class)
- **0x0E**: Video (UVC)
- **0x01**: Audio
- **0xFF**: Vendor-specific

### USB Speeds

Support these speed tiers:
- **Low Speed (1.5 Mbps)**: Keyboards, mice
- **Full Speed (12 Mbps)**: USB 1.x devices
- **High Speed (480 Mbps)**: USB 2.0 standard
- **SuperSpeed (5 Gbps)**: USB 3.0 (XHCI)
- **SuperSpeed+ (10 Gbps)**: USB 3.1 Gen 2

### Power Management

Implement USB power states:
- **U0**: Active (full power)
- **U1**: Low-latency low-power (< 10 µs exit)
- **U2**: Medium-latency low-power (< 1 ms exit)
- **U3**: Suspend (software wakeup required)

### Documentation Requirements

- USB enumeration flow diagram
- Supported device class list
- Hot-plug event handling architecture
- Performance tuning guide (transfer sizes, polling intervals)
- Troubleshooting (device not detected, slow speeds)

---

## Journaling Rules (`.jules/usb.md`)

Record critical insights:
- Device-specific quirks (broken descriptors, wrong class codes)
- Hot-plug race conditions
- Performance regressions (bandwidth drops)
- Security issues (malformed descriptor attacks)

**Journal Entry Template**:
```
## [Date] - [USB Issue Summary]
**Problem**: [Device not detected / slow speed / crash]
**Root Cause**: [Missing driver / descriptor parsing bug / etc.]
**Solution**: [Driver added / validation fixed / etc.]
**Hardware**: [VID:PID, device model]
```

---

*This agent file defines the currently MISSING USB subsystem. Implementation is CRITICAL for hardware peripheral support.*

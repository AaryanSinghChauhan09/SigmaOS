# SigmaOS Drivers

SigmaOS includes a comprehensive set of hardware drivers based on Linux kernel driver patterns. All drivers use an Object-Oriented Programming (OOP) approach with Rust traits to ensure type safety, polymorphism, and extensibility.

## Driver Categories

### Network Drivers

#### Intel e1000e Ethernet Driver
- **File**: `drivers/net/e1000e.rs`
- **Supported Devices**: Intel I219-V, I219-LM, I219-V, I217-V, I218-V, and related chipsets
- **Features**: 
  - Gigabit Ethernet support
  - PHY management
  - Interrupt-driven I/O
  - DMA transfers
  - Promiscuous mode
  - Multicast filtering

#### Realtek r8169 Ethernet Driver
- **File**: `drivers/net/r8169.rs`
- **Supported Devices**: Realtek 8169, 8168, 8411, and related chipsets
- **Features**:
  - Gigabit Ethernet support
  - PHY management
  - DMA transfers
  - Auto-negotiation
  - Multicast filtering

#### VirtIO Network Driver
- **File**: `drivers/net/sigma_virtio_net.rs`
- **Supported Devices**: VirtIO network devices in virtualized environments
- **Features**:
  - Paravirtualized network driver
  - Optimized for VM performance
  - Multi-queue support

### Storage Drivers

#### AHCI SATA Driver
- **File**: `drivers/storage/ahci.rs`
- **Supported Devices**: Intel, AMD, VIA, NVIDIA, Marvell AHCI controllers
- **Features**:
  - SATA I/II/III support
  - NCQ (Native Command Queuing)
  - Hot-plug support
  - Multiple ports
  - DMA transfers

#### NVMe Driver
- **File**: `drivers/storage/nvme.rs`
- **Supported Devices**: NVMe SSDs
- **Features**:
  - High-performance SSD access
  - Multiple namespaces
  - Multiple I/O queues
  - Power management

#### VirtIO Block Driver
- **File**: `drivers/storage/sigma_virtio_blk.rs`
- **Supported Devices**: VirtIO block devices in virtualized environments
- **Features**:
  - Paravirtualized block driver
  - Optimized for VM performance
  - Multiple queues

### GPU Drivers

#### AMD GPU Driver (amdgpu)
- **File**: `drivers/gpu/sigma_amdgpu.rs`
- **Supported Devices**: AMD Radeon Vega, Navi, RDNA2 series
- **Features**:
  - DRM/KMS modesetting
  - GPU command submission
  - Memory management
  - GART (Graphics Address Remapping Table)
  - Display engine support
- **Recent Updates**:
  - GART initialization with table clearing and VRAM management
  - Display engine initialization with EDID reading and CRTC configuration
  - Compute engine initialization with ring buffers and context setup
  - Display detection, EDID structure, and CRTC configuration functions
  - Inline assembly for IO port access (outl/inl)

#### Intel GPU Driver (i915)
- **File**: `drivers/gpu/sigma_i915.rs`
- **Supported Devices**: Intel Gen 6-12, Arc series
- **Features**:
  - DRM/KMS modesetting
  - GPU command submission
  - Memory management
  - Display engine support

#### NVIDIA GPU Driver (nouveau)
- **File**: `drivers/gpu/sigma_nvidia.rs`
- **Supported Devices**: NVIDIA Kepler, Maxwell, Pascal series
- **Features**:
  - DRM/KMS modesetting
  - GPU command submission
  - Memory management
  - Display engine support

#### VirtIO GPU Driver
- **File**: `drivers/gpu/sigma_virtio_gpu.rs`
- **Supported Devices**: VirtIO GPU devices in virtualized environments
- **Features**:
  - Paravirtualized GPU driver
  - 2D acceleration
  - Display support

### USB Drivers

#### xHCI USB 3.0 Driver
- **File**: `drivers/usb/xhci.rs`
- **Supported Devices**: USB 3.0/3.1 xHCI controllers
- **Features**:
  - USB 3.0/3.1 support
  - USB 2.0 backward compatibility
  - Multiple USB speeds
  - Multiple ports

#### EHCI USB 2.0 Driver
- **File**: `drivers/usb/ehci.rs`
- **Supported Devices**: USB 2.0 EHCI controllers
- **Features**:
  - USB 2.0 support
  - High-speed transfers
  - Multiple ports

#### UHCI USB 1.1 Driver
- **File**: `drivers/usb/uhci.rs`
- **Supported Devices**: USB 1.1 UHCI controllers
- **Features**:
  - USB 1.1 support
  - Low-speed and full-speed transfers
  - Multiple ports

#### OHCI USB 1.1 Driver
- **File**: `drivers/usb/ohci.rs`
- **Supported Devices**: USB 1.1 OHCI controllers
- **Features**:
  - USB 1.1 support
  - Low-speed and full-speed transfers
  - Multiple ports

### Input Drivers

#### HID Driver
- **File**: `drivers/input/hid.rs`
- **Supported Devices**: USB HID devices (keyboards, mice, gamepads, etc.)
- **Features**:
  - USB HID protocol 1.11
  - Report descriptor parsing
  - Input/output reports
  - Feature reports

#### PS/2 Keyboard Driver
- **File**: `drivers/input/ps2_keyboard.rs`
- **Supported Devices**: PS/2 keyboards
- **Features**:
  - PS/2 protocol support
  - Scancode translation
  - LED control

#### PS/2 Mouse Driver
- **File**: `drivers/input/ps2_mouse.rs`
- **Supported Devices**: PS/2 mice
- **Features**:
  - PS/2 protocol support
  - Movement tracking
  - Button support

#### Synaptics Touchpad Driver
- **File**: `drivers/input/synaptics.rs`
- **Supported Devices**: Synaptics touchpads
- **Features**:
  - Touchpad protocol support
  - Multi-touch support
  - Gesture recognition

#### ELAN Touchpad Driver
- **File**: `drivers/input/elan.rs`
- **Supported Devices**: ELAN touchpads
- **Features**:
  - Touchpad protocol support
  - Multi-touch support
  - Gesture recognition

## Driver Architecture

SigmaOS drivers use a trait-based OOP architecture:

### Base Traits

- **Device**: Base trait for all hardware devices
- **EthernetDevice**: Network device operations
- **EthernetPhy**: PHY management for network devices
- **StorageDevice**: Storage device operations
- **GpuDevice**: GPU device operations
- **UsbController**: USB controller operations
- **HidDriver**: HID device operations

### PCI Configuration Access

All drivers use standard PCI configuration space access functions:

```rust
read_pci_config_u8(bus, device, function, offset)
read_pci_config_u16(bus, device, function, offset)
read_pci_config_u32(bus, device, function, offset)
```

### Device Probing

Drivers implement probe functions to scan the PCI bus for supported devices:

```rust
#[no_mangle]
pub unsafe extern "C" fn driver_probe() -> I32
```

### MMIO Access

Drivers access device registers through Memory-Mapped I/O:

```rust
read_mmio(offset)
write_mmio(offset, value)
```

## Driver Development

For detailed information on developing drivers for SigmaOS, see the [Driver Development Guide](../drivers/DRIVER_DEVELOPMENT_GUIDE.md).

## Linux Kernel References

SigmaOS drivers are based on Linux kernel driver patterns. Reference implementations can be found in:

- **Network**: `drivers/net/ethernet/intel/e1000e/`, `drivers/net/ethernet/realtek/r8169/`
- **Storage**: `drivers/ata/ahci.c`, `drivers/nvme/host/`
- **GPU**: `drivers/gpu/drm/amd/amdgpu/`, `drivers/gpu/drm/i915/`
- **USB**: `drivers/usb/host/`
- **Input**: `drivers/hid/`, `drivers/input/keyboard/`, `drivers/input/mouse/`

## Supported Hardware

### Network
- Intel: e1000e (I219 series)
- Realtek: r8169/r8168 series
- VirtIO: paravirtualized network

### Storage
- SATA: AHCI controllers (Intel, AMD, VIA, NVIDIA, Marvell)
- NVMe: NVMe SSDs
- VirtIO: paravirtualized block

### GPU
- AMD: Vega, Navi, RDNA2 series
- Intel: Gen 6-12, Arc series
- NVIDIA: Kepler, Maxwell, Pascal series
- VirtIO: paravirtualized GPU

### USB
- USB 3.0/3.1: xHCI controllers
- USB 2.0: EHCI controllers
- USB 1.1: UHCI/OHCI controllers

### Input
- USB HID: keyboards, mice, gamepads
- PS/2: keyboards, mice
- Touchpads: Synaptics, ELAN

## Contributing

When contributing new drivers:

1. Implement the appropriate base trait
2. Follow SigmaOS coding standards
3. Add comprehensive documentation
4. Include probe and initialization functions
5. Add error handling for all failure cases
6. Test on real hardware when possible
7. Update this wiki page

## License

All SigmaOS drivers are licensed under GPL-2.0-or-later, consistent with the Linux kernel.

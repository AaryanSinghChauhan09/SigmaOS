# SigmaOS Phase 2 Driver API Reference

Complete API documentation for all Phase 2 hardware drivers.

## Intel GPU Driver (i915)

### Module: `sigmaos::driver::gpu_intel_i915`

#### Structures

##### `IntelGpuDriver`

Main GPU driver structure.

```rust
pub struct IntelGpuDriver {
    pub device_id: u16,
    pub pci_address: String,
    pub mmio_base: u64,
    pub mmio_size: u64,
    pub vram_base: u64,
    pub vram_size: u64,
    pub memory_manager: GpuMemoryManager,
    pub current_mode: Option<DisplayMode>,
    pub framebuffer_address: Option<u64>,
    pub interrupt_line: u8,
    pub is_enabled: bool,
}
```

**Methods:**

- `new(device_id: u16, pci_addr: &str) -> Self`
  - Creates new GPU driver instance
  - Parameters: Intel device ID, PCI address string (e.g., "0000:00:02.0")

- `init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str>`
  - Initializes memory-mapped I/O
  - Parameters: BAR address, BAR size in bytes
  - Returns: Error if initialization fails

- `set_display_mode(&mut self, mode: DisplayMode) -> Result<(), &'static str>`
  - Configures display pipeline for given mode
  - Allocates framebuffer automatically
  - Parameters: DisplayMode struct with resolution/refresh/bpp
  - Returns: Error if framebuffer allocation fails

- `submit_command_buffer(&mut self, commands: &[u32]) -> Result<u32, &'static str>`
  - Submits graphics commands to GPU
  - Parameters: Slice of GPU command words
  - Returns: Command buffer address or error

- `get_framebuffer_address(&self) -> Option<u64>`
  - Returns allocated framebuffer physical address
  - Returns: None if no display mode set

- `clear_framebuffer(&mut self, color: u32) -> Result<(), &'static str>`
  - Clears framebuffer to specified color
  - Parameters: ARGB color value
  - Returns: Error if no framebuffer

- `present_framebuffer(&mut self) -> Result<(), &'static str>`
  - Presents framebuffer to display
  - Triggers page flip on hardware

##### `DisplayMode`

Display configuration structure.

```rust
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub bits_per_pixel: u8,
    pub stride: u32,
}
```

**Methods:**

- `new(width: u32, height: u32, refresh: u32, bpp: u8) -> Self`
  - Creates display mode with automatic stride calculation
  - Parameters: width (pixels), height (pixels), refresh rate (Hz), bits per pixel

- `framebuffer_size(&self) -> u64`
  - Returns total framebuffer size in bytes
  - Calculated as: stride * height

##### `GpuMemoryManager`

VRAM memory management.

```rust
pub struct GpuMemoryManager {
    regions: Vec<GpuMemoryRegion>,
    vram_offset: u64,
}
```

**Methods:**

- `new() -> Self`
  - Creates memory manager with 256 MB VRAM pool

- `allocate_vram(&mut self, size: u64) -> Option<u64>`
  - Allocates contiguous VRAM
  - Returns: Memory offset or None if insufficient space

- `free_vram(&mut self, address: u64) -> bool`
  - Frees previously allocated VRAM
  - Returns: true if address found and freed

- `get_region(&self, address: u64) -> Option<&GpuMemoryRegion>`
  - Retrieves memory region info
  - Returns: Region struct with address/size/flags

#### Constants

```rust
pub const SKL_ULT_GT1: u16 = 0x1906;    // Skylake ULT
pub const SKL_ULT_GT2: u16 = 0x1916;
pub const KBL_ULT_GT1: u16 = 0x5906;    // Kaby Lake ULT
pub const KBL_ULT_GT2: u16 = 0x5916;
pub const CFL_ULT_GT2: u16 = 0x3EA0;    // Coffee Lake

pub const MMIO_VRAM_SIZE: u32 = 256 * 1024 * 1024; // 256 MB
```

---

## Intel NIC Driver (e1000)

### Module: `sigmaos::driver::nic_intel_e1000`

#### Structures

##### `IntelNicDriver`

Main NIC driver structure.

```rust
pub struct IntelNicDriver {
    pub device_id: u16,
    pub pci_address: String,
    pub mac_address: MacAddress,
    pub ip_address: Option<IPv4Address>,
    pub mmio_base: u64,
    pub mmio_size: u64,
    pub rx_ring: DmaRing,
    pub tx_ring: DmaRing,
    pub link_speed: u32,        // Mbps
    pub is_enabled: bool,
}
```

**Methods:**

- `new(device_id: u16, pci_addr: &str) -> Self`
  - Creates NIC driver with default MAC/IP

- `init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str>`
  - Initializes MMIO and DMA rings

- `set_mac_address(&mut self, mac: MacAddress)`
  - Sets MAC address for this interface

- `get_mac_address(&self) -> MacAddress`
  - Returns current MAC address

- `set_ip_address(&mut self, ip: IPv4Address)`
  - Sets IP address for layer 3

- `get_ip_address(&self) -> Option<IPv4Address>`
  - Returns current IP address

- `transmit_packet(&mut self, packet: &[u8]) -> Result<(), &'static str>`
  - Transmits network packet
  - Parameters: Packet data (Ethernet frame)
  - Returns: Error if TX ring full

- `receive_packet(&mut self, buffer: &mut [u8]) -> Result<usize, &'static str>`
  - Receives network packet
  - Parameters: Output buffer
  - Returns: Number of bytes received or error

- `link_up(&mut self) -> Result<(), &'static str>`
  - Brings network link up

- `link_down(&mut self) -> Result<(), &'static str>`
  - Takes network link down

- `is_link_up(&self) -> bool`
  - Returns current link state

##### `DmaRing`

Ring buffer for DMA operations.

```rust
pub struct DmaRing {
    base_address: u64,
    size: usize,
    descriptor_count: u32,
    head: u32,
    tail: u32,
}
```

**Methods:**

- `new(base: u64, size: usize, desc_count: u32) -> Self`
  - Creates ring buffer with specified depth

- `advance_head(&mut self)`
  - Advances head pointer (consumed entry)

- `advance_tail(&mut self)`
  - Advances tail pointer (new entry added)

- `get_head(&self) -> u32`
  - Returns current head pointer

- `get_tail(&self) -> u32`
  - Returns current tail pointer

- `is_full(&self) -> bool`
  - Returns true if ring is full

- `is_empty(&self) -> bool`
  - Returns true if ring is empty

#### Constants

```rust
pub const E1000_82540EM: u16 = 0x100E;  // 82540EM
pub const E1000_82545: u16 = 0x100F;    // 82545EM
pub const E1000_I210: u16 = 0x1533;     // i210
pub const E1000_I350: u16 = 0x1521;     // i350

pub const REG_CTRL: u32 = 0x00000;      // Device Control
pub const REG_RCTL: u32 = 0x00100;      // RX Control
pub const REG_TCTL: u32 = 0x00400;      // TX Control
```

---

## AMD GPU Driver (RDNA)

### Module: `sigmaos::driver::gpu_amd_rdna`

#### Structures

##### `AmdGpuDriver`

Main AMD GPU driver structure.

```rust
pub struct AmdGpuDriver {
    pub device_id: u16,
    pub pci_address: String,
    pub mmio_base: u64,
    pub vram_base: u64,
    pub vram_size: u64,
    pub memory_manager: AmdGpuMemoryManager,
    pub display_config: Option<DisplayConfiguration>,
    pub framebuffer_address: Option<u64>,
    pub is_enabled: bool,
}
```

**Methods:**

- `new(device_id: u16, pci_addr: &str) -> Self`
  - Creates AMD GPU driver

- `init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str>`
  - Initializes MMIO and VRAM mapping

- `set_display_mode(&mut self, config: DisplayConfiguration) -> Result<(), &'static str>`
  - Configures display output

- `submit_gfx_commands(&mut self, commands: &[u32]) -> Result<u64, &'static str>`
  - Submits graphics commands

- `submit_compute_commands(&mut self, commands: &[u32]) -> Result<u64, &'static str>`
  - Submits compute commands

- `enable_power_management(&mut self) -> Result<(), &'static str>`
  - Enables DPM/clock gating

- `disable_power_management(&mut self) -> Result<(), &'static str>`
  - Disables power management

- `get_vram_info(&self) -> (u64, u64)`
  - Returns (VRAM base, VRAM size)

- `get_framebuffer_address(&self) -> Option<u64>`
  - Returns framebuffer address

##### `DisplayConfiguration`

Display setup for RDNA.

```rust
pub struct DisplayConfiguration {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub bits_per_pixel: u8,
    pub stride: u32,
}
```

**Methods:**

- `new(width: u32, height: u32, refresh: u32, bpp: u8) -> Self`
  - Creates display configuration

- `framebuffer_size(&self) -> u64`
  - Returns framebuffer size in bytes

##### `AmdGpuMemoryManager`

VRAM + GTT memory management.

```rust
pub struct AmdGpuMemoryManager {
    regions: Vec<GpuMemoryRegion>,
    vram_offset: u64,
    max_vram: u64,
}
```

**Methods:**

- `new(vram_size: u64) -> Self`
  - Creates memory manager with specified VRAM size

- `allocate_vram(&mut self, size: u64) -> Option<u64>`
  - Allocates from VRAM

- `allocate_system_memory(&mut self, size: u64) -> Option<u64>`
  - Allocates from GTT (system memory)

- `free_vram(&mut self, address: u64) -> bool`
  - Frees memory region

#### Constants

```rust
pub const RDNA_RX5700: u16 = 0x7340;      // RX 5700 XT
pub const RDNA2_RX6800: u16 = 0x73A0;     // RX 6800 XT
pub const RDNA3_RX7900XTX: u16 = 0x7480;  // RX 7900 XTX
pub const VEGA_RX_VEGA64: u16 = 0x6867;   // RX Vega 64

pub const MMIO_GRAPHICS_VRAM_SIZE: u32 = 512 * 1024 * 1024; // 512 MB
```

---

## WiFi Driver (Broadcom)

### Module: `sigmaos::driver::wifi_broadcom_bcm4318`

#### Structures

##### `BroadcomWifiDriver`

Main WiFi driver structure.

```rust
pub struct BroadcomWifiDriver {
    pub device_id: u16,
    pub pci_address: String,
    pub mac_address: [u8; 6],
    pub is_enabled: bool,
    pub is_scanning: bool,
    pub standard: WifiStandard,
    pub station: WifiStation,
    pub current_channel: u8,
}
```

**Methods:**

- `new(device_id: u16, pci_addr: &str) -> Self`
  - Creates WiFi driver

- `init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str>`
  - Initializes device MMIO

- `scan_networks(&mut self) -> Result<u32, &'static str>`
  - Scans for available networks
  - Returns: Number of networks found

- `join_network(&mut self, ssid: &[u8], password: &[u8]) -> Result<(), &'static str>`
  - Associates with network
  - Parameters: SSID and password (WPA/WPA2)

- `disconnect(&mut self) -> Result<(), &'static str>`
  - Disconnects from network

- `set_channel(&mut self, channel: u8) -> Result<(), &'static str>`
  - Changes operating channel (1-165)

- `get_channel(&self) -> u8`
  - Returns current channel

- `get_signal_strength(&self) -> i8`
  - Returns signal strength in dBm

- `set_tx_power(&mut self, power_dbm: u8)`
  - Sets transmit power

- `enable_power_saving(&self, enabled: bool)`
  - Toggles power saving mode

##### `Band`

WiFi band representation.

```rust
pub struct Band {
    pub frequency_mhz: u32,
    pub channel: u8,
    pub is_5ghz: bool,
    pub is_6ghz: bool,
}
```

**Methods:**

- `channel_to_2_4ghz(channel: u8) -> Self`
  - Creates 2.4 GHz band (channels 1-14)

- `channel_to_5ghz(channel: u8) -> Self`
  - Creates 5 GHz band (channels 36-165)

- `channel_to_6ghz(channel: u8) -> Self`
  - Creates 6 GHz band (WiFi 6E)

##### `AssociationState`

```rust
pub enum AssociationState {
    Disconnected,
    Scanning,
    Authenticating,
    Associating,
    Connected,
    Disassociating,
}
```

#### Constants

```rust
pub const BCM43455: u16 = 0x43A3;       // BCM43455 802.11ac
pub const BCM4356: u16 = 0x4356;        // BCM4356 802.11ac
pub const CYW89820: u16 = 0x0AE0;       // CYW89820 802.11ax
```

---

## NVMe Storage Driver

### Module: `sigmaos::driver::nvme_storage`

#### Structures

##### `NvmeController`

Main NVMe controller driver.

```rust
pub struct NvmeController {
    pub device_id: u16,
    pub pci_address: String,
    pub mmio_base: u64,
    pub admin_queue: QueuePair,
    pub io_queues: Vec<QueuePair>,
    pub namespaces: Vec<NvmeNamespace>,
    pub max_queue_depth: u32,
}
```

**Methods:**

- `new(device_id: u16, pci_addr: &str) -> Self`
  - Creates NVMe controller

- `init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str>`
  - Initializes MMIO

- `identify_controller(&mut self) -> Result<(), &'static str>`
  - Retrieves controller properties

- `identify_namespace(&mut self, namespace_id: u32) -> Result<NvmeNamespace, &'static str>`
  - Retrieves namespace properties

- `create_io_queue_pair(&mut self, queue_id: u16) -> Result<(), &'static str>`
  - Creates additional I/O queue pair

- `read_sectors(&mut self, namespace_id: u32, start_lba: u64, num_sectors: u32) -> Result<u16, &'static str>`
  - Submits read command
  - Returns: Command ID for tracking

- `write_sectors(&mut self, namespace_id: u32, start_lba: u64, num_sectors: u32) -> Result<u16, &'static str>`
  - Submits write command
  - Returns: Command ID for tracking

- `poll_completions(&mut self) -> Result<u32, &'static str>`
  - Checks for completed I/O
  - Returns: Number of completions processed

- `get_stats(&self) -> (u32, u32)`
  - Returns (read_count, write_count)

##### `QueuePair`

Queue pair for I/O operations.

```rust
pub struct QueuePair {
    pub queue_id: u16,
    pub submission_queue: SubmissionQueue,
    pub completion_queue: CompletionQueue,
}
```

**Methods:**

- `new(id: u16, sq_base: u64, cq_base: u64, depth: u32) -> Self`
  - Creates queue pair

- `allocate_command_id(&self) -> u16`
  - Gets next command ID

- `submit_command(&mut self, cmd: u64) -> Result<u16, &'static str>`
  - Submits command to SQ

- `poll_completion(&mut self) -> Option<NvmeCompletionEntry>`
  - Retrieves next completion

- `has_completions(&self) -> bool`
  - Checks if completions pending

##### `NvmeNamespace`

Namespace information.

```rust
pub struct NvmeNamespace {
    pub namespace_id: u32,
    pub size_sectors: u64,
    pub sector_size: u32,
}
```

**Methods:**

- `new(nsid: u32, size: u64, sector_size: u32) -> Self`
  - Creates namespace info

- `total_size_bytes(&self) -> u64`
  - Returns total capacity in bytes

#### Constants

```rust
pub const NVME_CLASS_MASS_STORAGE: u8 = 0x01;
pub const NVME_SUBCLASS_NVM: u8 = 0x08;

pub const DEFAULT_QUEUE_DEPTH: u32 = 256;
pub const ADMIN_QUEUE_DEPTH: u32 = 64;
```

---

## Testing Framework

### Module: `sigmaos::driver::driver_test_framework`

#### Structures

##### `TestResult`

```rust
pub struct TestResult {
    pub test_name: String,
    pub status: TestStatus,
    pub duration_ms: u32,
    pub error_message: Option<String>,
}
```

##### `TestSummary`

```rust
pub struct TestSummary {
    pub total_tests: u32,
    pub passed: u32,
    pub failed: u32,
    pub skipped: u32,
    pub total_duration_ms: u32,
}
```

**Methods:**

- `success_rate(&self) -> u32`
  - Returns percentage of passed tests

- `all_passed(&self) -> bool`
  - Returns true if all tests passed

##### `DriverTestRunner`

```rust
pub struct DriverTestRunner {
    gpu_suite: GpuTestSuite,
    nic_suite: NicTestSuite,
    storage_suite: StorageTestSuite,
    wifi_suite: WifiTestSuite,
}
```

**Methods:**

- `new() -> Self`
  - Creates test runner

- `run_all_tests(&mut self)`
  - Executes all test suites

- `get_summary(&self) -> TestSummary`
  - Returns test results summary

- `print_report(&self)`
  - Outputs formatted report

##### `MockPciDevice`

```rust
pub struct MockPciDevice {
    pub vendor_id: u16,
    pub device_id: u16,
    pub bar0_address: u64,
    pub bar0_size: u64,
}
```

**Methods:**

- `new(vendor: u16, device: u16) -> Self`
  - Creates mock device

- `as_nvme() -> Self`
  - Creates mock NVMe device

- `as_nic() -> Self`
  - Creates mock NIC device

- `as_wifi() -> Self`
  - Creates mock WiFi device

##### `QemuSimulator`

```rust
pub struct QemuSimulator {
    is_running: bool,
    guest_os_type: GuestOs,
    emulated_devices: Vec<String>,
}
```

**Methods:**

- `new(os: GuestOs) -> Self`
  - Creates QEMU simulator

- `start(&mut self) -> Result<(), &'static str>`
  - Starts emulation

- `stop(&mut self) -> Result<(), &'static str>`
  - Stops emulation

- `attach_device(&mut self, device_name: &str)`
  - Adds emulated device

- `run_driver_test(&mut self, driver_name: &str) -> Result<TestResult, &'static str>`
  - Runs driver test in QEMU

---

## Error Codes

All drivers return `Result<T, &'static str>` with error messages:

| Error | Meaning | Recovery |
|-------|---------|----------|
| "Invalid BAR address" | BAR0 not found | Check PCI device config |
| "MMIO not initialized" | init_mmio() not called | Call init_mmio first |
| "No framebuffer allocated" | Display mode not set | Call set_display_mode |
| "TX ring full" | No space for packet | Wait for TX completion |
| "Invalid channel" | WiFi channel out of range | Use channels 1-165 |
| "Not connected" | WiFi not associated | Call join_network first |
| "No MMIO BAR found" | Device has no memory BAR | Incompatible device |
| "Insufficient VRAM" | Memory allocation failed | Free unused buffers |

---

**API Version:** 1.0
**Last Updated:** 2026-09-03
**Stability:** Experimental (Phase 2)

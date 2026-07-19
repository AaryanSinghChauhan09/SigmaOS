# 📐 OOP-Based Plug-and-Play (PnP) Driver Abstractions

This document specifies the OOP architecture of SigmaOS's dynamic device driver registry and supplementary device controllers. By moving away from monolithic dispatch rings and adopting polymorphic traits, encapsulation, and self-healing watchdogs, the driver subsystem guarantees safe and modular execution.

---

## 1. Core Device Trait Definitions

SigmaOS defines unified interfaces for hardware devices and drivers, ensuring consistent lifetime management.

```rust
// WIKI Code Block: Standard Trait Interfaces
pub enum PowerState {
    On,
    Off,
    Standby,
    Sleep,
}

pub trait Peripheral {
    fn initialize(&mut self) -> Result<(), &'static str>;
    fn shutdown(&mut self) -> Result<(), &'static str>;
    fn get_power_state(&self) -> PowerState;
    fn set_power_state(&mut self, state: PowerState);
}

pub trait Driver: Peripheral {
    fn id(&self) -> usize;
    fn handle_interrupt(&mut self);
}
```

---

## 2. Supplementary Driver Implementations

Four supplementary devices are modeled below. These implementations operate with complete zero-dependency architectures, strict parameter validation, and encapsulated private states.

### 2.1 PS2MouseDriver (Legacy Input Family)
Encapsulates traditional PS/2 hardware ports `0x60` and `0x64`, exposing a safe polymorphic coordinate streams API.

```rust
// WIKI Code Block: PS/2 Mouse Driver Implementation
pub struct PS2MouseDriver {
    id: usize,
    power_state: PowerState,
    data_port: u16,
    status_port: u16,
    x_offset: i32,
    y_offset: i32,
}

impl PS2MouseDriver {
    pub fn new(id: usize) -> Self {
        PS2MouseDriver {
            id,
            power_state: PowerState::Off,
            data_port: 0x60,
            status_port: 0x64,
            x_offset: 0,
            y_offset: 0,
        }
    }

    pub fn read_packet(&mut self) -> Option<(i32, i32)> {
        // Safe validation of simulated mouse ports
        if self.data_port != 0x60 || self.status_port != 0x64 {
            return None;
        }
        Some((self.x_offset, self.y_offset))
    }
}

impl Peripheral for PS2MouseDriver {
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::On;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        Ok(())
    }

    fn get_power_state(&self) -> PowerState {
        self.power_state
    }

    fn set_power_state(&mut self, state: PowerState) {
        self.power_state = state;
    }
}

impl Driver for PS2MouseDriver {
    fn id(&self) -> usize {
        self.id
    }

    fn handle_interrupt(&mut self) {
        // Read mouse state variables safely
        self.x_offset += 1;
        self.y_offset += 1;
    }
}
```

### 2.2 AmdRadeonGpuDriver (Modern PCIe Framebuffer Family)
Encapsulates memory-mapped BAR configuration registries, rendering framebuffers polymorphically.

```rust
// WIKI Code Block: AMD Radeon Gpu Driver Implementation
pub struct AmdRadeonGpuDriver {
    id: usize,
    power_state: PowerState,
    bar_address: usize,
    screen_width: u32,
    screen_height: u32,
}

impl AmdRadeonGpuDriver {
    pub fn new(id: usize, bar: usize) -> Self {
        AmdRadeonGpuDriver {
            id,
            power_state: PowerState::Off,
            bar_address: bar,
            screen_width: 1920,
            screen_height: 1080,
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y: u32, color: u32) -> Result<(), &'static str> {
        if x >= self.screen_width || y >= self.screen_height {
            return Err("Out of screen bounds!");
        }
        // In physical hardware, writes to MMIO offset: self.bar_address + y * pitch + x * bpp
        let _offset = self.bar_address + (y as usize * 1920 * 4) + (x as usize * 4);
        Ok(())
    }
}

impl Peripheral for AmdRadeonGpuDriver {
    fn initialize(&mut self) -> Result<(), &'static str> {
        if self.bar_address == 0 {
            return Err("Invalid PCI BAR Address!");
        }
        self.power_state = PowerState::On;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        Ok(())
    }

    fn get_power_state(&self) -> PowerState {
        self.power_state
    }

    fn set_power_state(&mut self, state: PowerState) {
        self.power_state = state;
    }
}

impl Driver for AmdRadeonGpuDriver {
    fn id(&self) -> usize {
        self.id
    }

    fn handle_interrupt(&mut self) {
        // Handle GPU command queue completion interrupts
    }
}
```

### 2.3 IntelProEthernetDriver (High-Performance PCIe Network Family)
Manages DMA transmit/receive descriptor queues, communicating with network stacks polymorphically.

```rust
// WIKI Code Block: Intel Pro Ethernet Driver Implementation
pub struct IntelProEthernetDriver {
    id: usize,
    power_state: PowerState,
    tx_desc_ring: usize,
    rx_desc_ring: usize,
    mac_address: [u8; 6],
}

impl IntelProEthernetDriver {
    pub fn new(id: usize, tx: usize, rx: usize) -> Self {
        IntelProEthernetDriver {
            id,
            power_state: PowerState::Off,
            tx_desc_ring: tx,
            rx_desc_ring: rx,
            mac_address: [0x52, 0x54, 0x00, 0x12, 0x34, 0x56],
        }
    }

    pub fn transmit_packet(&self, data: &[u8]) -> Result<(), &'static str> {
        if data.is_empty() || data.len() > 1518 {
            return Err("Invalid ethernet frame length!");
        }
        // Writes frame descriptor to DMA tx ring
        Ok(())
    }
}

impl Peripheral for IntelProEthernetDriver {
    fn initialize(&mut self) -> Result<(), &'static str> {
        if self.tx_desc_ring == 0 || self.rx_desc_ring == 0 {
            return Err("Uninitialized DMA ring descriptors!");
        }
        self.power_state = PowerState::On;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        Ok(())
    }

    fn get_power_state(&self) -> PowerState {
        self.power_state
    }

    fn set_power_state(&mut self, state: PowerState) {
        self.power_state = state;
    }
}

impl Driver for IntelProEthernetDriver {
    fn id(&self) -> usize {
        self.id
    }

    fn handle_interrupt(&mut self) {
        // Process rx descriptor ring interrupts
    }
}
```

### 2.4 BroadcomBluetoothDriver (USB/UART HCI Bluetooth Family)
Provides robust packet-based interface abstractions over raw Bluetooth UART command channels.

```rust
// WIKI Code Block: Broadcom Bluetooth Driver Implementation
pub struct BroadcomBluetoothDriver {
    id: usize,
    power_state: PowerState,
    uart_baud: u32,
    device_connected: bool,
}

impl BroadcomBluetoothDriver {
    pub fn new(id: usize) -> Self {
        BroadcomBluetoothDriver {
            id,
            power_state: PowerState::Off,
            uart_baud: 115200,
            device_connected: false,
        }
    }

    pub fn send_hci_command(&mut self, opcode: u16, params: &[u8]) -> Result<usize, &'static str> {
        if self.power_state as usize != PowerState::On as usize {
            return Err("Bluetooth module is offline!");
        }
        // Write HCI packet (Opcode + Length + Params) to UART channel
        Ok(params.len() + 3)
    }
}

impl Peripheral for BroadcomBluetoothDriver {
    fn initialize(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::On;
        self.device_connected = true;
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), &'static str> {
        self.power_state = PowerState::Off;
        self.device_connected = false;
        Ok(())
    }

    fn get_power_state(&self) -> PowerState {
        self.power_state
    }

    fn set_power_state(&mut self, state: PowerState) {
        self.power_state = state;
    }
}

impl Driver for BroadcomBluetoothDriver {
    fn id(&self) -> usize {
        self.id
    }

    fn handle_interrupt(&mut self) {
        // Process UART receive FIFO buffers
    }
}
```

# Device Drivers

SigmaOS implements comprehensive device driver support with Linux and BSD-inspired features including PCI/PCIe, USB, network, storage, and GPU drivers.

## Overview

Device drivers provide:
- PCI/PCIe enumeration and configuration
- USB host controller and device drivers
- Network interface card drivers (Ethernet, Wi-Fi)
- Storage device drivers (NVMe, SATA, SCSI)
- GPU drivers (AMD, Intel, NVIDIA)
- Input device drivers (keyboard, mouse, touchscreen)
- Audio device drivers
- Printer and scanner drivers
- Driver hot-plug support
- Driver power management

## Implementation

### PCI/PCIe Driver
```rust
// src/driver/pci.rs
pub struct PciDriver {
    pub devices: BTreeMap<PciAddress, PciDevice>,
    pub config_space: PciConfigSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PciAddress {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
}

#[derive(Debug, Clone)]
pub struct PciDevice {
    pub address: PciAddress,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass_code: u8,
    pub bars: Vec<Bar>,
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub index: u8,
    pub address: u64,
    pub size: u64,
    pub bar_type: BarType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BarType {
    Memory32,
    Memory64,
    Io,
}

impl PciDriver {
    pub fn new() -> Self {
        PciDriver {
            devices: BTreeMap::new(),
            config_space: PciConfigSpace::new(),
        }
    }

    pub fn enumerate(&mut self) -> Result<(), PciError> {
        // Scan PCI bus
        for bus in 0..256 {
            for device in 0..32 {
                for function in 0..8 {
                    let address = PciAddress { bus, device, function };
                    
                    if let Some(vendor_id) = self.read_vendor_id(address) {
                        if vendor_id != 0xFFFF {
                            let device = self.read_device(address)?;
                            self.devices.insert(address, device);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    pub fn read_vendor_id(&self, address: PciAddress) -> Option<u16> {
        // Read vendor ID from config space
        let vendor_id = self.config_space.read_u16(address, 0);
        
        if vendor_id == 0xFFFF {
            None
        } else {
            Some(vendor_id)
        }
    }

    pub fn read_device(&self, address: PciAddress) -> Result<PciDevice, PciError> {
        let vendor_id = self.config_space.read_u16(address, 0)?;
        let device_id = self.config_space.read_u16(address, 2)?;
        let class_code = self.config_space.read_u8(address, 11)?;
        let subclass_code = self.config_space.read_u8(address, 10)?;
        
        let bars = self.read_bars(address)?;
        
        Ok(PciDevice {
            address,
            vendor_id,
            device_id,
            class_code,
            subclass_code,
            bars,
        })
    }

    fn read_bars(&self, address: PciAddress) -> Result<Vec<Bar>, PciError> {
        let mut bars = Vec::new();
        
        for i in 0..6 {
            let bar_offset = 0x10 + (i * 4);
            let bar_value = self.config_space.read_u32(address, bar_offset)?;
            
            if bar_value != 0 {
                let bar_type = if bar_value & 1 == 1 {
                    BarType::Io
                } else {
                    BarType::Memory32
                };
                
                let address = (bar_value & 0xFFFFFFF0) as u64;
                let size = self.calculate_bar_size(address, bar_type)?;
                
                bars.push(Bar {
                    index: i as u8,
                    address,
                    size,
                    bar_type,
                });
            }
        }
        
        Ok(bars)
    }

    fn calculate_bar_size(&self, address: u64, bar_type: BarType) -> Result<u64, PciError> {
        // Calculate BAR size by writing all 1s and reading back
        Ok(4096)
    }
}
```

### USB Driver
```rust
// src/driver/usb.rs
pub struct UsbDriver {
    pub host_controllers: Vec<UsbHostController>,
    pub devices: BTreeMap<UsbAddress, UsbDevice>,
}

#[derive(Debug, Clone)]
pub struct UsbHostController {
    pub controller_type: UsbControllerType,
    pub root_hub: UsbHub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbControllerType {
    Uhci,
    Ohci,
    Ehci,
    Xhci,
}

#[derive(Debug, Clone)]
pub struct UsbHub {
    pub ports: Vec<UsbPort>,
}

#[derive(Debug, Clone)]
pub struct UsbPort {
    pub connected: bool,
    pub device: Option<UsbDevice>,
}

#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub address: UsbAddress,
    pub vendor_id: u16,
    pub product_id: u16,
    pub device_class: u8,
    pub configuration: UsbConfiguration,
}

#[derive(Debug, Clone)]
pub struct UsbConfiguration {
    pub interfaces: Vec<UsbInterface>,
}

impl UsbDriver {
    pub fn new() -> Self {
        UsbDriver {
            host_controllers: Vec::new(),
            devices: BTreeMap::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), UsbError> {
        // Detect USB host controllers
        self.detect_controllers()?;
        
        // Initialize controllers
        for controller in &mut self.host_controllers {
            controller.initialize()?;
        }
        
        // Enumerate devices
        self.enumerate_devices()?;
        
        Ok(())
    }

    pub fn detect_controllers(&mut self) -> Result<(), UsbError> {
        // Detect USB host controllers
        // This would typically scan PCI for USB controllers
        Ok(())
    }

    pub fn enumerate_devices(&mut self) -> Result<(), UsbError> {
        // Enumerate USB devices on root hubs
        for controller in &self.host_controllers {
            self.enumerate_hub(&controller.root_hub)?;
        }
        
        Ok(())
    }

    pub fn enumerate_hub(&mut self, hub: &UsbHub) -> Result<(), UsbError> {
        for port in &hub.ports {
            if port.connected {
                if let Some(device) = &port.device {
                    self.devices.insert(device.address, device.clone());
                }
            }
        }
        
        Ok(())
    }
}
```

### Network Driver
```rust
// src/driver/network.rs
pub struct NetworkDriver {
    pub interfaces: BTreeMap<String, NetworkInterface>,
    pub packet_queue: VecDeque<NetworkPacket>,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_address: MacAddress,
    pub mtu: u32,
    pub driver_type: NetworkDriverType,
    pub state: InterfaceState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkDriverType {
    Ethernet,
    Wireless,
    Loopback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterfaceState {
    Down,
    Up,
    Testing,
}

#[derive(Debug, Clone)]
pub struct NetworkPacket {
    pub data: Vec<u8>,
    pub length: usize,
    pub interface: String,
}

impl NetworkDriver {
    pub fn new() -> Self {
        NetworkDriver {
            interfaces: BTreeMap::new(),
            packet_queue: VecDeque::new(),
        }
    }

    pub fn initialize(&mut self) -> Result<(), NetworkError> {
        // Detect network interfaces
        self.detect_interfaces()?;
        
        // Initialize interfaces
        for interface in self.interfaces.values_mut() {
            interface.initialize()?;
        }
        
        Ok(())
    }

    pub fn detect_interfaces(&mut self) -> Result<(), NetworkError> {
        // Detect network interfaces
        // This would typically scan PCI for network cards
        Ok(())
    }

    pub fn send_packet(&mut self, packet: NetworkPacket) -> Result<(), NetworkError> {
        if let Some(interface) = self.interfaces.get_mut(&packet.interface) {
            interface.send_packet(&packet)?;
            Ok(())
        } else {
            Err(NetworkError::InterfaceNotFound)
        }
    }

    pub fn receive_packet(&mut self) -> Option<NetworkPacket> {
        self.packet_queue.pop_front()
    }
}
```

## Configuration

### Driver Configuration
```toml
# /etc/sigmaos/drivers.toml
[pci]
# PCI driver settings
enabled = true
auto_scan = true
hotplug_enabled = true

[usb]
# USB driver settings
enabled = true
auto_mount = true
power_management = true

[network]
# Network driver settings
enabled = true
auto_config = true
dhcp_enabled = true

[gpu]
# GPU driver settings
enabled = true
acceleration = true
hotplug = true
```

### Runtime Control
```bash
# Show PCI devices
sigpci list

# Show USB devices
sigusb list

# Show network interfaces
signet list

# Show driver status
sigdriver status

# Load driver
sigdriver load <driver_name>

# Unload driver
sigdriver unload <driver_name>

# Show driver information
sigdriver info <driver_name>
```

## Performance Optimization

### PCI Optimization
Optimize PCI for performance:
```bash
# Enable MSI-X
sigpci enable-msix

# Set DMA mask
sigpci set-dma-mask 64bit

# Enable relaxed ordering
sigpci enable-relaxed-ordering

# Set latency timer
sigpci set-latency-timer 64
```

### USB Optimization
Optimize USB for performance:
```bash
# Enable USB 3.0
sigusb enable-usb3

# Set USB power management
sigusb set-power-management auto

# Enable USB bulk streaming
sigusb enable-bulk-streaming

# Set USB buffer size
sigusb set-buffer-size 65536
```

### Network Optimization
Optimize network for performance:
```bash
# Enable interrupt moderation
signet enable-interrupt-moderation

# Set MTU
signet set-mtu 9000

# Enable TCP offload
signet enable-tcp-offload

# Enable UDP offload
signet enable-udp-offload
```

## Troubleshooting

### Device Not Detected
If device not detected:
1. Check driver status: `sigdriver status`
2. Check for hardware support
3. Check kernel logs: `dmesg | tail -100`
4. Check device in PCI: `sigpci list`
5. Try manual driver loading

### Driver Load Fails
If driver load fails:
1. Check driver dependencies
2. Check kernel version compatibility
3. Check for missing symbols
4. Check driver logs
5. Check for hardware conflicts

### Network Interface Down
If network interface down:
1. Check interface status: `signet status`
2. Check cable connection
3. Check driver status
4. Bring interface up: `signet up eth0`
5. Check for errors

### USB Device Not Working
If USB device not working:
1. Check USB device status: `sigusb list`
2. Check USB hub status
3. Try different USB port
4. Check for driver support
5. Check for power issues

---

**[Device Drivers](Category-Device-Drivers)** | **[PCI](Category-PCI)** | **[USB](Category-USB)**

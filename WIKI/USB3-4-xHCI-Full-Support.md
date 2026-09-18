# USB3/4 xHCI Full Support

SigmaOS implements full USB 3.x and USB 4.x support with xHCI (eXtensible Host Controller Interface) for high-speed USB connectivity, including USB 3.0, 3.1, 3.2, 4.0, and Thunderbolt integration.

## Overview

USB3/4 xHCI provides:
- USB 3.0 (5 Gbps), 3.1 (10 Gbps), 3.2 (20 Gbps), 4.0 (40 Gbps)
- Thunderbolt 3/4 support
- USB Type-C connector support
- USB Power Delivery (PD)
- USB4 tunneling
- Dynamic bandwidth allocation
- Hot-plug support
- Power management

## Architecture

### USB Stack
```
Application → USB Driver → USB Core → xHCI Driver → Hardware
                                     ↓
                                   USB Hubs
                                     ↓
                                   USB Devices
```

### xHCI Architecture
- **Host Controller**: Manages USB communication
- **Device Context**: Per-device state
- **Endpoint Context**: Per-endpoint configuration
- **Transfer Rings**: Command and data transfer queues
- **Event Rings**: Event notification queues

## Implementation

### xHCI Driver
```rust
// src/driver/usb/xhci.rs
pub struct XhciDriver {
    pub mmio_base: *mut u8,
    pub capabilities: XhciCapabilities,
    pub operational: XhciOperational,
    pub runtime: XhciRuntime,
    pub devices: BTreeMap<u8, UsbDevice>,
    pub slots: BTreeMap<u8, bool>,
}

impl XhciDriver {
    pub fn new(pci_device: &PciDevice) -> Result<Self, UsbError> {
        let mmio_base = pci_device.map_bar(0)?;
        
        let driver = XhciDriver {
            mmio_base,
            capabilities: XhciCapabilities::new(mmio_base),
            operational: XhciOperational::new(mmio_base),
            runtime: XhciRuntime::new(mmio_base),
            devices: BTreeMap::new(),
            slots: BTreeMap::new(),
        };
        
        // Initialize controller
        driver.initialize()?;
        
        Ok(driver)
    }

    pub fn initialize(&self) -> Result<(), UsbError> {
        // Reset controller
        self.reset()?;
        
        // Initialize operational registers
        self.operational.initialize()?;
        
        // Initialize runtime registers
        self.runtime.initialize()?;
        
        // Start controller
        self.start()?;
        
        Ok(())
    }

    pub fn reset(&self) -> Result<(), UsbError> {
        // HCRST - Controller reset
        self.operational.write_register(XHCI_USBCMD, 1 << 1);
        
        // Wait for reset to complete
        while self.operational.read_register(XHCI_USBCMD) & (1 << 1) != 0 {
            std::thread::sleep(Duration::from_millis(1));
        }
        
        Ok(())
    }

    pub fn start(&self) -> Result<(), UsbError> {
        // Run/Stop bit
        self.operational.write_register(XHCI_USBCMD, 1 << 0);
        Ok(())
    }

    pub fn allocate_slot(&mut self) -> Result<u8, UsbError> {
        // Find free slot
        for (slot, used) in self.slots.iter() {
            if !*used {
                self.slots.insert(*slot, true);
                return Ok(*slot);
            }
        }
        
        Err(UsbError::NoSlotsAvailable)
    }

    pub fn enable_slot(&mut self, slot: u8) -> Result<(), UsbError> {
        // Enable slot command
        let command = XhciCommand::EnableSlot { slot };
        self.submit_command(command)?;
        Ok(())
    }
}
```

### USB Device Management
```rust
// src/driver/usb/device.rs
pub struct UsbDevice {
    pub slot_id: u8,
    pub port_id: u8,
    pub speed: UsbSpeed,
    pub device_descriptor: UsbDeviceDescriptor,
    pub configurations: Vec<UsbConfiguration>,
    pub interfaces: Vec<UsbInterface>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbSpeed {
    LowSpeed,
    FullSpeed,
    HighSpeed,
    SuperSpeed,
    SuperSpeedPlus,
    SuperSpeedPlusX2,
}

impl UsbDevice {
    pub fn new(slot_id: u8, port_id: u8, speed: UsbSpeed) -> Self {
        UsbDevice {
            slot_id,
            port_id,
            speed,
            device_descriptor: UsbDeviceDescriptor::default(),
            configurations: Vec::new(),
            interfaces: Vec::new(),
        }
    }

    pub fn read_descriptor(&mut self) -> Result<(), UsbError> {
        // Read device descriptor
        self.device_descriptor = self.read_device_descriptor()?;
        
        // Read configurations
        for i in 0..self.device_descriptor.num_configurations {
            let config = self.read_configuration(i)?;
            self.configurations.push(config);
        }
        
        Ok(())
    }

    pub fn configure(&mut self, config_index: u8) -> Result<(), UsbError> {
        // Set configuration
        self.set_configuration(config_index)?;
        
        // Initialize interfaces
        let config = &self.configurations[config_index as usize];
        for interface in &config.interfaces {
            self.interfaces.push(interface.clone());
        }
        
        Ok(())
    }
}
```

### USB Power Delivery
```rust
// src/driver/usb/pd.rs
pub struct UsbPowerDelivery {
    pub port: UsbPort,
    pub pdo: Vec<PowerDataObject>,
    pub current_pdo: Option<PowerDataObject>,
    pub contract: Option<PdContract>,
}

#[derive(Debug, Clone)]
pub struct PowerDataObject {
    pub voltage_mv: u32,
    pub current_ma: u32,
    pub power_mw: u32,
}

impl UsbPowerDelivery {
    pub fn new(port: UsbPort) -> Self {
        UsbPowerDelivery {
            port,
            pdo: Vec::new(),
            current_pdo: None,
            contract: None,
        }
    }

    pub fn negotiate_power(&mut self, required_power: u32) -> Result<(), UsbError> {
        // Find suitable PDO
        for pdo in &self.pdo {
            if pdo.power_mw >= required_power {
                self.current_pdo = Some(pdo.clone());
                self.request_power(pdo)?;
                return Ok(());
            }
        }
        
        Err(UsbError::PowerNegotiationFailed)
    }

    pub fn request_power(&self, pdo: &PowerDataObject) -> Result<(), UsbError> {
        // Send power request
        self.port.send_pd_request(pdo)?;
        
        // Wait for acceptance
        if !self.port.wait_pd_acceptance()? {
            return Err(UsbError::PowerRequestRejected);
        }
        
        Ok(())
    }
}
```

## Configuration

### USB Configuration
```toml
# /etc/sigmaos/usb.toml
[xhci]
# xHCI driver settings
enabled = true
max_slots = 128
max_ports = 16

[power_delivery]
# USB Power Delivery settings
enabled = true
default_power = 15
max_power = 100

[autosuspend]
# Auto-suspend settings
enabled = true
timeout_seconds = 60
```

### Runtime Control
```bash
# Initialize xHCI
sigusb init

# List USB devices
sigusb list

# View device details
sigusb info <device_id>

# Enable auto-suspend
sigusb autosuspend enable

# Set power limit
sigusb set-power <device_id> 15

# Negotiate power
sigusb negotiate-power <device_id> 30

# View power delivery status
sigusb pd-status <device_id>
```

## Performance Optimization

### Bandwidth Allocation
Optimize bandwidth allocation:
```bash
# Set maximum bandwidth
sigusb set-bandwidth <device_id> high

# Set balanced bandwidth
sigusb set-bandwidth <device_id> balanced

# Set low bandwidth
sigusb set-bandwidth <device_id> low
```

### Power Management
Optimize power management:
```bash
# Enable auto-suspend
sigusb autosuspend enable

# Set timeout
sigusb autosuspend timeout 120

# Disable auto-suspend
sigusb autosuspend disable
```

### USB4
Enable USB4 for best performance:
```bash
# Enable USB4
sigusb usb4 enable

# Enable tunneling
sigusb usb4 tunneling enable

# View USB4 status
sigusb usb4 status
```

## Troubleshooting

### Device Not Detected
If USB device is not detected:
1. Check xHCI status: `sigusb status`
2. Check for devices: `sigusb list`
3. Check kernel logs: `dmesg | tail -50`
4. Reset xHCI: `sigusb reset`
5. Check hardware connections

### Power Negotiation Fails
If power negotiation fails:
1. Check PDO list: `sigusb pd-list <device_id>`
2. Try lower power: `sigusb negotiate-power <device_id> 15`
3. Check USB cable quality
4. Check port capabilities
5. Try different port

### Poor Performance
If USB performance is poor:
1. Check speed: `sigusb info <device_id>`
2. Check bandwidth: `sigusb bandwidth <device_id>`
3. Set high bandwidth: `sigusb set-bandwidth <device_id> high`
4. Disable auto-suspend: `sigusb autosuspend disable`
5. Check for USB hub

### Connection Drops
If connection drops:
1. Check cable quality
2. Check for interference
3. Disable auto-suspend: `sigusb autosuspend disable`
4. Increase timeout: `sigusb autosuspend timeout 300`
5. Check hardware

---

**[Hardware Support](Category-Hardware)** | **[USB Drivers](Device-Drivers)** | **[Thunderbolt Integration](Thunderbolt-Integration)**

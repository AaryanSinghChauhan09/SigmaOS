// SPDX-License-Identifier: MIT
// SigmaOS Hardware Detection - Arch Linux Inspired
// Implements hardware detection similar to Arch's hwdetect

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Hardware component types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareType {
    Cpu,
    Memory,
    Storage,
    Network,
    Graphics,
    Audio,
    Input,
    Usb,
    Pci,
    Acpi,
    Firmware,
    Dmi,
    Smbios,
}

impl HardwareType {
    pub fn as_str(&self) -> &str {
        match self {
            HardwareType::Cpu => "cpu",
            HardwareType::Memory => "memory",
            HardwareType::Storage => "storage",
            HardwareType::Network => "network",
            HardwareType::Graphics => "graphics",
            HardwareType::Audio => "audio",
            HardwareType::Input => "input",
            HardwareType::Usb => "usb",
            HardwareType::Pci => "pci",
            HardwareType::Acpi => "acpi",
            HardwareType::Firmware => "firmware",
            HardwareType::Dmi => "dmi",
            HardwareType::Smbios => "smbios",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "cpu" => Some(HardwareType::Cpu),
            "memory" => Some(HardwareType::Memory),
            "storage" => Some(HardwareType::Storage),
            "network" => Some(HardwareType::Network),
            "graphics" => Some(HardwareType::Graphics),
            "audio" => Some(HardwareType::Audio),
            "input" => Some(HardwareType::Input),
            "usb" => Some(HardwareType::Usb),
            "pci" => Some(HardwareType::Pci),
            "acpi" => Some(HardwareType::Acpi),
            "firmware" => Some(HardwareType::Firmware),
            "dmi" => Some(HardwareType::Dmi),
            "smbios" => Some(HardwareType::Smbios),
            _ => None,
        }
    }
}

/// Hardware information
#[derive(Debug, Clone, PartialEq)]
pub struct HardwareInfo {
    pub device_type: HardwareType,
    pub vendor: String,
    pub product: String,
    pub device_id: String,
    pub subsystem_vendor: Option<String>,
    pub subsystem_device: Option<String>,
    pub driver: Option<String>,
    pub class_code: Option<String>,
    pub irqs: Vec<u32>,
    pub dma_channels: Vec<u32>,
    pub io_ports: Vec<String>,
    pub memory_regions: Vec<String>,
    pub capabilities: Vec<String>,
    pub firmware_version: Option<String>,
    pub serial_number: Option<String>,
    pub uuid: Option<String>,
}

impl HardwareInfo {
    /// Create a new hardware info
    pub fn new(device_type: HardwareType, vendor: String, product: String, device_id: String) -> Self {
        Self {
            device_type,
            vendor,
            product,
            device_id,
            subsystem_vendor: None,
            subsystem_device: None,
            driver: None,
            class_code: None,
            irqs: Vec::new(),
            dma_channels: Vec::new(),
            io_ports: Vec::new(),
            memory_regions: Vec::new(),
            capabilities: Vec::new(),
            firmware_version: None,
            serial_number: None,
            uuid: None,
        }
    }

    /// Set subsystem vendor
    pub fn subsystem_vendor(mut self, vendor: impl Into<String>) -> Self {
        self.subsystem_vendor = Some(vendor.into());
        self
    }

    /// Set subsystem device
    pub fn subsystem_device(mut self, device: impl Into<String>) -> Self {
        self.subsystem_device = Some(device.into());
        self
    }

    /// Set driver
    pub fn driver(mut self, driver: impl Into<String>) -> Self {
        self.driver = Some(driver.into());
        self
    }

    /// Set class code
    pub fn class_code(mut self, code: impl Into<String>) -> Self {
        self.class_code = Some(code.into());
        self
    }

    /// Add IRQ
    pub fn irq(mut self, irq: u32) -> Self {
        self.irqs.push(irq);
        self
    }

    /// Add DMA channel
    pub fn dma_channel(mut self, channel: u32) -> Self {
        self.dma_channels.push(channel);
        self
    }

    /// Add I/O port
    pub fn io_port(mut self, port: impl Into<String>) -> Self {
        self.io_ports.push(port.into());
        self
    }

    /// Add memory region
    pub fn memory_region(mut self, region: impl Into<String>) -> Self {
        self.memory_regions.push(region.into());
        self
    }

    /// Add capability
    pub fn capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities.push(cap.into());
        self
    }

    /// Set firmware version
    pub fn firmware_version(mut self, version: impl Into<String>) -> Self {
        self.firmware_version = Some(version.into());
        self
    }

    /// Set serial number
    pub fn serial_number(mut self, serial: impl Into<String>) -> Self {
        self.serial_number = Some(serial.into());
        self
    }

    /// Set UUID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// Get device description
    pub fn get_description(&self) -> String {
        format!("{} {} ({})", self.vendor, self.product, self.device_id)
    }

    /// Check if device has driver
    pub fn has_driver(&self) -> bool {
        self.driver.is_some()
    }

    /// Check if device is missing driver
    pub fn is_missing_driver(&self) -> bool {
        self.driver.is_none()
    }
}

/// CPU information
#[derive(Debug, Clone, PartialEq)]
pub struct CpuInfo {
    pub vendor: String,
    pub model: String,
    pub model_name: String,
    pub stepping: String,
    pub microcode: Option<String>,
    pub flags: Vec<String>,
    pub bogomips: f64,
    pub mhz: u32,
    pub cache_size: u32,
    pub physical_id: u32,
    pub core_id: u32,
    pub cpu_cores: u32,
    pub siblings: u32,
}

impl CpuInfo {
    /// Create a new CPU info
    pub fn new(vendor: String, model: String, model_name: String) -> Self {
        Self {
            vendor,
            model,
            model_name,
            stepping: String::new(),
            microcode: None,
            flags: Vec::new(),
            bogomips: 0.0,
            mhz: 0,
            cache_size: 0,
            physical_id: 0,
            core_id: 0,
            cpu_cores: 1,
            siblings: 1,
        }
    }

    /// Add flag
    pub fn add_flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }

    /// Set microcode
    pub fn microcode(mut self, microcode: impl Into<String>) -> Self {
        self.microcode = Some(microcode.into());
        self
    }

    /// Set bogomips
    pub fn bogomips(mut self, bogomips: f64) -> Self {
        self.bogomips = bogomips;
        self
    }

    /// Set MHz
    pub fn mhz(mut self, mhz: u32) -> Self {
        self.mhz = mhz;
        self
    }

    /// Set cache size
    pub fn cache_size(mut self, size: u32) -> Self {
        self.cache_size = size;
        self
    }

    /// Set physical ID
    pub fn physical_id(mut self, id: u32) -> Self {
        self.physical_id = id;
        self
    }

    /// Set core ID
    pub fn core_id(mut self, id: u32) -> Self {
        self.core_id = id;
        self
    }

    /// Set CPU cores
    pub fn cpu_cores(mut self, cores: u32) -> Self {
        self.cpu_cores = cores;
        self
    }

    /// Set siblings
    pub fn siblings(mut self, siblings: u32) -> Self {
        self.siblings = siblings;
        self
    }
}

/// Memory information
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryInfo {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub memory_type: String,
    pub memory_speed: u32,
    pub channels: u32,
}

impl MemoryInfo {
    /// Create a new memory info
    pub fn new(total: u64, free: u64) -> Self {
        Self {
            total,
            free,
            available: free,
            buffers: 0,
            cached: 0,
            swap_total: 0,
            swap_free: 0,
            memory_type: String::new(),
            memory_speed: 0,
            channels: 1,
        }
    }

    /// Set available memory
    pub fn available(mut self, available: u64) -> Self {
        self.available = available;
        self
    }

    /// Set buffers
    pub fn buffers(mut self, buffers: u64) -> Self {
        self.buffers = buffers;
        self
    }

    /// Set cached
    pub fn cached(mut self, cached: u64) -> Self {
        self.cached = cached;
        self
    }

    /// Set swap info
    pub fn swap(mut self, total: u64, free: u64) -> Self {
        self.swap_total = total;
        self.swap_free = free;
        self
    }

    /// Set memory type
    pub fn memory_type(mut self, mem_type: impl Into<String>) -> Self {
        self.memory_type = mem_type.into();
        self
    }

    /// Set memory speed
    pub fn memory_speed(mut self, speed: u32) -> Self {
        self.memory_speed = speed;
        self
    }

    /// Set channels
    pub fn channels(mut self, channels: u32) -> Self {
        self.channels = channels;
        self
    }

    /// Get used memory
    pub fn used(&self) -> u64 {
        self.total.saturating_sub(self.available)
    }

    /// Get memory usage percentage
    pub fn usage_percent(&self) -> f64 {
        if self.total > 0 {
            (self.used() as f64 / self.total as f64) * 100.0
        } else {
            0.0
        }
    }
}

/// Network interface information
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInfo {
    pub name: String,
    pub mac_address: String,
    pub ip_addresses: Vec<String>,
    pub ipv6_addresses: Vec<String>,
    pub driver: String,
    pub vendor: String,
    pub product: String,
    pub speed: u32,
    pub duplex: String,
    pub mtu: u32,
    pub operstate: String,
    pub wireless: bool,
    pub frequency: Option<u32>,
    pub channel: Option<u32>,
    pub signal_strength: Option<i32>,
}

impl NetworkInfo {
    /// Create a new network info
    pub fn new(name: String, mac_address: String, driver: String, vendor: String, product: String) -> Self {
        Self {
            name,
            mac_address,
            ip_addresses: Vec::new(),
            ipv6_addresses: Vec::new(),
            driver,
            vendor,
            product,
            speed: 0,
            duplex: String::new(),
            mtu: 1500,
            operstate: String::new(),
            wireless: false,
            frequency: None,
            channel: None,
            signal_strength: None,
        }
    }

    /// Add IP address
    pub fn add_ip(mut self, ip: impl Into<String>) -> Self {
        self.ip_addresses.push(ip.into());
        self
    }

    /// Add IPv6 address
    pub fn add_ipv6(mut self, ipv6: impl Into<String>) -> Self {
        self.ipv6_addresses.push(ipv6.into());
        self
    }

    /// Set speed
    pub fn speed(mut self, speed: u32) -> Self {
        self.speed = speed;
        self
    }

    /// Set duplex
    pub fn duplex(mut self, duplex: impl Into<String>) -> Self {
        self.duplex = duplex.into();
        self
    }

    /// Set MTU
    pub fn mtu(mut self, mtu: u32) -> Self {
        self.mtu = mtu;
        self
    }

    /// Set operstate
    pub fn operstate(mut self, state: impl Into<String>) -> Self {
        self.operstate = state.into();
        self
    }

    /// Set wireless
    pub fn wireless(mut self, wireless: bool) -> Self {
        self.wireless = wireless;
        self
    }

    /// Set frequency
    pub fn frequency(mut self, freq: u32) -> Self {
        self.frequency = Some(freq);
        self
    }

    /// Set channel
    pub fn channel(mut self, channel: u32) -> Self {
        self.channel = Some(channel);
        self
    }

    /// Set signal strength
    pub fn signal_strength(mut self, strength: i32) -> Self {
        self.signal_strength = Some(strength);
        self
    }

    /// Check if interface is up
    pub fn is_up(&self) -> bool {
        self.operstate == "up" || self.operstate == "unknown"
    }

    /// Check if interface has IP
    pub fn has_ip(&self) -> bool {
        !self.ip_addresses.is_empty() || !self.ipv6_addresses.is_empty()
    }
}

/// Storage device information
#[derive(Debug, Clone, PartialEq)]
pub struct StorageInfo {
    pub name: String,
    pub device_path: String,
    pub model: String,
    pub serial: String,
    pub vendor: String,
    pub size: u64,
    pub sector_size: u32,
    pub rotational: bool,
    pub filesystem: Option<String>,
    pub mount_points: Vec<String>,
    pub partitions: Vec<PartitionInfo>,
    pub ssd: bool,
    pub nvme: bool,
    pub sata: bool,
    pub usb: bool,
    pub raidable: bool,
}

impl StorageInfo {
    /// Create a new storage info
    pub fn new(name: String, device_path: String, model: String, vendor: String, size: u64) -> Self {
        Self {
            name,
            device_path,
            model,
            serial: String::new(),
            vendor,
            size,
            sector_size: 512,
            rotational: true,
            filesystem: None,
            mount_points: Vec::new(),
            partitions: Vec::new(),
            ssd: false,
            nvme: false,
            sata: false,
            usb: false,
            raidable: false,
        }
    }

    /// Set serial
    pub fn serial(mut self, serial: impl Into<String>) -> Self {
        self.serial = serial.into();
        self
    }

    /// Set sector size
    pub fn sector_size(mut self, size: u32) -> Self {
        self.sector_size = size;
        self
    }

    /// Set rotational
    pub fn rotational(mut self, rotational: bool) -> Self {
        self.rotational = rotational;
        self
    }

    /// Set filesystem
    pub fn filesystem(mut self, fs: impl Into<String>) -> Self {
        self.filesystem = Some(fs.into());
        self
    }

    /// Add mount point
    pub fn mount_point(mut self, mount: impl Into<String>) -> Self {
        self.mount_points.push(mount.into());
        self
    }

    /// Add partition
    pub fn partition(mut self, partition: PartitionInfo) -> Self {
        self.partitions.push(partition);
        self
    }

    /// Set SSD
    pub fn ssd(mut self, ssd: bool) -> Self {
        self.ssd = ssd;
        self
    }

    /// Set NVMe
    pub fn nvme(mut self, nvme: bool) -> Self {
        self.nvme = nvme;
        self
    }

    /// Set SATA
    pub fn sata(mut self, sata: bool) -> Self {
        self.sata = sata;
        self
    }

    /// Set USB
    pub fn usb(mut self, usb: bool) -> Self {
        self.usb = usb;
        self
    }

    /// Set raidable
    pub fn raidable(mut self, raidable: bool) -> Self {
        self.raidable = raidable;
        self
    }

    /// Get size in human readable format
    pub fn get_size_human(&self) -> String {
        if self.size >= 1_000_000_000_000 {
            format!("{:.1} TB", self.size as f64 / 1_000_000_000_000.0)
        } else if self.size >= 1_000_000_000 {
            format!("{:.1} GB", self.size as f64 / 1_000_000_000.0)
        } else if self.size >= 1_000_000 {
            format!("{:.1} MB", self.size as f64 / 1_000_000.0)
        } else {
            format!("{} bytes", self.size)
        }
    }
}

/// Partition information
#[derive(Debug, Clone, PartialEq)]
pub struct PartitionInfo {
    pub number: u32,
    pub name: String,
    pub size: u64,
    pub start_sector: u64,
    pub end_sector: u64,
    pub filesystem: Option<String>,
    pub mount_point: Option<String>,
    pub flags: Vec<String>,
    pub uuid: Option<String>,
    pub label: Option<String>,
    pub type_id: Option<String>,
}

impl PartitionInfo {
    /// Create a new partition info
    pub fn new(number: u32, name: String, size: u64) -> Self {
        Self {
            number,
            name,
            size,
            start_sector: 0,
            end_sector: 0,
            filesystem: None,
            mount_point: None,
            flags: Vec::new(),
            uuid: None,
            label: None,
            type_id: None,
        }
    }

    /// Set start sector
    pub fn start_sector(mut self, sector: u64) -> Self {
        self.start_sector = sector;
        self
    }

    /// Set end sector
    pub fn end_sector(mut self, sector: u64) -> Self {
        self.end_sector = sector;
        self
    }

    /// Set filesystem
    pub fn filesystem(mut self, fs: impl Into<String>) -> Self {
        self.filesystem = Some(fs.into());
        self
    }

    /// Set mount point
    pub fn mount_point(mut self, mount: impl Into<String>) -> Self {
        self.mount_point = Some(mount.into());
        self
    }

    /// Add flag
    pub fn flag(mut self, flag: impl Into<String>) -> Self {
        self.flags.push(flag.into());
        self
    }

    /// Set UUID
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set type ID
    pub fn type_id(mut self, type_id: impl Into<String>) -> Self {
        self.type_id = Some(type_id.into());
        self
    }
}

/// Graphics card information
#[derive(Debug, Clone, PartialEq)]
pub struct GraphicsInfo {
    pub vendor: String,
    pub product: String,
    pub device_id: String,
    pub driver: String,
    pub vram: u64,
    pub bus_id: String,
    pub capabilities: Vec<String>,
    pub resolutions: Vec<String>,
    pub refresh_rates: Vec<u32>,
    pub connected_monitors: Vec<String>,
}

impl GraphicsInfo {
    /// Create a new graphics info
    pub fn new(vendor: String, product: String, device_id: String, driver: String) -> Self {
        Self {
            vendor,
            product,
            device_id,
            driver,
            vram: 0,
            bus_id: String::new(),
            capabilities: Vec::new(),
            resolutions: Vec::new(),
            refresh_rates: Vec::new(),
            connected_monitors: Vec::new(),
        }
    }

    /// Set VRAM
    pub fn vram(mut self, vram: u64) -> Self {
        self.vram = vram;
        self
    }

    /// Set bus ID
    pub fn bus_id(mut self, bus_id: impl Into<String>) -> Self {
        self.bus_id = bus_id.into();
        self
    }

    /// Add capability
    pub fn capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities.push(cap.into());
        self
    }

    /// Add resolution
    pub fn resolution(mut self, res: impl Into<String>) -> Self {
        self.resolutions.push(res.into());
        self
    }

    /// Add refresh rate
    pub fn refresh_rate(mut self, rate: u32) -> Self {
        self.refresh_rates.push(rate);
        self
    }

    /// Add monitor
    pub fn monitor(mut self, monitor: impl Into<String>) -> Self {
        self.connected_monitors.push(monitor.into());
        self
    }
}

/// Audio device information
#[derive(Debug, Clone, PartialEq)]
pub struct AudioInfo {
    pub name: String,
    pub card_id: String,
    pub vendor: String,
    pub product: String,
    pub driver: String,
    pub capabilities: Vec<String>,
    pub sample_rates: Vec<u32>,
    pub bit_depths: Vec<u32>,
    pub channels: u32,
}

impl AudioInfo {
    /// Create a new audio info
    pub fn new(name: String, card_id: String, vendor: String, product: String, driver: String) -> Self {
        Self {
            name,
            card_id,
            vendor,
            product,
            driver,
            capabilities: Vec::new(),
            sample_rates: Vec::new(),
            bit_depths: Vec::new(),
            channels: 2,
        }
    }

    /// Add capability
    pub fn capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities.push(cap.into());
        self
    }

    /// Add sample rate
    pub fn sample_rate(mut self, rate: u32) -> Self {
        self.sample_rates.push(rate);
        self
    }

    /// Add bit depth
    pub fn bit_depth(mut self, depth: u32) -> Self {
        self.bit_depths.push(depth);
        self
    }

    /// Set channels
    pub fn channels(mut self, channels: u32) -> Self {
        self.channels = channels;
        self
    }
}

/// Input device information
#[derive(Debug, Clone, PartialEq)]
pub struct InputInfo {
    pub name: String,
    pub device_path: String,
    pub driver: String,
    pub vendor: String,
    pub product: String,
    pub input_type: String,
    pub capabilities: Vec<String>,
    pub buttons: u32,
    pub axes: u32,
    pub leds: Vec<String>,
}

impl InputInfo {
    /// Create a new input info
    pub fn new(name: String, device_path: String, driver: String, vendor: String, product: String, input_type: String) -> Self {
        Self {
            name,
            device_path,
            driver,
            vendor,
            product,
            input_type,
            capabilities: Vec::new(),
            buttons: 0,
            axes: 0,
            leds: Vec::new(),
        }
    }

    /// Add capability
    pub fn capability(mut self, cap: impl Into<String>) -> Self {
        self.capabilities.push(cap.into());
        self
    }

    /// Set buttons
    pub fn buttons(mut self, buttons: u32) -> Self {
        self.buttons = buttons;
        self
    }

    /// Set axes
    pub fn axes(mut self, axes: u32) -> Self {
        self.axes = axes;
        self
    }

    /// Add LED
    pub fn led(mut self, led: impl Into<String>) -> Self {
        self.leds.push(led.into());
        self
    }
}

/// USB device information
#[derive(Debug, Clone, PartialEq)]
pub struct UsbInfo {
    pub bus_id: String,
    pub device_id: String,
    pub vendor: String,
    pub product: String,
    pub manufacturer: String,
    pub product_name: String,
    pub serial_number: String,
    pub device_class: String,
    pub driver: Option<String>,
    pub speed: String,
    pub max_power: u32,
    pub is_hub: bool,
    pub children: Vec<String>,
}

impl UsbInfo {
    /// Create a new USB info
    pub fn new(bus_id: String, device_id: String, vendor: String, product: String) -> Self {
        Self {
            bus_id,
            device_id,
            vendor,
            product,
            manufacturer: String::new(),
            product_name: String::new(),
            serial_number: String::new(),
            device_class: String::new(),
            driver: None,
            speed: String::new(),
            max_power: 0,
            is_hub: false,
            children: Vec::new(),
        }
    }

    /// Set manufacturer
    pub fn manufacturer(mut self, manufacturer: impl Into<String>) -> Self {
        self.manufacturer = manufacturer.into();
        self
    }

    /// Set product name
    pub fn product_name(mut self, name: impl Into<String>) -> Self {
        self.product_name = name.into();
        self
    }

    /// Set serial number
    pub fn serial_number(mut self, serial: impl Into<String>) -> Self {
        self.serial_number = serial.into();
        self
    }

    /// Set device class
    pub fn device_class(mut self, class: impl Into<String>) -> Self {
        self.device_class = class.into();
        self
    }

    /// Set driver
    pub fn driver(mut self, driver: impl Into<String>) -> Self {
        self.driver = Some(driver.into());
        self
    }

    /// Set speed
    pub fn speed(mut self, speed: impl Into<String>) -> Self {
        self.speed = speed.into();
        self
    }

    /// Set max power
    pub fn max_power(mut self, power: u32) -> Self {
        self.max_power = power;
        self
    }

    /// Set is hub
    pub fn is_hub(mut self, is_hub: bool) -> Self {
        self.is_hub = is_hub;
        self
    }

    /// Add child
    pub fn child(mut self, child: impl Into<String>) -> Self {
        self.children.push(child.into());
        self
    }
}

/// Hardware detection manager
#[derive(Debug, Clone)]
pub struct HardwareDetector {
    pub cpus: Vec<CpuInfo>,
    pub memory: MemoryInfo,
    pub storage_devices: Vec<StorageInfo>,
    pub network_interfaces: Vec<NetworkInfo>,
    pub graphics_cards: Vec<GraphicsInfo>,
    pub audio_devices: Vec<AudioInfo>,
    pub input_devices: Vec<InputInfo>,
    pub usb_devices: Vec<UsbInfo>,
    pub pci_devices: Vec<HardwareInfo>,
    pub acpi_devices: Vec<HardwareInfo>,
    pub dmi_info: Option<HardwareInfo>,
    pub smbios_info: Option<HardwareInfo>,
    pub detected_at: u64,
}

impl HardwareDetector {
    /// Create a new hardware detector
    pub fn new() -> Self {
        Self {
            cpus: Vec::new(),
            memory: MemoryInfo::new(0, 0),
            storage_devices: Vec::new(),
            network_interfaces: Vec::new(),
            graphics_cards: Vec::new(),
            audio_devices: Vec::new(),
            input_devices: Vec::new(),
            usb_devices: Vec::new(),
            pci_devices: Vec::new(),
            acpi_devices: Vec::new(),
            dmi_info: None,
            smbios_info: None,
            detected_at: 0,
        }
    }

    /// Detect all hardware
    pub fn detect_all(&mut self) -> io::Result<()> {
        self.detect_cpus()?;
        self.detect_memory()?;
        self.detect_storage()?;
        self.detect_network()?;
        self.detect_graphics()?;
        self.detect_audio()?;
        self.detect_input()?;
        self.detect_usb()?;
        self.detect_pci()?;
        self.detect_acpi()?;
        self.detect_dmi()?;
        self.detect_smbios()?;

        self.detected_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Ok(())
    }

    /// Detect CPUs
    pub fn detect_cpus(&mut self) -> io::Result<()> {
        // Read CPU information from /proc/cpuinfo
        let cpuinfo_path = Path::new("/proc/cpuinfo");
        if cpuinfo_path.exists() {
            let file = fs::File::open(cpuinfo_path)?;
            let reader = BufReader::new(file);

            let mut current_cpu = BTreeMap::new();
            let mut cpus = Vec::new();

            for line in reader.lines() {
                let line = line?;
                let line = line.trim();

                if line.is_empty() {
                    if !current_cpu.is_empty() {
                        if let Some(cpu) = self.parse_cpu_info(&current_cpu) {
                            cpus.push(cpu);
                        }
                        current_cpu.clear();
                    }
                    continue;
                }

                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim().to_string();
                    let value = value.trim().to_string();
                    current_cpu.insert(key, value);
                }
            }

            // Process last CPU
            if !current_cpu.is_empty() {
                if let Some(cpu) = self.parse_cpu_info(&current_cpu) {
                    cpus.push(cpu);
                }
            }

            self.cpus = cpus;
        }

        Ok(())
    }

    /// Parse CPU info from proc
    fn parse_cpu_info(&self, cpu_map: &BTreeMap<String, String>) -> Option<CpuInfo> {
        let vendor = cpu_map.get("vendor_id")
            .or_else(|| cpu_map.get("vendor"))
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        let model = cpu_map.get("model")
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        let model_name = cpu_map.get("model name")
            .or_else(|| cpu_map.get("Processor"))
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        let mut cpu = CpuInfo::new(vendor, model, model_name);

        // Set optional fields
        if let Some(stepping) = cpu_map.get("stepping") {
            cpu.stepping = stepping.clone();
        }

        if let Some(flags) = cpu_map.get("flags").or_else(|| cpu_map.get("Features")) {
            cpu.flags = flags.split_whitespace().map(String::from).collect();
        }

        if let Some(mhz) = cpu_map.get("cpu MHz") {
            if let Ok(mhz_val) = mhz.parse::<u32>() {
                cpu.mhz = mhz_val;
            }
        }

        if let Some(cache) = cpu_map.get("cache size") {
            if let Ok(cache_val) = cache.split_whitespace().next().unwrap_or("0").parse::<u32>() {
                cpu.cache_size = cache_val;
            }
        }

        if let Some(physical_id) = cpu_map.get("physical id") {
            if let Ok(id) = physical_id.parse::<u32>() {
                cpu.physical_id = id;
            }
        }

        if let Some(core_id) = cpu_map.get("core id") {
            if let Ok(id) = core_id.parse::<u32>() {
                cpu.core_id = id;
            }
        }

        if let Some(cores) = cpu_map.get("cpu cores") {
            if let Ok(cores_val) = cores.parse::<u32>() {
                cpu.cpu_cores = cores_val;
            }
        }

        if let Some(siblings) = cpu_map.get("siblings") {
            if let Ok(siblings_val) = siblings.parse::<u32>() {
                cpu.siblings = siblings_val;
            }
        }

        Some(cpu)
    }

    /// Detect memory
    pub fn detect_memory(&mut self) -> io::Result<()> {
        // Read memory information from /proc/meminfo
        let meminfo_path = Path::new("/proc/meminfo");
        if meminfo_path.exists() {
            let file = fs::File::open(meminfo_path)?;
            let reader = BufReader::new(file);

            let mut total = 0;
            let mut free = 0;
            let mut available = 0;
            let mut buffers = 0;
            let mut cached = 0;
            let mut swap_total = 0;
            let mut swap_free = 0;

            for line in reader.lines() {
                let line = line?;
                let line = line.trim();

                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value.trim();

                    let size_kb = value.split_whitespace()
                        .next()
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0) * 1024; // Convert KB to bytes

                    match key {
                        "MemTotal" => total = size_kb,
                        "MemFree" => free = size_kb,
                        "MemAvailable" => available = size_kb,
                        "Buffers" => buffers = size_kb,
                        "Cached" => cached = size_kb,
                        "SwapTotal" => swap_total = size_kb,
                        "SwapFree" => swap_free = size_kb,
                        _ => {}
                    }
                }
            }

            self.memory = MemoryInfo::new(total, free)
                .available(available)
                .buffers(buffers)
                .cached(cached)
                .swap(swap_total, swap_free);
        }

        Ok(())
    }

    /// Detect storage devices
    pub fn detect_storage(&mut self) -> io::Result<()> {
        // Read storage information from /sys/block
        let block_path = Path::new("/sys/block");
        if block_path.exists() {
            for entry in fs::read_dir(block_path)? {
                let entry = entry?;
                let path = entry.path();

                let name = entry.file_name().to_string_lossy().to_string();

                // Skip loop devices and RAM disks
                if name.starts_with("loop") || name.starts_with("ram") {
                    continue;
                }

                // Read device information
                if let Ok(size_str) = fs::read_to_string(path.join("size")) {
                    if let Ok(size_sectors) = size_str.trim().parse::<u64>() {
                        let size = size_sectors * 512; // Assume 512-byte sectors

                        let mut storage = StorageInfo::new(
                            name.clone(),
                            format!("/dev/{}", name),
                            String::new(),
                            String::new(),
                            size,
                        );

                        // Read device properties
                        if let Ok(rotational) = fs::read_to_string(path.join("queue/rotational")) {
                            storage.rotational = rotational.trim() == "1";
                        }

                        if let Ok(nvme) = fs::read_to_string(path.join("device/transport")) {
                            storage.nvme = nvme.trim() == "nvme";
                        }

                        // Read device model
                        if let Ok(model) = fs::read_to_string(path.join("device/model")) {
                            storage.model = model.trim().to_string();
                        }

                        // Read vendor
                        if let Ok(vendor) = fs::read_to_string(path.join("device/vendor")) {
                            storage.vendor = vendor.trim().to_string();
                        }

                        // Read serial
                        if let Ok(serial) = fs::read_to_string(path.join("device/serial")) {
                            storage.serial = serial.trim().to_string();
                        }

                        self.storage_devices.push(storage);
                    }
                }
            }
        }

        Ok(())
    }

    /// Detect network interfaces
    pub fn detect_network(&mut self) -> io::Result<()> {
        // Read network interfaces from /sys/class/net
        let net_path = Path::new("/sys/class/net");
        if net_path.exists() {
            for entry in fs::read_dir(net_path)? {
                let entry = entry?;
                let path = entry.path();

                let name = entry.file_name().to_string_lossy().to_string();

                // Skip loopback interface
                if name == "lo" {
                    continue;
                }

                let mut network = NetworkInfo::new(
                    name.clone(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                );

                // Read MAC address
                if let Ok(mac) = fs::read_to_string(path.join("address")) {
                    network.mac_address = mac.trim().to_string();
                }

                // Read operstate
                if let Ok(state) = fs::read_to_string(path.join("operstate")) {
                    network.operstate = state.trim().to_string();
                }

                // Read MTU
                if let Ok(mtu) = fs::read_to_string(path.join("mtu")) {
                    if let Ok(mtu_val) = mtu.trim().parse::<u32>() {
                        network.mtu = mtu_val;
                    }
                }

                // Read speed
                if let Ok(speed) = fs::read_to_string(path.join("speed")) {
                    if let Ok(speed_val) = speed.trim().parse::<u32>() {
                        network.speed = speed_val;
                    }
                }

                // Read duplex
                if let Ok(duplex) = fs::read_to_string(path.join("duplex")) {
                    network.duplex = duplex.trim().to_string();
                }

                // Check if wireless
                if path.join("wireless").exists() {
                    network.wireless = true;
                }

                self.network_interfaces.push(network);
            }
        }

        Ok(())
    }

    /// Detect graphics cards
    pub fn detect_graphics(&mut self) -> io::Result<()> {
        // Read graphics information from /sys/class/drm
        let drm_path = Path::new("/sys/class/drm");
        if drm_path.exists() {
            for entry in fs::read_dir(drm_path)? {
                let entry = entry?;
                let path = entry.path();

                let name = entry.file_name().to_string_lossy().to_string();

                // Skip connectors (card0-HDMI-A-1, etc.)
                if !name.starts_with("card") {
                    continue;
                }

                let mut graphics = GraphicsInfo::new(
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                );

                // Read device information
                if let Ok(vendor) = fs::read_to_string(path.join("device/vendor")) {
                    graphics.vendor = vendor.trim().to_string();
                }

                if let Ok(device) = fs::read_to_string(path.join("device/device")) {
                    graphics.device_id = device.trim().to_string();
                }

                if let Ok(driver) = fs::read_to_string(path.join("device/driver/uevent")) {
                    for line in driver.lines() {
                        if let Some((key, value)) = line.split_once('=') {
                            if key == "DRIVER" {
                                graphics.driver = value.to_string();
                            }
                        }
                    }
                }

                // Read VRAM size
                if let Ok(vram) = fs::read_to_string(path.join("device/mem_info_vram_total")) {
                    if let Ok(vram_val) = vram.trim().parse::<u64>() {
                        graphics.vram = vram_val;
                    }
                }

                self.graphics_cards.push(graphics);
            }
        }

        Ok(())
    }

    /// Detect audio devices
    pub fn detect_audio(&mut self) -> io::Result<()> {
        // Read audio information from /proc/asound/cards
        let cards_path = Path::new("/proc/asound/cards");
        if cards_path.exists() {
            let file = fs::File::open(cards_path)?;
            let reader = BufReader::new(file);

            for line in reader.lines() {
                let line = line?;
                let line = line.trim();

                // Parse lines like: " 0 [HDMI          ]: HDA-Intel - HDA Intel HDMI"
                if line.starts_with(' ') && line.contains('[') {
                    let parts: Vec<&str> = line.splitn(4, ' ').collect();
                    if parts.len() >= 4 {
                        let card_id = parts[0].trim();
                        let name = parts[2].trim_matches(['[', ']']);
                        let description = parts[3];

                        let audio = AudioInfo::new(
                            name.to_string(),
                            card_id.to_string(),
                            String::new(),
                            description.to_string(),
                            String::new(),
                        );

                        self.audio_devices.push(audio);
                    }
                }
            }
        }

        Ok(())
    }

    /// Detect input devices
    pub fn detect_input(&mut self) -> io::Result<()> {
        // Read input devices from /proc/bus/input/devices
        let input_path = Path::new("/proc/bus/input/devices");
        if input_path.exists() {
            let file = fs::File::open(input_path)?;
            let reader = BufReader::new(file);

            let mut current_device = BTreeMap::new();

            for line in reader.lines() {
                let line = line?;
                let line = line.trim();

                if line.is_empty() {
                    if !current_device.is_empty() {
                        if let Some(input) = self.parse_input_device(&current_device) {
                            self.input_devices.push(input);
                        }
                        current_device.clear();
                    }
                    continue;
                }

                if let Some((key, value)) = line.split_once('=') {
                    let key = key.trim().to_string();
                    let value = value.trim().to_string();
                    current_device.insert(key, value);
                }
            }

            // Process last device
            if !current_device.is_empty() {
                if let Some(input) = self.parse_input_device(&current_device) {
                    self.input_devices.push(input);
                }
            }
        }

        Ok(())
    }

    /// Parse input device info
    fn parse_input_device(&self, device_map: &BTreeMap<String, String>) -> Option<InputInfo> {
        let name = device_map.get("N: Name")
            .cloned()
            .unwrap_or_else(|| "Unknown".to_string());

        let device_path = device_map.get("H: Handlers")
            .and_then(|h| h.split_whitespace().find(|s| s.starts_with("event")))
            .map(|s| format!("/dev/input/{}", s))
            .unwrap_or_else(|| String::new());

        let vendor = device_map.get("I: Vendor")
            .and_then(|v| v.split_whitespace().nth(1))
            .unwrap_or("0000")
            .to_string();

        let product = device_map.get("I: Product")
            .and_then(|p| p.split_whitespace().nth(1))
            .unwrap_or("0000")
            .to_string();

        let input_type = device_map.get("B: EV")
            .map(|_| "unknown".to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let mut input = InputInfo::new(name, device_path, String::new(), vendor, product, input_type);

        // Parse capabilities
        if let Some(ev) = device_map.get("B: EV") {
            let ev_bits = ev.trim_start_matches("0x");
            if let Ok(ev_val) = u32::from_str_radix(ev_bits, 16) {
                if ev_val & 0x01 != 0 { // EV_SYN
                    input = input.capability("sync");
                }
                if ev_val & 0x02 != 0 { // EV_KEY
                    input = input.capability("keys");
                }
                if ev_val & 0x04 != 0 { // EV_REL
                    input = input.capability("relative");
                }
                if ev_val & 0x08 != 0 { // EV_ABS
                    input = input.capability("absolute");
                }
            }
        }

        // Parse buttons
        if let Some(bn) = device_map.get("B: KEY") {
            let key_bits = bn.trim_start_matches("0x");
            if let Ok(key_val) = u32::from_str_radix(key_bits, 16) {
                input = input.buttons(key_val.count_ones());
            }
        }

        // Parse axes
        if let Some(abs) = device_map.get("B: ABS") {
            let abs_bits = abs.trim_start_matches("0x");
            if let Ok(abs_val) = u32::from_str_radix(abs_bits, 16) {
                input = input.axes(abs_val.count_ones());
            }
        }

        Some(input)
    }

    /// Detect USB devices
    pub fn detect_usb(&mut self) -> io::Result<()> {
        // Read USB devices from /sys/bus/usb/devices
        let usb_path = Path::new("/sys/bus/usb/devices");
        if usb_path.exists() {
            for entry in fs::read_dir(usb_path)? {
                let entry = entry?;
                let path = entry.path();

                let name = entry.file_name().to_string_lossy().to_string();

                // Skip USB interfaces (they have format like 1-1:1.0)
                if name.contains(':') {
                    continue;
                }

                let mut usb = UsbInfo::new(
                    String::new(),
                    name.clone(),
                    String::new(),
                    String::new(),
                );

                // Read bus ID
                if let Ok(busnum) = fs::read_to_string(path.join("busnum")) {
                    usb.bus_id = busnum.trim().to_string();
                }

                // Read device ID
                if let Ok(devnum) = fs::read_to_string(path.join("devnum")) {
                    usb.device_id = devnum.trim().to_string();
                }

                // Read vendor
                if let Ok(vendor) = fs::read_to_string(path.join("idVendor")) {
                    usb.vendor = vendor.trim().to_string();
                }

                // Read product
                if let Ok(product) = fs::read_to_string(path.join("idProduct")) {
                    usb.product = product.trim().to_string();
                }

                // Read manufacturer
                if let Ok(manufacturer) = fs::read_to_string(path.join("manufacturer")) {
                    usb.manufacturer = manufacturer.trim().to_string();
                }

                // Read product name
                if let Ok(product_name) = fs::read_to_string(path.join("product")) {
                    usb.product_name = product_name.trim().to_string();
                }

                // Read serial number
                if let Ok(serial) = fs::read_to_string(path.join("serial")) {
                    usb.serial_number = serial.trim().to_string();
                }

                // Read device class
                if let Ok(class) = fs::read_to_string(path.join("bDeviceClass")) {
                    usb.device_class = class.trim().to_string();
                }

                // Check if hub
                if path.join("bDeviceClass").exists() {
                    if let Ok(class) = fs::read_to_string(path.join("bDeviceClass")) {
                        usb.is_hub = class.trim() == "09";
                    }
                }

                self.usb_devices.push(usb);
            }
        }

        Ok(())
    }

    /// Detect PCI devices
    pub fn detect_pci(&mut self) -> io::Result<()> {
        // Read PCI devices from /sys/bus/pci/devices
        let pci_path = Path::new("/sys/bus/pci/devices");
        if pci_path.exists() {
            for entry in fs::read_dir(pci_path)? {
                let entry = entry?;
                let path = entry.path();

                let name = entry.file_name().to_string_lossy().to_string();

                let mut pci = HardwareInfo::new(
                    HardwareType::Pci,
                    String::new(),
                    String::new(),
                    name.clone(),
                );

                // Read vendor
                if let Ok(vendor) = fs::read_to_string(path.join("vendor")) {
                    pci.vendor = vendor.trim().to_string();
                }

                // Read device
                if let Ok(device) = fs::read_to_string(path.join("device")) {
                    pci.product = device.trim().to_string();
                }

                // Read subsystem vendor
                if let Ok(subsystem_vendor) = fs::read_to_string(path.join("subsystem_vendor")) {
                    pci.subsystem_vendor = Some(subsystem_vendor.trim().to_string());
                }

                // Read subsystem device
                if let Ok(subsystem_device) = fs::read_to_string(path.join("subsystem_device")) {
                    pci.subsystem_device = Some(subsystem_device.trim().to_string());
                }

                // Read class
                if let Ok(class) = fs::read_to_string(path.join("class")) {
                    pci.class_code = Some(class.trim().to_string());
                }

                // Read driver
                if let Ok(driver_link) = fs::read_link(path.join("driver")) {
                    pci.driver = driver_link.file_name().map(|s| s.to_string_lossy().to_string());
                }

                self.pci_devices.push(pci);
            }
        }

        Ok(())
    }

    /// Detect ACPI devices
    pub fn detect_acpi(&mut self) -> io::Result<()> {
        // Read ACPI devices from /proc/acpi
        let acpi_path = Path::new("/proc/acpi");
        if acpi_path.exists() {
            // This would scan ACPI devices
            // For now, just create a placeholder
            let acpi = HardwareInfo::new(
                HardwareType::Acpi,
                "ACPI".to_string(),
                "ACPI".to_string(),
                "acpi".to_string(),
            );
            self.acpi_devices.push(acpi);
        }

        Ok(())
    }

    /// Detect DMI information
    pub fn detect_dmi(&mut self) -> io::Result<()> {
        // Read DMI information from /sys/class/dmi/id
        let dmi_path = Path::new("/sys/class/dmi/id");
        if dmi_path.exists() {
            let mut dmi = HardwareInfo::new(
                HardwareType::Dmi,
                String::new(),
                String::new(),
                "dmi".to_string(),
            );

            // Read BIOS information
            if let Ok(bios_vendor) = fs::read_to_string(dmi_path.join("bios_vendor")) {
                dmi.vendor = bios_vendor.trim().to_string();
            }

            if let Ok(bios_version) = fs::read_to_string(dmi_path.join("bios_version")) {
                dmi.firmware_version = Some(bios_version.trim().to_string());
            }

            if let Ok(bios_date) = fs::read_to_string(dmi_path.join("bios_date")) {
                dmi.capabilities.push(format!("BIOS Date: {}", bios_date.trim()));
            }

            // Read system information
            if let Ok(sys_vendor) = fs::read_to_string(dmi_path.join("sys_vendor")) {
                dmi.product = sys_vendor.trim().to_string();
            }

            if let Ok(sys_product) = fs::read_to_string(dmi_path.join("product_name")) {
                dmi.product = sys_product.trim().to_string();
            }

            if let Ok(sys_version) = fs::read_to_string(dmi_path.join("product_version")) {
                dmi.capabilities.push(format!("Product Version: {}", sys_version.trim()));
            }

            if let Ok(sys_serial) = fs::read_to_string(dmi_path.join("product_serial")) {
                dmi.serial_number = Some(sys_serial.trim().to_string());
            }

            if let Ok(sys_uuid) = fs::read_to_string(dmi_path.join("product_uuid")) {
                dmi.uuid = Some(sys_uuid.trim().to_string());
            }

            self.dmi_info = Some(dmi);
        }

        Ok(())
    }

    /// Detect SMBIOS information
    pub fn detect_smbios(&mut self) -> io::Result<()> {
        // Read SMBIOS information from /sys/class/dmi/id
        let smbios_path = Path::new("/sys/class/dmi/id");
        if smbios_path.exists() {
            let mut smbios = HardwareInfo::new(
                HardwareType::Smbios,
                String::new(),
                String::new(),
                "smbios".to_string(),
            );

            // Read SMBIOS tables
            if let Ok(smbios_ver) = fs::read_to_string(smbios_path.join("smbios_version")) {
                smbios.firmware_version = Some(smbios_ver.trim().to_string());
            }

            // Read board information
            if let Ok(board_vendor) = fs::read_to_string(smbios_path.join("board_vendor")) {
                smbios.vendor = board_vendor.trim().to_string();
            }

            if let Ok(board_name) = fs::read_to_string(smbios_path.join("board_name")) {
                smbios.product = board_name.trim().to_string();
            }

            if let Ok(board_version) = fs::read_to_string(smbios_path.join("board_version")) {
                smbios.capabilities.push(format!("Board Version: {}", board_version.trim()));
            }

            if let Ok(board_serial) = fs::read_to_string(smbios_path.join("board_serial")) {
                smbios.serial_number = Some(board_serial.trim().to_string());
            }

            self.smbios_info = Some(smbios);
        }

        Ok(())
    }

    /// Get hardware summary
    pub fn get_summary(&self) -> HardwareSummary {
        HardwareSummary {
            cpu_count: self.cpus.len(),
            total_memory: self.memory.total,
            memory_used: self.memory.used(),
            storage_devices: self.storage_devices.len(),
            network_interfaces: self.network_interfaces.len(),
            graphics_cards: self.graphics_cards.len(),
            audio_devices: self.audio_devices.len(),
            input_devices: self.input_devices.len(),
            usb_devices: self.usb_devices.len(),
            pci_devices: self.pci_devices.len(),
        }
    }

    /// Find devices by type
    pub fn find_by_type(&self, device_type: HardwareType) -> Vec<&HardwareInfo> {
        let mut devices = Vec::new();

        match device_type {
            HardwareType::Pci => devices.extend(self.pci_devices.iter()),
            HardwareType::Acpi => devices.extend(self.acpi_devices.iter()),
            HardwareType::Dmi => {
                if let Some(ref dmi) = self.dmi_info {
                    devices.push(dmi);
                }
            }
            HardwareType::Smbios => {
                if let Some(ref smbios) = self.smbios_info {
                    devices.push(smbios);
                }
            }
            _ => {}
        }

        devices
    }

    /// Find devices missing drivers
    pub fn find_missing_drivers(&self) -> Vec<&HardwareInfo> {
        self.pci_devices.iter()
            .filter(|d| d.driver.is_none())
            .collect()
    }

    /// Export hardware report
    pub fn export_report(&self) -> String {
        let mut report = String::new();

        report.push_str("# SigmaOS Hardware Report\n");
        report.push_str(&format!("Generated: {}\n\n", self.detected_at));

        // CPU Information
        report.push_str("## CPU Information\n");
        for (i, cpu) in self.cpus.iter().enumerate() {
            report.push_str(&format!("### CPU {}\n", i));
            report.push_str(&format!("- Vendor: {}\n", cpu.vendor));
            report.push_str(&format!("- Model: {}\n", cpu.model_name));
            report.push_str(&format!("- Cores: {}\n", cpu.cpu_cores));
            report.push_str(&format!("- MHz: {}\n", cpu.mhz));
            report.push_str(&format!("- Flags: {}\n\n", cpu.flags.join(", ")));
        }

        // Memory Information
        report.push_str("## Memory Information\n");
        report.push_str(&format!("- Total: {} bytes\n", self.memory.total));
        report.push_str(&format!("- Used: {} bytes\n", self.memory.used()));
        report.push_str(&format!("- Available: {} bytes\n", self.memory.available));
        report.push_str(&format!("- Usage: {:.1}%\n\n", self.memory.usage_percent()));

        // Storage Information
        report.push_str("## Storage Devices\n");
        for storage in &self.storage_devices {
            report.push_str(&format!("- {} ({}): {}\n", storage.name, storage.device_path, storage.get_size_human()));
        }
        report.push_str("\n");

        // Network Information
        report.push_str("## Network Interfaces\n");
        for net in &self.network_interfaces {
            report.push_str(&format!("- {} ({})\n", net.name, net.mac_address));
        }
        report.push_str("\n");

        // Graphics Information
        report.push_str("## Graphics Cards\n");
        for gfx in &self.graphics_cards {
            report.push_str(&format!("- {} {} ({})\n", gfx.vendor, gfx.product, gfx.driver));
        }
        report.push_str("\n");

        // USB Devices
        report.push_str("## USB Devices\n");
        for usb in &self.usb_devices {
            report.push_str(&format!("- {} {} ({})\n", usb.vendor, usb.product, usb.device_id));
        }
        report.push_str("\n");

        // PCI Devices
        report.push_str("## PCI Devices\n");
        for pci in &self.pci_devices {
            report.push_str(&format!("- {} {} ({})\n", pci.vendor, pci.product, pci.device_id));
        }

        report
    }
}

impl Default for HardwareDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Hardware summary
#[derive(Debug, Clone)]
pub struct HardwareSummary {
    pub cpu_count: usize,
    pub total_memory: u64,
    pub memory_used: u64,
    pub storage_devices: usize,
    pub network_interfaces: usize,
    pub graphics_cards: usize,
    pub audio_devices: usize,
    pub input_devices: usize,
    pub usb_devices: usize,
    pub pci_devices: usize,
}

/// Hardware module loader
#[derive(Debug, Clone)]
pub struct ModuleLoader {
    pub loaded_modules: Vec<String>,
    pub available_modules: Vec<String>,
    pub module_dependencies: BTreeMap<String, Vec<String>>,
    pub blacklisted_modules: Vec<String>,
}

impl ModuleLoader {
    /// Create a new module loader
    pub fn new() -> Self {
        Self {
            loaded_modules: Vec::new(),
            available_modules: Vec::new(),
            module_dependencies: BTreeMap::new(),
            blacklisted_modules: Vec::new(),
        }
    }

    /// Load modules from /lib/modules
    pub fn load_available_modules(&mut self) -> io::Result<()> {
        let modules_path = Path::new("/lib/modules");
        if modules_path.exists() {
            // This would scan for available kernel modules
            // For now, just create placeholder modules
            self.available_modules = vec![
                "ext4".to_string(),
                "btrfs".to_string(),
                "xfs".to_string(),
                "nvidia".to_string(),
                "amdgpu".to_string(),
                "i915".to_string(),
                "rtl8xxxu".to_string(),
                "ath9k".to_string(),
                "snd_hda_intel".to_string(),
                "usbhid".to_string(),
            ];
        }
        Ok(())
    }

    /// Load module
    pub fn load_module(&mut self, module: &str) -> Result<(), String> {
        if self.loaded_modules.contains(&module.to_string()) {
            return Ok(()); // Already loaded
        }

        if self.blacklisted_modules.contains(&module.to_string()) {
            return Err(format!("Module {} is blacklisted", module));
        }

        // Check dependencies
        if let Some(deps) = self.module_dependencies.get(module) {
            for dep in deps {
                if !self.loaded_modules.contains(dep) {
                    self.load_module(dep)?;
                }
            }
        }

        // In real implementation, would execute modprobe
        self.loaded_modules.push(module.to_string());
        Ok(())
    }

    /// Unload module
    pub fn unload_module(&mut self, module: &str) -> Result<(), String> {
        if let Some(pos) = self.loaded_modules.iter().position(|m| m == module) {
            self.loaded_modules.remove(pos);
            Ok(())
        } else {
            Err(format!("Module {} is not loaded", module))
        }
    }

    /// Check if module is loaded
    pub fn is_loaded(&self, module: &str) -> bool {
        self.loaded_modules.contains(&module.to_string())
    }

    /// Get loaded modules
    pub fn get_loaded_modules(&self) -> &[String] {
        &self.loaded_modules
    }

    /// Get available modules
    pub fn get_available_modules(&self) -> &[String] {
        &self.available_modules
    }

    /// Add module dependency
    pub fn add_dependency(&mut self, module: String, dependency: String) {
        self.module_dependencies
            .entry(module)
            .or_insert_with(Vec::new)
            .push(dependency);
    }

    /// Blacklist module
    pub fn blacklist_module(&mut self, module: String) {
        if !self.blacklisted_modules.contains(&module) {
            self.blacklisted_modules.push(module);
        }
    }

    /// Unblacklist module
    pub fn unblacklist_module(&mut self, module: &str) -> bool {
        if let Some(pos) = self.blacklisted_modules.iter().position(|m| m == module) {
            self.blacklisted_modules.remove(pos);
            true
        } else {
            false
        }
    }

    /// Load modules for detected hardware
    pub fn load_modules_for_hardware(&mut self, detector: &HardwareDetector) -> Result<Vec<String>, String> {
        let mut loaded = Vec::new();

        // Load storage modules
        for storage in &detector.storage_devices {
            if storage.nvme {
                self.load_module("nvme")?;
                loaded.push("nvme".to_string());
            }
            if storage.sata {
                self.load_module("ahci")?;
                loaded.push("ahci".to_string());
            }
            if storage.usb {
                self.load_module("usb-storage")?;
                loaded.push("usb-storage".to_string());
            }
        }

        // Load network modules
        for net in &detector.network_interfaces {
            if net.wireless {
                // Would detect specific wireless chip and load appropriate module
                self.load_module("cfg80211")?;
                loaded.push("cfg80211".to_string());
            }
        }

        // Load graphics modules
        for gfx in &detector.graphics_cards {
            if gfx.driver.contains("nvidia") {
                self.load_module("nvidia")?;
                loaded.push("nvidia".to_string());
            } else if gfx.driver.contains("amdgpu") {
                self.load_module("amdgpu")?;
                loaded.push("amdgpu".to_string());
            } else if gfx.driver.contains("i915") {
                self.load_module("i915")?;
                loaded.push("i915".to_string());
            }
        }

        // Load audio modules
        for audio in &detector.audio_devices {
            self.load_module("snd_hda_intel")?;
            loaded.push("snd_hda_intel".to_string());
        }

        // Load USB modules
        for usb in &detector.usb_devices {
            self.load_module("usbhid")?;
            loaded.push("usbhid".to_string());
        }

        Ok(loaded)
    }

    /// Get module information
    pub fn get_module_info(&self, module: &str) -> Option<ModuleInfo> {
        if self.loaded_modules.contains(&module.to_string()) {
            Some(ModuleInfo {
                name: module.to_string(),
                loaded: true,
                dependencies: self.module_dependencies.get(module).cloned().unwrap_or_default(),
                size: 0, // Would get actual size
                refcount: 0, // Would get actual refcount
            })
        } else {
            None
        }
    }

    /// Export module configuration
    pub fn export_config(&self) -> String {
        let mut config = String::new();

        config.push_str("# SigmaOS Kernel Modules Configuration\n");
        config.push_str("# Generated by SigmaOS Module Loader\n\n");

        config.push_str("## Loaded Modules\n");
        for module in &self.loaded_modules {
            config.push_str(&format!("load={}\n", module));
        }

        config.push_str("\n## Available Modules\n");
        for module in &self.available_modules {
            config.push_str(&format!("available={}\n", module));
        }

        config.push_str("\n## Module Dependencies\n");
        for (module, deps) in &self.module_dependencies {
            config.push_str(&format!("depends={} requires={}\n", module, deps.join(",")));
        }

        config.push_str("\n## Blacklisted Modules\n");
        for module in &self.blacklisted_modules {
            config.push_str(&format!("blacklist={}\n", module));
        }

        config
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Module information
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub loaded: bool,
    pub dependencies: Vec<String>,
    pub size: u64,
    pub refcount: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_type() {
        assert_eq!(HardwareType::Cpu.as_str(), "cpu");
        assert_eq!(HardwareType::from_str("memory"), Some(HardwareType::Memory));
    }

    #[test]
    fn test_cpu_info() {
        let cpu = CpuInfo::new("Intel".to_string(), "0".to_string(), "Intel Core i7".to_string())
            .add_flag("sse4_2")
            .add_flag("avx2")
            .mhz(3400)
            .cpu_cores(8);

        assert_eq!(cpu.vendor, "Intel");
        assert_eq!(cpu.mhz, 3400);
        assert_eq!(cpu.cpu_cores, 8);
        assert!(cpu.flags.contains(&"sse4_2".to_string()));
    }

    #[test]
    fn test_memory_info() {
        let memory = MemoryInfo::new(16_000_000_000, 8_000_000_000)
            .available(12_000_000_000)
            .buffers(1_000_000_000)
            .cached(2_000_000_000);

        assert_eq!(memory.total, 16_000_000_000);
        assert_eq!(memory.used(), 4_000_000_000);
        assert!(memory.usage_percent() > 0.0);
    }

    #[test]
    fn test_storage_info() {
        let storage = StorageInfo::new("sda".to_string(), "/dev/sda".to_string(), "Samsung SSD".to_string(), "Samsung".to_string(), 500_000_000_000)
            .ssd(true)
            .filesystem("ext4".to_string())
            .mount_point("/".to_string());

        assert_eq!(storage.name, "sda");
        assert!(storage.ssd);
        assert_eq!(storage.filesystem, Some("ext4".to_string()));
        assert!(storage.mount_points.contains(&"/".to_string()));
    }

    #[test]
    fn test_network_info() {
        let network = NetworkInfo::new("eth0".to_string(), "00:11:22:33:44:55".to_string(), "e1000e".to_string(), "Intel".to_string(), "Ethernet Controller".to_string())
            .add_ip("192.168.1.100")
            .add_ipv6("fe80::1")
            .speed(1000)
            .operstate("up".to_string());

        assert_eq!(network.name, "eth0");
        assert_eq!(network.mac_address, "00:11:22:33:44:55");
        assert!(network.has_ip());
        assert!(network.is_up());
    }

    #[test]
    fn test_hardware_detector() {
        let mut detector = HardwareDetector::new();
        detector.detect_all().unwrap();

        let summary = detector.get_summary();
        assert!(summary.cpu_count > 0 || summary.total_memory > 0);
    }

    #[test]
    fn test_module_loader() {
        let mut loader = ModuleLoader::new();
        loader.load_available_modules().unwrap();

        assert!(loader.load_module("ext4").is_ok());
        assert!(loader.is_loaded("ext4"));
        assert!(loader.unload_module("ext4").is_ok());
        assert!(!loader.is_loaded("ext4"));
    }

    #[test]
    fn test_module_dependencies() {
        let mut loader = ModuleLoader::new();
        loader.add_dependency("nvidia".to_string(), "nvidia-modeset".to_string());
        loader.add_dependency("nvidia-modeset".to_string(), "nvidia-drm".to_string());

        assert!(loader.load_module("nvidia").is_ok());
        assert!(loader.is_loaded("nvidia"));
        assert!(loader.is_loaded("nvidia-modeset"));
        assert!(loader.is_loaded("nvidia-drm"));
    }

    #[test]
    fn test_hardware_report() {
        let mut detector = HardwareDetector::new();
        detector.detect_all().unwrap();

        let report = detector.export_report();
        assert!(report.contains("# SigmaOS Hardware Report"));
        assert!(report.contains("## CPU Information"));
    }
}

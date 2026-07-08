// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/net/r8169.rs — Realtek r8169 Ethernet Driver
//
// Implements the Realtek r8169/r8168 Ethernet Controller driver.
// Supports Realtek 8169, 8168, 8411, and related chipsets.
// Based on Linux kernel r8169 driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::ethernet_device_base::{EthernetDevice, EthernetPhy, EthernetAddress, EthernetSpeed, EthernetDuplex, EthernetLinkStatus, EthernetStats, ETH_OK, ETH_ERR_NO_DEVICE, ETH_ERR_INIT_FAILED, ETH_ERR_INVALID_PARAM, ETH_ERR_TIMEOUT};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Realtek Vendor IDs ─────────────────────────────────────────

pub const REALTEK_VENDOR_ID: U16 = 0x10EC;

// ─── r8169 Device IDs ─────────────────────────────────────

pub const RTL8169_DEV_ID: U16 = 0x8169;
pub const RTL8168B_DEV_ID: U16 = 0x8168;
pub const RTL8168C_DEV_ID: U16 = 0x8167;
pub const RTL8168D_DEV_ID: U16 = 0x8166;
pub const RTL8168E_DEV_ID: U16 = 0x8165;
pub const RTL8411_DEV_ID: U16 = 0x8411;
pub const RTL8168EP_DEV_ID: U16 = 0x8168;
pub const RTL8168H_DEV_ID: U16 = 0x8168;

// ─── r8169 Register Offsets ─────────────────────────────

pub const RTL_IDR0: U32 = 0x00;
pub const RTL_IDR1: U32 = 0x04;
pub const RTL_IDR2: U32 = 0x08;
pub const RTL_IDR3: U32 = 0x0C;
pub const RTL_MAR0: U32 = 0x08;
pub const RTL_MAR4: U32 = 0x0C;
pub const RTL_TXDESC0: U32 = 0x20;
pub const RTL_TXDESC1: U32 = 0x24;
pub const RTL_TXDESC2: U32 = 0x28;
pub const RTL_TXDESC3: U32 = 0x2C;
pub const RTL_TXDESC4: U32 = 0x30;
pub const RTL_RXDESC0: U32 = 0xE0;
pub const RTL_RXDESC1: U32 = 0xE4;
pub const RTL_RXDESC2: U32 = 0xE8;
pub const RTL_RXDESC3: U32 = 0xEC;
pub const RTL_RXDESC4: U32 = 0xF0;
pub const RTL_RXMISS: U32 = 0x4C;
pub const RTL_FIFOTMS: U32 = 0x50;
pub const RTL_CSCR: U32 = 0x74;
pub const RTL_CONFIG0: U32 = 0x52;
pub const RTL_CONFIG1: U32 = 0x53;
pub const RTL_CONFIG2: U32 = 0x54;
pub const RTL_CONFIG3: U32 = 0x55;
pub const RTL_CONFIG4: U32 = 0x56;
pub const RTL_CONFIG5: U32 = 0x57;
pub const RTL_TIMERINT: U32 = 0x58;
pub const RTL_PHYAR: U32 = 0x60;
pub const RTL_PHYSTATUS: U32 = 0x6C;
pub const RTL_CR: U32 = 0x00;
pub const RTL_CMD: U32 = 0x37;
pub const RTL_RXCFG: U32 = 0x44;
pub const RTL_MIS0: U32 = 0x4E;
pub const RTL_MIS1: U32 = 0x50;
pub const RTL_MIS2: U32 = 0x52;
pub const RTL_MIS3: U32 = 0x54;
pub const RTL_MIS4: U32 = 0x56;
pub const RTL_TXCFG: U32 = 0x40;
pub const RTL_RXADDR: U32 = 0x38;
pub const RTL_TXADDR: U32 = 0x20;
pub const RTL_RXCOUNT: U32 = 0x30;
pub const RTL_TXCOUNT: U32 = 0x38;
pub const RTL_ISR: U32 = 0x3E;
pub const RTL_IMR: U32 = 0x3C;
pub const RTL_TCR: U32 = 0x40;
pub const RTL_RCR: U32 = 0x44;
pub const RTL_TPPOLL: U32 = 0xE0;
pub const RTL_PBR: U32 = 0x2A;
pub const RTL_MISC: U32 = 0x2C;

// ─── r8169 Receive Descriptor ─────────────────────────────

#[repr(C)]
pub struct R8169RxDescriptor {
    pub buffer_addr: U64,
    pub buffer_addr_high: U32,
    pub length: U16,
    pub flags: U16,
}

impl R8169RxDescriptor {
    pub const fn new() -> Self {
        R8169RxDescriptor {
            buffer_addr: 0,
            buffer_addr_high: 0,
            length: 0,
            flags: 0,
        }
    }
}

// ─── r8169 Transmit Descriptor ───────────────────────────

#[repr(C)]
pub struct R8169TxDescriptor {
    pub buffer_addr: U64,
    pub buffer_addr_high: U32,
    pub length: U16,
    pub flags: U16,
    pub vlan_tag: U16,
}

impl R8169TxDescriptor {
    pub const fn new() -> Self {
        R8169TxDescriptor {
            buffer_addr: 0,
            buffer_addr_high: 0,
            length: 0,
            flags: 0,
            vlan_tag: 0,
        }
    }
}

// ─── r8169 Controller Structure ─────────────────────────

pub struct R8169Controller {
    pub mmio_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub enabled: bool,
    pub mac_address: EthernetAddress,
    pub mtu: U32,
    pub link_status: EthernetLinkStatus,
    pub promiscuous: bool,
    pub multicast: bool,
    pub all_multicast: bool,
    pub rx_ring: [R8169RxDescriptor; 256],
    pub tx_ring: [R8169TxDescriptor; 256],
    pub rx_head: U16,
    pub rx_tail: U16,
    pub tx_head: U16,
    pub tx_tail: U16,
    pub stats: EthernetStats,
    pub phy_addr: U8,
}

impl R8169Controller {
    pub const fn new() -> Self {
        R8169Controller {
            mmio_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            enabled: false,
            mac_address: EthernetAddress::new(),
            mtu: 1500,
            link_status: EthernetLinkStatus::new(),
            promiscuous: false,
            multicast: true,
            all_multicast: false,
            rx_ring: [R8169RxDescriptor::new(); 256],
            tx_ring: [R8169TxDescriptor::new(); 256],
            rx_head: 0,
            rx_tail: 0,
            tx_head: 0,
            tx_tail: 0,
            stats: EthernetStats::new(),
            phy_addr: 1,
        }
    }

    /// Read MMIO register
    unsafe fn read_mmio(&self, offset: U32) -> U32 {
        let ptr = (self.mmio_base + offset as U64) as *const U32;
        *ptr
    }

    /// Write MMIO register
    unsafe fn write_mmio(&self, offset: U32, value: U32) {
        let ptr = (self.mmio_base + offset as U64) as *mut U32;
        *ptr = value
    }

    /// Initialize r8169 controller
    fn init_r8169(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        unsafe {
            // Reset controller
            self.reset_controller();

            // Read MAC address
            self.read_mac_address();

            // Initialize receive ring
            self.init_rx_ring();

            // Initialize transmit ring
            self.init_tx_ring();

            // Configure receive
            self.configure_receive();

            // Configure transmit
            self.configure_transmit();

            // Enable interrupts
            self.enable_interrupts();

            // Enable link
            self.enable_link();
        }

        self.initialized = true;
        self.enabled = true;

        ETH_OK
    }

    /// Reset controller
    unsafe fn reset_controller(&mut self) {
        let mut cmd = self.read_mmio(RTL_CMD);
        cmd |= 0x00000010; // RST
        self.write_mmio(RTL_CMD, cmd);

        let mut timeout = 10000;
        while timeout > 0 {
            let cmd = self.read_mmio(RTL_CMD);
            if cmd & 0x00000010 == 0 {
                break;
            }
            timeout -= 1;
        }
    }

    /// Read MAC address
    unsafe fn read_mac_address(&mut self) {
        let mac0 = self.read_mmio(RTL_IDR0);
        let mac1 = self.read_mmio(RTL_IDR1);
        let mac2 = self.read_mmio(RTL_IDR2);
        let mac3 = self.read_mmio(RTL_IDR3);

        self.mac_address = EthernetAddress::from_bytes(
            (mac0 & 0xFF) as u8,
            ((mac0 >> 8) & 0xFF) as u8,
            ((mac0 >> 16) & 0xFF) as u8,
            ((mac0 >> 24) & 0xFF) as u8,
            (mac1 & 0xFF) as u8,
            ((mac1 >> 8) & 0xFF) as u8,
        );
    }

    /// Initialize receive ring
    unsafe fn init_rx_ring(&mut self) {
        let rx_ring_base = &self.rx_ring as *const R8169RxDescriptor as U64;
        
        self.write_mmio(RTL_RXADDR, (rx_ring_base & 0xFFFFFFFF) as U32);
        self.write_mmio(RTL_RXADDR + 4, ((rx_ring_base >> 32) & 0xFFFFFFFF) as U32);

        self.rx_head = 0;
        self.rx_tail = 0;
    }

    /// Initialize transmit ring
    unsafe fn init_tx_ring(&mut self) {
        let tx_ring_base = &self.tx_ring as *const R8169TxDescriptor as U64;
        
        self.write_mmio(RTL_TXADDR, (tx_ring_base & 0xFFFFFFFF) as U32);
        self.write_mmio(RTL_TXADDR + 4, ((tx_ring_base >> 32) & 0xFFFFFFFF) as U32);

        self.tx_head = 0;
        self.tx_tail = 0;
    }

    /// Configure receive
    unsafe fn configure_receive(&mut self) {
        let mut rcr = self.read_mmio(RTL_RCR);
        rcr |= 0x00000001; // Accept broadcast
        rcr |= 0x00000002; // Accept multicast
        rcr |= 0x00000004; // Accept my MAC
        rcr |= 0x00000008; // Accept all physical
        rcr |= 0x00000010; // Accept error packets
        rcr |= 0x00000020; // Accept runt packets
        rcr |= 0x00008000; // Enable receiver
        self.write_mmio(RTL_RCR, rcr);
    }

    /// Configure transmit
    unsafe fn configure_transmit(&mut self) {
        let mut tcr = self.read_mmio(RTL_TCR);
        tcr |= 0x00000001; // Enable transmitter
        tcr |= 0x00000002; // Enable loopback
        tcr |= 0x00000004; // Enable CRC
        self.write_mmio(RTL_TCR, tcr);
    }

    /// Enable interrupts
    unsafe fn enable_interrupts(&mut self) {
        let mut imr = self.read_mmio(RTL_IMR);
        imr |= 0x00000001; // Link OK
        imr |= 0x00000002; // TX OK
        imr |= 0x00000004; // RX OK
        imr |= 0x00000008; // TX error
        imr |= 0x00000010; // RX error
        imr |= 0x00000020; // RX overflow
        self.write_mmio(RTL_IMR, imr);
    }

    /// Enable link
    unsafe fn enable_link(&mut self) {
        let mut cmd = self.read_mmio(RTL_CMD);
        cmd |= 0x00000008; // Enable RX
        cmd |= 0x00000004; // Enable TX
        self.write_mmio(RTL_CMD, cmd);

        self.link_status.link_up = true;
        self.link_status.speed = EthernetSpeed::Speed1000;
        self.link_status.duplex = EthernetDuplex::Full;
        self.link_status.autoneg = true;
    }
}

// ─── Implement EthernetDevice Trait ─────────────────────

impl EthernetDevice for R8169Controller {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let vendor_id = REALTEK_VENDOR_ID;
        self.init_r8169(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        "Realtek r8169 Ethernet Controller"
    }

    fn get_mac_address(&self) -> EthernetAddress {
        self.mac_address
    }

    fn set_mac_address(&mut self, mac: EthernetAddress) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.mac_address = mac;
        ETH_OK
    }

    fn get_mtu(&self) -> U32 {
        self.mtu
    }

    fn set_mtu(&mut self, mtu: U32) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        if mtu < 64 || mtu > 9700 {
            return ETH_ERR_INVALID_PARAM;
        }

        self.mtu = mtu;
        ETH_OK
    }

    fn get_link_status(&self) -> EthernetLinkStatus {
        self.link_status
    }

    fn set_link_config(&mut self, speed: EthernetSpeed, duplex: EthernetDuplex) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.link_status.speed = speed;
        self.link_status.duplex = duplex;
        ETH_OK
    }

    fn set_autoneg(&mut self, enable: bool) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.link_status.autoneg = enable;
        ETH_OK
    }

    fn set_promiscuous(&mut self, enable: bool) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.promiscuous = enable;
        ETH_OK
    }

    fn set_multicast(&mut self, enable: bool) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.multicast = enable;
        ETH_OK
    }

    fn add_multicast_address(&mut self, mac: EthernetAddress) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        ETH_OK
    }

    fn remove_multicast_address(&mut self, mac: EthernetAddress) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        ETH_OK
    }

    fn set_all_multicast(&mut self, enable: bool) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.all_multicast = enable;
        ETH_OK
    }

    fn enable(&mut self) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.enabled = true;
        ETH_OK
    }

    fn disable(&mut self) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.enabled = false;
        ETH_OK
    }

    fn transmit(&mut self, buffer: *const U8, length: U32) -> I32 {
        if !self.initialized || !self.enabled {
            return ETH_ERR_INIT_FAILED;
        }

        unsafe {
            let tail = self.tx_tail as usize;
            self.tx_ring[tail].buffer_addr = buffer as U64;
            self.tx_ring[tail].length = length as U16;
            self.tx_ring[tail].flags = 0x03; // EOP, LS

            self.tx_tail = ((self.tx_tail + 1) % 256) as U16;

            self.stats.tx_packets += 1;
            self.stats.tx_bytes += length as U64;
        }

        ETH_OK
    }

    fn receive(&mut self, buffer: *mut U8, max_length: U32) -> I32 {
        if !self.initialized || !self.enabled {
            return ETH_ERR_INIT_FAILED;
        }

        unsafe {
            let head = self.rx_head as usize;
            let tail = self.rx_tail as usize;

            if head != tail {
                let desc = &self.rx_ring[head];
                let length = desc.length as U32;

                if length <= max_length {
                    let src = desc.buffer_addr as *const U8;
                    for i in 0..length as usize {
                        *buffer.add(i) = *src.add(i);
                    }

                    self.rx_head = ((self.rx_head + 1) % 256) as U16;

                    self.stats.rx_packets += 1;
                    self.stats.rx_bytes += length as U64;

                    length as I32
                } else {
                    ETH_ERR_INVALID_PARAM
                }
            } else {
                0 // No packets available
            }
        }
    }

    fn get_stats(&self) -> EthernetStats {
        self.stats
    }

    fn reset_stats(&mut self) {
        self.stats = EthernetStats::new();
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        unsafe {
            self.reset_controller();
        }

        ETH_OK
    }

    fn shutdown(&mut self) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        self.enabled = false;
        self.initialized = false;
        ETH_OK
    }
}

// ─── Implement EthernetPhy Trait ─────────────────────

impl EthernetPhy for R8169Controller {
    fn read_phy(&self, phy_addr: U8, reg: U8) -> U16 {
        if !self.initialized {
            return 0;
        }

        unsafe {
            let phyar = ((phy_addr as U32) << 21) | ((reg as U32) << 16) | 0x00000000;
            self.write_mmio(RTL_PHYAR, phyar);

            let mut timeout = 10000;
            while timeout > 0 {
                let phyar = self.read_mmio(RTL_PHYAR);
                if phyar & 0x80000000 != 0 {
                    return (phyar & 0xFFFF) as U16;
                }
                timeout -= 1;
            }

            0
        }
    }

    fn write_phy(&mut self, phy_addr: U8, reg: U8, value: U16) -> I32 {
        if !self.initialized {
            return ETH_ERR_INIT_FAILED;
        }

        unsafe {
            let phyar = ((phy_addr as U32) << 21) | ((reg as U32) << 16) | ((value as U32) & 0xFFFF) | 0x80000000;
            self.write_mmio(RTL_PHYAR, phyar);

            let mut timeout = 10000;
            while timeout > 0 {
                let phyar = self.read_mmio(RTL_PHYAR);
                if phyar & 0x80000000 == 0 {
                    return ETH_OK;
                }
                timeout -= 1;
            }

            ETH_ERR_TIMEOUT
        }
    }

    fn get_phy_id(&self, phy_addr: U8) -> U32 {
        let id1 = self.read_phy(phy_addr, 2);
        let id2 = self.read_phy(phy_addr, 3);
        ((id1 as U32) << 16) | (id2 as U32)
    }

    fn reset_phy(&mut self, phy_addr: U8) -> I32 {
        self.write_phy(phy_addr, 0, 0x8000)
    }

    fn get_phy_link_status(&self, phy_addr: U8) -> bool {
        let bmsr = self.read_phy(phy_addr, 1);
        bmsr & 0x0004 != 0
    }

    fn get_phy_speed_duplex(&self, phy_addr: U8) -> (EthernetSpeed, EthernetDuplex) {
        let anlpar = self.read_phy(phy_addr, 5);

        let speed = if anlpar & 0x2000 != 0 {
            EthernetSpeed::Speed1000
        } else if anlpar & 0x0100 != 0 {
            EthernetSpeed::Speed100
        } else {
            EthernetSpeed::Speed10
        };

        let duplex = if anlpar & 0x0100 != 0 {
            EthernetDuplex::Full
        } else {
            EthernetDuplex::Half
        };

        (speed, duplex)
    }
}

// ─── Global r8169 Controller ─────────────────────────

static mut G_R8169: R8169Controller = R8169Controller::new();

// ─── C-ABI Exports ─────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn r8169_init(pci_bar: U64, device_id: U16) -> I32 {
    G_R8169.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn r8169_is_initialized() -> I32 {
    if G_R8169.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn r8169_shutdown() -> I32 {
    G_R8169.shutdown()
}

/// Probe for r8169 devices
#[no_mangle]
pub unsafe extern "C" fn r8169_probe() -> I32 {
    let mut found_devices = 0;
    
    for bus in 0..256u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                
                if vendor_id == REALTEK_VENDOR_ID && is_r8169_device(device_id) {
                    let bar0 = read_pci_config_u32(bus, device, function, 0x10);
                    let mmio_base = (bar0 & 0xFFFFFFF0) as U64;
                    
                    let result = G_R8169.init(mmio_base, device_id);
                    
                    if result == ETH_OK {
                        found_devices += 1;
                        return ETH_OK;
                    }
                }
            }
        }
    }
    
    if found_devices > 0 {
        ETH_OK
    } else {
        ETH_ERR_NO_DEVICE
    }
}

unsafe fn is_r8169_device(device_id: U16) -> bool {
    matches!(device_id,
        RTL8169_DEV_ID |
        RTL8168B_DEV_ID |
        RTL8168C_DEV_ID |
        RTL8168D_DEV_ID |
        RTL8168E_DEV_ID |
        RTL8411_DEV_ID |
        RTL8168EP_DEV_ID |
        RTL8168H_DEV_ID
    )
}

unsafe fn read_pci_config_u16(bus: U8, device: U8, function: U8, offset: U8) -> U16 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    let value = inl(0xCFC);
    let shift = ((offset & 2) as u32) * 8;
    ((value >> shift) & 0xFFFF) as U16
}

unsafe fn read_pci_config_u32(bus: U8, device: U8, function: U8, offset: U8) -> U32 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    inl(0xCFC)
}

unsafe fn outl(port: U16, value: U32) {
    // x86 assembly for outl instruction
    // In a real kernel, this would use inline assembly
}

unsafe fn inl(port: U16) -> U32 {
    // x86 assembly for inl instruction
    // In a real kernel, this would use inline assembly
    0 // Stub
}

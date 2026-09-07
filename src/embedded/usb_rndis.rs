#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based USB RNDIS for SigmaOS
/// Implements NDIS (Network Device Interface Specification) model ethernet and Wi-Fi drivers.
/// Inspired by Linux sk_buff, BSD mbuf, and standard NDIS OID state queries.

use std::boxed::Box;

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RNDISID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RNDISError { Success = 0, NotFound = 1, InvalidOid = 2, BufferOverflow = 3 }

pub trait USBRNDIS {
    fn id(&self) -> RNDISID;
    fn is_connected(&self) -> bool;
    fn set_connected(&self, value: bool);
}

#[repr(C)]
pub struct SimpleUSBRNDIS {
    pub id: RNDISID,
    pub connected: AtomicUsize,
}

impl SimpleUSBRNDIS {
    pub fn new(id: RNDISID) -> Self {
        SimpleUSBRNDIS {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl USBRNDIS for SimpleUSBRNDIS {
    fn id(&self) -> RNDISID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
    fn set_connected(&self, value: bool) {
        self.connected.store(if value { 1 } else { 0 }, Ordering::SeqCst);
    }
}

pub trait RNDISController {
    fn init(&mut self, rndis_id: RNDISID) -> Result<(), RNDISError>;
    fn send_packet(&self, rndis_id: RNDISID, packet: &[u8]) -> Result<usize, RNDISError>;
    fn receive_packet(&self, rndis_id: RNDISID, buffer: &mut [u8]) -> Result<usize, RNDISError>;
    fn get_rndis(&self, id: RNDISID) -> Option<&dyn USBRNDIS>;
}

#[repr(C)]
pub struct SimpleRNDISController {
    pub rndis_devices: Vec<Option<Box<dyn USBRNDIS>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRNDISController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleRNDISController {
            rndis_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RNDISController for SimpleRNDISController {
    fn init(&mut self, rndis_id: RNDISID) -> Result<(), RNDISError> {
        for rndis_option in &mut self.rndis_devices {
            if let Some(ref mut rndis) = *rndis_option {
                let dev_ref: &dyn USBRNDIS = rndis.as_ref();
                if dev_ref.id() == rndis_id {
                    dev_ref.set_connected(true);
                    return Ok(());
                }
            }
        }
        Err(RNDISError::NotFound)
    }
    
    fn send_packet(&self, rndis_id: RNDISID, _packet: &[u8]) -> Result<usize, RNDISError> {
        if self.get_rndis(rndis_id).is_some() {
            Ok(0)
        } else {
            Err(RNDISError::NotFound)
        }
    }
    
    fn receive_packet(&self, rndis_id: RNDISID, buffer: &mut [u8]) -> Result<usize, RNDISError> {
        if self.get_rndis(rndis_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(RNDISError::NotFound)
        }
    }
    
    fn get_rndis(&self, id: RNDISID) -> Option<&dyn USBRNDIS> {
        for rndis_option in &self.rndis_devices {
            if let Some(ref rndis) = *rndis_option {
                let dev_ref: &dyn USBRNDIS = rndis.as_ref();
                if dev_ref.id() == id { return Some(dev_ref); }
            }
        }
        None
    }
}

pub trait RNDISMessage {
    fn send_msg(&self, rndis_id: RNDISID, msg_id: u32, data: &[u8]) -> Result<(), RNDISError>;
    fn receive_msg(&self, rndis_id: RNDISID, buffer: &mut [u8]) -> Result<(u32, usize), RNDISError>;
}

#[repr(C)]
pub struct SimpleRNDISMessage {
    pub controller: SimpleRNDISController,
}

impl SimpleRNDISMessage {
    pub fn new(controller: SimpleRNDISController) -> Self {
        SimpleRNDISMessage { controller }
    }
}

impl RNDISMessage for SimpleRNDISMessage {
    fn send_msg(&self, rndis_id: RNDISID, _msg_id: u32, _data: &[u8]) -> Result<(), RNDISError> {
        if self.controller.get_rndis(rndis_id).is_some() {
            Ok(())
        } else {
            Err(RNDISError::NotFound)
        }
    }
    
    fn receive_msg(&self, rndis_id: RNDISID, buffer: &mut [u8]) -> Result<(u32, usize), RNDISError> {
        if self.controller.get_rndis(rndis_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok((0, buffer.len()))
        } else {
            Err(RNDISError::NotFound)
        }
    }
}

// ==============================================================================
// 1. BSD/Linux-style mbuf / sk_buff Network Packet Descriptor
// ==============================================================================
#[repr(C)]
pub struct SkBuff {
    pub data: [u8; 2048],
    pub len: usize,
    pub protocol_ethertype: u16, // e.g. 0x0800 for IPv4, 0x0806 for ARP
}

impl SkBuff {
    pub fn new() -> Self {
        Self {
            data: [0; 2048],
            len: 0,
            protocol_ethertype: 0,
        }
    }

    pub fn push_data(&mut self, bytes: &[u8]) -> Result<(), RNDISError> {
        if self.len + bytes.len() > 2048 {
            return Err(RNDISError::BufferOverflow);
        }
        for i in 0..bytes.len() {
            self.data[self.len + i] = bytes[i];
        }
        self.len += bytes.len();
        Ok(())
    }
}

// ==============================================================================
// 2. NDIS Object Identifier (OID) Queries & Sets (Ethernet / Wi-Fi Support)
// ==============================================================================
pub const OID_GEN_PHYSICAL_MEDIUM: u32 = 0x00010202;
pub const OID_GEN_LINK_SPEED: u32 = 0x00010107;
pub const OID_802_3_CURRENT_ADDRESS: u32 = 0x01010102;
pub const OID_802_11_SSID: u32 = 0x0d010102;

#[repr(C)]
pub struct RndisOidManager {
    pub physical_medium: u32,  // 1 = Ethernet, 2 = Wi-Fi
    pub link_speed_bps: u64,
    pub mac_address: [u8; 6],
}

impl RndisOidManager {
    pub fn new(medium: u32) -> Self {
        Self {
            physical_medium: medium,
            link_speed_bps: 1_000_000_000, // 1 Gbps default
            mac_address: [0x00, 0x1A, 0x2B, 0x3C, 0x4D, 0x5E],
        }
    }

    pub fn query_oid(&self, oid: u32, out_buffer: &mut [u8]) -> Result<usize, RNDISError> {
        match oid {
            OID_GEN_PHYSICAL_MEDIUM => {
                if out_buffer.len() < 4 { return Err(RNDISError::BufferOverflow); }
                out_buffer[0] = self.physical_medium as u8;
                Ok(4)
            }
            OID_GEN_LINK_SPEED => {
                if out_buffer.len() < 8 { return Err(RNDISError::BufferOverflow); }
                let bytes = self.link_speed_bps.to_le_bytes();
                out_buffer[..8].copy_from_slice(&bytes);
                Ok(8)
            }
            OID_802_3_CURRENT_ADDRESS => {
                if out_buffer.len() < 6 { return Err(RNDISError::BufferOverflow); }
                out_buffer[..6].copy_from_slice(&self.mac_address);
                Ok(6)
            }
            _ => Err(RNDISError::InvalidOid),
        }
    }
}

// ==============================================================================
// 3. 802.11 Wi-Fi Connection & Key Handshake State Machine
// ==============================================================================
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WifiLinkState {
    Disconnected,
    Scanning,
    Associated,
    Wpa2Handshake4Way,
    Connected,
}

pub struct WifiStateManager {
    pub state: WifiLinkState,
    pub target_ssid: [u8; 32],
}

impl WifiStateManager {
    pub fn new() -> Self {
        Self {
            state: WifiLinkState::Disconnected,
            target_ssid: [0; 32],
        }
    }

    pub fn associate(&mut self, ssid: &[u8]) {
        let len = ssid.len().min(32);
        self.target_ssid[..len].copy_from_slice(&ssid[..len]);
        self.state = WifiLinkState::Associated;
    }

    pub fn progress_handshake(&mut self, step: u32) -> bool {
        if self.state == WifiLinkState::Associated && step == 1 {
            self.state = WifiLinkState::Wpa2Handshake4Way;
        }
        if self.state == WifiLinkState::Wpa2Handshake4Way && step == 4 {
            self.state = WifiLinkState::Connected;
            return true; // Connection established!
        }
        false
    }
}

impl Default for WifiStateManager {
    fn default() -> Self {
        Self::new()
    }
}

// ==============================================================================
// Vec Implementation
// ==============================================================================
struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}

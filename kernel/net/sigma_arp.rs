// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Networking - ARP Resolution
//! Address Resolution Protocol implementation
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

type SigmaU8  = u8;
type SigmaU16 = u16;
type SigmaU32 = u32;
type SigmaU64 = u64;
type SigmaI32 = i32;

pub const MAX_ARP_ENTRIES: usize = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ArpEntry {
    pub ip_address: [u8; 4],
    pub mac_address: [u8; 6],
    pub interface: u32,
    pub state: u8,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ArpPacket {
    pub hardware_type: u16,
    pub protocol_type: u16,
    pub hardware_size: u8,
    pub protocol_size: u8,
    pub opcode: u16,
    pub sender_mac: [u8; 6],
    pub sender_ip: [u8; 4],
    pub target_mac: [u8; 6],
    pub target_ip: [u8; 4],
}

static mut ARP_TABLE: [ArpEntry; MAX_ARP_ENTRIES] = [ArpEntry {
    ip_address: [0; 4],
    mac_address: [0; 6],
    interface: 0,
    state: 0,
    timestamp: 0,
}; MAX_ARP_ENTRIES];

static mut ARP_COUNT: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn sigma_arp_init() {
    ARP_COUNT = 0;
    for i in 0..MAX_ARP_ENTRIES {
        ARP_TABLE[i].state = 0;
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_arp_lookup(ip: *const u8, mac: *mut u8) -> i32 {
    for i in 0..ARP_COUNT {
        if ARP_TABLE[i].state == 1 {
            let mut match_ip = true;
            for j in 0..4 {
                if ARP_TABLE[i].ip_address[j] != *ip.add(j) {
                    match_ip = false;
                    break;
                }
            }
            
            if match_ip {
                for j in 0..6 {
                    *mac.add(j) = ARP_TABLE[i].mac_address[j];
                }
                return 0;
            }
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn sigma_arp_add(ip: *const u8, mac: *const u8, interface: u32) -> i32 {
    if ARP_COUNT >= MAX_ARP_ENTRIES {
        return -1;
    }

    let idx = ARP_COUNT;
    for i in 0..4 {
        ARP_TABLE[idx].ip_address[i] = *ip.add(i);
    }
    
    for i in 0..6 {
        ARP_TABLE[idx].mac_address[i] = *mac.add(i);
    }
    
    ARP_TABLE[idx].interface = interface;
    ARP_TABLE[idx].state = 1;
    ARP_TABLE[idx].timestamp = 0;
    
    ARP_COUNT += 1;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_arp_remove(ip: *const u8) -> i32 {
    for i in 0..ARP_COUNT {
        if ARP_TABLE[i].state == 1 {
            let mut match_ip = true;
            for j in 0..4 {
                if ARP_TABLE[i].ip_address[j] != *ip.add(j) {
                    match_ip = false;
                    break;
                }
            }
            
            if match_ip {
                ARP_TABLE[i].state = 0;
                return 0;
            }
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn sigma_arp_send_request(target_ip: *const u8) -> i32 {
    // TODO: Send ARP request packet
    let _ = target_ip;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_arp_process_packet(packet: *const ArpPacket) -> i32 {
    if packet.is_null() {
        return -1;
    }
    
    let pkt = &*packet;
    
    if pkt.opcode == 2 {
        // ARP Reply - add to table
        sigma_arp_add(pkt.sender_ip.as_ptr(), pkt.sender_mac.as_ptr(), 0);
    }
    
    0
}

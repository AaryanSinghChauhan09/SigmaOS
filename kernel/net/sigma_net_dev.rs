// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Networking - Network Device Abstraction
//! Manages NIC driver registration and packet queues.
//! no_std, no alloc.

#![no_std]
#![allow(dead_code)]

pub const MAX_NET_DEVICES: usize = 4;
pub const MAX_MAC_LEN: usize = 6;
pub const MAX_MTU: usize = 1500;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NetDevice {
    pub name: [u8; 16],
    pub mac_addr: [u8; MAX_MAC_LEN],
    pub mtu: u16,
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub active: bool,
    pub up: bool,
    
    // Function pointers for driver callbacks (simulated)
    pub dev_id: u32,
}

static mut NET_DEVICES: [NetDevice; MAX_NET_DEVICES] = [NetDevice {
    name: [0; 16], mac_addr: [0; 6], mtu: 1500, rx_packets: 0, tx_packets: 0,
    active: false, up: false, dev_id: 0,
}; MAX_NET_DEVICES];

#[no_mangle]
pub unsafe extern "C" fn sigma_net_dev_register(
    name: *const u8, mac: *const u8, dev_id: u32
) -> i32 {
    if name.is_null() || mac.is_null() { return -1; }
    
    for i in 0..MAX_NET_DEVICES {
        if !NET_DEVICES[i].active {
            // copy name
            let mut j = 0;
            while j < 15 && *name.add(j) != 0 {
                NET_DEVICES[i].name[j] = *name.add(j);
                j += 1;
            }
            NET_DEVICES[i].name[j] = 0;
            
            // copy mac
            for k in 0..6 {
                NET_DEVICES[i].mac_addr[k] = *mac.add(k);
            }
            
            NET_DEVICES[i].dev_id = dev_id;
            NET_DEVICES[i].up = false;
            NET_DEVICES[i].active = true;
            return i as i32;
        }
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn sigma_net_dev_set_up(idx: i32, up: bool) -> i32 {
    if idx < 0 || idx as usize >= MAX_NET_DEVICES { return -1; }
    let dev = &mut NET_DEVICES[idx as usize];
    if !dev.active { return -1; }
    
    dev.up = up;
    0
}

#[no_mangle]
pub unsafe extern "C" fn sigma_net_dev_get_mac(idx: i32, out_mac: *mut u8) -> i32 {
    if idx < 0 || idx as usize >= MAX_NET_DEVICES || out_mac.is_null() { return -1; }
    let dev = &NET_DEVICES[idx as usize];
    if !dev.active { return -1; }
    
    for i in 0..6 {
        *out_mac.add(i) = dev.mac_addr[i];
    }
    0
}

// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// drivers/net/sigma_virtio_net.rs — VirtIO Network Driver
// Language: Rust #![no_std] — OOP via VirtioNet implementing NicDevice

#![no_std]
use crate::kernel::net::sigma_net::{MacAddr, MTU, NicDevice, RX_RING_SZ, TX_RING_SZ};

// ── VirtIO MMIO Register Offsets ──────────────────────────────────────────────
const VIRTIO_MMIO_MAGIC:           usize = 0x000;
const VIRTIO_MMIO_VERSION:         usize = 0x004;
const VIRTIO_MMIO_DEVICE_ID:       usize = 0x008;
const VIRTIO_MMIO_STATUS:          usize = 0x070;
const VIRTIO_MMIO_DEVICE_FEATURES: usize = 0x010;
const VIRTIO_MMIO_DRIVER_FEATURES: usize = 0x020;
const VIRTIO_MMIO_QUEUE_SEL:       usize = 0x030;
const VIRTIO_MMIO_QUEUE_NUM_MAX:   usize = 0x034;
const VIRTIO_MMIO_QUEUE_NUM:       usize = 0x038;
const VIRTIO_MMIO_QUEUE_READY:     usize = 0x044;
const VIRTIO_MMIO_QUEUE_NOTIFY:    usize = 0x050;
const VIRTIO_MMIO_QUEUE_DESC_LOW:  usize = 0x080;
const VIRTIO_MMIO_QUEUE_DESC_HIGH: usize = 0x084;
const VIRTIO_MMIO_QUEUE_AVAIL_LOW: usize = 0x090;
const VIRTIO_MMIO_QUEUE_AVAIL_HIGH:usize = 0x094;
const VIRTIO_MMIO_QUEUE_USED_LOW:  usize = 0x0A0;
const VIRTIO_MMIO_QUEUE_USED_HIGH: usize = 0x0A4;
const VIRTIO_MMIO_CONFIG:          usize = 0x100;

// VirtIO Status bits
const STATUS_ACK:      u32 = 1;
const STATUS_DRIVER:   u32 = 2;
const STATUS_DRIVER_OK: u32 = 4;
const STATUS_FEATURES_OK: u32 = 8;

// VirtIO Net Feature bits
const VIRTIO_NET_F_MAC:  u32 = 1 << 5;
const VIRTIO_NET_F_MRG_RXBUF: u32 = 1 << 15;

// Descriptor flags
const VRING_DESC_F_NEXT:  u16 = 1;
const VRING_DESC_F_WRITE: u16 = 2;

const QUEUE_DEPTH: usize = 64;

// ── VirtQueue Descriptor ──────────────────────────────────────────────────────
#[repr(C, align(16))]
struct VirtDesc {
    addr:  u64,
    len:   u32,
    flags: u16,
    next:  u16,
}

#[repr(C, align(2))]
struct VirtAvail {
    flags: u16,
    idx:   u16,
    ring:  [u16; QUEUE_DEPTH],
}

#[repr(C)]
struct VirtUsedElem { id: u32, len: u32 }

#[repr(C, align(4))]
struct VirtUsed {
    flags: u16,
    idx:   u16,
    ring:  [VirtUsedElem; QUEUE_DEPTH],
}

// ── VirtIO Net Header (prepended to each packet) ──────────────────────────────
#[repr(C)]
struct VirtioNetHdr {
    flags:       u8,
    gso_type:    u8,
    hdr_len:     u16,
    gso_size:    u16,
    csum_start:  u16,
    csum_offset: u16,
}

const NET_HDR_SIZE: usize = core::mem::size_of::<VirtioNetHdr>();

// ── VirtIO Net Driver ─────────────────────────────────────────────────────────
pub struct VirtioNet {
    mmio:      usize,
    mac:       MacAddr,
    // RX virtqueue (queue 0)
    rx_desc:   [VirtDesc;   QUEUE_DEPTH],
    rx_avail:  VirtAvail,
    rx_used:   VirtUsed,
    rx_bufs:   [[u8; MTU + NET_HDR_SIZE]; QUEUE_DEPTH],
    rx_last:   u16,
    // TX virtqueue (queue 1)
    tx_desc:   [VirtDesc;   QUEUE_DEPTH],
    tx_avail:  VirtAvail,
    tx_used:   VirtUsed,
    tx_bufs:   [[u8; MTU + NET_HDR_SIZE]; QUEUE_DEPTH],
    tx_head:   u16,
}

impl VirtioNet {
    pub fn new(mmio: usize) -> Self {
        Self {
            mmio, mac: MacAddr([0;6]),
            rx_desc:  core::array::from_fn(|_| VirtDesc{addr:0,len:0,flags:0,next:0}),
            rx_avail: VirtAvail{flags:0,idx:0,ring:[0u16;QUEUE_DEPTH]},
            rx_used:  VirtUsed{flags:0,idx:0,ring:core::array::from_fn(|_| VirtUsedElem{id:0,len:0})},
            rx_bufs:  [[0u8; MTU + NET_HDR_SIZE]; QUEUE_DEPTH],
            rx_last:  0,
            tx_desc:  core::array::from_fn(|_| VirtDesc{addr:0,len:0,flags:0,next:0}),
            tx_avail: VirtAvail{flags:0,idx:0,ring:[0u16;QUEUE_DEPTH]},
            tx_used:  VirtUsed{flags:0,idx:0,ring:core::array::from_fn(|_| VirtUsedElem{id:0,len:0})},
            tx_bufs:  [[0u8; MTU + NET_HDR_SIZE]; QUEUE_DEPTH],
            tx_head:  0,
        }
    }

    pub fn probe(mmio: usize) -> bool {
        let magic = unsafe { (mmio as *const volatile u32).read_volatile() };
        magic == 0x74726976 && unsafe { ((mmio + VIRTIO_MMIO_DEVICE_ID) as *const volatile u32).read_volatile() } == 1
    }

    pub fn init(&mut self) -> bool {
        if !Self::probe(self.mmio) { return false; }
        // Init sequence per VirtIO spec
        self.write32(VIRTIO_MMIO_STATUS, 0);
        self.write32(VIRTIO_MMIO_STATUS, STATUS_ACK | STATUS_DRIVER);

        // Negotiate features: request MAC + no merge-rxbuf
        let dev_feat = self.read32(VIRTIO_MMIO_DEVICE_FEATURES);
        let drv_feat = dev_feat & (VIRTIO_NET_F_MAC);
        self.write32(VIRTIO_MMIO_DRIVER_FEATURES, drv_feat);
        self.write32(VIRTIO_MMIO_STATUS, STATUS_ACK | STATUS_DRIVER | STATUS_FEATURES_OK);
        if self.read32(VIRTIO_MMIO_STATUS) & STATUS_FEATURES_OK == 0 { return false; }

        // Read MAC from config space (bytes 0-5)
        for i in 0..6 {
            self.mac.0[i] = unsafe {
                ((self.mmio + VIRTIO_MMIO_CONFIG + i) as *const volatile u8).read_volatile()
            };
        }

        // Set up RX queue (0)
        self.setup_queue(0,
            self.rx_desc.as_ptr() as u64,
            &self.rx_avail as *const _ as u64,
            &self.rx_used  as *const _ as u64);
        // Fill RX descriptors
        for i in 0..QUEUE_DEPTH {
            let buf_phys = self.rx_bufs[i].as_ptr() as u64;
            self.rx_desc[i] = VirtDesc {
                addr: buf_phys, len: (MTU + NET_HDR_SIZE) as u32,
                flags: VRING_DESC_F_WRITE, next: 0,
            };
            self.rx_avail.ring[i] = i as u16;
        }
        self.rx_avail.idx = QUEUE_DEPTH as u16;
        self.write32(VIRTIO_MMIO_QUEUE_NOTIFY, 0);

        // Set up TX queue (1)
        self.setup_queue(1,
            self.tx_desc.as_ptr() as u64,
            &self.tx_avail as *const _ as u64,
            &self.tx_used  as *const _ as u64);

        self.write32(VIRTIO_MMIO_STATUS,
            STATUS_ACK | STATUS_DRIVER | STATUS_FEATURES_OK | STATUS_DRIVER_OK);
        true
    }

    fn setup_queue(&self, idx: u32, desc: u64, avail: u64, used: u64) {
        self.write32(VIRTIO_MMIO_QUEUE_SEL, idx);
        let max = self.read32(VIRTIO_MMIO_QUEUE_NUM_MAX);
        let num = (QUEUE_DEPTH as u32).min(max);
        self.write32(VIRTIO_MMIO_QUEUE_NUM, num);
        self.write32(VIRTIO_MMIO_QUEUE_DESC_LOW,   (desc  & 0xFFFF_FFFF) as u32);
        self.write32(VIRTIO_MMIO_QUEUE_DESC_HIGH,  (desc  >> 32) as u32);
        self.write32(VIRTIO_MMIO_QUEUE_AVAIL_LOW,  (avail & 0xFFFF_FFFF) as u32);
        self.write32(VIRTIO_MMIO_QUEUE_AVAIL_HIGH, (avail >> 32) as u32);
        self.write32(VIRTIO_MMIO_QUEUE_USED_LOW,   (used  & 0xFFFF_FFFF) as u32);
        self.write32(VIRTIO_MMIO_QUEUE_USED_HIGH,  (used  >> 32) as u32);
        self.write32(VIRTIO_MMIO_QUEUE_READY, 1);
    }

    fn read32(&self,  off: usize) -> u32 { unsafe { ((self.mmio+off) as *const volatile u32).read_volatile() } }
    fn write32(&self, off: usize, v: u32) { unsafe { ((self.mmio+off) as *mut volatile u32).write_volatile(v); } }
}

impl NicDevice for VirtioNet {
    fn mac(&self) -> MacAddr { self.mac }

    fn recv(&mut self, buf: &mut [u8; MTU]) -> usize {
        let used_idx = self.rx_used.idx;
        if self.rx_last == used_idx { return 0; }
        let elem = &self.rx_used.ring[(self.rx_last as usize) % QUEUE_DEPTH];
        let desc_id = elem.id as usize % QUEUE_DEPTH;
        let rx_len  = elem.len as usize;
        let payload_len = rx_len.saturating_sub(NET_HDR_SIZE).min(MTU);
        buf[..payload_len].copy_from_slice(&self.rx_bufs[desc_id][NET_HDR_SIZE..NET_HDR_SIZE+payload_len]);
        // Recycle descriptor
        let next_avail = self.rx_avail.idx as usize % QUEUE_DEPTH;
        self.rx_avail.ring[next_avail] = desc_id as u16;
        self.rx_avail.idx = self.rx_avail.idx.wrapping_add(1);
        self.rx_last = self.rx_last.wrapping_add(1);
        self.write32(VIRTIO_MMIO_QUEUE_NOTIFY, 0);
        payload_len
    }

    fn send(&mut self, buf: &[u8], len: usize) {
        let len = len.min(MTU);
        let slot = (self.tx_head as usize) % QUEUE_DEPTH;
        // Zero net header
        self.tx_bufs[slot][..NET_HDR_SIZE].fill(0);
        self.tx_bufs[slot][NET_HDR_SIZE..NET_HDR_SIZE+len].copy_from_slice(&buf[..len]);
        let phys = self.tx_bufs[slot].as_ptr() as u64;
        self.tx_desc[slot] = VirtDesc {
            addr: phys, len: (NET_HDR_SIZE + len) as u32, flags: 0, next: 0,
        };
        let avail_slot = self.tx_avail.idx as usize % QUEUE_DEPTH;
        self.tx_avail.ring[avail_slot] = slot as u16;
        self.tx_avail.idx = self.tx_avail.idx.wrapping_add(1);
        self.tx_head = self.tx_head.wrapping_add(1);
        self.write32(VIRTIO_MMIO_QUEUE_NOTIFY, 1);
        // Spin-wait for TX completion
        let mut i = 0u32;
        while self.tx_used.idx != self.tx_avail.idx && i < 100_000 { i += 1; }
    }

    fn link_up(&self) -> bool {
        let status = unsafe { ((self.mmio + VIRTIO_MMIO_CONFIG + 6) as *const volatile u16).read_volatile() };
        status & 1 != 0
    }
}

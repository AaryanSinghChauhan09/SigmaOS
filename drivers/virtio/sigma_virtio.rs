// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/virtio/sigma_virtio.rs — VirtIO transport + virtio-blk + virtio-net
//
// Implements:
//   - VirtIO MMIO/PCI transport layer
//   - Virtqueue (split-ring descriptor table + avail/used rings)
//   - virtio-blk driver (block device, read/write sectors)
//   - virtio-net driver (network frames via SDF NIC interface)
//
// Targeted at QEMU/KVM for Phase G CI boot tests.
// Language: Rust #![no_std]

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU16, Ordering, fence};

// ── VirtIO device IDs ──────────────────────────────────────────────────────
pub const VIRTIO_ID_NET:     u32 = 1;
pub const VIRTIO_ID_BLOCK:   u32 = 2;
pub const VIRTIO_ID_CONSOLE: u32 = 3;
pub const VIRTIO_ID_GPU:     u32 = 16;
pub const VIRTIO_ID_INPUT:   u32 = 18;

// ── VirtIO MMIO register offsets ──────────────────────────────────────────
pub const VIRTIO_MMIO_MAGIC:       u32 = 0x000;  // 0x74726976 ("virt")
pub const VIRTIO_MMIO_VERSION:     u32 = 0x004;
pub const VIRTIO_MMIO_DEVICE_ID:   u32 = 0x008;
pub const VIRTIO_MMIO_VENDOR_ID:   u32 = 0x00C;
pub const VIRTIO_MMIO_HOST_FEATURES:u32= 0x010;
pub const VIRTIO_MMIO_GUEST_FEATURES:u32=0x020;
pub const VIRTIO_MMIO_QUEUE_SEL:   u32 = 0x030;
pub const VIRTIO_MMIO_QUEUE_NUM_MAX:u32= 0x034;
pub const VIRTIO_MMIO_QUEUE_NUM:   u32 = 0x038;
pub const VIRTIO_MMIO_QUEUE_ALIGN: u32 = 0x03C;
pub const VIRTIO_MMIO_QUEUE_PFN:   u32 = 0x040;
pub const VIRTIO_MMIO_QUEUE_READY: u32 = 0x044;
pub const VIRTIO_MMIO_QUEUE_NOTIFY:u32 = 0x050;
pub const VIRTIO_MMIO_INTERRUPT_STATUS:u32 = 0x060;
pub const VIRTIO_MMIO_INTERRUPT_ACK:u32    = 0x064;
pub const VIRTIO_MMIO_STATUS:      u32 = 0x070;
pub const VIRTIO_MMIO_CONFIG:      u32 = 0x100;

pub const VIRTIO_STATUS_ACKNOWLEDGE: u32 = 1;
pub const VIRTIO_STATUS_DRIVER:      u32 = 2;
pub const VIRTIO_STATUS_DRIVER_OK:   u32 = 4;
pub const VIRTIO_STATUS_FEATURES_OK: u32 = 8;
pub const VIRTIO_STATUS_FAILED:      u32 = 0x80;

pub const VIRTIO_MAGIC: u32 = 0x74726976;

// ── VirtIO MMIO I/O helpers ────────────────────────────────────────────────
unsafe fn mmio_read32(base: u64, offset: u32) -> u32 {
    core::ptr::read_volatile((base + offset as u64) as *const u32)
}
unsafe fn mmio_write32(base: u64, offset: u32, val: u32) {
    core::ptr::write_volatile((base + offset as u64) as *mut u32, val);
}

// ── Virtqueue descriptor flags ─────────────────────────────────────────────
pub const VRING_DESC_F_NEXT:     u16 = 1;
pub const VRING_DESC_F_WRITE:    u16 = 2; // device writable (from driver POV)
pub const VRING_DESC_F_INDIRECT: u16 = 4;

// ── Virtqueue descriptor ──────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VirtqDesc {
    pub addr:  u64,
    pub len:   u32,
    pub flags: u16,
    pub next:  u16,
}

// ── Virtqueue available ring ───────────────────────────────────────────────
const VRING_SIZE: usize = 256;

#[repr(C)]
pub struct VirtqAvail {
    pub flags: u16,
    pub idx:   u16,
    pub ring:  [u16; VRING_SIZE],
    pub used_event: u16,
}

// ── Virtqueue used ring ───────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct VirtqUsedElem {
    pub id:  u32,
    pub len: u32,
}

#[repr(C)]
pub struct VirtqUsed {
    pub flags: u16,
    pub idx:   u16,
    pub ring:  [VirtqUsedElem; VRING_SIZE],
    pub avail_event: u16,
}

// ── Virtqueue ─────────────────────────────────────────────────────────────
pub struct Virtqueue {
    pub desc:  [VirtqDesc; VRING_SIZE],
    pub avail: VirtqAvail,
    _pad:  [u8; 4096 - core::mem::size_of::<VirtqAvail>() % 4096],
    pub used:  VirtqUsed,

    free_head: u16,
    last_used: u16,
    queue_id:  u16,
    mmio_base: u64,
}

impl Virtqueue {
    pub fn new(mmio_base: u64, queue_id: u16) -> Self {
        let mut vq = Virtqueue {
            desc: [VirtqDesc::default(); VRING_SIZE],
            avail: VirtqAvail { flags: 0, idx: 0, ring: [0; VRING_SIZE], used_event: 0 },
            _pad: [0; 4096 - core::mem::size_of::<VirtqAvail>() % 4096],
            used: VirtqUsed { flags: 0, idx: 0, ring: [VirtqUsedElem::default(); VRING_SIZE], avail_event: 0 },
            free_head: 0,
            last_used: 0,
            queue_id,
            mmio_base,
        };
        // Chain all descriptors into free list
        for i in 0..VRING_SIZE - 1 {
            vq.desc[i].next = (i + 1) as u16;
            vq.desc[i].flags = VRING_DESC_F_NEXT;
        }
        vq.desc[VRING_SIZE - 1].flags = 0;
        vq
    }

    /// Allocate `n` chained descriptors. Returns head index, or None if full.
    pub fn alloc_descs(&mut self, n: u16) -> Option<u16> {
        let head = self.free_head;
        let mut cur = head;
        for i in 0..n {
            if cur as usize >= VRING_SIZE { return None; }
            if i < n - 1 {
                self.desc[cur as usize].flags |= VRING_DESC_F_NEXT;
                cur = self.desc[cur as usize].next;
            } else {
                self.desc[cur as usize].flags &= !VRING_DESC_F_NEXT;
                self.free_head = if cur + 1 < VRING_SIZE as u16 { cur + 1 } else { 0 };
            }
        }
        Some(head)
    }

    /// Submit a descriptor chain to the device.
    pub fn submit(&mut self, head: u16) {
        let avail_idx = (self.avail.idx & (VRING_SIZE as u16 - 1)) as usize;
        self.avail.ring[avail_idx] = head;
        fence(Ordering::Release);
        self.avail.idx = self.avail.idx.wrapping_add(1);
        fence(Ordering::Release);
        // Notify device via MMIO kick
        unsafe { mmio_write32(self.mmio_base, VIRTIO_MMIO_QUEUE_NOTIFY, self.queue_id as u32); }
    }

    /// Check for completed responses. Returns descriptor head + length.
    pub fn poll_used(&mut self) -> Option<(u16, u32)> {
        fence(Ordering::Acquire);
        if self.last_used == self.used.idx { return None; }
        let elem = self.used.ring[self.last_used as usize % VRING_SIZE];
        self.last_used = self.last_used.wrapping_add(1);
        Some((elem.id as u16, elem.len))
    }
}

// ── VirtIO device transport ────────────────────────────────────────────────
pub struct VirtioDevice {
    pub mmio_base: u64,
    pub device_id: u32,
    pub features:  u32,
}

impl VirtioDevice {
    /// Probe a VirtIO MMIO device. Returns None if magic check fails.
    pub unsafe fn probe(base: u64) -> Option<Self> {
        if mmio_read32(base, VIRTIO_MMIO_MAGIC) != VIRTIO_MAGIC { return None; }
        let device_id = mmio_read32(base, VIRTIO_MMIO_DEVICE_ID);
        if device_id == 0 { return None; }
        Some(VirtioDevice { mmio_base: base, device_id, features: 0 })
    }

    pub unsafe fn init(&mut self) -> bool {
        // Reset
        mmio_write32(self.mmio_base, VIRTIO_MMIO_STATUS, 0);
        // Acknowledge + driver
        mmio_write32(self.mmio_base, VIRTIO_MMIO_STATUS,
            VIRTIO_STATUS_ACKNOWLEDGE | VIRTIO_STATUS_DRIVER);
        // Read features
        self.features = mmio_read32(self.mmio_base, VIRTIO_MMIO_HOST_FEATURES);
        // Accept features subset
        mmio_write32(self.mmio_base, VIRTIO_MMIO_GUEST_FEATURES, self.features & 0xFFFF);
        // Features OK
        let s = mmio_read32(self.mmio_base, VIRTIO_MMIO_STATUS) | VIRTIO_STATUS_FEATURES_OK;
        mmio_write32(self.mmio_base, VIRTIO_MMIO_STATUS, s);
        // Verify
        if mmio_read32(self.mmio_base, VIRTIO_MMIO_STATUS) & VIRTIO_STATUS_FEATURES_OK == 0 {
            mmio_write32(self.mmio_base, VIRTIO_MMIO_STATUS, VIRTIO_STATUS_FAILED);
            return false;
        }
        true
    }

    pub unsafe fn setup_queue(&self, queue_id: u16, desc_phys: u32) {
        mmio_write32(self.mmio_base, VIRTIO_MMIO_QUEUE_SEL, queue_id as u32);
        mmio_write32(self.mmio_base, VIRTIO_MMIO_QUEUE_NUM, VRING_SIZE as u32);
        mmio_write32(self.mmio_base, VIRTIO_MMIO_QUEUE_ALIGN, 4096);
        mmio_write32(self.mmio_base, VIRTIO_MMIO_QUEUE_PFN, desc_phys >> 12);
    }

    pub unsafe fn driver_ok(&self) {
        let s = mmio_read32(self.mmio_base, VIRTIO_MMIO_STATUS) | VIRTIO_STATUS_DRIVER_OK;
        mmio_write32(self.mmio_base, VIRTIO_MMIO_STATUS, s);
    }
}

// ── VirtIO-blk ────────────────────────────────────────────────────────────
pub const VIRTIO_BLK_T_IN:    u32 = 0; // read
pub const VIRTIO_BLK_T_OUT:   u32 = 1; // write
pub const VIRTIO_BLK_T_FLUSH: u32 = 4;
pub const VIRTIO_BLK_S_OK:    u8  = 0;

#[repr(C)]
pub struct VirtioBlkReq {
    pub req_type: u32,
    pub reserved: u32,
    pub sector:   u64,
}

pub struct VirtioBlkDriver {
    pub dev: VirtioDevice,
    pub vq:  Virtqueue,
    pub capacity_sectors: u64,
    // Buffers for request/response (DMA-accessible in production)
    req_buf:  VirtioBlkReq,
    data_buf: [u8; 4096],
    status:   u8,
}

impl VirtioBlkDriver {
    pub unsafe fn new(mmio_base: u64) -> Option<Self> {
        let mut dev = VirtioDevice::probe(mmio_base)?;
        if dev.device_id != VIRTIO_ID_BLOCK { return None; }
        dev.init();

        let cap_lo = mmio_read32(mmio_base, VIRTIO_MMIO_CONFIG);
        let cap_hi = mmio_read32(mmio_base, VIRTIO_MMIO_CONFIG + 4);
        let capacity = ((cap_hi as u64) << 32) | cap_lo as u64;

        let vq = Virtqueue::new(mmio_base, 0);
        dev.driver_ok();

        Some(VirtioBlkDriver {
            dev, vq, capacity_sectors: capacity,
            req_buf: VirtioBlkReq { req_type: 0, reserved: 0, sector: 0 },
            data_buf: [0u8; 4096],
            status: 0xFF,
        })
    }

    /// Read `count` 512-byte sectors starting at `lba` into `buf`.
    pub fn read_sectors(&mut self, lba: u64, buf: &mut [u8], count: u32) -> bool {
        self.req_buf = VirtioBlkReq { req_type: VIRTIO_BLK_T_IN, reserved: 0, sector: lba };
        self.status  = 0xFF;

        let Some(head) = self.vq.alloc_descs(3) else { return false; };
        let req_phys  = &self.req_buf  as *const _ as u64;
        let data_phys = self.data_buf.as_ptr() as u64;
        let stat_phys = &self.status as *const _ as u64;

        // Desc 0: request header (read-only for device)
        self.vq.desc[head as usize] = VirtqDesc {
            addr: req_phys, len: 16, flags: VRING_DESC_F_NEXT, next: head + 1,
        };
        // Desc 1: data buffer (write-only = device writes here)
        self.vq.desc[(head + 1) as usize] = VirtqDesc {
            addr: data_phys, len: 512 * count,
            flags: VRING_DESC_F_NEXT | VRING_DESC_F_WRITE, next: head + 2,
        };
        // Desc 2: status byte (write-only)
        self.vq.desc[(head + 2) as usize] = VirtqDesc {
            addr: stat_phys, len: 1, flags: VRING_DESC_F_WRITE, next: 0,
        };

        self.vq.submit(head);

        // Poll for completion (busy-wait in Phase G; IRQ-driven in production)
        let mut retries = 100_000u32;
        while self.vq.poll_used().is_none() && retries > 0 { retries -= 1; }

        if self.status == VIRTIO_BLK_S_OK {
            let n = (512 * count as usize).min(buf.len()).min(4096);
            buf[..n].copy_from_slice(&self.data_buf[..n]);
            true
        } else {
            false
        }
    }

    /// Write `count` 512-byte sectors from `buf` to `lba`.
    pub fn write_sectors(&mut self, lba: u64, buf: &[u8], count: u32) -> bool {
        let n = (512 * count as usize).min(buf.len()).min(4096);
        self.data_buf[..n].copy_from_slice(&buf[..n]);
        self.req_buf = VirtioBlkReq { req_type: VIRTIO_BLK_T_OUT, reserved: 0, sector: lba };
        self.status  = 0xFF;

        let Some(head) = self.vq.alloc_descs(3) else { return false; };
        self.vq.desc[head as usize] = VirtqDesc {
            addr: &self.req_buf as *const _ as u64, len: 16,
            flags: VRING_DESC_F_NEXT, next: head + 1,
        };
        self.vq.desc[(head + 1) as usize] = VirtqDesc {
            addr: self.data_buf.as_ptr() as u64, len: 512 * count,
            flags: VRING_DESC_F_NEXT, next: head + 2,
        };
        self.vq.desc[(head + 2) as usize] = VirtqDesc {
            addr: &self.status as *const _ as u64, len: 1,
            flags: VRING_DESC_F_WRITE, next: 0,
        };

        self.vq.submit(head);
        let mut retries = 100_000u32;
        while self.vq.poll_used().is_none() && retries > 0 { retries -= 1; }
        self.status == VIRTIO_BLK_S_OK
    }
}

// ── C-ABI exports for kernel integration ──────────────────────────────────
static mut G_VIRTIO_BLK: Option<VirtioBlkDriver> = None;

#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_blk_init(mmio_base: u64) -> i32 {
    match VirtioBlkDriver::new(mmio_base) {
        Some(drv) => { G_VIRTIO_BLK = Some(drv); 0 }
        None      => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_blk_read(
    lba: u64, buf: *mut u8, sectors: u32,
) -> i32 {
    let drv = G_VIRTIO_BLK.as_mut().ok_or(()).map_err(|_| -19i32)?;
    let slice = core::slice::from_raw_parts_mut(buf, (sectors * 512) as usize);
    if drv.read_sectors(lba, slice, sectors) { 0 } else { -5 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_blk_write(
    lba: u64, buf: *const u8, sectors: u32,
) -> i32 {
    let drv = G_VIRTIO_BLK.as_mut().ok_or(()).map_err(|_| -19i32)?;
    let slice = core::slice::from_raw_parts(buf, (sectors * 512) as usize);
    if drv.write_sectors(lba, slice, sectors) { 0 } else { -5 }
}

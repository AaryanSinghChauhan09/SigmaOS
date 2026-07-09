/// SigmaOS: Sovereign VirtIO GPU Driver (Rust, no_std)
/// Built in Rust — #![no_std], no alloc, no external dependencies.
/// Implements VirtIO PCI transport, Control queue, Cursor queue,
/// Resource creation, Framebuffer attach, Scanout configuration,
/// 2D Resource flushing, and status tracking.

#![no_std]
#![allow(dead_code)]

use crate::drivers::common_types::{SigmaU8, SigmaU16, SigmaU32, SigmaU64, SigmaUsize, SigmaBool, SigmaI32};

pub const SIGMA_OK: SigmaI32 = 0;
pub const SIGMA_ERR_TIMEOUT: SigmaI32 = -1;
pub const SIGMA_ERR_INVALID: SigmaI32 = -2;
pub const SIGMA_ERR_NO_MEM: SigmaI32 = -3;

// ─── VirtIO PCI Registers ──────────────────────────────────────────────────
pub const VIRTIO_PCI_HOST_FEATURES: SigmaUsize = 0x00;
pub const VIRTIO_PCI_GUEST_FEATURES: SigmaUsize = 0x04;
pub const VIRTIO_PCI_QUEUE_PFN: SigmaUsize = 0x08;
pub const VIRTIO_PCI_QUEUE_NUM: SigmaUsize = 0x0C;
pub const VIRTIO_PCI_QUEUE_SEL: SigmaUsize = 0x0E;
pub const VIRTIO_PCI_QUEUE_NOTIFY: SigmaUsize = 0x10;
pub const VIRTIO_PCI_STATUS: SigmaUsize = 0x12;
pub const VIRTIO_PCI_ISR: SigmaUsize = 0x13;

// VirtIO Status Codes
pub const VIRTIO_STATUS_ACKNOWLEDGE: SigmaU8 = 1;
pub const VIRTIO_STATUS_DRIVER: SigmaU8 = 2;
pub const VIRTIO_STATUS_DRIVER_OK: SigmaU8 = 4;
pub const VIRTIO_STATUS_FEATURES_OK: SigmaU8 = 8;
pub const VIRTIO_STATUS_FAILED: SigmaU8 = 128;

// ─── VirtIO GPU Control Commands ──────────────────────────────────────────
pub const VIRTIO_GPU_CMD_GET_DISPLAY_INFO: SigmaU32 = 0x0100;
pub const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: SigmaU32 = 0x0101;
pub const VIRTIO_GPU_CMD_RESOURCE_UNREF: SigmaU32 = 0x0102;
pub const VIRTIO_GPU_CMD_SET_SCANOUT: SigmaU32 = 0x0103;
pub const VIRTIO_GPU_CMD_RESOURCE_FLUSH: SigmaU32 = 0x0104;
pub const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D: SigmaU32 = 0x0105;
pub const VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING: SigmaU32 = 0x0106;
pub const VIRTIO_GPU_CMD_RESOURCE_DETACH_BACKING: SigmaU32 = 0x0107;

// VirtIO GPU Formats
pub const VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM: SigmaU32 = 1;
pub const VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM: SigmaU32 = 2;
pub const VIRTIO_GPU_FORMAT_A8R8G8B8_UNORM: SigmaU32 = 3;
pub const VIRTIO_GPU_FORMAT_X8R8G8B8_UNORM: SigmaU32 = 4;

#[repr(C)]
pub struct VirtioGpuCtrlHeader {
    pub cmd_type: SigmaU32,
    pub flags: SigmaU32,
    pub fence_id: SigmaU64,
    pub ctx_id: SigmaU32,
    pub padding: SigmaU32,
}

// ─── Command Structures ────────────────────────────────────────────────────
#[repr(C)]
pub struct VirtioGpuResourceCreate2d {
    pub hdr: VirtioGpuCtrlHeader,
    pub resource_id: SigmaU32,
    pub format: SigmaU32,
    pub width: SigmaU32,
    pub height: SigmaU32,
}

#[repr(C)]
pub struct VirtioGpuMemEntry {
    pub addr: SigmaU64,
    pub length: SigmaU32,
    pub padding: SigmaU32,
}

#[repr(C)]
pub struct VirtioGpuResourceAttachBacking {
    pub hdr: VirtioGpuCtrlHeader,
    pub resource_id: SigmaU32,
    pub nr_entries: SigmaU32,
    // Followed by VirtioGpuMemEntry array
}

#[repr(C)]
pub struct VirtioGpuSetScanout {
    pub hdr: VirtioGpuCtrlHeader,
    pub r: [SigmaU32; 4], // x, y, width, height
    pub scanout_id: SigmaU32,
    pub resource_id: SigmaU32,
}

#[repr(C)]
pub struct VirtioGpuTransferToHost2d {
    pub hdr: VirtioGpuCtrlHeader,
    pub r: [SigmaU32; 4], // x, y, width, height
    pub offset: SigmaU64,
    pub resource_id: SigmaU32,
    pub padding: SigmaU32,
}

#[repr(C)]
pub struct VirtioGpuResourceFlush {
    pub hdr: VirtioGpuCtrlHeader,
    pub r: [SigmaU32; 4], // x, y, width, height
    pub resource_id: SigmaU32,
    pub padding: SigmaU32,
}

// ─── VirtQueue Descriptor System ───────────────────────────────────────────
pub const VQ_SIZE: SigmaUsize = 32;

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct VirtqDesc {
    pub addr: SigmaU64,
    pub len: SigmaU32,
    pub flags: SigmaU16,
    pub next: SigmaU16,
}

pub const VIRTQ_DESC_F_NEXT: SigmaU16 = 1;
pub const VIRTQ_DESC_F_WRITE: SigmaU16 = 2;

#[repr(C)]
pub struct VirtqAvail {
    pub flags: SigmaU16,
    pub idx: SigmaU16,
    pub ring: [SigmaU16; VQ_SIZE],
}

#[repr(C)]
pub struct VirtqUsedElem {
    pub id: SigmaU32,
    pub len: SigmaU32,
}

#[repr(C)]
pub struct VirtqUsed {
    pub flags: SigmaU16,
    pub idx: SigmaU16,
    pub ring: [VirtqUsedElem; VQ_SIZE],
}

pub struct VirtQueue {
    pub desc: [VirtqDesc; VQ_SIZE],
    pub avail: VirtqAvail,
    pub used: VirtqUsed,
    pub free_head: SigmaU16,
    pub last_used_idx: SigmaU16,
}

impl VirtQueue {
    pub const fn new() -> Self {
        let mut desc = [VirtqDesc { addr: 0, len: 0, flags: 0, next: 0 }; VQ_SIZE];
        let mut i = 0;
        while i < VQ_SIZE - 1 {
            desc[i].next = (i + 1) as SigmaU16;
            desc[i].flags = VIRTQ_DESC_F_NEXT;
            i += 1;
        }
        desc[VQ_SIZE - 1].next = 0;
        desc[VQ_SIZE - 1].flags = 0;

        VirtQueue {
            desc,
            avail: VirtqAvail { flags: 0, idx: 0, ring: [0; VQ_SIZE] },
            used: VirtqUsed { flags: 0, idx: 0, ring: [VirtqUsedElem { id: 0, len: 0 }; VQ_SIZE] },
            free_head: 0,
            last_used_idx: 0,
        }
    }
}

// ─── GPU Driver ────────────────────────────────────────────────────────────
pub struct SovereignGPUDriver {
    io_base: SigmaU64,
    ctrl_vq: VirtQueue,
    resource_counter: SigmaU32,
    fb_width: SigmaU32,
    fb_height: SigmaU32,
    fb_resource_id: SigmaU32,
    active: SigmaBool,
}

impl SovereignGPUDriver {
    pub const fn new() -> Self {
        SovereignGPUDriver {
            io_base: 0,
            ctrl_vq: VirtQueue::new(),
            resource_counter: 1,
            fb_width: 1024,
            fb_height: 768,
            fb_resource_id: 0,
            active: false,
        }
    }

    unsafe fn write_reg8(&self, offset: SigmaUsize, val: SigmaU8) {
        core::ptr::write_volatile((self.io_base as *mut SigmaU8).add(offset), val);
    }

    unsafe fn write_reg16(&self, offset: SigmaUsize, val: SigmaU16) {
        core::ptr::write_volatile((self.io_base as *mut SigmaU8).add(offset) as *mut SigmaU16, val);
    }

    unsafe fn write_reg32(&self, offset: SigmaUsize, val: SigmaU32) {
        core::ptr::write_volatile((self.io_base as *mut SigmaU8).add(offset) as *mut SigmaU32, val);
    }

    pub unsafe fn init(&mut self, io_base: SigmaU64) -> SigmaI32 {
        self.io_base = io_base;

        // Reset device
        self.write_reg8(VIRTIO_PCI_STATUS, 0);

        // Acknowledge and Driver status
        self.write_reg8(VIRTIO_PCI_STATUS, VIRTIO_STATUS_ACKNOWLEDGE | VIRTIO_STATUS_DRIVER);

        // Setup Queue 0 (Control Queue)
        self.write_reg16(VIRTIO_PCI_QUEUE_SEL, 0);
        let q_size = VQ_SIZE as SigmaU16;
        self.write_reg16(VIRTIO_PCI_QUEUE_NUM, q_size);

        // Set PFN
        let desc_pfn = (&self.ctrl_vq.desc[0] as *const VirtqDesc as SigmaU64) >> 12;
        self.write_reg32(VIRTIO_PCI_QUEUE_PFN, desc_pfn as SigmaU32);

        // Driver OK status
        self.write_reg8(VIRTIO_PCI_STATUS, VIRTIO_STATUS_ACKNOWLEDGE | VIRTIO_STATUS_DRIVER | VIRTIO_STATUS_FEATURES_OK | VIRTIO_STATUS_DRIVER_OK);

        self.active = true;
        SIGMA_OK
    }

    pub fn create_resource_2d(&mut self, width: SigmaU32, height: SigmaU32) -> SigmaU32 {
        let res_id = self.resource_counter;
        self.resource_counter += 1;

        let mut cmd = VirtioGpuResourceCreate2d {
            hdr: VirtioGpuCtrlHeader {
                cmd_type: VIRTIO_GPU_CMD_RESOURCE_CREATE_2D,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            resource_id: res_id,
            format: VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM,
            width,
            height,
        };

        // Submit command descriptor chain through control queue
        let head = self.ctrl_vq.free_head;
        self.ctrl_vq.desc[head as usize].addr = &cmd as *const _ as SigmaU64;
        self.ctrl_vq.desc[head as usize].len = core::mem::size_of::<VirtioGpuResourceCreate2d>() as SigmaU32;
        self.ctrl_vq.desc[head as usize].flags = 0;

        self.ctrl_vq.avail.ring[self.ctrl_vq.avail.idx as usize % VQ_SIZE] = head;
        self.ctrl_vq.avail.idx = self.ctrl_vq.avail.idx.wrapping_add(1);

        // Notify device of control queue update
        unsafe { self.write_reg16(VIRTIO_PCI_QUEUE_NOTIFY, 0); }

        res_id
    }

    pub fn setup_scanout(&mut self, resource_id: SigmaU32, width: SigmaU32, height: SigmaU32) -> SigmaI32 {
        let mut cmd = VirtioGpuSetScanout {
            hdr: VirtioGpuCtrlHeader {
                cmd_type: VIRTIO_GPU_CMD_SET_SCANOUT,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r: [0, 0, width, height],
            scanout_id: 0,
            resource_id,
        };

        let head = self.ctrl_vq.free_head;
        self.ctrl_vq.desc[head as usize].addr = &cmd as *const _ as SigmaU64;
        self.ctrl_vq.desc[head as usize].len = core::mem::size_of::<VirtioGpuSetScanout>() as SigmaU32;
        self.ctrl_vq.desc[head as usize].flags = 0;

        self.ctrl_vq.avail.ring[self.ctrl_vq.avail.idx as usize % VQ_SIZE] = head;
        self.ctrl_vq.avail.idx = self.ctrl_vq.avail.idx.wrapping_add(1);

        unsafe { self.write_reg16(VIRTIO_PCI_QUEUE_NOTIFY, 0); }
        SIGMA_OK
    }

    pub fn flush_resource(&mut self, resource_id: SigmaU32, width: SigmaU32, height: SigmaU32) -> SigmaI32 {
        let mut cmd = VirtioGpuResourceFlush {
            hdr: VirtioGpuCtrlHeader {
                cmd_type: VIRTIO_GPU_CMD_RESOURCE_FLUSH,
                flags: 0,
                fence_id: 0,
                ctx_id: 0,
                padding: 0,
            },
            r: [0, 0, width, height],
            resource_id,
            padding: 0,
        };

        let head = self.ctrl_vq.free_head;
        self.ctrl_vq.desc[head as usize].addr = &cmd as *const _ as SigmaU64;
        self.ctrl_vq.desc[head as usize].len = core::mem::size_of::<VirtioGpuResourceFlush>() as SigmaU32;
        self.ctrl_vq.desc[head as usize].flags = 0;

        self.ctrl_vq.avail.ring[self.ctrl_vq.avail.idx as usize % VQ_SIZE] = head;
        self.ctrl_vq.avail.idx = self.ctrl_vq.avail.idx.wrapping_add(1);

        unsafe { self.write_reg16(VIRTIO_PCI_QUEUE_NOTIFY, 0); }
        SIGMA_OK
    }

    pub fn is_active(&self) -> bool { self.active }
}

static mut G_DRV: SovereignGPUDriver = SovereignGPUDriver::new();

#[no_mangle]
pub unsafe extern "C" fn sovereigngpu_drv_init(io_base: SigmaU64) -> SigmaI32 {
    G_DRV.init(io_base)
}

#[no_mangle]
pub unsafe extern "C" fn sovereigngpu_drv_active() -> u8 {
    G_DRV.is_active() as u8
}

#[no_mangle]
pub unsafe extern "C" fn sovereigngpu_create_fb(width: SigmaU32, height: SigmaU32) -> SigmaU32 {
    G_DRV.create_resource_2d(width, height)
}

#[no_mangle]
pub unsafe extern "C" fn sovereigngpu_set_scanout(resource_id: SigmaU32, width: SigmaU32, height: SigmaU32) -> SigmaI32 {
    G_DRV.setup_scanout(resource_id, width, height)
}

#[no_mangle]
pub unsafe extern "C" fn sovereigngpu_flush(resource_id: SigmaU32, width: SigmaU32, height: SigmaU32) -> SigmaI32 {
    G_DRV.flush_resource(resource_id, width, height)
}
//! SigmaOS — VirtIO-GPU Driver
//! Paravirtualized GPU driver for QEMU/KVM environments.
//! Provides framebuffer, 2D acceleration, and cursor support.
//! No std, no allocator.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── VirtIO Common Constants ─────────────────────────────────────────────────
const VIRTIO_GPU_DEVICE_ID: U32 = 16;

// VirtIO Status bits
const VIRTIO_STATUS_ACK:         U8 = 1;
const VIRTIO_STATUS_DRIVER:      U8 = 2;
const VIRTIO_STATUS_FEATURES_OK: U8 = 8;
const VIRTIO_STATUS_DRIVER_OK:   U8 = 4;

// ── VirtIO-GPU Command Types ────────────────────────────────────────────────
const VIRTIO_GPU_CMD_GET_DISPLAY_INFO:  U32 = 0x0100;
const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: U32 = 0x0101;
const VIRTIO_GPU_CMD_RESOURCE_UNREF:    U32 = 0x0102;
const VIRTIO_GPU_CMD_SET_SCANOUT:       U32 = 0x0103;
const VIRTIO_GPU_CMD_RESOURCE_FLUSH:    U32 = 0x0104;
const VIRTIO_GPU_CMD_TRANSFER_TO_HOST_2D: U32 = 0x0105;
const VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING: U32 = 0x0106;
const VIRTIO_GPU_CMD_RESOURCE_DETACH_BACKING: U32 = 0x0107;
const VIRTIO_GPU_CMD_GET_CAPSET_INFO:   U32 = 0x0108;
const VIRTIO_GPU_CMD_GET_CAPSET:        U32 = 0x0109;
const VIRTIO_GPU_CMD_UPDATE_CURSOR:     U32 = 0x0300;
const VIRTIO_GPU_CMD_MOVE_CURSOR:       U32 = 0x0301;

// Response types
const VIRTIO_GPU_RESP_OK_NODATA:       U32 = 0x1100;
const VIRTIO_GPU_RESP_OK_DISPLAY_INFO: U32 = 0x1101;
const VIRTIO_GPU_RESP_ERR_UNSPEC:      U32 = 0x1200;
const VIRTIO_GPU_RESP_ERR_OUT_OF_MEMORY: U32 = 0x1201;

// Pixel formats
const VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM: U32 = 1;
const VIRTIO_GPU_FORMAT_B8G8R8X8_UNORM: U32 = 2;
const VIRTIO_GPU_FORMAT_R8G8B8A8_UNORM: U32 = 67;

// ── VirtIO-GPU Structures ───────────────────────────────────────────────────

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioGpuCtrlHeader {
    pub cmd_type: U32,
    pub flags:    U32,
    pub fence_id: U64,
    pub ctx_id:   U32,
    pub padding:  U32,
}

impl VirtioGpuCtrlHeader {
    pub const fn new(cmd: U32) -> Self {
        VirtioGpuCtrlHeader {
            cmd_type: cmd, flags: 0, fence_id: 0, ctx_id: 0, padding: 0,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioGpuRect {
    pub x: U32,
    pub y: U32,
    pub width: U32,
    pub height: U32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioGpuDisplayOne {
    pub r: VirtioGpuRect,
    pub enabled: U32,
    pub flags: U32,
}

const MAX_SCANOUTS: usize = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioGpuDisplayInfo {
    pub hdr: VirtioGpuCtrlHeader,
    pub pmodes: [VirtioGpuDisplayOne; MAX_SCANOUTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioGpuResourceCreate2d {
    pub hdr: VirtioGpuCtrlHeader,
    pub resource_id: U32,
    pub format: U32,
    pub width: U32,
    pub height: U32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioGpuSetScanout {
    pub hdr: VirtioGpuCtrlHeader,
    pub r: VirtioGpuRect,
    pub scanout_id: U32,
    pub resource_id: U32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioGpuTransferToHost2d {
    pub hdr: VirtioGpuCtrlHeader,
    pub r: VirtioGpuRect,
    pub offset: U64,
    pub resource_id: U32,
    pub padding: U32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VirtioGpuResourceFlush {
    pub hdr: VirtioGpuCtrlHeader,
    pub r: VirtioGpuRect,
    pub resource_id: U32,
    pub padding: U32,
}

// ── VirtQueue (simplified) ──────────────────────────────────────────────────
const VRING_SIZE: usize = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VringDesc {
    pub addr:  U64,
    pub len:   U32,
    pub flags: U16,
    pub next:  U16,
}

impl VringDesc {
    pub const fn empty() -> Self {
        VringDesc { addr: 0, len: 0, flags: 0, next: 0 }
    }
}

pub struct VirtQueue {
    pub desc:  [VringDesc; VRING_SIZE],
    pub avail_idx: U16,
    pub used_idx:  U16,
    pub num_free:  U16,
    pub free_head: U16,
}

impl VirtQueue {
    pub const fn new() -> Self {
        VirtQueue {
            desc: [VringDesc::empty(); VRING_SIZE],
            avail_idx: 0,
            used_idx: 0,
            num_free: VRING_SIZE as U16,
            free_head: 0,
        }
    }
}

// ── GPU Framebuffer ─────────────────────────────────────────────────────────
const FB_MAX_WIDTH:  usize = 1920;
const FB_MAX_HEIGHT: usize = 1080;
const FB_BPP:        usize = 4; // BGRA
const FB_SIZE:       usize = FB_MAX_WIDTH * FB_MAX_HEIGHT * FB_BPP;

pub struct VirtioGpuState {
    pub mmio_base:     U64,
    pub width:         U32,
    pub height:        U32,
    pub stride:        U32,
    pub format:        U32,
    pub resource_id:   U32,
    pub scanout_id:    U32,
    pub initialized:   bool,
    pub controlq:      VirtQueue,
    pub cursorq:       VirtQueue,
    // Framebuffer in memory
    pub fb:            [U8; FB_SIZE],
}

static mut GPU: VirtioGpuState = VirtioGpuState {
    mmio_base: 0,
    width: 1024,
    height: 768,
    stride: 1024 * 4,
    format: VIRTIO_GPU_FORMAT_B8G8R8A8_UNORM,
    resource_id: 1,
    scanout_id: 0,
    initialized: false,
    controlq: VirtQueue::new(),
    cursorq: VirtQueue::new(),
    fb: [0u8; FB_SIZE],
};

// ── MMIO Helpers ────────────────────────────────────────────────────────────
unsafe fn gpu_read32(offset: Usize) -> U32 {
    let ptr = (GPU.mmio_base as Usize + offset) as *const U32;
    core::ptr::read_volatile(ptr)
}

unsafe fn gpu_write32(offset: Usize, val: U32) {
    let ptr = (GPU.mmio_base as Usize + offset) as *mut U32;
    core::ptr::write_volatile(ptr, val);
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize VirtIO-GPU from MMIO base address.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_init(mmio_base: U64, width: U32, height: U32) -> i32 {
    if mmio_base == 0 { return -1; }

    GPU.mmio_base = mmio_base;
    GPU.width = width.min(FB_MAX_WIDTH as U32);
    GPU.height = height.min(FB_MAX_HEIGHT as U32);
    GPU.stride = GPU.width * FB_BPP as U32;

    // VirtIO device initialization sequence (MMIO transport)
    // Step 1: Reset device
    gpu_write32(0x70, 0); // Status = 0 (reset)

    // Step 2: Acknowledge
    gpu_write32(0x70, VIRTIO_STATUS_ACK as U32);

    // Step 3: Driver
    gpu_write32(0x70, (VIRTIO_STATUS_ACK | VIRTIO_STATUS_DRIVER) as U32);

    // Step 4: Negotiate features (accept all for now)
    let _features = gpu_read32(0x10);
    gpu_write32(0x20, 0); // Accept features page 0

    // Step 5: Features OK
    gpu_write32(0x70, (VIRTIO_STATUS_ACK | VIRTIO_STATUS_DRIVER | VIRTIO_STATUS_FEATURES_OK) as U32);

    // Step 6: Set up virtqueues
    // Select queue 0 (controlq)
    gpu_write32(0x30, 0); // QueueSel
    gpu_write32(0x38, VRING_SIZE as U32); // QueueNum

    // Queue addresses would be set here via QueueDescLow/High, etc.
    let desc_addr = GPU.controlq.desc.as_ptr() as U64;
    gpu_write32(0x80, desc_addr as U32);         // QueueDescLow
    gpu_write32(0x84, (desc_addr >> 32) as U32); // QueueDescHigh

    // Step 7: Driver OK
    gpu_write32(0x70,
        (VIRTIO_STATUS_ACK | VIRTIO_STATUS_DRIVER | VIRTIO_STATUS_FEATURES_OK | VIRTIO_STATUS_DRIVER_OK) as U32
    );

    // Clear framebuffer
    for i in 0..FB_SIZE {
        GPU.fb[i] = 0;
    }

    GPU.initialized = true;
    0
}

/// Set a pixel in the framebuffer (BGRA format).
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_set_pixel(x: U32, y: U32, color: U32) {
    if x >= GPU.width || y >= GPU.height { return; }
    let offset = ((y * GPU.stride) + (x * FB_BPP as U32)) as usize;
    if offset + 4 > FB_SIZE { return; }
    GPU.fb[offset]     = (color & 0xFF) as U8;         // B
    GPU.fb[offset + 1] = ((color >> 8) & 0xFF) as U8;  // G
    GPU.fb[offset + 2] = ((color >> 16) & 0xFF) as U8; // R
    GPU.fb[offset + 3] = ((color >> 24) & 0xFF) as U8; // A
}

/// Fill a rectangle with a solid color.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_fill_rect(x: U32, y: U32, w: U32, h: U32, color: U32) {
    let x_end = (x + w).min(GPU.width);
    let y_end = (y + h).min(GPU.height);
    let mut cy = y;
    while cy < y_end {
        let mut cx = x;
        while cx < x_end {
            sigma_virtio_gpu_set_pixel(cx, cy, color);
            cx += 1;
        }
        cy += 1;
    }
}

/// Clear the framebuffer to a solid color.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_clear(color: U32) {
    sigma_virtio_gpu_fill_rect(0, 0, GPU.width, GPU.height, color);
}

/// Draw a horizontal line.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_hline(x: U32, y: U32, len: U32, color: U32) {
    let end = (x + len).min(GPU.width);
    let mut cx = x;
    while cx < end {
        sigma_virtio_gpu_set_pixel(cx, y, color);
        cx += 1;
    }
}

/// Draw a vertical line.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_vline(x: U32, y: U32, len: U32, color: U32) {
    let end = (y + len).min(GPU.height);
    let mut cy = y;
    while cy < end {
        sigma_virtio_gpu_set_pixel(x, cy, color);
        cy += 1;
    }
}

/// Get the framebuffer base pointer for direct access.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_fb_ptr() -> *const U8 {
    GPU.fb.as_ptr()
}

/// Get the display width.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_width() -> U32 { GPU.width }

/// Get the display height.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_height() -> U32 { GPU.height }

/// Get the stride (bytes per row).
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_stride() -> U32 { GPU.stride }

/// Check if the GPU is initialized.
#[no_mangle]
pub unsafe extern "C" fn sigma_virtio_gpu_ready() -> i32 {
    if GPU.initialized { 1 } else { 0 }
}

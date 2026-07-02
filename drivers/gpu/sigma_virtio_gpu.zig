// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/gpu/sigma_virtio_gpu.zig — VirtIO GPU Driver (QEMU accelerated)
// Language: Zig — direct MMIO, comptime command structs
// Pattern: struct with methods

// ── VirtIO GPU Control Commands ──────────────────────────────────────────────

const VIRTIO_GPU_CMD_GET_DISPLAY_INFO:  u32 = 0x0100;
const VIRTIO_GPU_CMD_RESOURCE_CREATE_2D: u32 = 0x0101;
const VIRTIO_GPU_CMD_RESOURCE_UNREF:    u32 = 0x0102;
const VIRTIO_GPU_CMD_SET_SCANOUT:       u32 = 0x0103;
const VIRTIO_GPU_CMD_RESOURCE_FLUSH:    u32 = 0x0104;
const VIRTIO_GPU_CMD_TRANSFER_TO_HOST:  u32 = 0x0105;
const VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING: u32 = 0x0106;
const VIRTIO_GPU_RESP_OK_NODATA:        u32 = 0x1100;
const VIRTIO_GPU_RESP_OK_DISPLAY_INFO:  u32 = 0x1101;

// ── Pixel Format ─────────────────────────────────────────────────────────────

const VIRTIO_GPU_FORMAT_B8G8R8A8: u32 = 1;
const VIRTIO_GPU_FORMAT_R8G8B8A8: u32 = 67;

// ── Structs ───────────────────────────────────────────────────────────────────

const CtrlHdr = extern struct {
    type_:     u32,
    flags:     u32,
    fence_id:  u64,
    ctx_id:    u32,
    padding:   u32,
};

const Rect = extern struct { x: u32, y: u32, w: u32, h: u32 };

const CmdResourceCreate2D = extern struct {
    hdr:        CtrlHdr,
    resource_id: u32,
    format:     u32,
    width:      u32,
    height:     u32,
};

const CmdSetScanout = extern struct {
    hdr:        CtrlHdr,
    r:          Rect,
    scanout_id: u32,
    resource_id: u32,
};

const CmdResourceFlush = extern struct {
    hdr:         CtrlHdr,
    r:           Rect,
    resource_id: u32,
    padding:     u32,
};

const CmdTransferToHost = extern struct {
    hdr:         CtrlHdr,
    r:           Rect,
    offset:      u64,
    resource_id: u32,
    padding:     u32,
};

const MemEntry = extern struct {
    addr:    u64,
    length:  u32,
    padding: u32,
};

const CmdAttachBacking = extern struct {
    hdr:         CtrlHdr,
    resource_id: u32,
    nr_entries:  u32,
    entries:     [1]MemEntry,
};

// ── VirtQueue (simplified ring) ───────────────────────────────────────────────

const VRING_SIZE: usize = 16;

const VirtDesc = extern struct {
    addr:  u64,
    len:   u32,
    flags: u16,
    next:  u16,
};

const VirtAvail = extern struct {
    flags: u16,
    idx:   u16,
    ring:  [VRING_SIZE]u16,
};

const VirtUsedElem = extern struct { id: u32, len: u32 };

const VirtUsed = extern struct {
    flags: u16,
    idx:   u16,
    ring:  [VRING_SIZE]VirtUsedElem,
};

// ── VirtIO Device Layout (MMIO) ───────────────────────────────────────────────

const VIRTIO_MMIO_MAGIC:          usize = 0x000;
const VIRTIO_MMIO_VERSION:        usize = 0x004;
const VIRTIO_MMIO_DEVICE_ID:      usize = 0x008;
const VIRTIO_MMIO_VENDOR_ID:      usize = 0x00C;
const VIRTIO_MMIO_STATUS:         usize = 0x070;
const VIRTIO_MMIO_QUEUE_SEL:      usize = 0x030;
const VIRTIO_MMIO_QUEUE_NUM_MAX:  usize = 0x034;
const VIRTIO_MMIO_QUEUE_NUM:      usize = 0x038;
const VIRTIO_MMIO_QUEUE_NOTIFY:   usize = 0x050;
const VIRTIO_MMIO_QUEUE_DESC_LOW: usize = 0x080;
const VIRTIO_MMIO_QUEUE_DESC_HIGH:usize = 0x084;
const VIRTIO_MMIO_QUEUE_AVAIL_LOW: usize = 0x090;
const VIRTIO_MMIO_QUEUE_AVAIL_HIGH:usize = 0x094;
const VIRTIO_MMIO_QUEUE_USED_LOW:  usize = 0x0A0;
const VIRTIO_MMIO_QUEUE_USED_HIGH: usize = 0x0A4;
const VIRTIO_MMIO_QUEUE_READY:     usize = 0x044;

// ── VirtIO GPU Driver ─────────────────────────────────────────────────────────

pub const VirtioGpu = struct {
    mmio:        usize,
    width:       u32,
    height:      u32,
    framebuffer: usize, // physical address of pixel buffer
    resource_id: u32,

    // Virtqueue structures
    ctrl_desc:   [VRING_SIZE]VirtDesc,
    ctrl_avail:  VirtAvail,
    ctrl_used:   VirtUsed,
    avail_idx:   u16,
    used_last:   u16,

    // Scratch buffers for commands
    cmd_buf: [512]u8,
    rsp_buf: [512]u8,

    pub fn init(mmio: usize, fb_phys: usize, w: u32, h: u32) VirtioGpu {
        return VirtioGpu{
            .mmio        = mmio,
            .width       = w,
            .height      = h,
            .framebuffer = fb_phys,
            .resource_id = 1,
            .ctrl_desc   = @splat(VirtDesc{ .addr=0,.len=0,.flags=0,.next=0 }),
            .ctrl_avail  = @bitCast(@as(u128 * 3, 0)), // zero
            .ctrl_used   = @bitCast(@as(u128 * 5, 0)),
            .avail_idx   = 0,
            .used_last   = 0,
            .cmd_buf     = @splat(0),
            .rsp_buf     = @splat(0),
        };
    }

    pub fn setup(self: *VirtioGpu) bool {
        // Verify magic
        const magic = self.read32(VIRTIO_MMIO_MAGIC);
        if (magic != 0x74726976) return false; // "virt"
        if (self.read32(VIRTIO_MMIO_DEVICE_ID) != 16) return false; // GPU device

        // Status: ACKNOWLEDGE + DRIVER
        self.write32(VIRTIO_MMIO_STATUS, 0);
        self.write32(VIRTIO_MMIO_STATUS, 1 | 2);

        // Set up control virtqueue (queue 0)
        self.write32(VIRTIO_MMIO_QUEUE_SEL, 0);
        self.write32(VIRTIO_MMIO_QUEUE_NUM, VRING_SIZE);
        const desc_phys = @intFromPtr(&self.ctrl_desc);
        self.write32(VIRTIO_MMIO_QUEUE_DESC_LOW,  @truncate(desc_phys));
        self.write32(VIRTIO_MMIO_QUEUE_DESC_HIGH, @truncate(desc_phys >> 32));
        const avail_phys = @intFromPtr(&self.ctrl_avail);
        self.write32(VIRTIO_MMIO_QUEUE_AVAIL_LOW,  @truncate(avail_phys));
        self.write32(VIRTIO_MMIO_QUEUE_AVAIL_HIGH, @truncate(avail_phys >> 32));
        const used_phys = @intFromPtr(&self.ctrl_used);
        self.write32(VIRTIO_MMIO_QUEUE_USED_LOW,  @truncate(used_phys));
        self.write32(VIRTIO_MMIO_QUEUE_USED_HIGH, @truncate(used_phys >> 32));
        self.write32(VIRTIO_MMIO_QUEUE_READY, 1);

        // DRIVER_OK
        self.write32(VIRTIO_MMIO_STATUS, 1 | 2 | 4);

        // Create 2D resource
        _ = self.create_resource_2d();
        _ = self.attach_backing();
        _ = self.set_scanout();
        return true;
    }

    fn create_resource_2d(self: *VirtioGpu) bool {
        var cmd: CmdResourceCreate2D = .{
            .hdr = .{ .type_=VIRTIO_GPU_CMD_RESOURCE_CREATE_2D, .flags=0,
                      .fence_id=0, .ctx_id=0, .padding=0 },
            .resource_id = self.resource_id,
            .format = VIRTIO_GPU_FORMAT_B8G8R8A8,
            .width  = self.width,
            .height = self.height,
        };
        return self.submit_cmd(&cmd, @sizeOf(CmdResourceCreate2D));
    }

    fn attach_backing(self: *VirtioGpu) bool {
        var cmd: CmdAttachBacking = .{
            .hdr = .{ .type_=VIRTIO_GPU_CMD_RESOURCE_ATTACH_BACKING, .flags=0,
                      .fence_id=0, .ctx_id=0, .padding=0 },
            .resource_id = self.resource_id,
            .nr_entries  = 1,
            .entries = .{.{ .addr = self.framebuffer, .length = self.width * self.height * 4, .padding=0 }},
        };
        return self.submit_cmd(&cmd, @sizeOf(CmdAttachBacking));
    }

    fn set_scanout(self: *VirtioGpu) bool {
        var cmd: CmdSetScanout = .{
            .hdr = .{ .type_=VIRTIO_GPU_CMD_SET_SCANOUT, .flags=0,
                      .fence_id=0, .ctx_id=0, .padding=0 },
            .r = .{ .x=0, .y=0, .w=self.width, .h=self.height },
            .scanout_id  = 0,
            .resource_id = self.resource_id,
        };
        return self.submit_cmd(&cmd, @sizeOf(CmdSetScanout));
    }

    /// Flush dirty rectangle to screen
    pub fn flush(self: *VirtioGpu, x: u32, y: u32, w: u32, h: u32) bool {
        // Transfer host → scanout
        var t: CmdTransferToHost = .{
            .hdr = .{ .type_=VIRTIO_GPU_CMD_TRANSFER_TO_HOST, .flags=0,
                      .fence_id=0, .ctx_id=0, .padding=0 },
            .r = .{ .x=x, .y=y, .w=w, .h=h },
            .offset      = 0,
            .resource_id = self.resource_id,
            .padding     = 0,
        };
        _ = self.submit_cmd(&t, @sizeOf(CmdTransferToHost));
        // Flush
        var f: CmdResourceFlush = .{
            .hdr = .{ .type_=VIRTIO_GPU_CMD_RESOURCE_FLUSH, .flags=0,
                      .fence_id=0, .ctx_id=0, .padding=0 },
            .r = .{ .x=x, .y=y, .w=w, .h=h },
            .resource_id = self.resource_id,
            .padding     = 0,
        };
        return self.submit_cmd(&f, @sizeOf(CmdResourceFlush));
    }

    fn submit_cmd(self: *VirtioGpu, cmd: anytype, cmd_len: usize) bool {
        const desc_idx = self.avail_idx % VRING_SIZE;
        // Descriptor 0: command
        self.ctrl_desc[desc_idx] = VirtDesc{
            .addr  = @intFromPtr(cmd),
            .len   = @intCast(cmd_len),
            .flags = 1, // NEXT
            .next  = @intCast((desc_idx + 1) % VRING_SIZE),
        };
        // Descriptor 1: response
        self.ctrl_desc[(desc_idx + 1) % VRING_SIZE] = VirtDesc{
            .addr  = @intFromPtr(&self.rsp_buf),
            .len   = 64,
            .flags = 2, // WRITE
            .next  = 0,
        };
        self.ctrl_avail.ring[self.ctrl_avail.idx % VRING_SIZE] = @intCast(desc_idx);
        self.ctrl_avail.idx += 1;
        self.avail_idx      += 2;
        // Notify queue 0
        self.write32(VIRTIO_MMIO_QUEUE_NOTIFY, 0);
        // Spin wait for used ring
        var spin: u32 = 0;
        while (self.ctrl_used.idx == self.used_last and spin < 100_000) : (spin += 1) {}
        if (self.ctrl_used.idx != self.used_last) {
            self.used_last = self.ctrl_used.idx;
            const rsp = @as(*const u32, @ptrCast(&self.rsp_buf));
            return rsp.* == VIRTIO_GPU_RESP_OK_NODATA or rsp.* == VIRTIO_GPU_RESP_OK_DISPLAY_INFO;
        }
        return false;
    }

    fn read32(self: *const VirtioGpu, off: usize) u32 {
        const ptr: *const volatile u32 = @ptrFromInt(self.mmio + off);
        return ptr.*;
    }
    fn write32(self: *VirtioGpu, off: usize, val: u32) void {
        const ptr: *volatile u32 = @ptrFromInt(self.mmio + off);
        ptr.* = val;
    }
};

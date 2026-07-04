// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
// drivers/usb/sigma_xhci.zig — xHCI USB 3.x Host Controller Driver
// Language: Zig — direct MMIO, comptime register offsets, no libc

// ── xHCI Capability Registers (offset from BAR0) ─────────────────────────────
const CAP_CAPLENGTH:   usize = 0x00;
const CAP_HCIVERSION:  usize = 0x02;
const CAP_HCSPARAMS1:  usize = 0x04;
const CAP_HCSPARAMS2:  usize = 0x08;
const CAP_HCCPARAMS1:  usize = 0x10;
const CAP_DBOFF:       usize = 0x14;
const CAP_RTSOFF:      usize = 0x18;

// ── xHCI Operational Registers ────────────────────────────────────────────────
const OP_USBCMD:  usize = 0x00;
const OP_USBSTS:  usize = 0x04;
const OP_PAGESIZE: usize = 0x08;
const OP_DNCTRL:  usize = 0x14;
const OP_CRCR:    usize = 0x18; // Command Ring Control Register (64-bit)
const OP_DCBAAP:  usize = 0x30; // Device Context Base Address Array Pointer
const OP_CONFIG:  usize = 0x38;

// ── USBCMD bits ───────────────────────────────────────────────────────────────
const CMD_RUN:     u32 = 1 << 0;
const CMD_HCRST:   u32 = 1 << 1;
const CMD_EWE:     u32 = 1 << 10;

// ── USBSTS bits ───────────────────────────────────────────────────────────────
const STS_HCH:     u32 = 1 << 0; // Host Controller Halted
const STS_CNR:     u32 = 1 << 11; // Controller Not Ready

// ── Transfer Request Block (TRB) — 16 bytes ──────────────────────────────────
const Trb = extern struct {
    param:   u64,
    status:  u32,
    control: u32,
};

const TRB_TYPE_NORMAL:      u32 = 1 << 10;
const TRB_TYPE_SETUP_STAGE: u32 = 2 << 10;
const TRB_TYPE_DATA_STAGE:  u32 = 3 << 10;
const TRB_TYPE_STATUS_STAGE: u32 = 4 << 10;
const TRB_TYPE_LINK:         u32 = 6 << 10;
const TRB_TYPE_ENABLE_SLOT:  u32 = 9 << 10;
const TRB_TYPE_ADDRESS_DEV:  u32 = 11 << 10;
const TRB_TYPE_NOOP_CMD:     u32 = 23 << 10;
const TRB_CYCLE:             u32 = 1 << 0;
const TRB_IOC:               u32 = 1 << 5;

const RING_SIZE: usize = 256;

// ── Device Descriptor (simplified) ───────────────────────────────────────────
pub const UsbDevDesc = extern struct {
    length:       u8,
    desc_type:    u8,
    usb_spec:     u16,
    dev_class:    u8,
    dev_subclass: u8,
    dev_protocol: u8,
    max_pkt_sz0:  u8,
    id_vendor:    u16,
    id_product:   u16,
    dev_release:  u16,
    mfr_idx:      u8,
    product_idx:  u8,
    serial_idx:   u8,
    num_configs:  u8,
};

// ── Port Status ───────────────────────────────────────────────────────────────
pub const PortStatus = struct {
    connected: bool,
    enabled:   bool,
    speed:     u4, // 1=Full, 2=Low, 3=High, 4=Super
};

// ── xHCI Driver ───────────────────────────────────────────────────────────────
pub const XhciController = struct {
    mmio:      usize,
    op_base:   usize,  // mmio + cap_length
    db_base:   usize,  // doorbell array
    rt_base:   usize,  // runtime registers
    max_ports: u8,
    max_slots: u8,

    // Command ring
    cmd_ring:  [RING_SIZE]Trb,
    cmd_enq:   usize,
    cmd_cycle: u32,

    // Event ring (primary)
    evt_ring:  [RING_SIZE]Trb,
    evt_deq:   usize,
    evt_cycle: u32,

    // Device context base address array
    dcbaa: [256]u64,

    pub fn init(mmio: usize) XhciController {
        var self = XhciController{
            .mmio = mmio, .op_base = 0, .db_base = 0, .rt_base = 0,
            .max_ports = 0, .max_slots = 0,
            .cmd_ring = @splat(Trb{.param=0,.status=0,.control=0}),
            .cmd_enq = 0, .cmd_cycle = 1,
            .evt_ring = @splat(Trb{.param=0,.status=0,.control=0}),
            .evt_deq = 0, .evt_cycle = 1,
            .dcbaa = @splat(0),
        };
        const cap_len = self.read8(CAP_CAPLENGTH);
        self.op_base = mmio + cap_len;
        self.db_base = mmio + self.read32(CAP_DBOFF);
        self.rt_base = mmio + self.read32(CAP_RTSOFF);
        const hcsp1  = self.read32(CAP_HCSPARAMS1);
        self.max_slots = @truncate(hcsp1 & 0xFF);
        self.max_ports = @truncate((hcsp1 >> 24) & 0xFF);
        return self;
    }

    pub fn reset(self: *XhciController) bool {
        // Halt controller
        self.write_op32(OP_USBCMD, self.read_op32(OP_USBCMD) & ~CMD_RUN);
        var i: u32 = 0;
        while ((self.read_op32(OP_USBSTS) & STS_HCH) == 0 and i < 100_000) : (i += 1) {}
        if ((self.read_op32(OP_USBSTS) & STS_HCH) == 0) return false;

        // Reset
        self.write_op32(OP_USBCMD, CMD_HCRST);
        i = 0;
        while ((self.read_op32(OP_USBCMD) & CMD_HCRST) != 0 and i < 100_000) : (i += 1) {}
        i = 0;
        while ((self.read_op32(OP_USBSTS) & STS_CNR) != 0 and i < 100_000) : (i += 1) {}

        // Configure max slots
        self.write_op32(OP_CONFIG, self.max_slots);

        // Set up DCBAA
        self.dcbaa[0] = 0; // scratchpad — simplified, skip
        const dcbaa_phys: u64 = @intFromPtr(&self.dcbaa);
        self.write_op64(OP_DCBAAP, dcbaa_phys);

        // Set up command ring
        const crcr: u64 = @intFromPtr(&self.cmd_ring) | 1; // cycle bit
        self.write_op64(OP_CRCR, crcr);

        // Start controller
        self.write_op32(OP_USBCMD, CMD_RUN | CMD_EWE);
        i = 0;
        while ((self.read_op32(OP_USBSTS) & STS_HCH) != 0 and i < 100_000) : (i += 1) {}
        return (self.read_op32(OP_USBSTS) & STS_HCH) == 0;
    }

    pub fn port_status(self: *XhciController, port: u8) PortStatus {
        const portsc_off: usize = 0x400 + @as(usize, port) * 0x10;
        const portsc = self.read_op32(portsc_off);
        return PortStatus{
            .connected = (portsc & 1) != 0,
            .enabled   = (portsc & 2) != 0,
            .speed     = @truncate((portsc >> 10) & 0xF),
        };
    }

    fn enqueue_cmd(self: *XhciController, trb: Trb) void {
        const idx = self.cmd_enq;
        var t = trb;
        t.control = (t.control & ~@as(u32,1)) | self.cmd_cycle;
        self.cmd_ring[idx] = t;
        self.cmd_enq = (idx + 1) % (RING_SIZE - 1);
        if (self.cmd_enq == 0) {
            // Insert link TRB + toggle cycle
            self.cmd_ring[RING_SIZE - 1] = Trb{
                .param   = @intFromPtr(&self.cmd_ring),
                .status  = 0,
                .control = TRB_TYPE_LINK | TRB_CYCLE | (1 << 1),
            };
            self.cmd_cycle ^= 1;
        }
    }

    pub fn ring_doorbell(self: *XhciController, slot: u8, endpoint: u8) void {
        const db: *volatile u32 = @ptrFromInt(self.db_base + @as(usize, slot) * 4);
        db.* = endpoint;
    }

    fn read8 (self: *const XhciController, off: usize) u8  { const p: *const volatile u8  = @ptrFromInt(self.mmio + off); return p.*; }
    fn read32(self: *const XhciController, off: usize) u32 { const p: *const volatile u32 = @ptrFromInt(self.mmio + off); return p.*; }
    fn read_op32(self: *const XhciController, off: usize) u32  { const p: *const volatile u32 = @ptrFromInt(self.op_base + off); return p.*; }
    fn write_op32(self: *XhciController, off: usize, v: u32)   { const p: *volatile u32 = @ptrFromInt(self.op_base + off); p.* = v; }
    fn write_op64(self: *XhciController, off: usize, v: u64)   {
        self.write_op32(off,     @truncate(v));
        self.write_op32(off + 4, @truncate(v >> 32));
    }
};

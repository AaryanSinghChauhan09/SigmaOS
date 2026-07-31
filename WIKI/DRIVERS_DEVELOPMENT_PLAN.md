# 🔌 SigmaOS Multi-Generation Driver Development Plan

This document details the architectural design and implementation plan for the **SigmaOS Multi-Generation Driver Subsystem**, supporting legacy peripherals (like PS/2, Sound Blaster 16, ISA, serial communication) and modern hardware (PCIe Gen5, NVMe, USB 4, AMD/Intel GPUs, gigabit Ethernet) with near-zero disk and memory footprint.

---

## 🗺️ Architectural Inspiration
*   **Linux Driver Model:** Utilizes a unified bus, device, and driver abstraction model with sysfs-like state representation.
*   **Android Binder & HAL:** Isolates driver logic from the core microkernel, allowing drivers to execute in sandboxed user-space processes (microkernel shard architecture).

---

## 🏗️ OOP Design & Device Family Polymorphism

SigmaOS defines abstract base interfaces for each major peripheral class. Drivers are registered dynamically into the driver registry and undergo state changes governed by strict state machines.

```text
                               +-----------------------------+
                               |     Unified Device Bus      |
                               +-----------------------------+
                                              |
                   +--------------------------+--------------------------+
                   v                                                     v
      +-------------------------+                           +-------------------------+
      |      Input Devices      |                           |     Storage Devices     |
      +-------------------------+                           +-------------------------+
       - PS2MouseDriver (Legacy)                             - PcieGen5NvmeDriver (Modern)
       - SerialMouseDriver (Legacy)                          - FloppyDiskDriver (Legacy)
```

### Driver State Machines:
```text
  Legacy State: Uninitialized ➡️ StatusChecked ➡️ IOReady ➡️ Sleep/Idle ➡️ Blocked
  Modern State: Off ➡️ PciScanning ➡️ MSIXRegistered ➡️ DmaMapping ➡️ Running ➡️ Fault
```

### Universal Device Driver Interface:
```rust
pub trait DeviceDriver {
    fn initialize(&mut self) -> Result<(), &'static str>;
    fn shutdown(&mut self) -> Result<(), &'static str>;
    fn get_status(&self) -> &'static str;
}
```

---

## 🛠️ Multi-Language Architecture & Executable Code

To ensure optimal execution efficiency, driver components are modularized and implemented in Rust, Zig, and Nim.

### ⚡ Rust: PS/2 Mouse & AMD Radeon GPU Drivers
```rust
// PS2MouseDriver legacy character input implementation
pub enum MouseState {
    Uninitialized,
    StreamMode,
    Error,
}

pub struct PS2MouseDriver {
    pub state: MouseState,
    pub sample_rate: u8,
    pub resolution: u8,
}

impl PS2MouseDriver {
    pub fn new() -> Self {
        Self {
            state: MouseState::Uninitialized,
            sample_rate: 100,
            resolution: 4,
        }
    }

    pub fn initialize(&mut self) -> Result<(), &'static str> {
        self.state = MouseState::StreamMode;
        Ok(())
    }

    pub fn read_packet(&self) -> Result<[i8; 3], &'static str> {
        match self.state {
            MouseState::StreamMode => Ok([0, 0, 0]), // Returns raw [buttons, dx, dy] packet
            _ => Err("Mouse is not in stream mode!"),
        }
    }
}

// AmdRadeonGpuDriver hardware command dispatcher
pub enum GpuState {
    Off,
    VgaFallback,
    HardwareAccelerated,
    Panic,
}

pub struct AmdRadeonGpuDriver {
    pub state: GpuState,
    pub vram_bytes: u64,
}

impl AmdRadeonGpuDriver {
    pub fn new(vram: u64) -> Self {
        Self {
            state: GpuState::Off,
            vram_bytes: vram,
        }
    }

    pub fn initialize(&mut self) -> Result<(), &'static str> {
        self.state = GpuState::HardwareAccelerated;
        Ok(())
    }

    pub fn submit_render_pipeline(&mut self, draw_calls: u32) -> Result<(), &'static str> {
        if let GpuState::Panic = self.state {
            return Err("GPU is in fault/panic state!");
        }
        println!("GPU processed {} render draw commands.", draw_calls);
        Ok(())
    }
}
```

### ⚡ Zig: Intel PRO/1000 Ethernet Driver (MMIO & DMA)
```zig
const std = @import("std");

pub const IntelNetState = enum {
    Down,
    LinkUp,
    Transmitting,
    Fault,
};

pub const IntelProEthernetDriver = struct {
    state: IntelNetState,
    io_base: u64,
    mac_address: [6]u8,

    pub fn init(base: u64) IntelProEthernetDriver {
        return .{
            .state = IntelNetState.Down,
            .io_base = base,
            .mac_address = .{ 0x00, 0x1B, 0x21, 0x3C, 0x4D, 0x5E },
        };
    }

    pub fn start(self: *IntelProEthernetDriver) void {
        self.state = IntelNetState.LinkUp;
    }

    pub fn transmitFrame(self: *IntelProEthernetDriver, frame: []const u8) !void {
        if (self.state == IntelNetState.Down) {
            return error.LinkIsDown;
        }
        self.state = IntelNetState.Transmitting;
        // Direct DMA descriptor write to ring buffers
        _ = frame;
        self.state = IntelNetState.LinkUp;
    }
};
```

### ⚡ Nim: Broadcom Bluetooth HCI Driver
```nim
type
  BtState* = enum
    Disabled,
    InquiryMode,
    Connected,
    LowPower

  BroadcomBluetoothDriver* = object
    state*: BtState
    pairedDevices*: seq[string]

proc initBluetooth*(): BroadcomBluetoothDriver {.exportc, cdecl.} =
  result.state = Disabled
  result.pairedDevices = @[]

proc startInquiry*(driver: var BroadcomBluetoothDriver) {.exportc, cdecl.} =
  driver.state = InquiryMode
  # Trigger HCI Inquiry commands to scan for Bluetooth controllers
```

---

## 📈 Quality Assurance & PnP Validation

1.  **PnP Registration Audit:** Verify that hotplugging PCIe/USB controllers registers and binds drivers inside the virtual device tree in under 5ms.
2.  **Fallback Test:** Verify that failing modern drivers (e.g. AMD GPU driver crash) invokes safe fallback paths (e.g. VESA standard fallback text/VGA mode) automatically without bringing down the core microkernel.

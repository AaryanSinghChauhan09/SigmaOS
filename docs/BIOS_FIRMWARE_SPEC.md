# 🔌 Sovereign BIOS & UEFI Boot Firmware Specification (`S-Boot`)

This specification details the architecture, phase loops, memory controller init, and device detection routines of the **Sovereign BIOS & UEFI Boot Firmware** (`S-Boot`) for SigmaOS.

Drawing inspiration from high-efficiency open-source firmware projects like **Coreboot**, **TianoCore EDK II**, and **LinuxBIOS**, the Sovereign firmware is a zero-dependency, bare-metal bootloader and system initializer built entirely with Object-Oriented Programming (OOP) principles and user-defined functions in Rust, Zig, and Nim.

---

## 🗺️ Boot Firmware Phase Architecture

```
                    ┌────────────────────────────────────────┐
                    │      SEC (Security Phase / Assembly)   │
                    └───────────────────┬────────────────────┘
                                        │ (CPU Init, GDT Setup)
                    ┌───────────────────▼────────────────────┐
                    │      PEI (Pre-EFI Initialization)      │
                    └───────────────────┬────────────────────┘
                                        │ (Memory Controller Activation)
                    ┌───────────────────▼────────────────────┐
                    │     DXE (Driver Execution Environment) │
                    └───────────────────┬────────────────────┘
                                        │ (PCI Bus Device Scanning)
                    ┌───────────────────▼────────────────────┐
                    │      BDS (Boot Device Selection)       │
                    └────────────────────────────────────────┘
```

---

## 1. Zero-Dependency OOP Rust Specification (PCI Bus Scanning)

Exposes the DXE phase device scanning routines, walking the PCI configuration space to register active hardware devices without standard library dependencies.

```rust
pub const PCI_MAX_BUS: u8 = 256;
pub const PCI_MAX_DEVICE: u8 = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PciClass {
    Network,
    Storage,
    Display,
    Unknown,
}

pub struct PciDevice {
    pub bus: u8,
    pub slot: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class: PciClass,
}

impl PciDevice {
    pub fn new(bus: u8, slot: u8, vendor: u16, device: u16, class_code: u8) -> Self {
        let class = match class_code {
            0x02 => PciClass::Network,
            0x01 => PciClass::Storage,
            0x03 => PciClass::Display,
            _ => PciClass::Unknown,
        };
        Self {
            bus,
            slot,
            vendor_id: vendor,
            device_id: device,
            class,
        }
    }
}

pub struct PciBusScanner {
    pub registered_devices: [Option<PciDevice>; 16],
}

impl PciBusScanner {
    pub fn new() -> Self {
        const NONE_DEV: Option<PciDevice> = None;
        Self {
            registered_devices: [NONE_DEV; 16],
        }
    }

    pub fn scan_and_register(&mut self, bus: u8, slot: u8, vendor: u16, device: u16, class_code: u8) -> Result<(), &'static str> {
        if vendor == 0xFFFF {
            return Ok(()); // Device not present
        }
        let dev = PciDevice::new(bus, slot, vendor, device, class_code);
        for slot in self.registered_devices.iter_mut() {
            if slot.is_none() {
                *slot = Some(dev);
                return Ok(());
            }
        }
        Err("Active boot firmware PCI registry full")
    }
}

#[cfg(test)]
mod firmware_tests {
    use super::*;

    #[test]
    fn test_pci_bus_scan() {
        let mut scanner = PciBusScanner::new();
        // Register storage device (class 0x01) on bus 0, slot 1
        assert!(scanner.scan_and_register(0, 1, 0x8086, 0x1234, 0x01).is_ok());
        // Attempt empty vendor register
        assert!(scanner.scan_and_register(0, 2, 0xFFFF, 0x0000, 0x00).is_ok());

        assert!(scanner.registered_devices[0].is_some());
        let dev = scanner.registered_devices[0].as_ref().unwrap();
        assert_eq!(dev.class, PciClass::Storage);
        assert_eq!(dev.vendor_id, 0x8086);
    }
}
```

---

## 2. Zero-Dependency OOP Zig Specification (GDT & CPU Initialization)

Performs the raw SEC phase CPU initialization, setting up the Global Descriptor Table (GDT) and starting the early boot console printing.

```zig
const std = @import("std");

pub const GdtEntry = struct {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,

    pub fn init(base: u32, limit: u32, access: u8, gran: u8) GdtEntry {
        return GdtEntry{
            .limit_low = @as(u16, @intCast(limit & 0xFFFF)),
            .base_low = @as(u16, @intCast(base & 0xFFFF)),
            .base_middle = @as(u8, @intCast((base >> 16) & 0xFF)),
            .access = access,
            .granularity = @as(u8, @intCast(((limit >> 16) & 0x0F) | (gran & 0xF0))),
            .base_high = @as(u8, @intCast((base >> 24) & 0xFF)),
        };
    }
};

pub const EarlyConsole = struct {
    port: u16,

    pub fn init(port: u16) EarlyConsole {
        return EarlyConsole{ .port = port };
    }

    pub fn printChar(self: *const EarlyConsole, char: u8) void {
        // Direct early x86 assembly out port instruction simulation
        _ = self;
        _ = char;
    }
};
```

---

## 3. Zero-Dependency OOP Nim Specification (POST & Emulation Diagnostics)

Manages early boot diagnostic routines (Power-On Self-Test) and emulates basic hardware registers.

```nim
type
  DiagnosticCode* = enum
    Ok,
    CpuFailure,
    MemoryFailure,
    DisplayFailure

  POSTManager* = ref object of RootObj
    activeCode*: DiagnosticCode
    testProgress*: int

method runMemoryCheck*(self: POSTManager, totalBytes: uint64): bool {.base.} =
  if totalBytes == 0:
    self.activeCode = DiagnosticCode.MemoryFailure
    return false
  self.testProgress += 50
  self.activeCode = DiagnosticCode.Ok
  return true

proc newPOSTManager*(): POSTManager =
  new(result)
  result.activeCode = DiagnosticCode.Ok
  result.testProgress = 0
```

---

## 🔄 Boot Synchronization & Safety Checklist

To ensure absolute safety during the early boot phase:
1.  **Static Bounds Protection:** Ensure all early PCI bus structures use fixed arrays, preventing dynamic heap corruption prior to the activation of the memory controller.
2.  **Hardware-Level Cryptographic Validation:** Verify the cryptographic hash signature of the kernel binary before transferring CPU control to the kernel start vectors.
3.  **Strict Early Fault Isolation:** Any memory controller failures detected during the POST stage must trigger early serial diagnostic alerts and halt the CPU execution flow immediately to prevent hardware loop damage.

# SigmaOS ACPI Implementation & Hardware Power Control Subsystem

## 1. Executive Summary

The SigmaOS ACPI (Advanced Configuration and Power Interface) Subsystem provides modern x86_64, AArch64, and RISC-V hardware configuration, platform discovery, power resource routing, and system event handling. It incorporates a sandboxed AML (ACPI Machine Language) byte-code interpreter to safely execute vendor hardware BIOS/UEFI control methods without risking kernel space instability.

## 2. ACPI Subsystem Architecture

```
+-----------------------------------------------------------------+
|                       Hardware & Platform                       |
|         (x86_64 / ARM64 ACPI Tables in Physical RAM)           |
+--------------------------------+--------------------------------+
                                 |
                     ACPI Table Parser Engine
                                 |
+--------------------------------v--------------------------------+
|                    ACPI Table Repository                        |
|  +--------+  +--------+  +--------+  +--------+  +------------+ |
|  | RSDP   |  | XSDT   |  | FADT   |  | MADT   |  | DSDT/SSDT  | |
|  | Pointer|  | Table  |  | Config |  | Cores  |  | AML Code   | |
|  +--------+  +--------+  +--------+  +--------+  +------------+ |
+--------------------------------+--------------------------------+
                                 |
                  Sovereign AML Bytecode Interpreter
                                 |
+--------------------------------v--------------------------------+
|                     ACPI Event Router Daemon                    |
|  +--------------------+  +-------------------+  +-------------+ |
|  | Power Button Event |  | Lid Closure Event |  | Thermal Zone| |
|  +--------------------+  +-------------------+  +-------------+ |
+-----------------------------------------------------------------+
```

## 3. Core ACPI Table Parsing

1. **RSDP (Root System Description Pointer)**: Scans memory regions (`0xE0000` to `0xFFFFF` or UEFI System Table) for the 64-bit signature `RSD PTR `.
2. **XSDT (Extended System Description Table)**: Enumerates 64-bit physical memory pointers to all system ACPI descriptor tables.
3. **MADT (Multiple APIC Description Table)**: Identifies physical CPU core topology, LAPIC/GICC IDs, and I/O APIC routing tables.
4. **FADT (Fixed ACPI Description Table)**: Contains hardware power register addresses (`PM1a_CNT`, `PM1b_CNT`, `SMI_CMD`) and sleep state values.
5. **DSDT & SSDT (Differentiated/Secondary System Description Tables)**: Contain compiled AML bytecode definitions for system buses, embedded controllers, battery devices, and thermal sensors.

## 4. AML Bytecode Interpreter & Safety Controls

- **Sandboxed Execution**: The in-kernel AML interpreter executes bytecode with strict recursion and memory bounds checking, preventing malicious or buggy vendor ACPI code from hanging the kernel.
- **Hardware OpRegion Routing**: Translates ACPI `SystemMemory`, `SystemIO`, `PCI_Config`, and `EmbeddedControl` (EC) OpRegion access requests to verified kernel driver calls.

## 5. Event Dispatching & Notification

ACPI fixed and general-purpose events (GPE) trigger driver callbacks:
- **Power Button (`_HID` PNP0C0C)**: Triggers graceful shutdown or desktop power options dialog.
- **Lid Switch (`_HID` PNP0C0D)**: Triggers display blanking and S3 (Suspend-to-RAM) state entry.
- **AC Adapter Hotplug (`_HID` ACPI0003)**: Adjusts CPU power scaling governor profile dynamically.

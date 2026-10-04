# Drivers

SigmaOS implements a comprehensive device driver subsystem written entirely in Rust. Drivers run in kernel space with strict memory safety guarantees, no C unsafe blocks, and a clean hardware abstraction layer (HAL) that supports hot-plug, power management, and runtime PM.

---

## Architecture Overview

```
 ┌──────────────────────────────────────────────────────┐
 │              Hardware Abstraction Layer (HAL)         │
 │  src/hal/ — unified device interface                  │
 └──────────────┬───────────────────────────────────────┘
                │
 ┌──────────────▼───────────────────────────────────────┐
 │                 Driver Subsystems                     │
 │                                                       │
 │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐  │
 │  │  Storage │ │ Network  │ │  Audio   │ │ Input  │  │
 │  │ AHCI/NVMe│ │ Ethernet │ │ Intel HDA│ │Keyboard│  │
 │  └──────────┘ └──────────┘ └──────────┘ └────────┘  │
 │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌────────┐  │
 │  │   USB    │ │ Graphics │ │  PCI Bus │ │Bluetooth│  │
 │  │ XHCI/HID │ │Framebuf. │ │  ECAM   │ │  BT 5  │  │
 │  └──────────┘ └──────────┘ └──────────┘ └────────┘  │
 └──────────────────────────────────────────────────────┘
                │
 ┌──────────────▼───────────────────────────────────────┐
 │              Physical Hardware                        │
 └──────────────────────────────────────────────────────┘
```

---

## Storage Drivers

### AHCI / SATA (`src/drivers/ahci_sata.rs`)
- AHCI 1.3.1 specification compliant
- Native Command Queuing (NCQ): 32 commands in-flight
- Hot-plug detection via COMRESET
- SMART data readout
- Port Multiplier support

### NVMe (`src/drivers/nvme.rs`)
- NVMe 1.4 specification
- Admin + I/O queue pairs (up to 64 queues per namespace)
- Namespace management
- Power state transitions (PS0–PS5)
- Write zeroes, dataset management (TRIM)

---

## Network Drivers

### Ethernet (`src/drivers/ethernet.rs`)
- Generic Ethernet frame handler
- EthernetHeader: src/dst MAC, EtherType parsing
- VLAN 802.1Q tagging support
- Jumbo frames (up to 9000 bytes)

### Intel E1000 / RTL8139 / Virtio-Net
- Ring-based DMA descriptor queues
- Interrupt coalescing (NAPI-style polling)
- Scatter-gather I/O
- Hardware checksum offload

---

## Audio Drivers

### Intel HDA (`src/drivers/audio_intel_hda.rs`)
- Intel High Definition Audio specification
- Codec enumeration via verb/response interface
- Stream format: sample rate × bit depth × channels
- Base rates: 44100/48000/96000/192000 Hz
- Power management: D0–D3 states

### USB Audio (`src/drivers/usb_audio.rs`)
- USB Audio Class 1.0 and 2.0
- Isochronous transfer management
- Volume, mute, sample rate controls
- Hot-plug safe initialization

---

## PCI Bus (`src/drivers/pci_bus.rs`)

- PCIe ECAM (Enhanced Configuration Access Mechanism)
- Bus enumeration: segments 0–255, buses 0–255, devices 0–31, functions 0–7
- BAR mapping (32-bit and 64-bit, I/O and memory)
- MSI/MSI-X interrupt setup
- Power management (ACPI D-states)

---

## USB (`src/drivers/`)

### XHCI Controller
- USB 3.2 SuperSpeed (10 Gbps)
- Slot/endpoint context management
- Transfer rings: Control, Bulk, Interrupt, Isochronous
- TRB (Transfer Request Block) based DMA

### USB HID
- Keyboard, mouse, gamepad, touchscreen
- HID report descriptor parser
- N-key rollover (NKRO) support

---

## Framebuffer / GPU (`src/drivers/framebuffer.rs`)

- VESA/UEFI GOP framebuffer initialization
- Direct pixel write: RGB, RGBA, BGR formats
- Character cell rendering (for early boot console)
- DRM KMS integration for modesetting
- Basic 2D blit acceleration

---

## Bluetooth (`src/bluetooth/`)

- HCI (Host Controller Interface) over USB or UART
- BT 5.3: BR/EDR + BLE dual-mode
- L2CAP, RFCOMM, GATT, HFP, A2DP profiles
- LE Audio (LC3 codec) support

---

## Wireless (`src/wireless/`)

- cfg80211 / mac80211 equivalent in Rust
- Station mode: scan, authenticate, associate
- WPA2/WPA3 via wpa_supplicant-compatible state machine
- Monitor mode for packet capture
- Firmware loading: iwlwifi, ath9k, rtw88 compatible

---

## Power Management

### ACPI (`src/kernel/acpi_pm.rs`)
- ACPI table parsing: MADT, FADT, DSDT, SSDT
- Device power states: D0 (fully on) → D3 (off)
- System sleep: S1 (CPU halt), S3 (suspend-to-RAM), S4 (hibernate)
- Cooling device management: fan, thermal zone
- Battery and AC adapter state via `_BIF`/`_BST`

### Runtime PM
- Autosuspend: devices idle for > 2s suspend automatically
- RPM callbacks: `suspend`, `resume`, `idle` per driver
- Power domain grouping: GPU + display suspend together

---

## Driver Comparison

| Driver Area | Linux | Omarchy | Mint | **SigmaOS** |
|-------------|-------|---------|------|-------------|
| Language | C | C | C | **Rust** |
| Memory-safe | ❌ | ❌ | ❌ | ✅ |
| NVMe 1.4 | ✅ | ✅ | ✅ | ✅ |
| USB Audio 2.0 | ✅ | ✅ | ✅ | ✅ |
| AI power tuning | ❌ | ❌ | ❌ | ✅ |
| Hot-plug safe | ✅ | ✅ | ✅ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/drivers/ahci_sata.rs` | AHCI/SATA storage driver |
| `src/drivers/audio_intel_hda.rs` | Intel HDA audio |
| `src/drivers/usb_audio.rs` | USB Audio Class driver |
| `src/drivers/ethernet.rs` | Ethernet frame driver |
| `src/drivers/framebuffer.rs` | VGA/VESA framebuffer |
| `src/drivers/pci_bus.rs` | PCI/PCIe bus driver |
| `src/hal/` | Hardware abstraction layer |
| `src/kernel/acpi_pm.rs` | ACPI power management |
| `src/bluetooth/` | Bluetooth stack |
| `src/wireless/` | Wi-Fi stack |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/drivers/`, `src/hal/`, `src/bluetooth/`, `src/wireless/`
> - Add new driver entries as new `.rs` files appear in `src/drivers/`
> - Update USB/BT spec versions when support is expanded
> - Keep power management section current with `src/kernel/acpi_pm.rs`

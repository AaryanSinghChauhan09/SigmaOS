# SigmaOS Hardware Support

> This document describes the hardware support status, compatibility matrix, and hardware-specific configuration for SigmaOS.
>
> **Note:** SigmaOS is in v0.1 Alpha. Hardware testing has not yet been performed. This document describes the **target** hardware support for v1.0 and the known state of support for each category.

---

## Table of Contents

1. [Overview](#overview)
2. [CPU Support](#cpu-support)
3. [GPU Support](#gpu-support)
4. [Wi-Fi Support](#wi-fi-support)
5. [Bluetooth Support](#bluetooth-support)
6. [Audio Support](#audio-support)
7. [Storage](#storage)
8. [Display & Multi-Monitor](#display--multi-monitor)
9. [Input Devices](#input-devices)
10. [Suspend & Resume](#suspend--resume)
11. [Laptop-Specific Support](#laptop-specific-support)
12. [Hardware Compatibility List](#hardware-compatibility-list)
13. [Reporting Hardware Issues](#reporting-hardware-issues)

---

## Overview

SigmaOS targets **modern consumer hardware** released from 2020 onward. Older hardware may work but is not a testing priority for v1.0.

### Architecture Support

| Architecture | Status | Notes |
|-------------|--------|-------|
| x86_64 (AMD64) | ✅ Primary | Full support target |
| ARM64 (AArch64) | 🔨 Planned | Phase 8 |
| RISC-V | 🔲 Future | Post-v1.0 |
| 32-bit x86 | ❌ Not supported | End of life |

### Status Key

| Symbol | Meaning |
|--------|---------|
| ✅ Works | Fully functional, tested |
| 🟡 Mostly works | Minor issues, documented workarounds |
| 🟠 Partial | Core function works, some features missing |
| 🔲 Planned | On the roadmap, not yet implemented |
| ❌ Not supported | Out of scope |
| ⚠️ Requires firmware | Works only after installing non-free firmware |

---

## CPU Support

### Intel

| Generation | Architecture | Status | Notes |
|-----------|-------------|--------|-------|
| 14th gen (Raptor Lake Refresh) | Meteor Lake | 🔲 Planned | Requires kernel 6.7+ |
| 13th gen (Raptor Lake) | Raptor Lake | 🔲 Planned | Good upstream support |
| 12th gen (Alder Lake) | Alder Lake | 🔲 Planned | P-core/E-core scheduling |
| 11th gen (Tiger Lake) | Tiger Lake | 🔲 Planned | Stable support |
| 10th gen (Ice Lake, Comet Lake) | Ice Lake | 🔲 Planned | Stable |
| 9th gen and older | Skylake and older | 🔲 Planned | May work, not tested |

#### Intel-Specific Features

| Feature | Status |
|---------|--------|
| Hyper-Threading | Enabled by default |
| Intel Speed Step (P-states) | ✅ Via kernel cpufreq |
| Intel TurboBoost | ✅ Enabled by default |
| Intel P-core/E-core scheduling | 🔲 Planned (itd scheduler) |
| Intel vPro / AMT | ❌ Not supported |

### AMD

| Series | Architecture | Status | Notes |
|--------|-------------|--------|-------|
| Ryzen 7000 (Zen 4) | Zen 4 | 🔲 Planned | Requires kernel 6.1+ |
| Ryzen 6000 (Zen 3+) | Zen 3+ | 🔲 Planned | Stable upstream |
| Ryzen 5000 (Zen 3) | Zen 3 | 🔲 Planned | Excellent support |
| Ryzen 4000 (Zen 2) | Zen 2 | 🔲 Planned | Stable |
| Ryzen 3000 and older | Zen/Zen+ | 🔲 Planned | May work |

#### AMD-Specific Features

| Feature | Status |
|---------|--------|
| AMD Cool'n'Quiet | ✅ Via p-state driver |
| AMD Precision Boost | ✅ Via kernel |
| AMD fTPM | 🔲 Planned (for TPM2 support) |
| AMD Ryzen AI (NPU) | 🔲 Future |

---

## GPU Support

GPU support is one of the most complex areas of Linux hardware support. SigmaOS's approach:

- **AMD:** Open source `amdgpu` driver — first-class support
- **Intel:** Open source `i915`/`xe` driver — first-class support
- **NVIDIA:** Proprietary driver via sigma-nonfree — opt-in only

### Intel Graphics

| GPU Family | Driver | Status | Notes |
|-----------|--------|--------|-------|
| Intel Arc (A-series) | xe | 🔲 Planned | kernel 6.5+, DG2/Alchemist |
| Intel Iris Xe (12th-14th gen) | i915/xe | 🔲 Planned | GuC/HuC firmware required |
| Intel UHD (10th-11th gen) | i915 | 🔲 Planned | Stable |
| Intel HD (6th-9th gen) | i915 | 🔲 Planned | Stable |

#### Intel GPU Firmware

Intel integrated graphics require GuC and HuC firmware from `linux-firmware`. SigmaOS includes these by default.

```bash
# Verify GuC/HuC loaded:
sudo sigma-ctl gpu intel status
```

### AMD Graphics

| GPU Family | Driver | Status | Notes |
|-----------|--------|--------|-------|
| Radeon RX 7000 (RDNA 3) | amdgpu | 🔲 Planned | kernel 6.1+, firmware required |
| Radeon RX 6000 (RDNA 2) | amdgpu | 🔲 Planned | Excellent support |
| Radeon RX 5000 (RDNA 1) | amdgpu | 🔲 Planned | Stable |
| AMD Vega | amdgpu | 🔲 Planned | Stable |
| AMD Polaris | amdgpu | 🔲 Planned | Stable |
| AMD GCN (older) | amdgpu | 🔲 Planned | May have issues |
| AMD APU integrated (Ryzen) | amdgpu | 🔲 Planned | Stable, Vulkan supported |

#### PRIME GPU Switching (AMD + Intel)

Laptops with both AMD integrated and AMD/Intel discrete GPUs:

| Configuration | Status |
|--------------|--------|
| Intel iGPU + NVIDIA dGPU | 🔲 Planned (via PRIME) |
| Intel iGPU + AMD dGPU | 🔲 Planned (via DRI_PRIME) |
| AMD iGPU + AMD dGPU | 🔲 Planned (via DRI_PRIME) |

### NVIDIA Graphics

NVIDIA support requires the proprietary driver, available from `sigma-nonfree`:

| Driver | Status | Notes |
|--------|--------|-------|
| NVIDIA 545+ (open kernel module) | ⚠️ sigma-nonfree | Requires opt-in |
| NVIDIA 535 LTS | ⚠️ sigma-nonfree | Older, stable branch |
| Nouveau (open source) | 🟠 Partial | Basic display only, no acceleration |

**Important:** NVIDIA is not installed by default. The installer will offer it if NVIDIA hardware is detected.

### Vulkan Support

| GPU | Vulkan Status |
|-----|--------------|
| Intel (12th gen+) | 🔲 Planned (via ANV driver) |
| AMD | 🔲 Planned (via RADV driver) |
| NVIDIA (proprietary) | ⚠️ sigma-nonfree |

---

## Wi-Fi Support

### Intel Wi-Fi

| Chipset | Driver | Firmware | Status |
|---------|--------|----------|--------|
| Intel Wi-Fi 7 (BE200) | iwlwifi | intel-ucode | 🔲 Planned |
| Intel AX211 (Wi-Fi 6E) | iwlwifi | iwlwifi-ty | 🔲 Planned |
| Intel AX210 (Wi-Fi 6E) | iwlwifi | iwlwifi-ty | 🔲 Planned |
| Intel AX200 (Wi-Fi 6) | iwlwifi | iwlwifi-cc | 🔲 Planned |
| Intel 9260 (Wi-Fi 5) | iwlwifi | iwlwifi-9260 | 🔲 Planned |
| Intel 8265/8260 | iwlwifi | iwlwifi-8265 | 🔲 Planned |

Intel Wi-Fi firmware is included in SigmaOS by default (from `linux-firmware`).

### MediaTek Wi-Fi

| Chipset | Driver | Status |
|---------|--------|--------|
| MT7922 (Wi-Fi 6E) | mt7921e | 🔲 Planned |
| MT7921 (Wi-Fi 6) | mt7921e | 🔲 Planned |
| MT7663 | mt7663e | 🔲 Planned |

### Realtek Wi-Fi

| Chipset | Driver | Status | Notes |
|---------|--------|--------|-------|
| RTL8852AE | rtw89 | 🔲 Planned | Mainline kernel 5.16+ |
| RTL8852CE | rtw89 | 🔲 Planned | kernel 6.2+ |
| RTL8821CE | rtw88 | 🔲 Planned | Some firmware issues on older kernels |

### Broadcom Wi-Fi

| Chipset | Driver | Status | Notes |
|---------|--------|--------|-------|
| BCM4360, BCM4352 | wl (proprietary) | ⚠️ sigma-nonfree | Broadcom closed-source driver |
| BCM43602 | brcmfmac | 🟡 Mostly works | Open source, occasional issues |

Broadcom proprietary Wi-Fi drivers are available via `sigma-nonfree` only.

---

## Bluetooth Support

SigmaOS uses BlueZ for Bluetooth support.

| Version | Status | Notes |
|---------|--------|-------|
| Bluetooth 5.3 | 🔲 Planned | |
| Bluetooth 5.2 (LE Audio) | 🔲 Planned | |
| Bluetooth 5.0 | 🔲 Planned | |
| Bluetooth 4.x | 🔲 Planned | |

### Bluetooth Use Cases

| Use Case | Status |
|----------|--------|
| Audio (A2DP) | 🔲 Planned (via PipeWire) |
| Headset profile (HFP/HSP) | 🔲 Planned (via PipeWire) |
| Keyboard/Mouse | 🔲 Planned |
| File transfer (OBEX) | 🔲 Planned |
| Tethering | 🔲 Planned |

**Note:** Bluetooth is disabled by default. Enable in sigma-settings.

---

## Audio Support

SigmaOS uses **PipeWire** as the audio server (replacing both PulseAudio and JACK).

### Codec Support

| Codec | Status |
|-------|--------|
| Intel HD Audio (HDA) | 🔲 Planned |
| Intel SOF (Sound Open Firmware) | 🔲 Planned (required for 10th gen+ Intel laptops) |
| AMD audio (HDMI/DP) | 🔲 Planned |
| Realtek ALC series | 🔲 Planned |
| USB Audio (class-compliant) | 🔲 Planned |
| Bluetooth Audio (via PipeWire) | 🔲 Planned |

### Intel SOF

Modern Intel laptops (10th generation and newer) require Sound Open Firmware (SOF). SigmaOS includes SOF firmware. However, ALSA UCM (Use Case Manager) profiles may be missing for some laptop models.

```bash
# Check if SOF is loaded:
sigma-ctl audio sof status

# Apply UCM profile for your laptop model:
sigma-ctl audio ucm apply <model>
```

### Known Audio Issues (Target Resolution)

- Some laptop models require custom ALSA UCM configs — community-contributed configs will be accepted
- Internal microphone may not work on some Realtek codecs without manual configuration

---

## Storage

| Interface | Status | Notes |
|-----------|--------|-------|
| NVMe SSD | 🔲 Planned | Primary target |
| SATA SSD | 🔲 Planned | |
| SATA HDD | 🔲 Planned | |
| eMMC (tablet/budget laptops) | 🔲 Planned | |
| USB Mass Storage | 🔲 Planned | |
| SD Card (via SD card reader) | 🔲 Planned | |
| Thunderbolt NVMe | 🔲 Planned | |

### Filesystem Support

| Filesystem | Status | Default Use |
|-----------|--------|------------|
| btrfs | ✅ Default | Root, home |
| ext4 | 🔲 Planned | Secondary option |
| xfs | 🔲 Planned | Optional |
| FAT32 | 🔲 Planned | EFI partition |
| NTFS | 🔲 Planned | Read/write for dual-boot |
| exFAT | 🔲 Planned | Removable media |

---

## Display & Multi-Monitor

### Display Output

| Interface | Status | Notes |
|-----------|--------|-------|
| HDMI 2.1 | 🔲 Planned | |
| DisplayPort 1.4+ | 🔲 Planned | |
| USB-C / Thunderbolt (DP alt mode) | 🔲 Planned | |
| DVI | 🔲 Planned | Legacy |
| VGA | ❌ Not supported | Adapter required |

### Multi-Monitor

| Feature | Status |
|---------|--------|
| Dual monitor (same GPU) | 🔲 Planned |
| Triple monitor | 🔲 Planned |
| Mixed resolution (HiDPI + standard) | 🔲 Planned |
| Mixed refresh rate | 🔲 Planned |
| Hot-plug (monitor connect/disconnect) | 🔲 Planned |
| Lid close → external monitor only | 🔲 Planned |

### HiDPI / Display Scaling

| Feature | Status |
|---------|--------|
| Fractional scaling (Wayland) | 🔲 Planned |
| Per-monitor scaling (different scales per monitor) | 🔲 Planned |
| HiDPI font rendering | 🔲 Planned |

### Variable Refresh Rate (VRR/FreeSync/G-Sync)

| Feature | Status | Notes |
|---------|--------|-------|
| FreeSync (AMD) | 🔲 Planned | Via KMS VRR |
| G-Sync Compatible (NVIDIA) | ⚠️ sigma-nonfree | |
| VRR on Wayland | 🔲 Planned | Compositor support required |

---

## Input Devices

### Keyboard

| Feature | Status |
|---------|--------|
| Standard keyboard input | 🔲 Planned |
| Fn key remapping | 🔲 Planned |
| Keyboard backlight (Intel) | 🔲 Planned |
| Per-key RGB (if drivers exist) | 🔲 Future |

### Touchpad

| Feature | Status | Notes |
|---------|--------|-------|
| Basic pointer movement | 🔲 Planned | |
| Two-finger scroll | 🔲 Planned | |
| Two-finger click (right-click) | 🔲 Planned | |
| Three-finger gestures | 🔲 Planned | |
| Four-finger gestures | 🔲 Planned | |
| Palm rejection | 🔲 Planned | Via libinput |
| Pressure sensitivity | 🔲 Planned | Wacom and compatible |

### Touchscreen

| Feature | Status |
|---------|--------|
| Touch input | 🔲 Planned |
| Multi-touch | 🔲 Planned |
| Stylus input | 🔲 Planned (Wacom/USI) |
| Screen rotation (2-in-1) | 🔲 Planned |

---

## Suspend & Resume

Suspend/resume is historically one of the most problematic areas on Linux.

### Suspend Modes

| Mode | Status | Notes |
|------|--------|-------|
| Suspend to RAM (S3) | 🔲 Planned | Traditional sleep |
| Suspend to disk (hibernate) | 🔲 Planned | Requires swap partition |
| Modern Standby (S0ix) | 🔲 Planned | Intel/AMD low-power idle |
| Hybrid sleep | 🔲 Planned | RAM + disk combined |

### Known Suspend Issues (General)

- **Wi-Fi disconnect on resume:** Common issue. sigma-net will attempt auto-reconnect on resume.
- **Bluetooth disconnect on resume:** BlueZ handles re-pairing.
- **Audio reset on resume:** PipeWire handles audio device re-initialization.
- **GPU hang on resume:** AMD and Intel generally well-behaved. NVIDIA proprietary can have issues.

### Suspend Testing Target

For v1.0: suspend/resume working on 90% of tested devices (from the HCL).

---

## Laptop-Specific Support

### Dell

| Model Family | Status | Notes |
|-------------|--------|-------|
| XPS 13 (2022+) | 🔲 Target | Intel graphics, good Linux support |
| XPS 15 (2022+) | 🔲 Target | Intel + NVIDIA, PRIME needed |
| Latitude 7000 series | 🔲 Target | Business laptop |
| Inspiron 15 | 🔲 Target | Common consumer laptop |

### Lenovo

| Model Family | Status | Notes |
|-------------|--------|-------|
| ThinkPad X1 Carbon (Gen 10+) | 🔲 Target | Excellent Linux support historically |
| ThinkPad T14 (AMD) | 🔲 Target | AMD Ryzen, good support |
| IdeaPad 5 Pro | 🔲 Target | AMD or Intel |
| Legion (gaming) | 🔲 Target | NVIDIA, requires sigma-nonfree |

### HP

| Model Family | Status | Notes |
|-------------|--------|-------|
| EliteBook 840/850 G9 | 🔲 Target | Business laptop |
| Spectre x360 | 🔲 Target | 2-in-1, touchscreen |
| Envy 15 | 🔲 Target | Intel + NVIDIA |

### ASUS

| Model Family | Status | Notes |
|-------------|--------|-------|
| Zenbook 14 | 🔲 Target | Intel or AMD |
| ROG series | 🔲 Target | NVIDIA, gaming, sigma-nonfree |
| VivoBook 15 | 🔲 Target | Common consumer |

### Framework Laptop

| Model | Status | Notes |
|-------|--------|-------|
| Framework 13 (AMD) | 🔲 Target | Excellent Linux support |
| Framework 16 | 🔲 Target | AMD, modular GPU |

Framework laptops are a priority target due to their strong Linux community and reparability ethos aligning with SigmaOS values.

---

## Hardware Compatibility List

The Hardware Compatibility List (HCL) will be published alongside the v1.0 release. During alpha/beta, community members can contribute hardware reports.

### How to Submit a Hardware Report

```bash
# Generate hardware report
sigma-ctl hardware report --output /tmp/hardware-report.json

# Review the report (no personally identifiable information is included)
cat /tmp/hardware-report.json

# Submit report (opens browser to GitHub issue form)
sigma-ctl hardware submit-report /tmp/hardware-report.json
```

### Hardware Report Contents

The hardware report includes:
- CPU model and generation
- GPU model(s)
- Wi-Fi chipset
- Audio chipset
- RAM amount
- Storage type (NVMe/SSD/HDD)
- Test results for: boot, Wi-Fi, audio, suspend/resume
- SigmaOS version

The report does **not** include:
- Serial numbers
- MAC addresses
- User name or account information
- Network topology

---

## Reporting Hardware Issues

If SigmaOS does not work correctly on your hardware:

1. **Search existing issues:** `https://github.com/AaryanSinghChauhan09/sigma-hardware-detect/issues`
2. **Generate a report:** `sigma-ctl hardware report`
3. **File a new issue** with:
   - Hardware model (laptop model or motherboard + GPU)
   - The hardware report output
   - Description of what doesn't work
   - What you expected to happen
   - Kernel log: `sudo sigma-ctl journal kernel --last-boot > /tmp/kernel.log`

Hardware issues are triaged weekly by the hardware support team.

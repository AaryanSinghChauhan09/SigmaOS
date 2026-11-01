# SigmaOS Hardware Targets

## Support Tiers

- **Tier 1**: Fully supported and tested. Regressions are release-blocking.
- **Tier 2**: Should work; community-tested. Regressions are high-priority but not blocking.
- **Tier 3**: Best effort. May require manual configuration.
- **Unsupported**: Known to not work. Will not be fixed in v1.

## CPU Support Matrix

| CPU Family | Vendor | Minimum Gen | Tier | Notes |
|---|---|---|---|---|
| Intel Core (laptop) | Intel | 8th gen (Ice Lake) | 1 | Full power management |
| Intel Core (desktop) | Intel | 9th gen (Coffee Lake) | 1 | |
| Intel Core Ultra | Intel | 1st gen (Meteor Lake) | 1 | |
| AMD Ryzen (laptop) | AMD | Zen 2 (Ryzen 4000) | 1 | |
| AMD Ryzen (desktop) | AMD | Zen 2 (Ryzen 3000) | 1 | |
| Intel Atom / Celeron | Intel | Apollo Lake | 2 | Limited power mgmt |
| AMD Athlon | AMD | Zen+ | 2 | |
| Intel Core pre-8th gen | Intel | 6th–7th gen | 3 | No modern standby |
| AMD pre-Zen 2 | AMD | Zen / Zen+ | 3 | |

## GPU Support Matrix

| GPU | Vendor ID | Device ID | Driver | Tier | Vulkan | VA-API |
|---|---|---|---|---|---|---|
| Intel Arc A770 | 8086 | 56A0 | i915 | 1 | Yes | Yes |
| Intel Arc A750 | 8086 | 56A1 | i915 | 1 | Yes | Yes |
| Intel Iris Xe (laptop) | 8086 | 9A49 | i915 | 1 | Yes | Yes |
| Intel UHD 770 | 8086 | 4680 | i915 | 1 | Yes | Yes |
| Intel UHD 620 | 8086 | 3EA0 | i915 | 1 | Partial | Yes |
| AMD Radeon RX 7900 XTX | 1002 | 744C | amdgpu | 1 | Yes | Yes |
| AMD Radeon RX 7700 XT | 1002 | 7480 | amdgpu | 1 | Yes | Yes |
| AMD Radeon RX 6700 XT | 1002 | 73DF | amdgpu | 1 | Yes | Yes |
| AMD Radeon 780M (iGPU) | 1002 | 1900 | amdgpu | 1 | Yes | Yes |
| AMD Radeon RX 580 | 1002 | 67DF | amdgpu | 2 | Yes | Yes |
| NVIDIA RTX 4090 | 10DE | 2684 | nvidia | 2 | Yes (proprietary) | Yes |
| NVIDIA RTX 3070 | 10DE | 2484 | nvidia | 2 | Yes (proprietary) | Yes |
| NVIDIA GTX 1660 Ti | 10DE | 2182 | nvidia | 2 | Yes (proprietary) | No |
| NVIDIA GTX 1050 | 10DE | 1C81 | nvidia | 3 | Yes (proprietary) | No |
| VirtIO GPU | 1AF4 | 1050 | virtio-gpu | 1 | No | No |

**Note**: NVIDIA requires `sigma-nonfree` repository. Run `sigma drivers --install-nvidia`.

## WiFi Support Matrix

| Device | Vendor ID | Device ID | Driver | Tier | Notes |
|---|---|---|---|---|---|
| Intel Wi-Fi 6E AX211 | 8086 | 2725 | iwlwifi | 1 | |
| Intel Wi-Fi 6E AX210 | 8086 | 2725 | iwlwifi | 1 | |
| Intel Wi-Fi 6 AX200 | 8086 | 2723 | iwlwifi | 1 | |
| Intel Wi-Fi 5 9560 | 8086 | 9DF0 | iwlwifi | 1 | |
| MediaTek MT7922 | 14C3 | 7922 | mt7921e | 1 | |
| MediaTek MT7921 | 14C3 | 7961 | mt7921e | 1 | |
| Realtek RTW8852AE | 10EC | 8852 | rtw89 | 2 | Firmware required |
| Realtek RTL8821CE | 10EC | C821 | rtl8821ce | 2 | DKMS module |
| Broadcom BCM4360 | 14E4 | 43A0 | wl (proprietary) | 3 | Requires nonfree |
| Broadcom BCM4313 | 14E4 | 4727 | brcmsmac | Unsupported | Pre-2012 |

## Bluetooth Support Matrix

| Standard | Tier | Notes |
|---|---|---|
| Bluetooth 5.3 | 1 | Full support |
| Bluetooth 5.0/5.1/5.2 | 1 | Full support |
| Bluetooth 4.2 | 2 | A2DP, HFP work |
| Bluetooth 4.0 LE only | 2 | LE only, no classic audio |
| Pre-BT 4.0 | 3 | Basic functionality |

## Audio Support Matrix

| Device Type | Driver | Tier | Notes |
|---|---|---|---|
| Intel HDA (14th gen) | snd_hda_intel | 1 | SOF firmware |
| Intel HDA (8th–13th gen) | snd_hda_intel | 1 | |
| AMD HDA (Ryzen) | snd_hda_intel | 1 | |
| USB Audio Class 2.0 | snd_usb_audio | 1 | |
| USB Audio Class 1.0 | snd_usb_audio | 1 | |
| HDMI/DP Audio (Intel) | snd_hda_intel | 1 | Via GPU driver |
| HDMI/DP Audio (AMD) | snd_hda_intel | 1 | Via GPU driver |
| HDMI/DP Audio (NVIDIA) | snd_hda_intel | 2 | Requires proprietary |
| Bluetooth Audio | snd_soc | 1 | Via PipeWire |

Audio stack: PipeWire → WirePlumber → ALSA. PulseAudio compatibility via pipewire-pulse.

## Storage Support Matrix

| Type | Interface | Tier | Notes |
|---|---|---|---|
| NVMe SSD | PCIe 4.0/5.0 | 1 | Full speed |
| NVMe SSD | PCIe 3.0 | 1 | |
| SATA SSD | SATA III | 1 | |
| SATA HDD | SATA III | 1 | Slower install |
| eMMC | JEDEC 5.1 | 2 | Some tablets |
| SD Card | UHS-I/II | 2 | Via kernel sdmmc |
| USB 3.x Flash | USB-A/C | 1 | Live boot + install media |

## Known Unsupported Hardware

| Hardware | Reason |
|---|---|
| NVIDIA Optimus without prime-select | Cannot switch GPU cleanly in v1 |
| Broadcom BCM4312 and older | No open driver; proprietary driver EOL |
| Mediatek Helio SoCs (ARM) | ARM not supported in v1 |
| Apple Silicon M-series | ARM64, different firmware model |
| Qualcomm Snapdragon (laptop) | Upstream driver immature |
| Realtek 8188EE (USB) | Poor power management, frequent disconnects |

## Adding New Hardware

If your hardware is not listed:
1. Run `sigma hardware --report` to generate a hardware profile
2. File an issue at https://github.com/AaryanSinghChauhan09/SigmaOS-hardware
3. Attach the report output
4. The compatibility team will assess and update this matrix

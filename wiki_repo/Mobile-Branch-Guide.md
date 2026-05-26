# 📱 SigmaOS Mobile Branch — `release/mobile`

> **Sovereign Silicon in Your Pocket: No Android, No iOS, No Compromise.**

The `release/mobile` branch targets **ARM-based mobile hardware**, absorbing concepts from:
- **PostmarketOS** (mainline Linux for phones)
- **PinePhone / Mobian** (open hardware mobile Linux)
- **Sailfish OS** (independent Linux-based mobile OS)
- **Ubuntu Touch** (convergence desktop/mobile)
- **Android AOSP HAL** (hardware abstraction layer concepts)

---

## 🏗 Target Hardware

| Device Class | Examples | Status |
|-------------|----------|--------|
| ARM64 SBC | Raspberry Pi 4/5, PinePhone | ✅ Primary target |
| Qualcomm SDM | Snapdragon 845/865 phones | 🔧 Driver work |
| MediaTek Helio | Budget ARM phones | 🔧 Planned |
| Apple Silicon | M1/M2 (AArch64) | 🔬 Research |

---

## 🔧 ARM64 Boot Sequence

Absorbed from **U-Boot** and **PostmarketOS** boot chain concepts:

```
SPI Flash / eMMC
      ↓
sigma_aarch64_start.S  (Exception Level 2 → EL1)
      ↓
sigma_mmu_arm.cpp      (MMU page tables, 4KB granule)
      ↓
sigma_devicetree.cpp   (Device Tree Blob parser)
      ↓
sigma_kernel_main()    (Sovereign kernel entry)
      ↓
sigma_sh_run()         (Shell or GUI session)
```

---

## 📱 Mobile-Specific Drivers

| Driver | Source | Absorbs From |
|--------|--------|-------------|
| Touch screen | `sigma_touchscreen.cpp` | PostmarketOS libinput, Android InputFlinger |
| MIPI DSI display | `sigma_mipi_dsi.cpp` | Qualcomm DPU driver, DRM/KMS |
| Camera ISP | `sigma_camera_isp.cpp` | V4L2 concepts, libcamera |
| Cellular modem | `sigma_modem_qmi.cpp` | QMI protocol, ModemManager |
| WiFi (ath10k) | `sigma_ath10k.cpp` | ath10k firmware loading pattern |
| Bluetooth HCI | `sigma_hci.cpp` | BlueZ HCI layer concepts |
| Battery PMIC | `sigma_pmic.cpp` | ACPI/SPMI power management |
| Vibrator motor | `sigma_vibrator.cpp` | Android HAL vibrator concept |
| GPS NMEA | `sigma_gps_nmea.cpp` | NMEA 0183 sentence parser |

---

## 🖥 Mobile GUI Stack

Absorbed from **Weston** (Wayland), **Phosh** (GTK phone shell), **Plasma Mobile**:

```
┌──────────────────────────────────────┐
│  Zenith Mobile Shell (sigma_zenith)  │
│  ┌────────────┐  ┌────────────────┐  │
│  │  App Grid  │  │  Notification  │  │
│  │  Launcher  │  │     Panel      │  │
│  └────────────┘  └────────────────┘  │
├──────────────────────────────────────┤
│  sigma_compositor.cpp (frame buffer) │
│  sigma_touch_router.cpp (gesture)    │
├──────────────────────────────────────┤
│  sigma_mipi_dsi.cpp (display driver) │
│  sigma_touchscreen.cpp (input)       │
└──────────────────────────────────────┘
```

---

## 📶 Telephony Stack

Absorbed from **oFono** (telephony daemon), **QMI** protocol:

```cpp
/* sigma_modem_qmi.cpp */
struct QMIMessage {
    u8  service;       /* WDS=0x01, DMS=0x02, NAS=0x03, WMS=0x05 */
    u8  client_id;
    u16 transaction_id;
    u16 message_id;
    u16 length;
    u8  tlvs[];        /* Type-Length-Value payload */
};
```

Services implemented:
- **WDS** (Wireless Data Service) — LTE data connection
- **DMS** (Device Management Service) — IMEI, firmware version
- **NAS** (Network Access Service) — signal strength, network registration
- **WMS** (Wireless Messaging Service) — SMS send/receive

---

## 🔋 Power Management

Absorbed from **Linux PM** subsystem and **ACPI** mobile profiles:

| State | Trigger | Behavior |
|-------|---------|---------|
| S0 (Active) | User interaction | All cores at full frequency |
| S0ix (Connected Standby) | 5s idle | Non-essential cores off, network active |
| S3 (Suspend to RAM) | 30s idle | CPU halted, DRAM self-refresh |
| S4 (Hibernate) | Battery < 5% | RAM→eMMC snapshot, full power-off |

---

*Branch: `release/mobile` | Architecture: AArch64 (ARM64)*

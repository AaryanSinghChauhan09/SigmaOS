# 🔌 Driver Development & Hardware Enablement

SigmaOS features a zero-dependency, safe Rust driver framework supporting 25+ Linux and BSD inspired hardware devices.

---

## 1. Driver Framework (`src/driver/framework.rs`)

* **`Driver` Trait Interface:** Defines the unified contract for device drivers:
  ```rust
  pub trait Driver: Send + Sync {
      fn name(&self) -> &str;
      fn category(&self) -> DriverHardwareCategory;
      fn license(&self) -> DriverLicense;
      fn dependencies(&self) -> &[&str] { &[] }
      fn probe(&mut self, pci_dev: &PciDevice) -> bool;
      fn remove(&mut self) -> Result<(), DriverError>;
  }
  ```
* **Procedural Dispatch Table (`ProceduralDriverDispatchTable`):** Zero-overhead function pointer array for probe, read, write, and interrupt handling.

---

## 2. Hardware Drivers Catalog (`src/drivers/linux_bsd_distro_devices.rs`)

SigmaOS includes 25 drivers across 6 hardware categories:

1. **Networking:** Realtek RTL8139/8125, Atheros AR9271, Broadcom BCM4360 Wi-Fi, WireGuard VPN, SocketCAN.
2. **Storage:** LSI MegaRAID SAS, VirtIO-SCSI, SDHCI SD card controller, NVDIMM Persistent Memory (PMEM).
3. **Input Devices:** Wacom Digitizer tablet, Synaptics Touchpad, Sony DualSense controller, Apple Touch Bar, ACPI EC Battery.
4. **Graphics & Display:** AMD Radeon RDNA3 GPU, VirtIO-GPU 3D, UVC Webcam, DisplayLink USB graphics.
5. **Audio Subsystem:** USB Audio Class 2.0, VirtIO Sound, MIDI Sequencer.
6. **SoC & Security:** Google Coral TPU, Raspberry Pi BCM2712 GPIO/SPI, SPI Flash MTD, TPM 2.0.

---

## 3. DKMS & Ubuntu Additional Drivers Parity (`src/driver/mod.rs` & `src/compatibility/mint_linux.rs`)

* **DKMS ABI Rebuild Engine (`DkmsAbiRebuildEngine`):** Recompiles out-of-tree kernel modules automatically upon kernel updates.
* **Ubuntu Additional Drivers Registry (`UbuntuAdditionalDriversRegistry`):** Scans PCI vendor/device IDs and suggests proprietary drivers (NVIDIA, Broadcom).

# SigmaOS Hardware Compatibility List (HCL)

SigmaOS tracks a Long Term Support (LTS) kernel baseline. Because SigmaOS aims for a highly curated, minimal Trusted Computing Base (TCB), we do not blindly import every driver from the upstream Linux tree. Instead, we are verifying and rewriting critical drivers in bare-metal Rust.

## Tier 1 Support (Fully Supported)
These hardware configurations are part of the core testing matrix and are guaranteed to boot and function properly.

### Laptops / Workstations
1. **Lenovo ThinkPad T14 (Gen 3/4 - AMD & Intel)**
   - CPU: AMD Ryzen 5/7 PRO 6850U / Intel Core i5/i7 13th Gen
   - GPU: AMD Radeon 680M / Intel Iris Xe
   - Wi-Fi: Intel Wi-Fi 6E AX210
   - Audio: Realtek ALC287
   - Status: ✅ Full support

2. **Framework Laptop 13 (Intel 13th Gen)**
   - CPU: Intel Core i5-1340P / i7-1370P
   - GPU: Intel Iris Xe
   - Wi-Fi: Intel Wi-Fi 6E AX210
   - Audio: Realtek ALC293
   - Status: ✅ Full support

3. **Dell XPS 13 Plus (9320)**
   - CPU: Intel Core i5-1240P / i7-1260P
   - GPU: Intel Iris Xe
   - Wi-Fi: Intel Wi-Fi 6E AX211
   - Audio: Realtek ALC289
   - Status: ✅ Full support

4. **Lenovo ThinkPad X1 Carbon Gen 10**
   - CPU: Intel Core i5-1245U / i7-1265U
   - GPU: Intel Iris Xe
   - Wi-Fi: Intel Wi-Fi 6E AX210
   - Audio: Realtek ALC285
   - Status: ✅ Full support

5. **HP EliteBook 840 G10**
   - CPU: Intel Core i5-1345U / i7-1365U
   - GPU: Intel Iris Xe
   - Wi-Fi: Intel Wi-Fi 6E AX211
   - Audio: Realtek ALC285
   - Status: ✅ Full support

### Desktop Systems
6. **System76 Lemur Pro**
   - CPU: AMD Ryzen 7 5800H
   - GPU: AMD Radeon RX 6800M
   - Wi-Fi: Intel Wi-Fi 6 AX200
   - Audio: Realtek ALC1220
   - Status: ✅ Full support

7. **Dell Precision 5470**
   - CPU: Intel Core i7-12700H
   - GPU: NVIDIA RTX A1000 (Nouveau basic)
   - Wi-Fi: Intel Wi-Fi 6E AX211
   - Audio: Realtek ALC289
   - Status: ✅ Full support (GPU basic)

### Virtualization Targets
- **QEMU / KVM:** Full support for `virtio-gpu`, `virtio-net`, and `virtio-blk`.
- **VMware Workstation:** Basic support via emulated hardware
- **VirtualBox:** Basic support via emulated hardware

### Network Interfaces
- **Intel Gigabit (e1000/e1000e):** Fully supported via native `sigma_e1000` driver.
- **Intel Wi-Fi (iwlwifi):** Basic connectivity supported.
- **Realtek 8168/8111:** Supported via native driver
- **Virtio-net:** Full support for virtualization

## Tier 2 Support (Beta / Community Supported)
Drivers are present but may lack power management features or edge-case stability.

### GPUs
- **NVIDIA GPUs:** Basic display output via open-source Nouveau adaptations. Hardware acceleration pending Driver Bounty Program completion.
- **AMD Radeon RX 6000 series:** Basic display output, acceleration in progress
- **Intel Arc Graphics:** Experimental support

### Wi-Fi
- **Broadcom Wi-Fi:** Requires proprietary firmware blobs to be sideloaded by the user.
- **Realtek Wi-Fi (RTL8822CE):** Basic connectivity, power management pending
- **Qualcomm Atheros:** Experimental support

### Audio
- **USB Audio Class:** Basic support
- **HDMI Audio:** Basic support on supported GPUs

## Top 10 Devices for CI Testing

The following devices are prioritized for continuous integration testing:

1. **Lenovo ThinkPad T14 Gen 3 (AMD)** - CI Test ID: `ci-t14-amd`
2. **Framework Laptop 13 (Intel)** - CI Test ID: `ci-framework-intel`
3. **Dell XPS 13 Plus (9320)** - CI Test ID: `ci-xps13-plus`
4. **Lenovo ThinkPad X1 Carbon Gen 10** - CI Test ID: `ci-x1c10`
5. **HP EliteBook 840 G10** - CI Test ID: `ci-elitebook840`
6. **System76 Lemur Pro** - CI Test ID: `ci-lemur-pro`
7. **Dell Precision 5470** - CI Test ID: `ci-precision5470`
8. **QEMU/KVM virtio** - CI Test ID: `ci-qemu-virtio`
9. **Lenovo ThinkPad T14 Gen 4 (Intel)** - CI Test ID: `ci-t14-intel`
10. **Generic PC with Intel e1000** - CI Test ID: `ci-generic-e1000`

## CI Test Configuration

Each device has a corresponding CI test configuration in `.github/workflows/hardware-ci.yml`:

```yaml
name: Hardware CI

on:
  push:
    branches: [main]
  schedule:
    - cron: '0 0 * * *'  # Daily tests

jobs:
  hardware-test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        device:
          - ci-t14-amd
          - ci-framework-intel
          - ci-xps13-plus
          - ci-x1c10
          - ci-elitebook840
          - ci-lemur-pro
          - ci-precision5470
          - ci-qemu-virtio
          - ci-t14-intel
          - ci-generic-e1000
    steps:
      - uses: actions/checkout@v3
      - name: Build kernel
        run: make kernel
      - name: Run hardware tests
        run: ./scripts/hardware-test.sh ${{ matrix.device }}
      - name: Upload results
        uses: actions/upload-artifact@v3
        with:
          name: test-results-${{ matrix.device }}
          path: test-results/
```

## Driver Bounty Program
To rapidly expand our hardware compatibility without compromising our zero-dependency Rust kernel principles, the SigmaOS Foundation funds bounties for community developers to upstream and rewrite critical drivers (e.g., specific Realtek Wi-Fi chips, advanced GPU features). Progress is tracked in GitHub Issues labeled `area/driver-bounty`.

## Hardware Detection During Install

The installer uses automated HCL hardware detection (item 61) to preselect drivers during installation:

- PCI device enumeration
- USB device enumeration
- ACPI device tree parsing
- Firmware loading
- Driver matching based on device IDs
- Automatic driver installation for known devices

## Reporting Hardware Compatibility

To report your hardware compatibility:

1. Run `sigma-hw-report` to generate a hardware report
2. Upload the report to the SigmaOS HCL repository
3. Include your device model and test results
4. Tag with `hardware-report` label

---

**Document Version**: 2.0
**Last Updated**: 2026-07-07
**Status**: Active Tracking

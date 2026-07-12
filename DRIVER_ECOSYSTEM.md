# DRIVER ECOSYSTEM

SigmaOS ships hardware drivers as sovereign kernel shards — zero binary blobs,
zero kernel-update breakage.

## Network Interface Controller (NIC)

Auto-detects and programs the NIC at boot via PCIe endpoint analysis:

| Hardware | PCIe ID | Mode |
|---|---|---|
| VirtIO-Net (QEMU) | `1AF4:1000` | DMA ring buffer |
| RTL8139 (bare-metal) | `10EC:8139` | BMCR register programming |

```c
nic_init();
nic_probe(0x1AF4, 0x1000); // VirtIO-Net
nic_transmit(payload, length);
```

## WiFi Driver

Kernel-level 802.11 WPA3 stack:

```c
wifi_init();
wifi_scan(channels, SIGMA_WIFI_ALL);
wifi_connect("MySSID", wpa3_psk);
```

## Unknown Hardware Transpiler

`SovereignHWTranspiler` handles unknown PCIe devices:

```rust
let transpiler = SovereignHWTranspiler::new(pcie_id);
transpiler.generate_driver_stub()?;
```

## Roadmap

- [x] VirtIO-Net DMA ring buffer driver

- [x] RTL8139 BMCR driver

- [x] 802.11 WPA3 stub

- [ ] NVIDIA GPU open-firmware driver

- [ ] AMD AMDGPU driver

- [ ] Intel i915 open-spec driver

- [ ] Formal DDK verification harness

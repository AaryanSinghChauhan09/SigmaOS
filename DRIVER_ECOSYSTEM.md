# SigmaOS Driver Ecosystem

SigmaOS ships hardware drivers as sovereign kernel shards — zero binary blobs, zero kernel-update breakage.

## Network Drivers

### SovereignNICDriver

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

### SovereignWiFi

Kernel-level 802.11 WPA3 stack:

```c

wifi_init();
wifi_scan();
wifi_connect("MyNetwork", "passphrase", 2); // WPA3-SAE

```

### SovereignBluetooth

Ring-0 HCI driver with PQC pairing attestation:

```c

bt_init();
bt_probe(0x0BDA, 0xB00A); // Realtek USB BT
bt_pair("AA:BB:CC:DD:EE:FF", "Sovereign Keyboard");

```

## Storage Drivers

### SovereignStorageDriver

Block storage with VirtIO-Blk + ATA PIO fallback:

```c

storage_init();
storage_probe(0x1AF4, 0x1001); // VirtIO-Blk
storage_read(0, 512, buffer);  // Read sector 0 (MBR)

```

## Hardware Auto-Detection

`SovereignHWTranspiler` handles unknown PCIe devices:

```c

hw_transpiler_init();
hw_transpiler_profile(vendor_id, device_id);
// -> Sovereign driver shim generated automatically

```



# SigmaOS Hardware Compatibility List (HCL)

SigmaOS tracks a Long Term Support (LTS) kernel baseline. Because SigmaOS aims for a highly curated, minimal Trusted Computing Base (TCB), we do not blindly import every driver from the upstream Linux tree. Instead, we are verifying and rewriting critical drivers in bare-metal Rust.

## Tier 1 Support (Fully Supported)
These hardware configurations are part of the core testing matrix and are guaranteed to boot and function properly.

### Laptops / Workstations
1. **Lenovo ThinkPad T14 (Gen 3/4 - AMD & Intel)**
2. **Framework Laptop 13 (Intel 13th Gen)**
3. **Dell XPS 13 Plus (9320)**

### Virtualization Targets
- **QEMU / KVM:** Full support for `virtio-gpu`, `virtio-net`, and `virtio-blk`.

### Network Interfaces
- **Intel Gigabit (e1000/e1000e):** Fully supported via native `sigma_e1000` driver.
- **Intel Wi-Fi (iwlwifi):** Basic connectivity supported.

## Tier 2 Support (Beta / Community Supported)
Drivers are present but may lack power management features or edge-case stability.

- **NVIDIA GPUs:** Basic display output via open-source Nouveau adaptations. Hardware acceleration pending Driver Bounty Program completion.
- **Broadcom Wi-Fi:** Requires proprietary firmware blobs to be sideloaded by the user.

## Driver Bounty Program
To rapidly expand our hardware compatibility without compromising our zero-dependency Rust kernel principles, the SigmaOS Foundation funds bounties for community developers to upstream and rewrite critical drivers (e.g., specific Realtek Wi-Fi chips, advanced GPU features). Progress is tracked in GitHub Issues labeled `area/driver-bounty`.

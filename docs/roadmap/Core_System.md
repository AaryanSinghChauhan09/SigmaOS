# Core System Roadmap

## Kernel Strategy
SigmaOS maintains a highly curated, bare-metal Rust core. We track a Long Term Support (LTS) kernel baseline for compatibility, but aggressively replace legacy C subsystems with memory-safe Rust alternatives. 

## Hardware Compatibility List (HCL)
We prioritize a focused matrix of hardware over blind upstream driver merging. See the main `HCL.md` for current Tier 1 and Tier 2 supported devices.

## Driver Roadmap
- **Phase 1:** VirtIO ecosystem (net, blk, gpu) for VM development environments.
- **Phase 2:** Intel e1000/e1000e bare-metal networking.
- **Phase 3:** Intel and AMD integrated graphics support via native KMS implementations.
- **Ongoing:** The **Driver Bounty Program** funds contributors to upstream and rewrite critical drivers (e.g., Broadcom Wi-Fi, Realtek audio) in Rust.

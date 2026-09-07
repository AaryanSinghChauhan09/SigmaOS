# SigmaOS Living Wiki — Future Course of Action & Multi-Year Strategic Roadmap

This living knowledgebase document outlines the future strategic vision and phased development milestones for SigmaOS.

---

## 🚀 3-Phase Course of Action

### 1. Short-Term (Next 12–18 Months): Foundation & Compatibility
- **App Compatibility Expansion**: Refine compatibility layers for Linux ELF binaries, Wine/Proton, and macOS Rosetta translations.
- **Zenith Desktop Shell**: Complete Wayland layer-shell protocol integration, fractional HiDPI scaling, and accessibility features.
- **Automated CI/CD & Security Gates**: Maintain 100% test coverage across GitHub Actions workflows.

### 2. Mid-Term (2–4 Years): Core Shards & Hardware Independence
- **Shards Ecosystem Marketplace**: Launch declarative manifest app layer store (`ShardsMarketplaceEngine`).
- **Firmware-Free Drivers**: Replace opaque vendor binary blobs with native Rust drivers.
- **Composable Boot Sequences**: Implement UKI PE/COFF execution and cryptographic boot verification (`CryptographicBootChainEngine`).

### 3. Long-Term (5+ Years): Sovereign Computing Era
- **Cryptographic Boot & PQC Hardware Trust**: End-to-end post-quantum cryptographic validation from hardware reset vector to desktop.
- **Clustered Device Pooling**: Share GPUs, storage, and sensors across local mesh nodes (`ClusteredDevicePoolEngine`).
- **Network-Native OS State**: Pause sessions on desktop and resume seamlessly on mobile/laptop nodes (`NetworkNativeSessionEngine`).
- **Temporal Filesystem**: Native time-travel rollbacks for kernel, packages, and user files (`TemporalFilesystemEngine`).

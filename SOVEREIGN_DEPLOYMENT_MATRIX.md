# 🌐 SigmaOS Sovereign Deployment Matrix

SigmaOS is a **Universal Silicon Lattice** designed for absolute portability across diverse execution environments. By utilizing the **Universal Abstraction Layer (UAL)**, the system adapts its hardware-facing logic while maintaining a consistent, high-performance kernel.

---

## 🏗️ 1. Deployment Formats

| Format | Target Environment | Key Shards | Build Target |
|--------|-------------------|------------|--------------|
| **Bare-Metal** | Physical ARM/RISC-V/x86 | HAL, VMM, Scheduler | `x86_64-unknown-none` |
| **Browser (WASM)** | Modern Web Browsers | WASM-Bridge, Zenith UI | `wasm32-unknown-unknown` |
| **Virtualized** | QEMU / KVM / Proxmox | VirtIO, IPC Recovery | `x86_64` (Standard) |
| **Embedded (IoT)** | Microcontrollers / SBCs | HAL-Minimal, Lua Bridge | `arm-none-eabi` |

---

## 🍱 2. Build Profiles

Profiles allow you to tune the OS footprint for your specific use case. Toggle them via `./s-cli profile <name>`.

- **`server`**: Optimized for resilience. Enables **IPC Persistence Recovery** and **Multi-Core Scheduling**.
- **`iot`**: Optimized for footprint. Strips everything except the **HAL** and **Minimal IPC**.
- **`dev`**: Optimized for velocity. Enables **Native Loader**, **Lua Bridge**, and **Kernel Debugging**.
- **`browser`**: Optimized for simulation. Pairs the kernel-native logic with the **Zenith UI Dashboard**.

---

## 🚀 3. Cross-Platform Workflow

### Native Toolchain
All deployments are managed via the native **S-CLI**:
```bash
# 1. Select deployment profile
./s-cli profile server

# 2. Build for target architecture
./s-cli build aarch64

# 3. Verify in QEMU
./s-cli run aarch64
```

### Browser Deployment
For browser-based simulation, the core kernel shards are cross-compiled to WASM and served alongside the `web_ui`:
```bash
cd web_ui
npm run dev # Launches the Zenith Dashboard with WASM-bridged kernel logic
```

---

## 🛠️ 4. Troubleshooting

- **Linker Issues**: Ensure `-nostdlib` and `-ffreestanding` are used for bare-metal builds.
- **WASM Incompatibility**: Avoid using any MMIO or Assembly shards when targeting `wasm32`. Use the **UAL** to switch to the WASM-Bridge HAL.
- **VM Driver Gaps**: If running in QEMU, ensure the `VirtIO` drivers are enabled in your profile manifest.

---

*Questions? Email [aaryansinghchauhan090305@gmail.com](mailto:aaryansinghchauhan090305@gmail.com)*

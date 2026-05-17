# Σ SigmaOS v15.0 Zenith: Core Edition

## 🧩 The Minimalist Lattice

The **Core Edition** is the stripped-down, high-performance base of the Zenith architecture. It contains only the Sovereign Kernel, essential HAL drivers, and the `S-SHELL` terminal shard. Perfect for servers, IoT, and custom shard development.

### 🛠️ Key Features

* **Headless Architecture**: Optimized for SSH and serial console interaction.

* **Development-Native**: Pre-loaded with the `S-SDK` and `sigma-pkg` toolchain.

* **Resource Efficiency**: Idles at less than 128MB of RAM.

* **PQC-Hardened SSH**: Exclusive Dilithium-5 authenticated remote access.

### 📥 Installation Guide (Developer/Server)

1. **Prepare Media**: Flash `SigmaOS-v15.0-Zenith-Core.iso`.

2. **Connectivity**: Connect via Ethernet or Serial (COM1).

3. **Ignition**: Boot and select "Minimal Shard Seeding".

4. **Configuration**: Use `s-shell` to configure network and PQC keys.

5. **Expansion**: Use `sigma-pkg install <shard>` to add only the functionality you need.

### 💎 Exclusive Functions

* `shard-debug`: Low-level kernel tracing and shard memory inspection.

* `lattice-stat`: High-resolution telemetry for CPU, memory, and packet flow.

---
[Return to Global Home](Home)

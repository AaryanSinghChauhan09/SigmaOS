# OSS Absorption: QEMU, KVM & Firecracker — Virtualization

> **Status**: 🔄 Active | **Source Projects**: QEMU, KVM, Firecracker | **Target Shard**: `SigmaOS Virtualization & Hypervisor Layer`

---

## 1. Executive Summary

Virtualization is critical for both developer workloads and server deployments. SigmaOS absorbs the industry-standard Linux hypervisor stack (KVM) and pairs it with modern, lightweight virtual machine monitors (VMMs) to provide `sigma-vm`.

- **KVM**: The kernel-level hypervisor.
- **QEMU**: For full legacy hardware emulation.
- **Firecracker**: For ultra-lightweight, fast-booting microVMs (serverless/functions).

---

## 2. Key Features Absorbed

### 2.1 MicroVMs via Firecracker (`sigma-microvm`)

Firecracker (built by AWS in Rust) allows booting a Linux/SigmaOS VM in under 125ms with only ~5MB of memory overhead. SigmaOS integrates this for running untrusted code securely.

```rust
// userland/vm/microvm.rs
// SPDX-License-Identifier: MIT

pub struct MicroVmConfig {
    pub vcpu_count: u8,
    pub mem_size_mb: usize,
    pub kernel_path: PathBuf,
    pub rootfs_path: PathBuf,
}

impl MicroVmConfig {
    pub fn boot(&self) -> Result<()> {
        // Interacts with KVM via Firecracker API pattern
        println!("Σ [VM] Booting MicroVM ({} vCPUs, {}MB RAM)", self.vcpu_count, self.mem_size_mb);
        // ... KVM ioctls
        Ok(())
    }
}
```

```bash
$ sigma vm spawn --micro --memory 128M --cmd "python3 untrusted_script.py"
Σ [VM] MicroVM booted in 112ms. Execution complete. VM destroyed.
```

### 2.2 Full Emulation (`sigma-vm`)

For running Windows or older Linux distros, `sigma-vm` utilizes QEMU/KVM under the hood but wraps it in a Sovereign Lattice-compliant declarative configuration.

```bash
$ sigma vm create windows11 --os win11 --ram 8G --disk 64G
Σ [VM] Created virtual machine "windows11"
  Hardware: Q35 chipset, OVMF UEFI, VirtIO disk/net
  TPM 2.0: Emulated (required for Windows 11)

$ sigma vm start windows11
```

---

## 3. Architecture

```
┌────────────────────────────────────────────────────────────────┐
│               SIGMA-VM VIRTUALIZATION STACK                    │
│                                                                │
│  ┌──────────────────────────┐  ┌────────────────────────────┐  │
│  │ Legacy VM (Windows)      │  │ MicroVM (Serverless/App)   │  │
│  │ QEMU Device Model        │  │ Firecracker Device Model   │  │
│  │ (Full hardware emulation)│  │ (VirtIO only, minimal)     │  │
│  └────────────┬─────────────┘  └──────────────┬─────────────┘  │
│               │                               │                │
│  ┌────────────▼───────────────────────────────▼─────────────┐  │
│  │                     /dev/kvm                             │  │
│  │       SIGMA KVM HYPERVISOR (Hardware virtualization)     │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────┘
```

---

## 4. References & Standards

- KVM — `linux-kvm.org` (GPL-2.0)
- QEMU — `qemu.org` (GPL-2.0)
- Firecracker — `firecracker-microvm.github.io` (Apache-2.0)

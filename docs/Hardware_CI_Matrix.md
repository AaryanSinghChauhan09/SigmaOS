# SigmaOS Hardware CI Matrix

> Current: QEMU-only. Target: real hardware-in-the-loop for v16.0 Apex.

---

## Current CI Coverage (v15.0)

| Test Type | Target | Tool | Status |
|---|---|---|---|
| Kernel build (x86_64) | `x86_64-unknown-none` | `make all` | ✅ |
| Kernel build (arm64) | `aarch64-unknown-none` | `make ARCH=arm64 all` | 🔄 |
| Kernel build (riscv64) | `riscv64gc-unknown-none-elf` | `make ARCH=riscv64 all` | ⬜ |
| Unit tests | Host | `cargo test` / `vitest` | ✅ |
| QEMU x86_64 boot | `qemu-system-x86_64` | `make run-qemu` | ⬜ blocked on bootable kernel |
| QEMU ARM64 boot | `qemu-system-aarch64` | `make run-qemu ARCH=arm64` | ⬜ |
| QEMU RISC-V boot | `qemu-system-riscv64` | `make run-qemu ARCH=riscv64` | ⬜ |
| Lint / clippy | Host | `cargo clippy -D warnings` | ✅ |
| SPDX headers | All source files | `make check-spdx` | ⬜ |
| Stub checker | All source files | `make check-stubs` | ✅ |
| sigpkg build | `sigma-pkg build` | CI artefact | ⬜ |
| sigma-pkg verify | Dilithium-5 sig check | `sigma-pkg verify` | ⬜ |

---

## QEMU Test Matrix (target: v0.1 milestone)

Each test must pass before the v0.1 ISO tag is created:

```yaml
# .github/workflows/sigma_qemu.yml — target matrix
strategy:
  matrix:
    include:
      - arch: x86_64
        machine: q35
        cpu: host        # KVM on Linux runners
        memory: 256M
        boot: uefi
      - arch: x86_64
        machine: q35
        cpu: qemu64      # no KVM (GitHub Actions)
        memory: 256M
        boot: uefi
      - arch: aarch64
        machine: virt
        cpu: cortex-a57
        memory: 256M
        boot: uefi       # EDK2 UEFI for ARM
      - arch: riscv64
        machine: virt
        cpu: rv64         # RISC-V CLINT + PLIC
        memory: 256M
        boot: bios        # OpenSBI
```

Each QEMU test:
1. Boots the ISO
2. Waits for `sigma-sh>` prompt on serial
3. Runs: `echo hello && sigma-pkg list && sigma-pkg install sigma-hello && sigma-hello`
4. Asserts: exit 0, output contains "Hello from SigmaOS"
5. Sends: `shutdown now`
6. Asserts: QEMU exits with code 0

---

## Real Hardware CI Matrix (target: v16.0 Apex)

Once the kernel is bootable, add physical hardware runners:

| Hardware | Form Factor | CPU | RAM | Storage | Priority |
|---|---|---|---|---|---|
| Generic x86_64 laptop | Laptop | Intel Core i5-8th gen | 8 GB | NVMe 256 GB | 🔴 High |
| Raspberry Pi 4 (4 GB) | SBC | BCM2711 ARM64 | 4 GB | SD Card | 🔴 High |
| Raspberry Pi 5 (8 GB) | SBC | BCM2712 ARM64 | 8 GB | SD Card | 🟠 Medium |
| AMD Ryzen laptop | Laptop | Ryzen 5 6600U | 16 GB | NVMe | 🟠 Medium |
| NVIDIA GPU desktop | Desktop | Intel + RTX 3060 | 16 GB | NVMe | 🟡 Low (NVIDIA) |
| Old x86_64 laptop | Laptop | Intel Core 2 / i3 | 4 GB | HDD/SSD | 🟡 Low (compat) |
| Cloud VM (ARM64) | VM | AWS Graviton 3 | 2 GB | EBS | 🟠 Medium |
| RISC-V board | SBC | SiFive HiFive Unmatched | 8 GB | SD Card | 🟡 Low |

### Hardware Runner Setup
```bash
# Install GitHub Actions self-hosted runner on physical machine
# Each machine runs: sigma_qemu.yml + sigma_hardware.yml
# sigma_hardware.yml: boots from USB, runs test suite over serial/SSH
```

---

## Driver Smoke Tests (per-driver CI)

When a driver lands, add a QEMU smoke test:

| Driver | QEMU Device Flag | Smoke Test |
|---|---|---|
| e1000 NIC | `-net nic,model=e1000 -net user` | `ping 10.0.2.2` |
| VirtIO-net | `-net nic,model=virtio -net user` | `ping 10.0.2.2` |
| NVMe | `-drive file=test.img,if=none,id=nvm -device nvme,drive=nvm` | `sigma-disks list` shows device |
| VirtIO-blk | `-drive file=test.img,if=virtio` | Block device readable |
| xHCI USB | `-usb -device qemu-xhci -device usb-kbd` | Keystrokes reach sigma-sh |
| VirtIO-GPU | `-device virtio-gpu-pci` | Framebuffer mapped, pixels visible |
| HDA audio | `-soundhw hda` | `/dev/snd` appears |

---

## Performance Benchmarks in CI

Run on every `main` push, track regressions:

| Benchmark | Tool | Target | Regression Threshold |
|---|---|---|---|
| Context switch latency | `sigma-perf context-switch` | <50 ns | +20% = fail |
| Kyber-1024 throughput (AVX-512) | `sigma-perf kyber` | ≥5.8M ops/s | -15% = fail |
| Kernel build time | `make all` elapsed | <120s | +30% = fail |
| ISO size | `du -sh *.iso` | <150 MB (v0.1) | >200 MB = fail |
| QEMU boot to prompt | time to `sigma-sh>` | <5s | >10s = fail |
| sigma-pkg install | time for local install | <1s | >3s = fail |

---

*See also: [ROADMAP.md](../ROADMAP.md) · [docs/Minimal_SigmaOS_v0.1.md](Minimal_SigmaOS_v0.1.md) · `.github/workflows/sigma_qemu.yml`*

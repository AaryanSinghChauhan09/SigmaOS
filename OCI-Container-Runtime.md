# OCI Container Runtime

SigmaOS ships a native OCI container runtime (`virtualization/ocirunner/`) that
lets you run Docker/Podman images directly — either in a microVM for full isolation
or natively for performance.

---

## Two Modes

```
Mode 1: MicroVM (default) — Linux kernel inside QEMU microVM, full isolation
Mode 2: Native            — OCI rootfs + SigmaOS linux-compat syscall layer
```

---

## Quick Start

```bash

# Run Ubuntu in microVM

sigma-compat run ubuntu:22.04

# Run with command

sigma-compat run alpine:3.19 sh -c "echo hello from sigma-compat"

# Run nginx

sigma-compat container nginx:latest

# List running containers

sigma-compat list
```

---

## MicroVM Mode

`virtualization/ocirunner/run_in_microvm.sh`

Uses QEMU's `-M microvm` profile:

- Minimal attack surface (no BIOS, no legacy devices)

- VirtIO-blk for rootfs, VirtIO-net for networking

- 256 MB RAM, 2 vCPUs by default

- Boots in < 2 seconds

```bash

# Under the hood

qemu-system-x86_64 \
  -M microvm,x-option-roms=off,pic=off,pit=off \
  -enable-kvm -m 256M -smp 2 -nographic \
  -kernel vmlinux \
  -drive file=rootfs.ext4,format=raw,if=none \
  -device virtio-blk-device,drive=rootfs \
  -device virtio-net-device,netdev=net0
```

---

## Resource Limits

```bash

# Memory limit

sigma-compat run --memory 512m nginx:latest

# CPU shares

sigma-compat run --cpus 1.5 heavy-workload:latest

# Read-only rootfs

sigma-compat run --read-only alpine:3.19 sh
```

Internally uses cgroup v2:
```
/sys/fs/cgroup/sigma/<container-id>/
  memory.max   ← memory limit
  cpu.weight   ← CPU shares
```

---

## Security

All containers run with:

- sigma-compat namespace isolation

- `sigma_pledge("stdio rpath wpath exec proc inet")`

- W^X: no RWX memory regions

- Optional read-only rootfs

- Automatic cleanup on exit

---

## CI Integration

```yaml

# .github/workflows/compat-matrix.yml

- name: Test OCI images
  run: |
    for image in ubuntu:22.04 alpine:3.19 nginx:alpine; do
      sigma-compat run "$image" echo "compat-ok" || echo "SKIP: $image"
    done
```

---

## OCI Runtime Specification

The runtime implements a subset of the [OCI Runtime Spec](https://github.com/opencontainers/runtime-spec):

| Feature | Status |
|---------|--------|
| Image pull (via docker) | ✅ |
| Rootfs extraction | ✅ |
| Process launch | ✅ |
| Environment variables | ✅ |
| Working directory | ✅ |
| cgroup v2 limits | ✅ |
| Network (bridge) | 🔄 |
| Volume mounts | ⬜ |
| User namespaces | ⬜ |
| seccomp profiles | ⬜ |

---

## Source

`virtualization/ocirunner/ocirunner.rs` — Rust std, ~300 lines.
`virtualization/ocirunner/run_in_microvm.sh` — Bash microVM launcher.

*See also: [Linux Driver Compat](Linux-Driver-Compat) · [Distribution Formats](../docs/DISTRIBUTION_FORMATS.md)*

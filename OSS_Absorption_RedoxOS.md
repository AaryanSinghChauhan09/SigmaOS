# OSS Absorption: RedoxOS — Rust Microkernel Architecture

> **Status**: 🔄 Active | **Source Project**: Redox OS | **Target Shard**: `SigmaOS Microkernel Core`

---

## 1. Executive Summary

Redox is a Unix-like operating system written entirely in Rust, utilizing a microkernel architecture where drivers, filesystems, and network stacks run in userspace. 

SigmaOS absorbs Redox's **scheme-based URL routing** for IPC and its pure-Rust driver architecture, blending it with the high-performance Linux compatibility layer.

---

## 2. Key Features Absorbed

### 2.1 Scheme-based IPC (`sigma-schemes`)

In Redox, everything is a URL scheme (e.g., `file:`, `tcp:`, `display:`). SigmaOS absorbs this into its IPC capability model:

```rust
// kernel/ipc/scheme.rs
// SPDX-License-Identifier: MIT

pub trait Scheme {
    fn open(&self, path: &str, flags: usize, uid: u32, gid: u32) -> Result<usize>;
    fn read(&self, id: usize, buf: &mut [u8]) -> Result<usize>;
    fn write(&self, id: usize, buf: &[u8]) -> Result<usize>;
    fn close(&self, id: usize) -> Result<()>;
}
```

```bash
# Interacting with a scheme
$ cat display:1
# (Outputs current framebuffer metadata)

$ echo "hello" > tcp:192.168.1.1:80
```

### 2.2 Userspace Drivers

SigmaOS runs all non-essential drivers (audio, USB, networking) in userspace, supervised by `sigma-init`, utilizing Rust's memory safety to prevent system panics.

```bash
$ sigma driver list
Σ [DRIVERS] Userspace Driver Instances:
  sigma-net-intel     (PID 234) — e1000e NIC
  sigma-audio-hda     (PID 235) — Intel HDA
  sigma-nvme          (PID 236) — NVMe storage
```

---

## 3. References & Standards

- Redox OS — `redox-os.org` (MIT)
- Microkernel concepts — Mach, L4

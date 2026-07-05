# Firecracker MicroVM Integration

## Overview

[Firecracker](https://github.com/firecracker-microvm/firecracker) is an open-source microVM hypervisor developed by AWS, licensed under **Apache-2.0**. It uses Linux KVM to provide hardware-isolated virtual machines with a minimal device model, achieving cold-start times under 125ms.

SigmaOS uses Firecracker as a **secure microVM fallback runtime** for two scenarios:

1. **OCI container isolation** — running untrusted OCI images inside a microVM instead of a process-level namespace, giving hardware-enforced separation.

2. **FaaS cold start** — spinning up short-lived function execution environments with predictable latency.

---

## Why Firecracker

| Concern | Solution |
|---|---|
| OCI namespace escape | microVM boundary — guest kernel cannot see host memory |
| FaaS cold start latency | Firecracker boots a minimal Linux guest in < 125ms |
| Device attack surface | Only virtio-net, virtio-blk, vsock — no USB, PCI bus |
| License compatibility | Apache-2.0 — no copyleft risk, run as external process |

---

## Integration Approach

SigmaOS does **not** link Firecracker as a library. It runs Firecracker as an **external subprocess** and communicates via its Unix socket REST API (`--api-sock`). This keeps the Firecracker process boundary intact and avoids any license entanglement.

```
sigma-container run <image>
        │
        ▼
  sigma-container (Rust)
        │  spawn
        ▼
  firecracker --api-sock /run/sigma/fc-<id>.sock
        │  REST PUT /boot-source, /drives, /network-interfaces, /actions
        ▼
  Linux guest kernel (vmlinux)
        │
  OCI rootfs as virtio-blk or overlayfs via virtiofs
```

### virtio Device Glue → Sigma Primitives

| Firecracker virtio device | SigmaOS primitive |
|---|---|
| virtio-net (tap device) | sigma-net network namespace |
| virtio-blk (rootfs image) | SigmaFS sparse image or overlayfs |
| vsock (CID-based IPC) | sigma-bus vsock transport |
| virtio-rng | sigma kernel CSPRNG entropy feed |

---

## Stub Source Files

- `virtualization/ocirunner/firecracker_launcher.rs` — Rust launcher

- `virtualization/ocirunner/README.md` — OCI runner overview

---

## Rust Code: Launching a Firecracker MicroVM

File: `virtualization/ocirunner/firecracker_launcher.rs`

```rust
//! Firecracker microVM launcher for SigmaOS OCI runner.
//! Communicates with the Firecracker REST API over a Unix socket
//! using sigma-curl (no external HTTP dependency).

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, Child};

const FC_BINARY: &str = "/usr/bin/firecracker";
const KERNEL_PATH: &str = "/boot/sigma-vmlinux";
const KERNEL_BOOT_ARGS: &str =
    "console=ttyS0 reboot=k panic=1 pci=off nomodules rw";

pub struct FirecrackerLauncher {
    api_sock: PathBuf,
    child: Option<Child>,
    vm_id: String,
}

impl FirecrackerLauncher {
    pub fn new(vm_id: &str) -> Self {
        Self {
            api_sock: PathBuf::from(format!("/run/sigma/fc-{}.sock", vm_id)),
            child: None,
            vm_id: vm_id.to_string(),
        }
    }

    /// Spawn the Firecracker process.
    pub fn spawn(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let child = Command::new(FC_BINARY)
            .args([
                "--api-sock", self.api_sock.to_str().unwrap(),
                "--log-path", &format!("/run/sigma/fc-{}.log", self.vm_id),
                "--level", "Warning",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()?;
        self.child = Some(child);
        // Give Firecracker time to bind the socket
        std::thread::sleep(std::time::Duration::from_millis(20));
        Ok(())
    }

    /// Configure boot source via REST PUT /boot-source.
    pub fn configure_boot(&self) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "kernel_image_path": KERNEL_PATH,
            "boot_args": KERNEL_BOOT_ARGS
        });
        sigma_curl::put_unix(&self.api_sock, "/boot-source", &payload)?;
        Ok(())
    }

    /// Attach a rootfs block device.
    pub fn attach_rootfs(&self, image: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({
            "drive_id": "rootfs",
            "path_on_host": image.to_str().unwrap(),
            "is_root_device": true,
            "is_read_only": false
        });
        sigma_curl::put_unix(&self.api_sock, "/drives/rootfs", &payload)?;
        Ok(())
    }

    /// Start the microVM (InstanceStart action).
    pub fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let payload = serde_json::json!({ "action_type": "InstanceStart" });
        sigma_curl::put_unix(&self.api_sock, "/actions", &payload)?;
        Ok(())
    }

    /// Stop and clean up.
    pub fn stop(&mut self) {
        if let Some(ref mut child) = self.child {
            let _ = child.kill();
        }
        let _ = std::fs::remove_file(&self.api_sock);
    }
}
```

---

## CI Job: `.github/workflows/microvm-oci.yml`

```yaml
name: MicroVM OCI Smoke Test
on:
  push:
    paths: ['virtualization/ocirunner/**', 'docs/integrations/Firecracker.md']

jobs:
  firecracker-smoke:
    runs-on: ubuntu-22.04
    steps:
      - uses: actions/checkout@v4

      - name: Install Firecracker
        run: |
          curl -fsSL https://github.com/firecracker-microvm/firecracker/releases/\
download/v1.7.0/firecracker-v1.7.0-x86_64.tgz | tar xz
          sudo mv release-v1.7.0-x86_64/firecracker-v1.7.0-x86_64 /usr/bin/firecracker
          sudo chmod +x /usr/bin/firecracker

      - name: Build OCI runner
        run: cargo build --manifest-path virtualization/ocirunner/Cargo.toml --release

      - name: Boot microVM smoke test
        run: |
          sudo ./target/release/sigma-oci-smoke --timeout 125ms
        # Exit criteria: guest kernel prints login prompt in < 125ms

```

---

## Exit Criteria

- `sigma-container run sigmaos/hello:latest` boots in a Firecracker microVM and prints `Hello`.

- Cold start (spawn → guest login prompt) measured at **< 125ms** in QEMU mode.

- The equivalent QEMU command `qemu-system-x86_64 -machine microvm` boots a Linux guest and confirms the hardware model matches.

---

## License Note

Firecracker is **Apache-2.0**. SigmaOS runs it as an external binary — no source is included in the SigmaOS repository. There is no copyleft risk. Attribution is in `docs/License_Map.md`.

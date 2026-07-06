# containerd + runc OCI Stack Integration

## Overview

SigmaOS uses **containerd** (Apache-2.0) as the OCI image manager and pull/unpack engine, and **runc** (Apache-2.0) as the low-level runtime that actually creates containers. A custom **SigmaOS shim** maps the OCI container lifecycle onto `sigma_pledge` / `sigma_unveil` sandboxing primitives.

---

## Component Roles

| Component | Role | License |
|---|---|---|
| containerd | Image pull, layer unpacking, snapshot management | Apache-2.0 |
| runc | OCI runtime: `create`, `start`, `kill`, `delete` | Apache-2.0 |
| sigma-shim | containerd shim v2 — maps OCI to sigma sandbox | MIT (new) |
| sigma-container | CLI frontend: `sigma-container run <image>` | MIT (new) |

---

## Architecture

```
sigma-container run <image>
       │
       ▼
  containerd (socket: /run/containerd/containerd.sock)
       │  pull image layers → SigmaFS overlay snapshot
       │  spawn shim: containerd-shim-sigma-v2
       ▼
  sigma-shim (virtualization/ocirunner/containerd_shim.rs)
       │  translate OCI bundle → sigma_pledge + sigma_unveil
       │  exec runc create / start
       ▼
  runc → container process (sandboxed)
```

---

## Sigma Shim: OCI Lifecycle Mapping

| OCI Lifecycle Hook | Sigma Action |
|---|---|
| `create` | `sigma_pledge("stdio rpath wpath cpath inet")` — per image policy |
| `start` | `sigma_unveil("/", "rx")` then restrict to OCI rootfs |
| `prestart hook` | Set up sigma-net namespace, inject SVID identity |
| `poststop hook` | Revoke capability tokens, flush sigma-bus messages |

File: `virtualization/ocirunner/containerd_shim.rs`

```rust
//! SigmaOS containerd shim v2 implementation.
//! Translates OCI container lifecycle calls into sigma sandbox primitives.

use std::path::PathBuf;

pub struct SigmaShim {
    bundle_path: PathBuf,
    container_id: String,
}

impl SigmaShim {
    pub fn new(id: &str, bundle: &str) -> Self {
        Self {
            container_id: id.to_string(),
            bundle_path: PathBuf::from(bundle),
        }
    }

    /// Called on OCI `create`. Apply sigma_pledge to the container spec.
    pub fn on_create(&self) -> Result<(), Box<dyn std::error::Error>> {
        let spec_path = self.bundle_path.join("config.json");
        let spec = std::fs::read_to_string(&spec_path)?;
        let mut config: serde_json::Value = serde_json::from_str(&spec)?;

        // Inject sigma pledge annotation
        config["annotations"]["dev.sigmaos.pledge"] =
            serde_json::json!("stdio rpath wpath cpath inet proc");
        config["annotations"]["dev.sigmaos.unveil"] =
            serde_json::json!("/:/rx,/tmp:/rwc");

        std::fs::write(&spec_path, serde_json::to_string_pretty(&config)?)?;
        Ok(())
    }

    /// Called on OCI `start`. Fork into sigma-sandboxed runc.
    pub fn on_start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let status = std::process::Command::new("runc")
            .args([
                "create",
                "--bundle", self.bundle_path.to_str().unwrap(),
                &self.container_id,
            ])
            .status()?;
        if !status.success() {
            return Err("runc create failed".into());
        }
        std::process::Command::new("runc")
            .args(["start", &self.container_id])
            .status()?;
        Ok(())
    }

    /// Called on OCI `delete`. Clean up sigma resources.
    pub fn on_delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::process::Command::new("runc")
            .args(["delete", "--force", &self.container_id])
            .status()?;
        // TODO: release sigma capability tokens, flush vsock connections
        Ok(())
    }
}
```

---

## CLI Usage

```bash

# Pull and run an OCI image

sigma-container run sigmaos/hello:latest

# Run with explicit pledge/unveil override

sigma-container run \
  --pledge "stdio rpath inet" \
  --unveil "/data:/r" \
  sigmaos/web-server:1.0

# List running containers

sigma-container ps

# Stop a container

sigma-container stop <container-id>
```

---

## Exit Criteria

- `sigma-container run sigmaos/hello:latest` prints `Hello from SigmaOS!`

- Container process is constrained by `sigma_pledge`; any syscall outside the pledge causes SIGKILL.

- `sigma-container ps` lists running containers with their SVID identities.

# Wine / Proton Windows Compatibility

## Overview

SigmaOS runs Windows applications via **Wine** (LGPL-2.1) and **Proton** (BSD/custom) inside a **Firecracker microVM** containing a minimal Linux guest. This provides hardware isolation: Wine runs inside a Linux guest, the Linux guest runs inside the microVM, and the microVM runs on SigmaOS.

---

## Architecture

```
Windows EXE / Proton game
        │
        ▼
  Wine / Proton (inside Linux guest)
        │  WoW64 thunking, PE loader, Win32 API
        ▼
  Linux guest kernel (inside Firecracker microVM)
        │  virtio-fs → SigmaFS for path mapping
        │  virtio-net → sigma-net
        ▼
  Firecracker microVM
        ▼
  SigmaOS host
```

---

## File Layout

```
virtualization/compat/
└── sigma_wine_launcher.rs
```

---

## sigma_wine_launcher.rs

```rust
//! sigma-wine-launcher: launches Wine inside a Firecracker microVM.
//! Maps SigmaFS paths to the Windows C: drive via virtio-fs.

use crate::ocirunner::FirecrackerLauncher;
use std::path::{Path, PathBuf};

pub struct WineLauncher {
    vm: FirecrackerLauncher,
    wine_prefix: PathBuf,
}

impl WineLauncher {
    pub fn new(vm_id: &str, wine_prefix: &str) -> Self {
        Self {
            vm: FirecrackerLauncher::new(vm_id),
            wine_prefix: PathBuf::from(wine_prefix),
        }
    }

    /// Launch a Windows executable under Wine inside the microVM.
    pub fn launch(&mut self, exe_path: &str) -> Result<(), WineError> {
        // Spawn the Firecracker microVM
        self.vm.spawn().map_err(|_| WineError::VmSpawnFailed)?;
        self.vm.configure_boot().map_err(|_| WineError::VmBootFailed)?;

        // Attach Wine prefix as virtio-fs shared directory
        self.attach_wine_prefix()?;

        self.vm.start().map_err(|_| WineError::VmStartFailed)?;

        // Send exec request via vsock to the guest Wine agent
        let exe_windows_path = sigma_to_windows_path(exe_path);
        self.exec_in_guest(&exe_windows_path)?;
        Ok(())
    }

    fn attach_wine_prefix(&self) -> Result<(), WineError> {
        // virtio-fs tag maps to /wine/prefix in guest
        // Guest agent mounts it as C: in the Wine prefix
        println!(
            "Attaching Wine prefix {} as virtio-fs tag 'wineprefix'",
            self.wine_prefix.display()
        );
        Ok(())
    }

    fn exec_in_guest(&self, windows_path: &str) -> Result<(), WineError> {
        // Send command via vsock CID to the guest Wine executor agent
        println!("Executing in guest: wine '{}'", windows_path);
        // TODO: vsock connect → send JSON command → receive PID
        Ok(())
    }
}

/// Convert SigmaFS path to Windows path for Wine.
fn sigma_to_windows_path(sigma_path: &str) -> String {
    sigma_path.replace('/', "\\").replacen("\\home\\user", "C:\\Users\\User", 1)
}

#[derive(Debug)]
pub enum WineError {
    VmSpawnFailed,
    VmBootFailed,
    VmStartFailed,
    VirtioFsFailed,
    ExecFailed,
}
```

---

## CLI Usage

```bash
# Run a Windows EXE
sigma-wine-launcher run /data/windows/Notepad.exe

# Run with Proton (Steam gaming)
sigma-wine-launcher run \
  --proton /usr/lib/sigma-proton \
  /data/steam/game/game.exe

# List running Wine VMs
sigma-wine-launcher list

# Stop a Wine VM
sigma-wine-launcher stop <vm-id>
```

---

## Path Mapping

| SigmaFS path | Wine/Windows path |
|---|---|
| `/home/user/Documents/` | `C:\Users\User\Documents\` |
| `/data/windows/` | `C:\` |
| `/tmp/` | `Z:\tmp\` |
| `/data/steam/` | `/steam/` (Proton Steam root) |

---

## Gaming: Proton in sigma-pod

For Steam gaming, Proton runs in a `sigma-pod` OCI container:

```bash
sigma-pod run \
  --image sigmaos/proton-9:latest \
  --volume /data/steam:/steam \
  --device /dev/dri/renderD128 \
  -- \
  proton run /steam/game/game.exe
```

---

## Exit Criteria

- `sigma-wine-launcher run /data/windows/Notepad.exe` opens Notepad inside a Firecracker microVM.
- SigmaFS files in `/home/user/` are accessible as `C:\Users\User\` within Wine.
- Proton container runs a Windows game binary without crashing on boot.

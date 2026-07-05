# Flatpak-Inspired App Distribution and Sandboxing

## Overview

SigmaOS uses [Flatpak](https://flatpak.org/) (LGPL-2.1) as the **external runtime** for distributing and sandboxing desktop applications. A Portal API bridges app requests (filesystem, camera, microphone) to the host via sigma-portal, replacing the XDG portal interface with a SigmaOS-native capability model.

---

## Architecture

```
flatpak run io.sigmaos.SigmaEdit
        │
        ▼
  Flatpak runtime (external process)
        │  bubblewrap sandbox + Flatpak permissions
        │  portal D-Bus calls → sigma-portal
        ▼
  sigma-portal (userland/portal/sigma_portal.rs)
        │  capability check against sigma_pledge/sigma_unveil
        ▼
  Host filesystem / camera / microphone
```

---

## sigma-pkg push --flatpak

Publishing a SigmaOS app to Flathub:

```bash

# Build Flatpak bundle

sigma-pkg push --flatpak \
  --app-id io.sigmaos.SigmaEdit \
  --runtime org.freedesktop.Platform//23.08 \
  --sdk org.freedesktop.Sdk//23.08 \
  --source ./sigma-edit/ \
  --sign-key $SIGMA_SIGNING_KEY \
  --output sigma-edit.flatpak

# Submit to registry

sigma-pkg publish sigma-edit.flatpak \
  --registry https://registry.sigmaos.dev
```

---

## File Layout

```
userland/portal/
├── sigma_portal.rs
└── README.md
```

---

## sigma-portal: Portal API

`userland/portal/sigma_portal.rs`:

```rust
//! sigma-portal: bridges Flatpak D-Bus portal calls to SigmaOS capabilities.
//! Implements a subset of the XDG Portal specification using sigma primitives.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum PortalPermission {
    ReadFile(String),
    WriteFile(String),
    Camera,
    Microphone,
    Notification,
    OpenURI(String),
}

pub struct SigmaPortal {
    /// App ID → granted permissions
    grants: HashMap<String, Vec<PortalPermission>>,
}

impl SigmaPortal {
    pub fn new() -> Self {
        Self { grants: HashMap::new() }
    }

    /// Request a permission for an app. Prompts the user if not pre-granted.
    pub fn request_permission(
        &mut self,
        app_id: &str,
        perm: PortalPermission,
    ) -> Result<bool, PortalError> {
        // Check sigma_pledge for this app
        if self.is_pre_granted(app_id, &perm) {
            log::info!("portal: pre-granted {:?} to {}", perm, app_id);
            return Ok(true);
        }
        // Show user prompt via Zenith compositor dialog
        let granted = self.prompt_user(app_id, &perm)?;
        if granted {
            self.grants.entry(app_id.to_string()).or_default().push(perm);
        }
        Ok(granted)
    }

    /// Open a file via the file chooser portal.
    pub fn open_file_chooser(
        &self,
        app_id: &str,
        filter: &str,
    ) -> Result<String, PortalError> {
        // Invoke zenith-dialog file chooser
        let output = std::process::Command::new("sigma-dialog")
            .args(["open-file", "--filter", filter, "--app", app_id])
            .output()
            .map_err(|_| PortalError::DialogFailed)?;
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn is_pre_granted(&self, app_id: &str, perm: &PortalPermission) -> bool {
        self.grants.get(app_id)
            .map(|perms| perms.iter().any(|p| std::mem::discriminant(p) == std::mem::discriminant(perm)))
            .unwrap_or(false)
    }

    fn prompt_user(&self, app_id: &str, perm: &PortalPermission) -> Result<bool, PortalError> {
        let msg = format!("{} requests {:?}", app_id, perm);
        let output = std::process::Command::new("sigma-dialog")
            .args(["confirm", "--message", &msg])
            .status()
            .map_err(|_| PortalError::DialogFailed)?;
        Ok(output.success())
    }
}

#[derive(Debug)]
pub enum PortalError {
    PermissionDenied,
    DialogFailed,
    AppNotFound,
}
```

---

## Flatpak Manifest Example

```yaml

# io.sigmaos.SigmaEdit.yml

app-id: io.sigmaos.SigmaEdit
runtime: org.freedesktop.Platform
runtime-version: '23.08'
sdk: org.freedesktop.Sdk
command: sigma-edit

finish-args:
  - --share=ipc
  - --socket=wayland
  - --filesystem=home
  - --talk-name=org.freedesktop.portal.Desktop

modules:
  - name: sigma-edit
    buildsystem: simple
    build-commands:
      - install -Dm755 sigma-edit /app/bin/sigma-edit
    sources:
      - type: archive
        url: https://registry.sigmaos.dev/src/sigma-edit-1.2.0.tar.zst
        sha256: abc123...
```

---

## Exit Criteria

- `flatpak run io.sigmaos.SigmaEdit` launches sigma-edit in a bubblewrap sandbox.

- File open dialog uses sigma-portal; selected file is readable by the app.

- Camera/microphone requests show sigma-portal confirmation dialog.

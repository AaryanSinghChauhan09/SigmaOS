# Driver Support & Kernel Stabilization 🔧

SigmaOS driver infrastructure is designed around three principles: **upstream-first** (track Linux LTS), **self-healing** (structured error codes + fallback modes), and **sovereign sourcing** (community build recipes, no pre-built blobs).

---

## 🗂️ Driver Subsystem Architecture

```
+------------------------------------------+
|           sigma_driver_manager.cpp        |  ← Hardware profile loader, self-heal
+--------------------+---------------------+
                     | 
         +-----------+------------+
         |                        | 
+--------+--------+   +-----------+----------+
| Kernel Modules  |   | sigma_driver_registry |  ← Community .srecipe catalogue
| (amdgpu, r8169, |   | DKMS auto-rebuild     | 
|  nvme, iwlwifi) |   +-----------+----------+
+--------+--------+               | 
         |                        | 
+--------+--------+   +-----------+----------+
| sigma_driver    |   | include/              | 
| _codes.h        |   | sigma_driver_codes.h  |  ← ZEN-DRIVER-xxxx error codes
+-----------------+   +----------------------+
```

---

## 📋 Structured Error Codes

All driver failures emit a `ZEN-DRIVER-xxxx` code, consistent with the broader SigmaOS error taxonomy:

| Code | Meaning | 
| --- | --- | 
| `ZEN-DRIVER-0401` | GPU initialization failed | 
| `ZEN-DRIVER-0402` | GPU firmware blob missing | 
| `ZEN-DRIVER-0403` | GPU fell back to VGA safe mode | 
| `ZEN-DRIVER-0501` | NIC driver initialization failed | 
| `ZEN-DRIVER-0502` | NIC firmware blob missing | 
| `ZEN-DRIVER-0601` | Audio subsystem failed | 
| `ZEN-DRIVER-0603` | Audio fell back to dummy device | 
| `ZEN-DRIVER-0701` | Storage driver initialization failed | 
| `ZEN-DRIVER-0705` | Forensic boot — read-only storage enforced | 
| `ZEN-DRIVER-0901` | DKMS module rebuild failed after kernel update | 
| `ZEN-DRIVER-0903` | DKMS ABI version mismatch | 
| `ZEN-DRIVER-0A01` | Kernel module (.ko) not found | 
| `ZEN-DRIVER-0A05` | Driver recipe signature verification failed | 

Use `sigma_driver_strerror(code)` to decode any error into a human-readable string.

---

## 🖥️ Hardware Profiles

The Driver Manager selects the appropriate module set based on the active hardware profile (set in Control Center or the Onboarding Wizard):

| Profile Flag | Description | Influenced By | 
| --- | --- | --- | 
| `SIGMA_HW_PROFILE_STANDARD` | General desktop hardware | Clear Linux, Ubuntu | 
| `SIGMA_HW_PROFILE_GAMING` | GPU + audio optimisations | SteamOS | 
| `SIGMA_HW_PROFILE_IOT_ARM64` | Lightweight ARM64 drivers (vc4, brcmfmac, mmc_block) | Raspberry Pi OS | 
| `SIGMA_HW_PROFILE_FORENSIC` | Read-only block storage enforced (CAINE model) | CAINE, SystemRescue | 
| `SIGMA_HW_PROFILE_SERVER` | Cloud hardware + i915 + NVMe | Fedora CoreOS | 

**Boot sequence:**

```cpp
// In sigma_kernel_main():
sigma_driver_init_hardware(SIGMA_HW_PROFILE_STANDARD);

// Recovery from Settings app:
sigma_driver_reload("amdgpu");
```

---

## 🔧 Self-Healing Behaviour

When a driver fails to load, `sigma_driver_manager.cpp` applies automatic fallback:

| Subsystem | Primary | Fallback | Error Emitted | 
| --- | --- | --- | --- | 
| GPU | `amdgpu` / `i915` / `nvidia` | VGA safe mode (800×600) | `ZEN-DRIVER-0403` | 
| Audio | `snd_hda_intel` | Dummy audio device | `ZEN-DRIVER-0603` | 
| Network | `iwlwifi` / `r8169` | No network (notify user) | `ZEN-DRIVER-0501` | 
| Storage | `nvme` / `ahci` | Read-only fallback | `ZEN-DRIVER-0701` | 

**Control Center "Fix it" buttons** call `sigma_driver_reload(module_name)` — no reboot required for non-storage subsystems.

---

## 📦 Sovereign Driver Registry

Inspired by **SlackBuilds.org** (no pre-built binaries) and **DKMS** (ABI-safe kernel rebuilds).

### Install Flow

```
[1] User browses sigma_driver_registry_list()
[2] Selects recipe → Registry fetches .srecipe from registry.sigmaos.dev
[3] Cryptographic signature verified against sovereign root key
[4] Build runs inside isolated Orchestrator container
[5] Output packaged as signed .spkg bundle
[6] Module registered with DKMS tracker
[7] On next kernel update → sigma_driver_registry_rebuild_dkms() auto-rebuilds
```

### Example CLI

```bash
sigma-drv list                  # Browse available recipes
sigma-drv install rtl8852be     # Install Realtek Wi-Fi 6 (ARM64 compatible)
sigma-drv rebuild-dkms          # Manually trigger DKMS rebuild
sigma-drv status amdgpu         # Check module load status
```

> [!IMPORTANT]
> Only `.srecipe` files signed by the **SigmaOS Sovereign Root Key** are permitted. Unsigned community recipes will fail at `ZEN-DRIVER-0A05`.

---

## 🚀 Roadmap

| Milestone | Status | 
| --- | --- | 
| `sigma_driver_codes.h` — full ZEN-DRIVER-xxxx taxonomy | ✅ Done | 
| `sigma_driver_manager.cpp` — profile-aware loader + self-heal | ✅ Done | 
| `sigma_driver_registry.cpp` — community recipe index + DKMS | ✅ Done | 
| Control Center "Reload Driver" button integration | 🔲 Planned | 
| Hardware test suite (automated boot tests per chipset) | 🔲 Planned (Phase 6) | 
| ARM64 cross-compilation toolchain for driver builds | 🔲 Planned (Phase 6) |

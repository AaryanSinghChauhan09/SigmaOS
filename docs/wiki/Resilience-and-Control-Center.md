# Resilience, Error Handling, and Control Center ⚙️🛡️

SigmaOS layers strict defensive mechanisms inspired by Qubes, Whonix, NixOS, and CAINE. 

---

## 🛡️ Sovereign Resilient Error Handling (Stage 1)

SigmaOS rejects opaque failures. Every crash or denial maps to a granular, deterministic error trace.

### Granular Diagnostic Codes (`sigma_error_codes.h`)
Sovereign error codes partition across subsystems:
*   `ZEN_101`: Container & Orchestrator Runtime failures.
*   `ZEN_203`: Network Sandbox permission blocks (inspired by Whonix's split gateway rules).
*   `ZEN_402`: UI Window allocation memory limits.
*   `ZEN_502`: Compositor VGA Fallback triggers.

### Structured NixOS-inspired Logging (`zenith_logger.cpp`)
All exceptions dump deterministic JSON strings to `zenithd.log`:
```json
{"timestamp_mock": 1774857600, "error_code": 502, "component": "Compositor", "description": "Hardware FB failed, safe VGA recovery triggered", "container_id": 0}
```

### Self-Healing Compositor (`sigma_compositor.cpp`)
If the native compositor fails to acquire a physical framebuffer:
1. It automatically fires code `ZEN_502` and maps its state to **VGA Safe Fallback Mode**.
2. Instead of freezing the kernel, it shifts screen mapping to safe debug outputs.
3. The self-healing wrapper (`zenith_compositor_heal()`) allows the system to trigger hot-reloads dynamically.

---

## ⚙️ Modular Control Center & Reproducible Profiles (Stage 2)

We have centralized configuration management into a robust control center (`sigma_control_center.cpp`).

### Declarative NixOS-style Configuration Replication
Users can completely replicate their system isolation environment by importing/exporting a single declarative state file (`settings.json`):
*   `zenith_settings_export(path)`: Serializes active flags (strict sandboxing, network isolation, active release channels).
*   `zenith_settings_import(path)`: Dynamically updates the security profile state deterministically.

### CAINE-style Forensic Boot Profile
Toggleable write-protection limits active storage nodes to write-protected read-only status:
*   When Forensic Mode is activated, all storage requests are locked.
*   Workstation processes are isolated dynamically from direct raw blocks, ensuring forensic integrity.

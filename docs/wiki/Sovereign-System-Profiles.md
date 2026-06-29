# Sovereign System Profiles & Hybrid C/Rust Interop 🦀⚙️

SigmaOS is designed for extreme environmental adaptability. To accomplish this without bloating the core kernel, SigmaOS introduces **Sovereign System Profiles** and a robust **Hybrid C/Rust execution bridge**.

---

## 🦀 Hybrid C/Rust FFI Interoperability (Stage 1)

While the bare-metal kernel core, bootloader, and raw hardware abstractions are implemented in ISO C11, all modern user-space engines, container monitoring tools, and the Zenith SDK support direct, safe, idiomatic Rust packaging.

### Zero-Overhead FFI Rust Bindings (`Cargo.toml` & `lib.rs`)
The `zenith-sdk` Rust crate wraps raw C symbols with memory safety guarantees (such as standard borrowing, lifetimes, and bounds checks):
*   Exposes raw imports: `zenith_launch_app_sandboxed`, `sys_print`, and `sys_ipc_send`.
*   Exposes a safe, idiomatic Rust struct `Application` that automatically coordinates with the Sovereign Orchestrator under-the-hood.

```rust
use zenith_sdk::Application;

fn main() {
    // Spawns a Whonix-style secure sandbox dynamically under the hood!
    let app = Application::new("Sovereign Rust App");
    app.run();
}
```

---

## ⚙️ Sovereign System Profiles (Stage 2)

SigmaOS formalizes dynamic security and resource enforcement models using central profile structures.

```
+-----------------------------------------------------------------------------------+
|                            Sovereign Profile Switcher                             | 
+---------------------+-------------------+-------------------+---------------------+
                      |                   |                   | 
                      v                   v                   v
              [CAINE Forensic]          [IoT]            [Enterprise]
              - Read-Only blocks   - 16MB Sandboxes     - Strict VFS ACLs
              - Whonix Firewall    - Curated Channel    - Deterministic logs
```

The unified control center (`sigma_control_center.cpp`) dynamically manages system properties per active profile:

### 1. `SIGMA_PROFILE_FORENSIC` (CAINE-inspired)
*   Enforces **read-only status** globally across all block devices.
*   Enforces Whonix-style default-deny gateway/workstation split rules.
*   Enforces cryptographically curated update channels.

### 2. `SIGMA_PROFILE_IOT` (Raspberry Pi OS optimized)
*   Limits containers to a strict **16MB resource budget**.
*   Locks system features to low-footprint, lean baseline processes.

### 3. `SIGMA_PROFILE_ENTERPRISE` (Audit Hardened)
*   Enforces mandatory **Zero-Trust VFS ACL checkups**.
*   Mandates fully traceable, deterministic logging.

### 4. `SIGMA_PROFILE_EDUCATION` (Exploratory overrides)
*   Loads permissive sandbox limits for user learning and playground development.

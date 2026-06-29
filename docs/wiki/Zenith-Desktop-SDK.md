# Zenith Desktop & Developer SDK 🎨🔒

Zenith is the native sovereign desktop environment for SigmaOS. Unlike traditional Linux/UNIX desktop environments (e.g., GNOME, KDE) which run application processes directly on the host system memory with basic Unix user permissions, Zenith applications are **fully containerized and sandboxed by default** via the Sovereign Orchestrator.

---

## 🚀 First-Run Onboarding Wizard

When a new user boots SigmaOS for the first time, they are greeted by a native **Sovereign Onboarding Wizard** (`sigma_onboarding.cpp`) inspired by Zorin OS's layout chooser and Elementary OS's first-run experience.

The wizard guides users through:
1.  **Welcome**: A clear explanation of what makes SigmaOS sovereign.
2.  **Profile Selection**: Direct toggle between Standard, Forensic (CAINE), IoT, Enterprise, and Education profiles.
3.  **Network Configuration**: Boots Whonix-style gateway/workstation firewall splits.
4.  **Declarative Config Import**: Import saved `settings.json` profiles from previous systems.

---

## 📦 Sovereign App Store

Zenith ships with a curated **Sovereign App Store** (`sigma_appstore.cpp`) inspired by Elementary's AppCenter:
*   All apps are listed with **explicit permission declarations** (Network access, FS scope, GPU usage).
*   Each installation fetches the `.srecipe` source build script from the Sovereign Registry.
*   Apps are compiled **locally** inside isolated orchestrator containers.
*   The final binary is cryptographically signed as a `.spkg` bundle before deployment.

No untrusted pre-compiled binaries. Ever.

---

## 🔒 The Secure App Sandbox Architecture

When you launch a GUI application on Zenith:
1. **The Sandbox Bridge** (`sigma_sandbox_bridge.cpp`) intercepts the execution request.
2. It sends an IPC request to the **Sovereign Orchestrator** to create a dedicated container shard.
3. The Orchestrator assigns a unique `container_id` and strict resource quotas (CPU limits, chroot jail, and virtual IP block).
4. The **Native Compositor** (`sigma_compositor.cpp`) maps the app's backing framebuffers to this exact `container_id`, preventing any unauthorized cross-app window read/writes.

```
+-------------------------------------------------------------+
|                     Zenith Compositor                       | 
+------------------------------+------------------------------+
                               | verified container memory access
                               v
+------------------------------+------------------------------+
|                    Zenith Sandbox Bridge                     | 
+------------------------------+------------------------------+
                               | IPC Shard Request
                               v
+------------------------------+------------------------------+
|                     Sovereign Orchestrator                  | 
+--------------+------------------------------+---------------+
               |                              | 
               v                              v
      [App Container Shard #1]       [App Container Shard #2]
```

---

## 🛠️ Zenith Developer SDK (Preview)

The SDK provides a native C++ framework for building responsive, secure apps that natively bind to our secure backend. A **Rust crate** (`zenith-sdk`) is also available for memory-safe application development.

### Hello World Reference (C++)

```cpp
#include <zenith.h>

using namespace Zenith;
using namespace Zenith::UI;

int main() {
    Application app("Sigma Hello World");
    app.createWindow(800, 600);
    Label title({ 300, 50, 200, 40 }, "Welcome to Sovereign Desktop");
    Button btn_click({ 325, 120, 150, 50 }, "Click Me!");
    app.addWidget(&title);
    app.addWidget(&btn_click);
    app.run();
    return SIGMA_SUCCESS;
}
```

### Hello World Reference (Rust)

```rust
use zenith_sdk::Application;

fn main() {
    let app = Application::new("Sovereign Rust App");
    app.run();
}
```

---

## 🚀 Key SDK Milestones

*   **M4b.1:** Defined unified SDK Architecture (`zenith.h`).
*   **M4b.2:** Released the initial Developer Preview with declarative C++ layout components and sandboxing hooks.
*   **M4b.3:** Rust crate (`zenith-sdk`) released for memory-safe app development.
*   **M4b.4:** Onboarding Wizard and Sovereign App Store launched.

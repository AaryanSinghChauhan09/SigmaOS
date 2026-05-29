# Zenith Desktop Environment & Developer SDK 🎨🔒

Zenith is the native sovereign desktop environment for SigmaOS. Unlike traditional Linux/UNIX desktop environments (e.g., GNOME, KDE) which run application processes directly on the host system memory with basic Unix user permissions, Zenith applications are **fully containerized and sandboxed by default** via the Sovereign Orchestrator.

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

The SDK provides a native C++ framework for building highly responsive, secure, and beautiful applications that natively bind to our secure backend.

### Hello World Reference

```cpp
#include <zenith.h>

using namespace Zenith;
using namespace Zenith::UI;

int main() {
    // 1. Initialize app & negotiate secure Container Shard automatically
    Application app("Sigma Hello World");

    // 2. Request a backing window from the sandboxed allocator
    app.createWindow(800, 600);

    // 3. Setup declarative widgets
    Label title({ 300, 50, 200, 40 }, "Welcome to Sovereign Desktop");
    Button btn_click({ 325, 120, 150, 50 }, "Click Me!");

    app.addWidget(&title);
    app.addWidget(&btn_click);

    // 4. Run native event loop
    app.run();

    return SIGMA_SUCCESS;
}
```

---

## 🚀 Key SDK Milestones

*   **M4b.1:** Defined unified SDK Architecture (`zenith.h`).
*   **M4b.2:** Released the initial Developer Preview containing declarative C++ layout components and sandboxing hooks.
*   **M4b.3:** Extending SDK bindings to Rust and Python (Ongoing).

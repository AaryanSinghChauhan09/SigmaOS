# Optional Shards Specification

This specification covers secondary hardware drivers, virtualization layers, and advanced system utilities that are dynamically loaded on-demand.

---

## 🔌 Layer 2: Legacy & Secondary Driver Shards

These drivers are loaded only if target devices are detected during the ACPI/PCI bus probe.

| Shard Path | Device Class | Core Purpose |
| :--- | :--- | :--- |
| `kernel/core/drivers/SovereignUSB3.cpp` | USB Controller | xHCI device loops, transfer rings, bulk/interrupt packets |
| `kernel/core/drivers/SovereignFireWire.cpp`| IEEE 1394 | Serial bus interface, high-speed media transfers |
| `kernel/core/drivers/SovereignPCMCIA.cpp`| Laptop Expansion | Legacy expansion bus, card information structure (CIS) |
| `kernel/core/drivers/SovereignTVTuner.cpp`| Media Tuner | Video capture interfaces, signal demodulation |

---

## ☁️ Hypervisor & Virtualization Shards

These components expose microVM execution and lightweight containerization interfaces.

| Shard Path | Component Class | Core Features |
| :--- | :--- | :--- |
| `kernel/core/system/SovereignKVM.cpp` | Virtual Machine | Hardware acceleration loops (Intel VT-x, AMD-V) |
| `kernel/core/system/SovereignLXC.cpp` | Container Engine | Group isolation wrappers, overlay mounts, net bridges |
| `kernel/core/system/SovereignWASM.cpp` | Runtime Engine | Sandboxed WebAssembly bytecode interpreter for userland apps |

---

## 🛠️ Layer 6: System Utilities & Desktop Shards

Advanced command-line tools and desktop composition managers.

| Shard Path | Subsystem | Functional Scope |
| :--- | :--- | :--- |
| `kernel/core/ui/SovereignWM.cpp` | Compositor | Desktop window manager, hardware composition, damage tracking |
| `kernel/core/ui/SovereignPanel.cpp` | Desktop Bar | System tray, application dock launcher, time/network indicators |
| `suites/S32_SystemTools/core/SovereignPlayground.cpp` | Developer Tool | Interactive sandbox for testing new system APIs in real-time |
| `kernel/core/observability/SovereignLogD.cpp` | Log Daemon | Gathers kernel ring buffer events and routes to persistent logs |

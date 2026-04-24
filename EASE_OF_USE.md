# Ease of Use & Extreme Modularisation

SigmaOS shatters the myth that a secure, bare-metal microkernel must be difficult to use. By combining **Sovereign Extensions (`s-ext`)**, **Application Sandboxing (`app_sandbox.c`)**, and **Declarative Manifests (`manifest_parser.c`)**, the system achieves unparalleled Developer Experience (DX) and User Experience (UX).

Located in `modules/ext/plugins/` and `modules/tools/cli/`.

## Competitive Advantages (USPs)

### 1. Zero-Configuration Sandboxing
- **Standard OS**: On Linux, to properly isolate an application, a user must configure complex `Docker` containers, `chroot` jails, or `AppArmor` profiles.
- **SigmaOS USP**: The kernel's Application Sandbox handles everything invisibly. When a user double-clicks an app, the OS automatically generates an isolated VFS namespace and negotiates a strictly enforced Memory Contract on the app's behalf. If the app tries to exceed its memory quota or reach out to the network without permission, the Sovereign Watchdog terminates it instantly. The user does *nothing*.

### 2. Manifest-Driven Automations
- **Standard OS**: Apps install background daemons or registry keys to launch at startup, define hotkeys, or change the UI theme, slowly bloating the system over time.
- **SigmaOS USP**: Third-party applications ship with a simple, declarative JSON manifest. When launched, the kernel's Manifest Parser reads it and dynamically wires the OS *around* the app. 
    - Need a global hotkey? The kernel temporarily registers it in Ring-0 and instantly unregisters it when the app closes. 
    - Prefer a dark theme? The Zenith UI Compositor shifts its palette temporarily while the app is focused.
    No permanent system bloat is ever created.

### 3. Ultimate Plugin Modularisation
- **Standard OS**: Customising the OS kernel requires deep C knowledge and recompilation.
- **SigmaOS USP**: With `s-ext`, any authorized application can inject logic directly into the UI render loop or the AI Scheduler's reward function. This turns the entire operating system into a modular, programmable playground, without sacrificing the security of the underlying capability architecture.

---

## 🚀 Developer Quick-Start

### 1. Clone & Build (Linux / macOS)

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
rustup toolchain install nightly
rustup target add aarch64-unknown-none riscv64imac-unknown-none-elf
npm ci                   # Install Zenith UI dev dependencies
chmod +x build_sovereign.sh
./build_sovereign.sh     # Builds all suites
```

### 2. Run Tests

```bash
./run_sigma_tests.sh     # Sovereign Atomic Test Runner
```

### 3. Boot in QEMU

```bash
make aarch64             # Cross-compile for ARM64
make run                 # Boot in QEMU (requires qemu-system-aarch64)
```

---

## 📡 IPC Usage Example

Send a message between two shards using the typed, capability-checked IPC layer:

```c
#include "sigma_kernel_types.h"

// Create a channel from Shard 1 (S03) to Shard 2 (S07)
int ch = ipc_create_channel("s03->s07", POOL_S03, POOL_S07, CAP_TENSOR);

// Send a tensor dispatch request — auto-persisted with seq_id
const u8 payload[] = { 0x01, 0x02, 0x03 };
k_status result = ipc_send(ch, SHARD_S03, MSG_TENSOR_DISPATCH,
                            payload, sizeof(payload));

if (result == K_ERR_NODEV) {
    // Persistence write failed → message was rolled back, not lost
    kprintf("[S03] Delivery deferred — persistence unavailable\n");
}

// Receive on the other end (S07)
SigmaIPCMsg msg;
if (ipc_recv(ch, &msg) == K_OK) {
    // Process msg.payload[0..msg.payload_len]
}
```

---

## 🔌 Adding a New Shard

```bash
# 1. Scaffold a new shard
make scaffold-S34_MyShard-c

# 2. Implement your shard init in suites/S34_MyShard/shard_init.c
# 3. Run header synthesis
node repair_build.js

# 4. Build — your shard is auto-discovered
./build_sovereign.sh
```

---

## 🖥️ Zenith UI Dashboard

Open `web_ui/index.html` in any modern browser to see:

| Panel | What it shows |
|-------|--------------|
| **System Monitor** | CPU, memory, task queue live counters |
| **Kernel Logs** | Real-time ring buffer from `kprintf()` |
| **AI Dispatch** | CPU vs NPU routing decisions per tensor op |
| **Neural Demo** | Live CNN inference pipeline visualization |
| **Persistence Panel** | Write / Checkpoint / Replicate / Recover events per shard |
| **Perf Metrics** | NPU vs CPU latency sparkline + utilization bars |

> The three kernel telemetry panels require authentication — enter any passphrase ≥ 8 characters on the login gate.

---

*Questions? Email [aaryansinghchauhan090305@gmail.com](mailto:aaryansinghchauhan090305@gmail.com)*

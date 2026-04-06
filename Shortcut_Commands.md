# SigmaOS Shortcut Commands & Keybindings

Unlike heavy desktop environments that intercept strokes in the user space (UI layer), SigmaOS defines fundamental keyboard shortcuts natively inside the compiled C11 kernel (`keyboard_master.c`). This bypasses the typical input latency, executing system tools instantaneously.

## ⌨️ Global Kernel-Level Shortcuts

These shortcuts map directly to the hardware IDT (Interrupt Descriptor Table) and instantly trigger Shard invocations or UI overlays.

| Shortcut Keybind | Target Shard / Tool | Function | Status |
| --- | --- | --- | --- |
| `Alt` + `S` | **Shard Explorer** | Instantly overlay the visual browser for available, unloaded `.c` capability shards. | Active |
| `Alt` + `E` | **Zen Editor** | Open the sovereign C11/Macro code editor without blocking the current terminal session. | Active |
| `Alt` + `R` | **Screen Recorder** | Hook directly into the hardware framebuffer to initiate lossless video capture. | Active |
| `Alt` + `C` | **Omni Shell** | Spawn a new instance of the 400+ command POSIX-native terminal. | Active |
| `Alt` + `Q` | **Kill Process** | Send `SIGTERM` instantly to the currently focused window or foreground daemon. | Core Mechanics |

## 🔗 Omni Shell Custom Shorthand Aliases

To maintain absolute execution speed, the `omni_shell` accepts the following unified command shorthands, redirecting securely to the parent `sigma-*` commands:

| Command Shorthand | Full Execution Target | Action Performed |
| --- | --- | --- |
| `invoke <shard>` | `sigma_invoke <shard>` | Dynamically compile and load the referenced library payload into ring-0. |
| `clean` | `system_cleaner` | Run amnesic memory dump, cache wipe, and browser data sanitization. |
| `optimize` | `sigma_auto_optimizer` | Force immediate heap fragmentation check and slab allocator rebalancing. |
| `bot <target>` | `remote_bot <target>` | Execute a bare-metal remote procedure call without `ssh` dependencies. |
| `academy` | `sigma_invoke academy` | Enter strict educational mode with locked-down routing limits. |
| `snapshot` | `backup_manager create` | Delta-log the current VFS state onto physical disk using native deduplication. |

## 🖱️ Agent Launcher Desktop Hubs

For external orchestrations (like external Windows `.lnk` shortcuts mapping into the Sigma workflow), the platform utilizes rapid batch/launcher scripts for AI tasks:

| Desktop Shortcut | Target Payload | Purpose |
| --- | --- | --- |
| **Aether Orchestrator** | `LAUNCH_AI_ORCHESTRATOR.bat` | Spawns the central Multi-AI prompt distribution network. |
| **OpenRoutines Hub** | `START_AETHER_ROUTINES.bat` | Maps to the internal Python automation hooks and macro tasks. |
| **Excel AI Filler** | `run_app.bat` | Targets spreadsheet formatting and manipulation algorithms. |
| **Email Agent** | `agent.py` | Spawns background NLP mail parsers. |

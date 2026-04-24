
# Sovereign Shell (`s-cli`)


The Sovereign Shell is the native, bare-metal scripting language and control interface for SigmaOS. It allows system administrators and developers to interact directly with the microkernel and its modules without relying on legacy POSIX shells like Bash.

Located in `modules/tools/cli/sovereign_shell.c`.


## Core Commands


| Command | Description | Subsystem Hook |
| :--- | :--- | :--- |
| `load <id>` | Hot-loads a kernel module/capsule dynamically | `capsule_load()` |
| `unload <id>`| Gracefully unloads a module without rebooting | `capsule_unload()` |
| `caps` | Lists all capability tokens owned by the current process | `cap_registry_query_module()` |
| `profile` | Triggers the AI scheduler profiling output | `profiler_analyze()` |
| `mesh` | Displays sovereign peer-to-peer network status | `mesh_net.c` |


## Design Philosophy


- **Hardware-Native**: The shell is designed to control hardware and modular services directly.
- **Sandboxed**: Shell sessions execute within the standard capability-based security model. A user cannot run `load` unless they hold the `CAP_EXECUTE` right for the module loading service.
- **Extensible**: New modules loaded into the kernel can expose custom `s-cli` commands.

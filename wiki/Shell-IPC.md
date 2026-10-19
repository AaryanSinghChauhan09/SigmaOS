# SigmaOS Shell IPC Components

## Overview
SigmaOS shell IPC manager providing inter-process communication for shell plugins, competitive with Omarchy shell IPC (omancy-shell, quickshell IPC contract).

## Features
- **IPC messaging** - Send messages between shell components
- **Plugin registry** - Register and manage shell plugins
- **Plugin enable/disable** - Enable or disable plugins
- **Plugin load/unload** - Load or unload plugins
- **Pending calls** - Track pending IPC calls
- **IPC methods** - Ping, Summon, Hide, Toggle, Call, etc.
- **IPC targets** - Shell, Plugin

## Structures

| Structure | Description |
|-----------|-------------|
| **IpcManager** | IPC manager with message routing |
| **IpcMessage** | Individual IPC message |
| **IpcResponse** | IPC response types |
| **IpcMethod** | Ping, Summon, Hide, Toggle, Call, etc. |
| **IpcTarget** | Shell, Plugin |
| **PluginInfo** | Plugin information |

## Usage Examples

```rust
// Create IPC manager
let mut mgr = IpcManager::new();

// Send an IPC message
let id = mgr.send(
    IpcTarget::Shell,
    IpcMethod::Ping,
    None,
);

// Register a plugin
let plugin = PluginInfo {
    id: "weather-widget".to_string(),
    name: "Weather Widget".to_string(),
    kind: "bar-widget".to_string(),
    entry_point: "Weather.qml".to_string(),
    is_enabled: false,
    is_loaded: false,
};
mgr.register_plugin(plugin);

// Enable and load plugin
mgr.enable_plugin("weather-widget").unwrap();
mgr.load_plugin("weather-widget").unwrap();

// Complete a pending call
mgr.complete_call(id);
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Shell & UX (P1)

## Related Files
- `src/shell/ipc.rs` - IPC manager implementation
- `wiki/Shell-IPC.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, COMMAND_QUERY_SEPARATION,
INTERFACE_SEGREGATION, INVERSION_OF_CONTROL, POLYMORPHISM,
COMPOSITION_OVER_INHERITANCE, RACE_FREE DESIGN, SAFE_CONCURRENCY,
DEADLOCK_AVOIDANCE, ZERO_COPY, MEMORY_SAFETY
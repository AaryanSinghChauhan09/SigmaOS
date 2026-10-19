# SigmaOS Plugin Architecture Components

## Overview
SigmaOS plugin architecture providing plug-in driver model for extensibility, competitive with Omarchy's plugin system (third-party plugins from git, managed from Setup > Plugins).

## Features
- **Plug-in driver model** for extensibility
- **Plugin kinds**: BarWidget, Panel, Overlay, Menu, Service, Theme, Effect
- **Auto-discovery** from plugin directories
- **Enable/disable plugins**
- **Load/unload plugins**
- **Manifest-based configuration** (schema version, id, name, version, author, etc.)
- **Dependency declaration**
- **keep_loaded** option for persistent plugins

## Plugin Kinds

| Kind | Description |
|------|-------------|
| **BarWidget** | Bar widget component |
| **Panel** | Floating panel window |
| **Overlay** | Fullscreen overlay |
| **Menu** | Menu surface |
| **Service** | Headless singleton service |
| **Theme** | Theme component |
| **Effect** | Visual effect |

## Structures

| Structure | Description |
|-----------|-------------|
| **PluginManager** | Plugin manager with register/enable/disable/load/unload |
| **Plugin** | Plugin with manifest, path, enabled, loaded state |
| **PluginManifest** | Plugin manifest with metadata |
| **PluginKind** | BarWidget, Panel, Overlay, Menu, Service, Theme, Effect |

## Usage Examples

```rust
// Create plugin manager
let mut mgr = PluginManager::new();

// Register a plugin
let manifest = PluginManifest {
    schema_version: 1,
    id: "weather-widget".to_string(),
    name: "Weather Widget".to_string(),
    version: "1.0.0".to_string(),
    author: "community".to_string(),
    description: "Weather bar widget".to_string(),
    kinds: vec![PluginKind::BarWidget],
    entry_points: HashMap::new(),
    keep_loaded: false,
    default_enabled: true,
    dependencies: vec![],
};
mgr.register(manifest, "/plugins/weather".to_string());

// Enable and load
mgr.enable("weather-widget").unwrap();
mgr.load("weather-widget").unwrap();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only, fs for auto-discovery)
- **Roadmap Phase**: Extensibility (P1)

## Related Files
- `src/plugin/manager.rs` - Plugin manager implementation
- `wiki/Plugin-Architecture.md` - Wiki documentation

## Principles Followed
PLUG_IN_DRIVER_MODEL, COMPOSITION_OVER_INHERITANCE, MODULARITY, COMPONENT_ISOLATION,
INVERSION_OF_CONTROL, INTERFACE_SEGREGATION, OPEN_CLOSED, SINGLE_RESPONSIBILITY,
LOOSE_COUPLING, DISCOVERABILITY, CODE_REUSABILITY, OPTIMISE_FOR_DELETION,
YOU_AREN_T_GONNA_NEED_IT, RULE_OF_THREE
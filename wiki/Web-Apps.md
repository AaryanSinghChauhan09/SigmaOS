# SigmaOS Web Apps Components

## Overview
SigmaOS web apps manager providing web application management, competitive with Omarchy web apps (omancy-webapp-install, omancy-webapp-remove, omancy-webapp-remove-all).

## Features
- **Install web apps** - Install web applications with name, URL, icon, category
- **Remove web apps** - Remove individual web apps
- **Remove all** - Remove all web apps
- **List web apps** - List all installed web apps
- **Get by ID/name** - Lookup web apps by ID or name

## Structures

| Structure | Description |
|-----------|-------------|
| **WebAppsManager** | Web apps manager |
| **WebApp** | Individual web application |

## Usage Examples

```rust
// Create web apps manager
let mut mgr = WebAppsManager::new();

// Install web app
let id = mgr.install(
    "GitHub".to_string(),
    "https://github.com".to_string(),
    Some("https://github.com/icon.png".to_string()),
    "Development".to_string(),
).unwrap();

// Get by name
let app = mgr.get_by_name("GitHub").unwrap();

// Remove web app
mgr.remove(&id).unwrap();

// Remove all
mgr.remove_all();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/webapps/manager.rs` - Web apps manager implementation
- `wiki/Web-Apps.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
OPEN_STANDARD_AND_INTEROPERABILITY, USER_FREEDOM_AND_CONTROL
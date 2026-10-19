# SigmaOS Clipboard Manager Components

## Overview
SigmaOS clipboard manager providing clipboard history with image previews and sensitive-content exclusion, competitive with Omarchy's clipboard manager.

## Features
- **Clipboard history** with up to 50 items
- **Image previews** with metadata (width, height, format, size)
- **Sensitive content exclusion** (toggleable)
- **Text, image, file, HTML, rich text** support
- **Clear history** functionality
- **Remove individual items**

## Content Types

| Type | Description |
|------|-------------|
| **Text** | Plain text content |
| **Image** | Image with dimensions and format |
| **File** | File paths |
| **Html** | HTML content |
| **RichText** | Rich text content |

## Structures

| Structure | Description |
|-----------|-------------|
| **ClipboardManager** | Clipboard manager with history |
| **ClipboardItem** | Individual clipboard item |
| **ClipboardContent** | Text, Image, File, Html, RichText |

## Usage Examples

```rust
// Create clipboard manager
let mut mgr = ClipboardManager::new();

// Copy text
mgr.copy(ClipboardContent::Text("hello".to_string()), Some("terminal".to_string()), false).unwrap();

// Copy image
mgr.copy(ClipboardContent::Image { width: 1920, height: 1080, format: "png".to_string(), data_len: 500000 }, None, false).unwrap();

// Toggle sensitive exclusion
mgr.toggle_sensitive_exclusion();

// List history
let history = mgr.list_history();
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Desktop & UX (P1)

## Related Files
- `src/clipboard/manager.rs` - Clipboard manager implementation
- `wiki/Clipboard-Manager.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING, SECURE_DEFAULTS,
MEMORY_SAFETY, ZERO_COPY, MEMORY_DISCIPLINE, PRIVACY
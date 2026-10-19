# SigmaOS Display Text Size Components

## Overview
SigmaOS display text size manager providing system-wide text scaling, competitive with Omarchy display text size (omancy-display-text-size).

## Features
- **Text size levels**: Small, Normal, Large, ExtraLarge, Custom
- **Scale factor** - Configurable scale (0.5x to 3.0x)
- **Increase/decrease** - Step-based adjustment
- **App overrides** - Per-application scale overrides
- **Reset** - Reset to normal size

## Structures

| Structure | Description |
|-----------|-------------|
| **DisplayTextSizeManager** | Text size manager |
| **TextSizeLevel** | Small, Normal, Large, ExtraLarge, Custom |

## Usage Examples

```rust
// Create text size manager
let mut mgr = DisplayTextSizeManager::new();

// Set level
mgr.set_level(TextSizeLevel::Large);

// Set custom scale
mgr.set_scale_factor(1.5).unwrap();

// Increase/decrease
mgr.increase(0.25).unwrap();
mgr.decrease(0.1).unwrap();

// App override
mgr.set_app_override("terminal".to_string(), 1.5).unwrap();
let scale = mgr.get_effective_scale(Some("terminal"));
```

## Status
- **Implementation**: Complete
- **Dependencies**: None (stdlib only)
- **Roadmap Phase**: Accessibility (P1)

## Related Files
- `src/display/text_size.rs` - Text size manager implementation
- `wiki/Display-Text-Size.md` - Wiki documentation

## Principles Followed
SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY, LAYERED DESIGN,
KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, USER_FREEDOM_AND_CONTROL, ACCESSIBILITY,
USER_CUSTOMIZATION, CLEAR_ABSTRACTION, LEVEL_OF_ABSTRACTION
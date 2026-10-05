# Desktop Layout Manager

## Overview

The Desktop Layout Manager provides comprehensive workspace layout management inspired by Linux Mint's workspace management and Omarchy's layout utilities. It supports multiple layout types, gap configuration, border sizing, and default layout selection.

## Features

- **Layout Types**: Tiling, Stacking, Tabbed, Floating, Grid, Columns, Rows, Spiral
- **Layout Management**: Add, remove, enable, disable workspace layouts
- **Gap Configuration**: Inner and outer gap configuration
- **Border Size**: Configurable window border size
- **Default Layout**: Set and track default layout
- **Layout Filtering**: List layouts by type
- **Default Layouts**: Tiling (default), Floating, Tabbed, Grid
- **Statistics**: Track total, tiling, and floating layout counts

## Components

### DesktopLayoutType

```rust
pub enum DesktopLayoutType {
    Tiling,    // Tiling layout
    Stacking,  // Stacking layout
    Tabbed,    // Tabbed layout
    Floating,  // Floating layout
    Grid,      // Grid layout
    Columns,   // Columns layout
    Rows,      // Rows layout
    Spiral,    // Spiral layout
}
```

### WorkspaceLayout

Workspace layout structure with:
- Layout ID and name
- Layout type
- Gaps (inner, outer)
- Border size
- Default flag

### DesktopLayoutManager

Main management interface with:
- Layout registration and management
- Gap and border configuration
- Default layout management
- Layout filtering by type
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopLayoutManager;

let mut manager = DesktopLayoutManager::new();

// Get all layouts
let layouts = manager.get_layouts();
println!("Total layouts: {}", layouts.len());

// Get default layout
if let Some(layout) = manager.get_default_layout() {
    println!("Default: {}", layout.name);
}
```

### Layout Management

```rust
// Add a new layout
let id = manager.add_layout("Spiral".to_string(), DesktopLayoutType::Spiral);

// Get layout by ID
if let Some(layout) = manager.get_layout(&id) {
    println!("Layout: {}", layout.name);
}

// Remove layout
manager.remove_layout(&id);
```

### Gap and Border Configuration

```rust
// Set layout gaps (inner, outer)
manager.set_layout_gaps(&id, 16, 16);

// Set border size
manager.set_layout_border_size(&id, 4);
```

### Default Layout

```rust
// Set default layout
manager.set_default_layout(&id);

// Get default layout
if let Some(layout) = manager.get_default_layout() {
    println!("Default: {}", layout.name);
}
```

### Layout Filtering

```rust
// Get layouts by type
let tiling = manager.get_layouts_by_type(DesktopLayoutType::Tiling);
println!("Tiling layouts: {}", tiling.len());

let floating = manager.get_layouts_by_type(DesktopLayoutType::Floating);
println!("Floating layouts: {}", floating.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total layouts: {}", stats.total_layouts);
println!("Tiling layouts: {}", stats.tiling_layouts);
println!("Floating layouts: {}", stats.floating_layouts);
```

## Default Configuration

The Layout Manager includes default layouts:

- **Tiling**: Default layout, 8px inner/outer gaps, 2px border
- **Floating**: 8px inner/outer gaps, 2px border
- **Tabbed**: 8px inner/outer gaps, 2px border
- **Grid**: 8px inner/outer gaps, 2px border

Default layout: Tiling

## AI Agent Maintenance Instructions

When maintaining the Layout Manager:

1. **Gap Validation**: Ensure gap values are reasonable (minimum 0)
2. **Border Validation**: Ensure border sizes are reasonable (minimum 0)
3. **Default Protection**: Prevent removal of default layout without replacement
4. **Layout Switching**: Integrate with actual window manager for layout switching
5. **Per-Workspace Layouts**: Support per-workspace layout configuration
6. **Layout Persistence**: Save and restore layout configurations
7. **Backend Integration**: Integrate with actual window manager (i3, Sway, etc.)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::layout_manager
```

## Future Enhancements

- Integration with actual window manager (i3, Sway, etc.)
- Per-workspace layout configuration
- Layout switching shortcuts
- Custom layout templates
- Layout presets
- Layout animation transitions
- Per-application layout rules
- Multi-monitor layout support
- Layout statistics and metrics
- Layout export/import

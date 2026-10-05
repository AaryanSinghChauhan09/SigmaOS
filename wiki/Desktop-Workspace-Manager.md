# Desktop Workspace Manager

## Overview

The Desktop Workspace Manager provides comprehensive workspace management inspired by Linux Mint's workspace handling and Omarchy's workspace utilities. It supports multiple workspaces, workspace types, switching, and dynamic workspace count management.

## Features

- **Workspace Types**: Normal, Special, Scratchpad
- **Workspace Management**: Add, remove, enable, disable workspaces
- **Active Workspace**: Track and switch active workspace
- **Visibility**: Show/hide workspaces
- **Window Count**: Track number of windows per workspace
- **Workspace Switching**: Switch to next, previous, or by index
- **Dynamic Count**: Adjust number of workspaces dynamically
- **Workspace Filtering**: List workspaces by type
- **Default Workspaces**: 4 normal workspaces
- **Statistics**: Track total, active, normal, and visible workspace counts

## Components

### DesktopWorkspaceType

```rust
pub enum DesktopWorkspaceType {
    Normal,      // Normal workspace
    Special,     // Special workspace
    Scratchpad,  // Scratchpad workspace
}
```

### DesktopWorkspace

Workspace structure with:
- Workspace ID and name
- Workspace type
- Active status
- Visibility status
- Window count

### DesktopWorkspaceManager

Main management interface with:
- Workspace registration and management
- Active workspace tracking and switching
- Visibility control
- Window count tracking
- Workspace switching (next, previous, index)
- Dynamic workspace count adjustment
- Workspace filtering by type
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopWorkspaceManager;

let mut manager = DesktopWorkspaceManager::new();

// Get all workspaces
let workspaces = manager.get_workspaces();
println!("Total workspaces: {}", workspaces.len());

// Get active workspace
if let Some(workspace) = manager.get_active_workspace() {
    println!("Active: {}", workspace.name);
}
```

### Workspace Management

```rust
// Add a new workspace
let id = manager.add_workspace("Scratchpad".to_string(), DesktopWorkspaceType::Scratchpad);

// Get workspace by ID
if let Some(workspace) = manager.get_workspace(&id) {
    println!("Workspace: {}", workspace.name);
}

// Remove workspace
manager.remove_workspace(&id);
```

### Active Workspace

```rust
// Set active workspace
manager.set_active_workspace(&id);

// Get active workspace
if let Some(workspace) = manager.get_active_workspace() {
    println!("Active: {}", workspace.name);
}
```

### Workspace Switching

```rust
// Switch to next workspace
manager.switch_to_next();

// Switch to previous workspace
manager.switch_to_previous();

// Switch to workspace by index (0-based)
manager.switch_to_index(2);
```

### Visibility and Window Count

```rust
// Set workspace visibility
manager.set_workspace_visible(&id, false);

// Set window count
manager.set_workspace_window_count(&id, 5);
```

### Dynamic Workspace Count

```rust
// Set number of workspaces
manager.set_num_workspaces(6);
```

### Workspace Filtering

```rust
// Get workspaces by type
let normal = manager.get_workspaces_by_type(DesktopWorkspaceType::Normal);
println!("Normal workspaces: {}", normal.len());

// Get only visible workspaces
let visible = manager.get_visible_workspaces();
println!("Visible: {}", visible.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total workspaces: {}", stats.total_workspaces);
println!("Active workspace: {:?}", stats.active_workspace);
println!("Normal workspaces: {}", stats.normal_workspaces);
println!("Visible workspaces: {}", stats.visible_workspaces);
```

## Default Configuration

The Workspace Manager includes default workspaces:

- **Workspace 1**: Normal type, active
- **Workspace 2**: Normal type
- **Workspace 3**: Normal type
- **Workspace 4**: Normal type

Default settings:
- **Number of Workspaces**: 4

## AI Agent Maintenance Instructions

When maintaining the Workspace Manager:

1. **Active Protection**: Prevent removal of active workspace without switching
2. **Minimum Protection**: Ensure at least one normal workspace exists
3. **Window Tracking**: Integrate with actual window manager for window counts
4. **Switching Sync**: Sync workspace switching with actual window manager
5. **Dynamic Limits**: Ensure reasonable limits on workspace count
6. **Persistence**: Save and restore workspace configuration
7. **Backend Integration**: Integrate with actual window manager (i3, Sway, etc.)

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::workspace_manager
```

## Future Enhancements

- Integration with actual window manager (i3, Sway, etc.)
- Per-workspace layout configuration
- Workspace naming
- Workspace persistence
- Workspace-specific wallpaper
- Keyboard shortcuts for workspace switching
- Workspace indicators
- Multi-monitor workspace support
- Workspace drag and drop reordering

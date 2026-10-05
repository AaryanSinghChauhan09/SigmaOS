# Desktop Menu Manager

## Overview

The Desktop Menu Manager provides comprehensive application menu management inspired by Linux Mint's menu and Omarchy's menu utilities. It supports hierarchical menu structures, categories, applications, separators, and search functionality.

## Features

- **Menu Entry Types**: Application, Category, Separator, Submenu
- **Menu Structure**: Hierarchical categories with parent-child relationships
- **Categories**: Pre-configured categories (Accessories, Internet, Settings)
- **Applications**: Application entries with commands and icons
- **Icons**: Custom icon support for each entry
- **Commands**: Executable commands for applications
- **Visibility**: Show/hide menu entries
- **Search**: Search entries by name and command
- **Entry Filtering**: List entries by type or category
- **Default Menu**: Pre-configured with common applications
- **Statistics**: Track total entries, categories, applications, and visible entries

## Components

### MenuEntryType

```rust
pub enum MenuEntryType {
    Application,  // Application launcher
    Category,    // Menu category
    Separator,   // Menu separator
    Submenu,     // Nested submenu
}
```

### MenuEntry

Menu entry structure with:
- Entry ID and name
- Entry type
- Custom icon (optional)
- Command (optional)
- Parent category ID (optional)
- Visibility status

### MenuManager

Main management interface with:
- Entry and category registration
- Hierarchical menu structure
- Icon and command management
- Parent-child relationships
- Visibility control
- Search functionality
- Entry filtering by type and category
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::MenuManager;

let mut manager = MenuManager::new();

// Get all entries
let entries = manager.get_entries();
println!("Total entries: {}", entries.len());

// Get categories
let categories = manager.get_categories();
println!("Categories: {}", categories.len());
```

### Entry Management

```rust
// Add a new application entry
let id = manager.add_entry("Calculator".to_string(), MenuEntryType::Application);

// Get entry by ID
if let Some(entry) = manager.get_entry(&id) {
    println!("Entry: {}", entry.name);
}

// Remove entry
manager.remove_entry(&id);
```

### Category Management

```rust
// Add a new category
let cat_id = manager.add_category("Games".to_string());

// Get categories
let categories = manager.get_categories();
for cat in categories {
    println!("Category: {}", cat.name);
}
```

### Entry Configuration

```rust
// Set entry icon
manager.set_entry_icon(&id, "calculator".to_string());

// Set entry command
manager.set_entry_command(&id, "sigma-calculator".to_string());

// Set entry parent (category)
manager.set_entry_parent(&id, cat_id.clone());

// Set entry visibility
manager.set_entry_visible(&id, false);
```

### Entry Filtering

```rust
// Get entries by type
let applications = manager.get_entries_by_type(MenuEntryType::Application);
println!("Applications: {}", applications.len());

// Get entries by category
let cat_id = manager.get_categories()[0].id.clone();
let category_entries = manager.get_entries_by_category(&cat_id);
println!("Category entries: {}", category_entries.len());

// Get only visible entries
let visible = manager.get_visible_entries();
println!("Visible: {}", visible.len());
```

### Search

```rust
// Search entries by name or command
let results = manager.search_entries("terminal");
for result in results {
    println!("Found: {}", result.name);
}
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total entries: {}", stats.total_entries);
println!("Categories: {}", stats.categories);
println!("Applications: {}", stats.applications);
println!("Visible entries: {}", stats.visible_entries);
```

## Default Configuration

The Menu Manager includes default categories and applications:

**Categories:**
- **Accessories**: Terminal, Text Editor
- **Internet**: Web Browser
- **Settings**: System Settings

## AI Agent Maintenance Instructions

When maintaining the Menu Manager:

1. **Command Validation**: Ensure commands exist before setting
2. **Icon Validation**: Ensure icon names exist in icon theme
3. **Parent Validation**: Ensure parent IDs reference valid categories
4. **Circular References**: Prevent circular parent-child relationships
5. **Desktop Files**: Integrate with actual .desktop file system
6. **Menu Backend**: Integrate with actual menu backend (GNOME, KDE, XFCE)
7. **Hot Reload**: Support hot reload when .desktop files change

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::menu_manager
```

## Future Enhancements

- Integration with actual .desktop file system
- Frequent applications tracking
- Recent applications
- Favorites and bookmarks
- Menu customization (drag and drop)
- Menu themes and styling
- Keyboard navigation
- Menu search with fuzzy matching
- Context menu support
- Dynamic menu updates
- Per-user menu customization

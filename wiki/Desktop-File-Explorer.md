# Desktop File Explorer

## Overview

The Desktop File Explorer provides file management functionality inspired by Linux Mint's Nemo and Omarchy's file utilities. It supports file operations, navigation, bookmarks, and file type handling.

## Features

- **File Types**: Regular, Directory, Symlink, Block Device, Char Device, FIFO, Socket, Unknown
- **View Modes**: Icon, List, Compact, Tree views
- **Sort Orders**: Name, Size, Modified, Type sorting
- **Navigation**: Navigate to paths, navigate up, back/forward history
- **File Operations**: Create files/directories, delete, rename
- **Bookmarks**: Save and navigate to bookmarked locations
- **Hidden Files**: Show/hide hidden files
- **File Info**: Size, permissions, modification time, executable status
- **Human-Readable Sizes**: Automatic size formatting (B, KB, MB, GB)

## Components

### FileType

```rust
pub enum FileType {
    Regular,      // Regular file
    Directory,    // Directory
    Symlink,      // Symbolic link
    BlockDevice,  // Block device
    CharDevice,   // Character device
    Fifo,         // Named pipe
    Socket,       // Unix socket
    Unknown,      // Unknown type
}
```

### FileInfo

Represents a file with:
- Path and name
- File type
- Size in bytes
- Modification time
- Permissions
- Hidden flag
- Executable flag
- Human-readable size formatting

### DesktopViewMode

```rust
pub enum DesktopViewMode {
    Icon,     // Icon view
    List,     // List view
    Compact,  // Compact list view
    Tree,     // Tree view
}
```

### DesktopSortOrder

```rust
pub enum DesktopSortOrder {
    Name,      // Sort by name
    Size,      // Sort by size
    Modified,  // Sort by modification time
    Type,      // Sort by file type
}
```

### DesktopFileExplorerConfig

File explorer configuration including:
- Show hidden files
- View mode
- Sort order
- Sort reverse
- Single-click navigation
- Show thumbnails

### DesktopFileExplorer

Main management interface with:
- Path navigation (absolute paths)
- Navigate up/back/forward
- History tracking
- File listing with filtering
- File operations (create, delete, rename)
- Bookmark management
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::DesktopFileExplorer;

let mut explorer = DesktopFileExplorer::new();

// Get current path
let path = explorer.get_current_path();

// Get files in current directory
let files = explorer.get_files();

// Get configuration
let config = explorer.get_config();
```

### Navigation

```rust
// Navigate to path
explorer.navigate(PathBuf::from("/home/user/Documents"))?;

// Navigate up
explorer.navigate_up()?;

// Navigate back in history
explorer.navigate_back()?;

// Navigate forward in history
explorer.navigate_forward()?;
```

### File Operations

```rust
// Create directory
explorer.create_directory("NewFolder".to_string())?;

// Create file
explorer.create_file("newfile.txt".to_string())?;

// Delete file or directory
explorer.delete("oldfile.txt")?;

// Rename file
explorer.rename("oldname.txt", "newname.txt".to_string())?;
```

### Bookmark Management

```rust
// Add bookmark
explorer.add_bookmark("Projects".to_string(), PathBuf::from("/home/user/Projects"));

// Get bookmark
if let Some(path) = explorer.get_bookmark("Projects") {
    println!("Projects path: {}", path.display());
}

// List bookmarks
let bookmarks = explorer.list_bookmarks();

// Remove bookmark
explorer.remove_bookmark("Projects");
```

### Configuration

```rust
let config = DesktopFileExplorerConfig {
    show_hidden: true,
    view_mode: DesktopViewMode::List,
    sort_order: DesktopSortOrder::Modified,
    sort_reverse: true,
    single_click: true,
    show_thumbnails: false,
};

explorer.set_config(config);
```

### Statistics

```rust
let stats = explorer.get_statistics();
println!("Current path: {}", stats.current_path);
println!("Total files: {}", stats.total_files);
println!("Directories: {}", stats.directories);
println!("Total size: {}", stats.total_size);
```

## AI Agent Maintenance Instructions

When maintaining the Desktop File Explorer:

1. **Path Safety**: Ensure all path operations are validated and use absolute paths
2. **File Operations**: Maintain proper error handling for file operations
3. **History Management**: Ensure back/forward navigation works correctly
4. **Hidden Files**: Respect hidden file flag in file listing
5. **Bookmark Persistence**: Ensure bookmarks are properly saved and loaded
6. **Configuration**: Keep configuration defaults sensible and secure

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::desktop_file_manager
```

## Future Enhancements

- Integration with actual filesystem APIs
- File search functionality
- Copy/paste/cut operations
- Drag and drop support
- File preview
- File permissions editing
- Archive extraction
- Split view
- Tab support
- Cloud storage integration

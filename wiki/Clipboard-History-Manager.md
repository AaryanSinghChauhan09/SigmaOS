# Clipboard History Manager

## Overview

The Clipboard History Manager provides comprehensive clipboard management inspired by Linux Mint's clipboard and Omarchy's clipboard utilities. It supports text, image, file, and HTML content with configurable history and restoration capabilities.

## Features

- **Content Types**: Text, Image, Files, HTML
- **Clipboard History**: Configurable history size (default: 50 items)
- **Source Tracking**: Track which application copied each item
- **Timestamp**: Track when each item was copied
- **History Browsing**: View and browse clipboard history
- **Type Filtering**: Filter history by content type
- **Restore**: Restore items from history to current clipboard
- **Clear Operations**: Clear current clipboard, history, or both
- **Statistics**: Track item counts by type

## Components

### ClipboardItemType

```rust
pub enum ClipboardItemType {
    Text,   // Plain text
    Image,  // Image data (base64 encoded)
    Files,  // File paths (newline-separated)
    HTML,   // HTML content
}
```

### ClipboardItem

Represents a clipboard item with:
- Unique item ID
- Item type
- Content string
- Timestamp
- Source application

### ClipboardHistoryManager

Main management interface with:
- Copy operations for all content types
- Current clipboard access
- History browsing and filtering
- Restore from history
- Clear operations
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::ClipboardHistoryManager;

let mut manager = ClipboardHistoryManager::new();

// Copy text
manager.copy_text("Hello, World!".to_string(), "Terminal".to_string());

// Get current text
if let Some(text) = manager.get_text() {
    println!("Current text: {}", text);
}
```

### Copying Content

```rust
// Copy text
manager.copy_text("Important text".to_string(), "Text Editor".to_string());

// Copy image (base64 encoded)
manager.copy_image("iVBORw0KGgoAAAANS...".to_string(), "Image Viewer".to_string());

// Copy files (newline-separated paths)
manager.copy_files("/tmp/file1.txt\n/tmp/file2.txt".to_string(), "File Manager".to_string());

// Copy HTML
manager.copy_html("<b>Bold text</b>".to_string(), "Web Browser".to_string());
```

### Accessing Current Content

```rust
// Get current item
if let Some(item) = manager.get_current() {
    println!("Type: {:?}", item.item_type);
    println!("Source: {}", item.source);
    println!("Timestamp: {}", item.timestamp);
}

// Get specific types
if let Some(text) = manager.get_text() {
    println!("Text: {}", text);
}

if let Some(image) = manager.get_image() {
    println!("Image data: {}", image);
}

if let Some(files) = manager.get_files() {
    println!("Files: {}", files);
}
```

### History Management

```rust
// Copy multiple items
manager.copy_text("First".to_string(), "App1".to_string());
manager.copy_text("Second".to_string(), "App2".to_string());
manager.copy_text("Third".to_string(), "App3".to_string());

// Get full history
let history = manager.get_history();
println!("History size: {}", history.len());

// Get history by type
let text_history = manager.get_history_by_type(ClipboardItemType::Text);
println!("Text items: {}", text_history.len());
```

### Restoring from History

```rust
// Copy multiple items
manager.copy_text("Item 1".to_string(), "App1".to_string());
manager.copy_text("Item 2".to_string(), "App2".to_string());

// Get the first item's ID
let first_id = manager.get_history()[1].id.clone();

// Restore it to current clipboard
manager.restore(&first_id)?;
assert_eq!(manager.get_text(), Some("Item 1".to_string()));
```

### Clear Operations

```rust
// Clear current clipboard
manager.clear();

// Clear history
manager.clear_history();

// Clear everything
manager.clear_all();
```

### History Size Configuration

```rust
// Set max history size
manager.set_max_history(100);

// Copy many items
for i in 0..150 {
    manager.copy_text(format!("Item {}", i), "App".to_string());
}

// History should be limited to 100
assert_eq!(manager.get_history().len(), 100);
```

### Statistics

```rust
manager.copy_text("Text".to_string(), "App".to_string());
manager.copy_image("img".to_string(), "App".to_string());
manager.copy_files("/tmp/file".to_string(), "App".to_string());
manager.copy_html("<b>HTML</b>".to_string(), "App".to_string());

let stats = manager.get_statistics();
println!("Total items: {}", stats.total_items);
println!("Text: {}", stats.text_count);
println!("Images: {}", stats.image_count);
println!("Files: {}", stats.files_count);
println!("HTML: {}", stats.html_count);
println!("Max history: {}", stats.max_history);
println!("Has current: {}", stats.has_current);
```

## AI Agent Maintenance Instructions

When maintaining the Clipboard History Manager:

1. **Content Validation**: Validate content based on type (e.g., valid base64 for images)
2. **History Limits**: Enforce max history size to prevent memory exhaustion
3. **Source Tracking**: Track source applications accurately
4. **Timestamp Accuracy**: Use accurate timestamps for history ordering
5. **Type Safety**: Ensure type-specific getters return correct types
6. **Memory Management**: Clear old history items when limit is reached

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::clipboard_manager
```

## Future Enhancements

- Integration with actual system clipboard (X11, Wayland)
- Persistent history across reboots
- Search within clipboard history
- Clipboard synchronization across devices
- Pinned items (never removed from history)
- Image preview in history
- File icons in history
- Privacy mode (clear sensitive data)
- Clipboard actions (open file, search text)
- Per-application history limits

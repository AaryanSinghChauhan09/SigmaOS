# Desktop Recent Files Manager

## Overview

The Desktop Recent Files Manager provides comprehensive recent file tracking inspired by Linux Mint's recent files tracker and Omarchy's recent documents utility. It tracks recently accessed files, categorizes them by type, and provides filtering and statistics.

## Features

- **File Types**: Document, Image, Video, Audio, Archive, Code, Other
- **Automatic Type Detection**: Detect file type from extension
- **Recent File Tracking**: Track recently accessed files with timestamps
- **Application Tracking**: Track which application opened each file
- **Access Updates**: Update access timestamp when files are opened
- **File Filtering**: Filter recent files by type or application
- **Configurable Limit**: Configurable maximum number of recent files (default: 50)
- **Automatic Trimming**: Automatically remove oldest files when limit is exceeded
- **Statistics**: Track total, document, image, and video counts

## Components

### RecentFileType

```rust
pub enum RecentFileType {
    Document,  // Text documents
    Image,     // Image files
    Video,     // Video files
    Audio,     // Audio files
    Archive,   // Compressed archives
    Code,      // Source code files
    Other,     // Other file types
}
```

### RecentFileEntry

Recent file entry with:
- File ID and name
- File path
- File type
- Last accessed timestamp
- Application that opened the file

### RecentFilesManager

Main management interface with:
- Recent file addition with automatic type detection
- Recent file addition with explicit type
- Access timestamp updates
- Recent file removal
- Clear all recent files
- Filtering by file type
- Filtering by application
- Automatic trimming when limit exceeded
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::desktop::RecentFilesManager;

let mut manager = RecentFilesManager::new();

// Get configuration
println!("Max recent files: {}", manager.get_max_recent_files());
```

### Add Recent Files

```rust
// Add with automatic type detection
let id = manager.add_recent_file(
    "document.txt".to_string(),
    PathBuf::from("/home/user/document.txt"),
    Some("TextEditor".to_string()),
);

// Add with explicit type
let id = manager.add_recent_file_with_type(
    "script.py".to_string(),
    PathBuf::from("/home/user/script.py"),
    RecentFileType::Code,
    Some("IDE".to_string()),
);
```

### Configuration

```rust
// Set maximum recent files
manager.set_max_recent_files(100);
```

### File Management

```rust
// Update file access time
manager.update_file_access(&id);

// Remove recent file
manager.remove_recent_file(&id);

// Clear all recent files
manager.clear_recent_files();
```

### Recent File Filtering

```rust
// Get all recent files (sorted by access time)
let files = manager.get_recent_files();
println!("Total recent files: {}", files.len());

// Get files by type
let documents = manager.get_recent_files_by_type(RecentFileType::Document);
println!("Documents: {}", documents.len());

let images = manager.get_recent_files_by_type(RecentFileType::Image);
println!("Images: {}", images.len());

// Get files by application
let editor_files = manager.get_recent_files_by_application("TextEditor");
println!("Files opened by TextEditor: {}", editor_files.len());
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total files: {}", stats.total_files);
println!("Max files: {}", stats.max_files);
println!("Documents: {}", stats.documents);
println!("Images: {}", stats.images);
println!("Videos: {}", stats.videos);
```

## File Type Detection

The manager automatically detects file types from extensions:

| Extension | Type |
|-----------|------|
| txt, doc, docx, pdf, odt, rtf, md | Document |
| png, jpg, jpeg, gif, bmp, svg, webp | Image |
| mp4, avi, mkv, mov, webm, flv | Video |
| mp3, wav, ogg, flac, aac, m4a | Audio |
| zip, tar, gz, rar, 7z, xz | Archive |
| rs, py, js, ts, c, cpp, h, java, go, sh | Code |

## Default Configuration

The Recent Files Manager includes default configuration:

- **Max Recent Files**: 50
- **Automatic Trimming**: Enabled
- **Type Detection**: Automatic

## AI Agent Maintenance Instructions

When maintaining the Recent Files Manager:

1. **Type Detection**: Extend `from_extension` method for new file types
2. **Application Tracking**: Integrate with actual application launchers
3. **Persistence**: Implement persistent storage for recent files
4. **Privacy**: Add privacy controls to exclude certain directories
5. **Access Monitoring**: Integrate with actual file access monitoring
6. **Cleanup**: Implement automatic cleanup of old/unavailable files
7. **Thumbnails**: Add thumbnail generation for image/video files

## Testing

Run the unit tests with:

```bash
cargo test --lib desktop::recent_files_manager
```

## Future Enhancements

- Persistent storage across sessions
- Thumbnail generation and caching
- Privacy controls and exclusion lists
- Automatic cleanup of unavailable files
- Search within recent files
- Recent files sync across devices
- Integration with cloud storage
- File preview in recent files dialog
- Keyboard shortcuts for quick access

//! File Manager (Nautilus/Thunar Inspiration)
//! File navigation, operations, and file properties

#![no_std]

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

const MAX_CLIPBOARD_ITEMS: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    RegularFile,
    Directory,
}

#[derive(Debug, Clone, Copy)]
pub struct FileEntry {
    pub inode_id: u32,
    pub name_hash: u32,
    pub size: u32,
    pub file_type: FileType,
}

pub struct Pane {
    pub current_directory_inode: u32,
    pub entries: [Option<FileEntry>; 16],
    pub selected_idx: usize,
}

impl Pane {
    pub fn new(root_inode: u32) -> Self {
        const EMPTY_ENTRY: Option<FileEntry> = None;
        Self {
            current_directory_inode: root_inode,
            entries: [EMPTY_ENTRY; 16],
            selected_idx: 0,
        }
    }
}

pub struct ClipboardBuffer {
    pub items: [Option<FileEntry>; MAX_CLIPBOARD_ITEMS],
    pub is_cut: bool,
}

impl ClipboardBuffer {
    pub fn new() -> Self {
        const EMPTY_ENTRY: Option<FileEntry> = None;
        Self {
            items: [EMPTY_ENTRY; MAX_CLIPBOARD_ITEMS],
            is_cut: false,
        }
    }

    pub fn clear(&mut self) {
        self.items.fill(None);
        self.is_cut = false;
    }
}

impl Default for ClipboardBuffer {
    fn default() -> Self {
        Self::new()
    }
}

/// Freedesktop & BSD Compliant Thumbnail Size Specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThumbnailSpecSize {
    Normal = 128,  // 128x128 (Normal)
    Large = 256,   // 256x256 (Large)
    XLarge = 512,  // 512x512 (X-Large)
    XXLarge = 1024 // 1024x1024 (XX-Large)
}

/// Thumbnail Media Format / Handler Category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThumbnailFormat {
    RasterImage,   // PNG, JPEG, WEBP, GIF (Linux gdk-pixbuf / BSD imlib2)
    VectorSvg,     // SVG (Linux librsvg / BSD resvg)
    VideoFrame,    // MP4, MKV, AVI, WEBM (Linux Tumbler / BSD ffmpegthumbnailer)
    PdfDocument,   // PDF, PS, EPS (Linux Poppler / BSD libgs)
    FontPreview,   // TTF, OTF, WOFF (Linux fontconfig / BSD freetype2)
    AudioAlbumArt, // MP3, FLAC, OGG (Linux TagLib / BSD id3lib)
    Unknown,
}

/// Cached Thumbnail Entry with PNG metadata
#[derive(Debug, Clone)]
pub struct ThumbnailEntry {
    pub source_uri: String,
    pub source_mtime: u64,
    pub size_spec: ThumbnailSpecSize,
    pub cache_path: String,
    pub format: ThumbnailFormat,
    pub width: u32,
    pub height: u32,
}

/// Cache Store for Desktop File Manager Thumbnails (~/.cache/thumbnails)
#[derive(Debug, Clone)]
pub struct ThumbnailCache {
    pub cached_entries: Vec<ThumbnailEntry>,
    pub base_cache_dir: String,
}

impl ThumbnailCache {
    pub fn new() -> Self {
        Self {
            cached_entries: Vec::new(),
            base_cache_dir: "/home/user/.cache/thumbnails".to_string(),
        }
    }

    pub fn get_cache_path(&self, uri: &str, size: ThumbnailSpecSize) -> String {
        let size_folder = match size {
            ThumbnailSpecSize::Normal => "normal",
            ThumbnailSpecSize::Large => "large",
            ThumbnailSpecSize::XLarge => "x-large",
            ThumbnailSpecSize::XXLarge => "xx-large",
        };
        // Simple hash calculation simulating MD5/SHA256 URL hashing
        let mut hash: u64 = 5381;
        for byte in uri.bytes() {
            hash = ((hash << 5).wrapping_add(hash)).wrapping_add(byte as u64);
        }
        alloc::format!("{}/{}/{:x}.png", self.base_cache_dir, size_folder, hash)
    }

    pub fn lookup(&self, uri: &str, mtime: u64, size: ThumbnailSpecSize) -> Option<&ThumbnailEntry> {
        self.cached_entries.iter().find(|entry| {
            entry.source_uri == uri && entry.source_mtime == mtime && entry.size_spec == size
        })
    }

    pub fn store(&mut self, entry: ThumbnailEntry) {
        self.cached_entries.retain(|e| !(e.source_uri == entry.source_uri && e.size_spec == entry.size_spec));
        self.cached_entries.push(entry);
    }
}

impl Default for ThumbnailCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Linux (Tumbler/Freedesktop) & BSD (ffmpegthumbnailer) Inspired Thumbnailer Engine
pub struct ThumbnailerEngine {
    pub cache: ThumbnailCache,
}

impl ThumbnailerEngine {
    pub fn new() -> Self {
        Self {
            cache: ThumbnailCache::new(),
        }
    }

    pub fn detect_format(&self, file_path: &str) -> ThumbnailFormat {
        if file_path.ends_with(".png") || file_path.ends_with(".jpg") || file_path.ends_with(".jpeg") || file_path.ends_with(".webp") {
            ThumbnailFormat::RasterImage
        } else if file_path.ends_with(".svg") {
            ThumbnailFormat::VectorSvg
        } else if file_path.ends_with(".mp4") || file_path.ends_with(".mkv") || file_path.ends_with(".webm") || file_path.ends_with(".avi") {
            ThumbnailFormat::VideoFrame
        } else if file_path.ends_with(".pdf") || file_path.ends_with(".ps") {
            ThumbnailFormat::PdfDocument
        } else if file_path.ends_with(".ttf") || file_path.ends_with(".otf") {
            ThumbnailFormat::FontPreview
        } else if file_path.ends_with(".mp3") || file_path.ends_with(".flac") || file_path.ends_with(".ogg") {
            ThumbnailFormat::AudioAlbumArt
        } else {
            ThumbnailFormat::Unknown
        }
    }

    pub fn generate_thumbnail(&mut self, file_path: &str, mtime: u64, size: ThumbnailSpecSize) -> ThumbnailEntry {
        let uri = alloc::format!("file://{}", file_path);
        if let Some(cached) = self.cache.lookup(&uri, mtime, size) {
            return cached.clone();
        }

        let format = self.detect_format(file_path);
        let cache_path = self.cache.get_cache_path(&uri, size);
        let max_dim = size as u32;

        let entry = ThumbnailEntry {
            source_uri: uri,
            source_mtime: mtime,
            size_spec: size,
            cache_path,
            format,
            width: max_dim,
            height: max_dim,
        };

        self.cache.store(entry.clone());
        entry
    }
}

impl Default for ThumbnailerEngine {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SovereignFileManager {
    pub active_pane: Pane,
    pub clipboard: ClipboardBuffer,
    pub thumbnailer: ThumbnailerEngine,
}

impl SovereignFileManager {
    pub fn new() -> Self {
        Self {
            active_pane: Pane::new(0),
            clipboard: ClipboardBuffer::new(),
            thumbnailer: ThumbnailerEngine::new(),
        }
    }
}

impl Default for SovereignFileManager {
    fn default() -> Self {
        Self::new()
    }
}

/// File
#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_directory: bool,
    pub is_hidden: bool,
}

impl File {
    pub fn new(name: &str, path: &str, is_directory: bool) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            size: 0,
            is_directory,
            is_hidden: name.starts_with('.'),
        }
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }
}

/// Clipboard
#[derive(Debug, Clone)]
pub struct Clipboard {
    pub files: Vec<String>,
    pub operation: ClipboardOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClipboardOperation {
    Copy,
    Cut,
}

impl Clipboard {
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            operation: ClipboardOperation::Copy,
        }
    }

    pub fn add_file(&mut self, path: &str) {
        self.files.push(path.to_string());
    }

    pub fn set_operation(&mut self, operation: ClipboardOperation) {
        self.operation = operation;
    }

    pub fn clear(&mut self) {
        self.files.clear();
    }
}

impl Default for Clipboard {
    fn default() -> Self {
        Self::new()
    }
}

/// File manager
pub struct FileManager {
    pub current_directory: String,
    pub selected_files: Vec<File>,
    pub clipboard: Clipboard,
    pub bookmarks: Vec<String>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            current_directory: "/home/user".to_string(),
            selected_files: Vec::new(),
            clipboard: Clipboard::new(),
            bookmarks: vec![
                "/home/user".to_string(),
                "/".to_string(),
                "/tmp".to_string(),
            ],
        }
    }

    pub fn navigate(&mut self, path: &str) {
        self.current_directory = path.to_string();
    }

    pub fn select_file(&mut self, file: File) {
        self.selected_files.push(file);
    }

    pub fn clear_selection(&mut self) {
        self.selected_files.clear();
    }

    pub fn copy_files(&mut self) {
        self.clipboard.set_operation(ClipboardOperation::Copy);
        for file in &self.selected_files {
            self.clipboard.add_file(&file.path);
        }
    }

    pub fn cut_files(&mut self) {
        self.clipboard.set_operation(ClipboardOperation::Cut);
        for file in &self.selected_files {
            self.clipboard.add_file(&file.path);
        }
    }

    pub fn paste(&mut self) -> Result<(), FMError> {
        Ok(())
    }

    pub fn create_file(&mut self, _name: &str) -> Result<(), FMError> {
        Ok(())
    }

    pub fn create_directory(&mut self, _name: &str) -> Result<(), FMError> {
        Ok(())
    }

    pub fn delete_files(&mut self) -> Result<(), FMError> {
        Ok(())
    }

    pub fn rename_file(&mut self, _old_name: &str, _new_name: &str) -> Result<(), FMError> {
        Ok(())
    }

    pub fn add_bookmark(&mut self, path: &str) {
        self.bookmarks.push(path.to_string());
    }

    pub fn remove_bookmark(&mut self, path: &str) {
        self.bookmarks.retain(|b| b != path);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FMError {
    FileNotFound,
    PermissionDenied,
    OperationFailed,
}

impl Default for FileManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sovereign_file_manager() {
        let mut sfm = SovereignFileManager::new();
        assert_eq!(sfm.active_pane.current_directory_inode, 0);
        sfm.clipboard.is_cut = true;
        assert!(sfm.clipboard.is_cut);

        let thumb = sfm.thumbnailer.generate_thumbnail("/home/user/video.mp4", 1700000000, ThumbnailSpecSize::Normal);
        assert_eq!(thumb.format, ThumbnailFormat::VideoFrame);
        assert_eq!(thumb.width, 128);
    }

    #[test]
    fn test_thumbnailer_engine_formats_and_cache() {
        let mut engine = ThumbnailerEngine::new();
        assert_eq!(engine.detect_format("/home/user/pic.png"), ThumbnailFormat::RasterImage);
        assert_eq!(engine.detect_format("/home/user/doc.pdf"), ThumbnailFormat::PdfDocument);
        assert_eq!(engine.detect_format("/home/user/font.ttf"), ThumbnailFormat::FontPreview);
        assert_eq!(engine.detect_format("/home/user/song.mp3"), ThumbnailFormat::AudioAlbumArt);

        let t1 = engine.generate_thumbnail("/home/user/pic.png", 100, ThumbnailSpecSize::Large);
        assert_eq!(t1.size_spec, ThumbnailSpecSize::Large);
        assert_eq!(t1.width, 256);

        let t2 = engine.generate_thumbnail("/home/user/pic.png", 100, ThumbnailSpecSize::Large);
        assert_eq!(t2.cache_path, t1.cache_path);
    }

    #[test]
    fn test_file() {
        let file = File::new("test.txt", "/home/user/test.txt", false);
        assert_eq!(file.name, "test.txt");
    }

    #[test]
    fn test_clipboard() {
        let mut clipboard = Clipboard::new();
        clipboard.add_file("/test/file.txt");
        assert_eq!(clipboard.files.len(), 1);
    }

    #[test]
    fn test_file_manager() {
        let mut fm = FileManager::new();
        fm.navigate("/tmp");
        assert_eq!(fm.current_directory, "/tmp");
    }
}

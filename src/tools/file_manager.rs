//! File Manager (Nautilus/Thunar Inspiration)
//! File navigation, operations, and file properties
extern crate alloc;



use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::vec;

const MAX_CLIPBOARD_ITEMS: usize = 16;

/// Linux & BSD distro-inspired folder color accent choices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FolderColor {
    /// Ubuntu Yaru Orange (#E95420)
    UbuntuOrange,
    /// Pop!_OS Teal (#48B9C7)
    PopTeal,
    /// Linux Mint Aqua (#2A9D8F)
    MintGreen,
    /// Fedora Adwaita Blue (#3584E4)
    FedoraBlue,
    /// ElementaryOS Strawberry Red (#E74C3C)
    ElementaryRed,
    /// FreeBSD Daemon Crimson (#AB1212)
    FreeBsdCrimson,
    /// OpenBSD Onyx (#222222)
    OpenBsdOnyx,
    /// Custom RGB Color
    CustomRgb(u8, u8, u8),
}

impl FolderColor {
    /// Returns HEX string representation of folder color
    pub fn to_hex(&self) -> String {
        match self {
            FolderColor::UbuntuOrange => "#E95420".to_string(),
            FolderColor::PopTeal => "#48B9C7".to_string(),
            FolderColor::MintGreen => "#2A9D8F".to_string(),
            FolderColor::FedoraBlue => "#3584E4".to_string(),
            FolderColor::ElementaryRed => "#E74C3C".to_string(),
            FolderColor::FreeBsdCrimson => "#AB1212".to_string(),
            FolderColor::OpenBsdOnyx => "#222222".to_string(),
            FolderColor::CustomRgb(r, g, b) => alloc::format!("#{:02X}{:02X}{:02X}", r, g, b),
        }
    }

    /// Returns RGB tuple (u8, u8, u8)
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        match self {
            FolderColor::UbuntuOrange => (0xE9, 0x54, 0x20),
            FolderColor::PopTeal => (0x48, 0xB9, 0xC7),
            FolderColor::MintGreen => (0x2A, 0x9D, 0x8F),
            FolderColor::FedoraBlue => (0x35, 0x84, 0xE4),
            FolderColor::ElementaryRed => (0xE7, 0x4C, 0x3C),
            FolderColor::FreeBsdCrimson => (0xAB, 0x12, 0x12),
            FolderColor::OpenBsdOnyx => (0x22, 0x22, 0x22),
            FolderColor::CustomRgb(r, g, b) => (*r, *g, *b),
        }
    }
}

/// Pre-configured distro folder color themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroFolderColorPreset {
    UbuntuYaru,
    PopOs,
    LinuxMint,
    FedoraAdwaita,
    ElementaryOs,
    BsdHardened,
}

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

pub struct SovereignFileManager {
    pub active_pane: Pane,
    pub clipboard: ClipboardBuffer,
}

impl SovereignFileManager {
    pub fn new() -> Self {
        Self {
            active_pane: Pane::new(0),
            clipboard: ClipboardBuffer::new(),
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
    pub color_tag: Option<String>,
}

impl File {
    pub fn new(name: &str, path: &str, is_directory: bool) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            size: 0,
            is_directory,
            is_hidden: name.starts_with('.'),
            color_tag: None,
        }
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_color_tag(&mut self, color: FolderColor) {
        self.color_tag = Some(color.to_hex());
    }

    pub fn clear_color_tag(&mut self) {
        self.color_tag = None;
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
    pub folder_colors: BTreeMap<String, String>,
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
            folder_colors: BTreeMap::new(),
        }
    }

    /// Sets custom folder accent color for directory path
    pub fn set_folder_color(&mut self, path: &str, color: FolderColor) {
        self.folder_colors.insert(path.to_string(), color.to_hex());
    }

    /// Retrieves assigned folder color hex for directory path
    pub fn get_folder_color(&self, path: &str) -> Option<&String> {
        self.folder_colors.get(path)
    }

    /// Removes custom folder color assignment
    pub fn remove_folder_color(&mut self, path: &str) {
        self.folder_colors.remove(path);
    }

    /// Applies a distro-inspired folder theme preset across default system folders
    pub fn apply_distro_folder_theme(&mut self, preset: DistroFolderColorPreset) {
        let default_color = match preset {
            DistroFolderColorPreset::UbuntuYaru => FolderColor::UbuntuOrange,
            DistroFolderColorPreset::PopOs => FolderColor::PopTeal,
            DistroFolderColorPreset::LinuxMint => FolderColor::MintGreen,
            DistroFolderColorPreset::FedoraAdwaita => FolderColor::FedoraBlue,
            DistroFolderColorPreset::ElementaryOs => FolderColor::ElementaryRed,
            DistroFolderColorPreset::BsdHardened => FolderColor::FreeBsdCrimson,
        };

        for bookmark in self.bookmarks.clone() {
            self.set_folder_color(&bookmark, default_color);
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

    #[test]
    fn test_folder_color_switcher_and_distro_presets() {
        let mut fm = FileManager::new();

        // 1. Set custom folder color
        fm.set_folder_color("/home/user/Documents", FolderColor::UbuntuOrange);
        assert_eq!(
            fm.get_folder_color("/home/user/Documents"),
            Some(&"#E95420".to_string())
        );

        // 2. Custom RGB
        fm.set_folder_color("/home/user/Custom", FolderColor::CustomRgb(100, 150, 200));
        assert_eq!(
            fm.get_folder_color("/home/user/Custom"),
            Some(&"#6496C8".to_string())
        );

        // 3. Remove folder color
        fm.remove_folder_color("/home/user/Custom");
        assert_eq!(fm.get_folder_color("/home/user/Custom"), None);

        // 4. File color tags
        let mut dir = File::new("Projects", "/home/user/Projects", true);
        dir.set_color_tag(FolderColor::PopTeal);
        assert_eq!(dir.color_tag, Some("#48B9C7".to_string()));
        dir.clear_color_tag();
        assert_eq!(dir.color_tag, None);

        // 5. Distro theme preset
        fm.apply_distro_folder_theme(DistroFolderColorPreset::LinuxMint);
        assert_eq!(
            fm.get_folder_color("/home/user"),
            Some(&"#2A9D8F".to_string())
        );
        assert_eq!(
            FolderColor::MintGreen.to_rgb(),
            (0x2A, 0x9D, 0x8F)
        );
    }
}

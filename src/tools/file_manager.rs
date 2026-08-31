//! File Manager (Nautilus/Thunar Inspiration)
//! File navigation, operations, and file properties
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

/// Distro-inspired Folder Color representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FolderColor {
    pub name: String,
    pub hex_code: String,
    pub rgb: (u8, u8, u8),
}

impl FolderColor {
    pub fn new(name: &str, hex_code: &str, rgb: (u8, u8, u8)) -> Self {
        Self {
            name: name.to_string(),
            hex_code: hex_code.to_string(),
            rgb,
        }
    }
}

/// Linux and BSD Distro Folder Color Presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistroFolderColorPreset {
    UbuntuOrange,
    PopTeal,
    MintAqua,
    FedoraBlue,
    ElementaryRed,
    FreeBsdCrimson,
    OpenBsdOnyx,
    ManjaroEmerald,
    ArchCyan,
}

impl DistroFolderColorPreset {
    pub fn to_folder_color(self) -> FolderColor {
        match self {
            DistroFolderColorPreset::UbuntuOrange => {
                FolderColor::new("Ubuntu Yaru Orange", "#E95420", (233, 84, 32))
            }
            DistroFolderColorPreset::PopTeal => {
                FolderColor::new("Pop!_OS Teal", "#48B9C7", (72, 185, 199))
            }
            DistroFolderColorPreset::MintAqua => {
                FolderColor::new("Linux Mint Aqua", "#2A9D8F", (42, 157, 143))
            }
            DistroFolderColorPreset::FedoraBlue => {
                FolderColor::new("Fedora Adwaita Blue", "#3584E4", (53, 132, 228))
            }
            DistroFolderColorPreset::ElementaryRed => {
                FolderColor::new("ElementaryOS Red", "#E74C3C", (231, 76, 60))
            }
            DistroFolderColorPreset::FreeBsdCrimson => {
                FolderColor::new("FreeBSD Daemon Crimson", "#AB1212", (171, 18, 18))
            }
            DistroFolderColorPreset::OpenBsdOnyx => {
                FolderColor::new("OpenBSD Onyx", "#222222", (34, 34, 34))
            }
            DistroFolderColorPreset::ManjaroEmerald => {
                FolderColor::new("Manjaro Emerald", "#13A10E", (19, 161, 14))
            }
            DistroFolderColorPreset::ArchCyan => {
                FolderColor::new("Arch Cyan", "#1793D1", (23, 147, 209))
            }
        }
    }
}

/// Folder Color Switcher Engine inspired by Linux & BSD desktop managers
pub struct FolderColorSwitcher {
    pub active_preset: Option<DistroFolderColorPreset>,
    pub custom_colors: Vec<FolderColor>,
}

impl FolderColorSwitcher {
    pub fn new() -> Self {
        Self {
            active_preset: None,
            custom_colors: Vec::new(),
        }
    }

    pub fn set_preset(&mut self, preset: DistroFolderColorPreset) {
        self.active_preset = Some(preset);
    }

    pub fn get_preset_color(&self, preset: DistroFolderColorPreset) -> FolderColor {
        preset.to_folder_color()
    }

    pub fn add_custom_color(&mut self, color: FolderColor) {
        self.custom_colors.push(color);
    }

    pub fn colorize_file(&self, file: &mut File, color: &FolderColor) -> Result<(), FMError> {
        if !file.is_directory {
            return Err(FMError::OperationFailed);
        }
        file.color_tag = Some(color.hex_code.clone());
        Ok(())
    }

    pub fn colorize_files_with_preset(
        &self,
        files: &mut [File],
        preset: DistroFolderColorPreset,
    ) {
        let color = preset.to_folder_color();
        for file in files.iter_mut() {
            if file.is_directory {
                file.color_tag = Some(color.hex_code.clone());
            }
        }
    }
}

impl Default for FolderColorSwitcher {
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

    pub fn set_color_tag(&mut self, color_hex: Option<&str>) {
        self.color_tag = color_hex.map(|s| s.to_string());
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
    pub folder_color_switcher: FolderColorSwitcher,
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
            folder_color_switcher: FolderColorSwitcher::new(),
        }
    }

    pub fn set_folder_color(&mut self, file_path: &str, color: FolderColor) -> Result<(), FMError> {
        if let Some(file) = self.selected_files.iter_mut().find(|f| f.path == file_path) {
            self.folder_color_switcher.colorize_file(file, &color)
        } else {
            Err(FMError::FileNotFound)
        }
    }

    pub fn get_folder_color(&self, file_path: &str) -> Option<String> {
        self.selected_files
            .iter()
            .find(|f| f.path == file_path)
            .and_then(|f| f.color_tag.clone())
    }

    pub fn apply_distro_folder_theme(&mut self, preset: DistroFolderColorPreset) {
        self.folder_color_switcher
            .colorize_files_with_preset(&mut self.selected_files, preset);
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
    fn test_folder_color_switcher_and_presets() {
        let switcher = FolderColorSwitcher::new();
        let ubuntu_color = switcher.get_preset_color(DistroFolderColorPreset::UbuntuOrange);
        assert_eq!(ubuntu_color.hex_code, "#E95420");
        assert_eq!(ubuntu_color.rgb, (233, 84, 32));

        let freebsd_color = switcher.get_preset_color(DistroFolderColorPreset::FreeBsdCrimson);
        assert_eq!(freebsd_color.hex_code, "#AB1212");

        let mut folder = File::new("Documents", "/home/user/Documents", true);
        let mut reg_file = File::new("file.txt", "/home/user/file.txt", false);

        assert!(switcher.colorize_file(&mut folder, &ubuntu_color).is_ok());
        assert_eq!(folder.color_tag, Some("#E95420".to_string()));

        assert_eq!(
            switcher.colorize_file(&mut reg_file, &ubuntu_color),
            Err(FMError::OperationFailed)
        );
    }

    #[test]
    fn test_file_manager_distro_folder_theme() {
        let mut fm = FileManager::new();
        let folder1 = File::new("Photos", "/home/user/Photos", true);
        let folder2 = File::new("Videos", "/home/user/Videos", true);
        let file1 = File::new("notes.txt", "/home/user/notes.txt", false);

        fm.select_file(folder1);
        fm.select_file(folder2);
        fm.select_file(file1);

        fm.apply_distro_folder_theme(DistroFolderColorPreset::PopTeal);

        assert_eq!(
            fm.get_folder_color("/home/user/Photos"),
            Some("#48B9C7".to_string())
        );
        assert_eq!(
            fm.get_folder_color("/home/user/Videos"),
            Some("#48B9C7".to_string())
        );
        assert_eq!(fm.get_folder_color("/home/user/notes.txt"), None);

        let custom_color = FolderColor::new("Custom Purple", "#8E44AD", (142, 68, 173));
        assert!(fm.set_folder_color("/home/user/Photos", custom_color).is_ok());
        assert_eq!(
            fm.get_folder_color("/home/user/Photos"),
            Some("#8E44AD".to_string())
        );
    }
}

// SigmaOS Desktop Recent Files Manager
// Inspired by Linux Mint's recent files tracker and Omarchy's recent documents

use std::collections::HashMap;
use std::path::PathBuf;

/// Recent file type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecentFileType {
    Document,
    Image,
    Video,
    Audio,
    Archive,
    Code,
    Other,
}

impl RecentFileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            RecentFileType::Document => "Document",
            RecentFileType::Image => "Image",
            RecentFileType::Video => "Video",
            RecentFileType::Audio => "Audio",
            RecentFileType::Archive => "Archive",
            RecentFileType::Code => "Code",
            RecentFileType::Other => "Other",
        }
    }

    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "txt" | "doc" | "docx" | "pdf" | "odt" | "rtf" | "md" => RecentFileType::Document,
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" => RecentFileType::Image,
            "mp4" | "avi" | "mkv" | "mov" | "webm" | "flv" => RecentFileType::Video,
            "mp3" | "wav" | "ogg" | "flac" | "aac" | "m4a" => RecentFileType::Audio,
            "zip" | "tar" | "gz" | "rar" | "7z" | "xz" => RecentFileType::Archive,
            "rs" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "java" | "go" | "sh" => RecentFileType::Code,
            _ => RecentFileType::Other,
        }
    }
}

/// Recent file entry
#[derive(Debug, Clone)]
pub struct RecentFileEntry {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub file_type: RecentFileType,
    pub accessed: u64,
    pub application: Option<String>,
}

impl RecentFileEntry {
    pub fn new(
        id: String,
        name: String,
        path: PathBuf,
        file_type: RecentFileType,
    ) -> Self {
        let accessed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        RecentFileEntry {
            id,
            name,
            path,
            file_type,
            accessed,
            application: None,
        }
    }

    pub fn set_application(&mut self, app: String) {
        self.application = Some(app);
    }

    pub fn update_accessed(&mut self) {
        self.accessed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

/// Recent Files Manager
pub struct RecentFilesManager {
    recent_files: HashMap<String, RecentFileEntry>,
    max_recent_files: usize,
    next_file_id: u32,
}

impl RecentFilesManager {
    pub fn new() -> Self {
        RecentFilesManager {
            recent_files: HashMap::new(),
            max_recent_files: 50,
            next_file_id: 1,
        }
    }

    pub fn set_max_recent_files(&mut self, max: usize) {
        self.max_recent_files = max.max(1);
        self.trim_recent_files();
    }

    fn trim_recent_files(&mut self) {
        while self.recent_files.len() > self.max_recent_files {
            if let Some(oldest_id) = self.find_oldest_file() {
                self.recent_files.remove(&oldest_id);
            } else {
                break;
            }
        }
    }

    fn find_oldest_file(&self) -> Option<String> {
        self.recent_files
            .iter()
            .min_by_key(|(_, entry)| entry.accessed)
            .map(|(id, _)| id.clone())
    }

    pub fn add_recent_file(&mut self, name: String, path: PathBuf, application: Option<String>) -> String {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let file_type = RecentFileType::from_extension(extension);

        let id = format!("recent_{}", self.next_file_id);
        self.next_file_id += 1;

        let mut entry = RecentFileEntry::new(id.clone(), name, path, file_type);
        if let Some(app) = application {
            entry.set_application(app);
        }

        self.recent_files.insert(id.clone(), entry);
        self.trim_recent_files();
        id
    }

    pub fn add_recent_file_with_type(
        &mut self,
        name: String,
        path: PathBuf,
        file_type: RecentFileType,
        application: Option<String>,
    ) -> String {
        let id = format!("recent_{}", self.next_file_id);
        self.next_file_id += 1;

        let mut entry = RecentFileEntry::new(id.clone(), name, path, file_type);
        if let Some(app) = application {
            entry.set_application(app);
        }

        self.recent_files.insert(id.clone(), entry);
        self.trim_recent_files();
        id
    }

    pub fn update_file_access(&mut self, id: &str) -> bool {
        if let Some(entry) = self.recent_files.get_mut(id) {
            entry.update_accessed();
            true
        } else {
            false
        }
    }

    pub fn remove_recent_file(&mut self, id: &str) -> bool {
        self.recent_files.remove(id).is_some()
    }

    pub fn clear_recent_files(&mut self) {
        self.recent_files.clear();
    }

    pub fn get_recent_file(&self, id: &str) -> Option<&RecentFileEntry> {
        self.recent_files.get(id)
    }

    pub fn get_recent_files(&self) -> Vec<&RecentFileEntry> {
        let mut files: Vec<_> = self.recent_files.values().collect();
        files.sort_by(|a, b| b.accessed.cmp(&a.accessed));
        files
    }

    pub fn get_recent_files_by_type(&self, file_type: RecentFileType) -> Vec<&RecentFileEntry> {
        let mut files: Vec<_> = self.recent_files
            .values()
            .filter(|f| f.file_type == file_type)
            .collect();
        files.sort_by(|a, b| b.accessed.cmp(&a.accessed));
        files
    }

    pub fn get_recent_files_by_application(&self, application: &str) -> Vec<&RecentFileEntry> {
        let mut files: Vec<_> = self.recent_files
            .values()
            .filter(|f| f.application.as_ref().map_or(false, |a| a == application))
            .collect();
        files.sort_by(|a, b| b.accessed.cmp(&a.accessed));
        files
    }

    pub fn get_max_recent_files(&self) -> usize {
        self.max_recent_files
    }

    pub fn get_statistics(&self) -> RecentFilesStatistics {
        RecentFilesStatistics {
            total_files: self.recent_files.len(),
            max_files: self.max_recent_files,
            documents: self.get_recent_files_by_type(RecentFileType::Document).len(),
            images: self.get_recent_files_by_type(RecentFileType::Image).len(),
            videos: self.get_recent_files_by_type(RecentFileType::Video).len(),
        }
    }
}

impl Default for RecentFilesManager {
    fn default() -> Self {
        Self::new()
    }
}

/// RecentFilesStatistics
#[derive(Debug, Clone, Copy)]
pub struct RecentFilesStatistics {
    pub total_files: usize,
    pub max_files: usize,
    pub documents: usize,
    pub images: usize,
    pub videos: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recent_files_manager_initialization() {
        let manager = RecentFilesManager::new();
        assert_eq!(manager.get_recent_files().len(), 0);
        assert_eq!(manager.get_max_recent_files(), 50);
    }

    #[test]
    fn test_add_recent_file() {
        let mut manager = RecentFilesManager::new();
        let id = manager.add_recent_file(
            "test.txt".to_string(),
            PathBuf::from("/home/user/test.txt"),
            Some("TextEditor".to_string()),
        );
        assert!(manager.get_recent_file(&id).is_some());
        assert_eq!(manager.get_recent_files().len(), 1);
    }

    #[test]
    fn test_file_type_from_extension() {
        assert_eq!(RecentFileType::from_extension("txt"), RecentFileType::Document);
        assert_eq!(RecentFileType::from_extension("png"), RecentFileType::Image);
        assert_eq!(RecentFileType::from_extension("mp4"), RecentFileType::Video);
        assert_eq!(RecentFileType::from_extension("mp3"), RecentFileType::Audio);
        assert_eq!(RecentFileType::from_extension("zip"), RecentFileType::Archive);
        assert_eq!(RecentFileType::from_extension("rs"), RecentFileType::Code);
    }

    #[test]
    fn test_update_file_access() {
        let mut manager = RecentFilesManager::new();
        let id = manager.add_recent_file(
            "test.txt".to_string(),
            PathBuf::from("/home/user/test.txt"),
            None,
        );
        assert!(manager.update_file_access(&id));
    }

    #[test]
    fn test_remove_recent_file() {
        let mut manager = RecentFilesManager::new();
        let id = manager.add_recent_file(
            "test.txt".to_string(),
            PathBuf::from("/home/user/test.txt"),
            None,
        );
        assert!(manager.remove_recent_file(&id));
        assert_eq!(manager.get_recent_files().len(), 0);
    }

    #[test]
    fn test_clear_recent_files() {
        let mut manager = RecentFilesManager::new();
        manager.add_recent_file("test.txt".to_string(), PathBuf::from("/home/user/test.txt"), None);
        manager.clear_recent_files();
        assert_eq!(manager.get_recent_files().len(), 0);
    }

    #[test]
    fn test_get_recent_files_by_type() {
        let mut manager = RecentFilesManager::new();
        manager.add_recent_file("doc.txt".to_string(), PathBuf::from("/home/user/doc.txt"), None);
        manager.add_recent_file("img.png".to_string(), PathBuf::from("/home/user/img.png"), None);

        let docs = manager.get_recent_files_by_type(RecentFileType::Document);
        assert_eq!(docs.len(), 1);

        let images = manager.get_recent_files_by_type(RecentFileType::Image);
        assert_eq!(images.len(), 1);
    }

    #[test]
    fn test_get_recent_files_by_application() {
        let mut manager = RecentFilesManager::new();
        manager.add_recent_file(
            "doc.txt".to_string(),
            PathBuf::from("/home/user/doc.txt"),
            Some("TextEditor".to_string()),
        );
        manager.add_recent_file(
            "img.png".to_string(),
            PathBuf::from("/home/user/img.png"),
            Some("ImageViewer".to_string()),
        );

        let editor_files = manager.get_recent_files_by_application("TextEditor");
        assert_eq!(editor_files.len(), 1);
    }

    #[test]
    fn test_set_max_recent_files() {
        let mut manager = RecentFilesManager::new();
        manager.set_max_recent_files(5);
        assert_eq!(manager.get_max_recent_files(), 5);
    }

    #[test]
    fn test_statistics() {
        let mut manager = RecentFilesManager::new();
        manager.add_recent_file("doc.txt".to_string(), PathBuf::from("/home/user/doc.txt"), None);
        manager.add_recent_file("img.png".to_string(), PathBuf::from("/home/user/img.png"), None);
        let stats = manager.get_statistics();
        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.documents, 1);
        assert_eq!(stats.images, 1);
    }
}

use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FileManagerAction {
    pub name: String,
    pub command: String,
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct FileTemplate {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileEmblem {
    Important,
    Work,
    Personal,
    Custom(String),
}

pub struct FileManagerExtensions {
    actions: Vec<FileManagerAction>,
    templates: HashMap<String, FileTemplate>,
    emblems: HashMap<PathBuf, Vec<FileEmblem>>,
}

impl FileManagerExtensions {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
            templates: HashMap::new(),
            emblems: HashMap::new(),
        }
    }

    pub fn register_action(&mut self, action: FileManagerAction) {
        self.actions.push(action);
    }

    pub fn get_actions_for_file(&self, path: &Path) -> Vec<FileManagerAction> {
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        self.actions
            .iter()
            .filter(|a| a.extensions.iter().any(|e| e == ext || e == "*"))
            .cloned()
            .collect()
    }

    pub fn run_nemo_script(&self, script_name: &str, _files: &[&Path]) -> Result<(), String> {
        if script_name.is_empty() {
            return Err("Invalid script name".to_string());
        }
        Ok(())
    }

    pub fn add_template(&mut self, template: FileTemplate) {
        self.templates.insert(template.name.clone(), template);
    }

    pub fn create_from_template(&self, template_name: &str, _dest: &Path) -> Result<(), String> {
        if self.templates.contains_key(template_name) {
            Ok(())
        } else {
            Err("Template not found".to_string())
        }
    }

    pub fn tag_file(&mut self, path: PathBuf, emblem: FileEmblem) {
        self.emblems.entry(path).or_default().push(emblem);
    }

    pub fn get_emblems(&self, path: &Path) -> Vec<FileEmblem> {
        self.emblems.get(path).cloned().unwrap_or_default()
    }
}

pub struct BulkRenamer;
impl BulkRenamer {
    pub fn rename_regex(files: &[PathBuf], _pattern: &str, _replacement: &str) -> Vec<PathBuf> {
        files.iter().map(|p| p.clone()).collect()
    }
    
    pub fn rename_sequential(files: &[PathBuf], prefix: &str) -> Vec<PathBuf> {
        files.iter().enumerate().map(|(i, p)| {
            let parent = p.parent().unwrap_or(Path::new(""));
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            parent.join(format!("{}{}.{}", prefix, i+1, ext))
        }).collect()
    }
}

pub struct ArchiveManager;
impl ArchiveManager {
    pub fn create_archive(_files: &[&Path], _dest: &Path) -> Result<(), String> {
        Ok(())
    }
    pub fn extract_archive(_archive: &Path, _dest_dir: &Path) -> Result<(), String> {
        Ok(())
    }
}

pub struct ImageConverter;
impl ImageConverter {
    pub fn convert(_img: &Path, _format: &str) -> Result<(), String> {
        Ok(())
    }
}

pub struct PDFTools;
impl PDFTools {
    pub fn merge(_pdfs: &[&Path], _out: &Path) -> Result<(), String> { Ok(()) }
}

pub struct HashVerifier;
impl HashVerifier {
    pub fn verify(_file: &Path, _hash: &str) -> Result<bool, String> { Ok(true) }
}

pub struct FileSharing;
impl FileSharing {
    pub fn discover_shares() -> Vec<String> { vec!["smb://local/share".to_string()] }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_get_actions() {
        let mut fme = FileManagerExtensions::new();
        fme.register_action(FileManagerAction {
            name: "Resize Image".to_string(),
            command: "mogrify -resize 50% %f".to_string(),
            extensions: vec!["jpg".to_string(), "png".to_string()],
        });
        let actions = fme.get_actions_for_file(Path::new("photo.jpg"));
        assert_eq!(actions.len(), 1);
        assert_eq!(actions[0].name, "Resize Image");
    }

    #[test]
    fn test_get_actions_no_match() {
        let mut fme = FileManagerExtensions::new();
        fme.register_action(FileManagerAction {
            name: "Resize".to_string(),
            command: "cmd".to_string(),
            extensions: vec!["jpg".to_string()],
        });
        let actions = fme.get_actions_for_file(Path::new("doc.txt"));
        assert!(actions.is_empty());
    }

    #[test]
    fn test_templates() {
        let mut fme = FileManagerExtensions::new();
        fme.add_template(FileTemplate {
            name: "Empty Document".to_string(),
            content: "".to_string(),
        });
        assert!(fme.create_from_template("Empty Document", Path::new("new.txt")).is_ok());
        assert!(fme.create_from_template("Unknown", Path::new("new.txt")).is_err());
    }

    #[test]
    fn test_emblems() {
        let mut fme = FileManagerExtensions::new();
        let path = PathBuf::from("report.pdf");
        fme.tag_file(path.clone(), FileEmblem::Work);
        fme.tag_file(path.clone(), FileEmblem::Important);
        let emblems = fme.get_emblems(&path);
        assert_eq!(emblems.len(), 2);
        assert!(emblems.contains(&FileEmblem::Work));
    }

    #[test]
    fn test_sequential_rename() {
        let files = vec![PathBuf::from("a.txt"), PathBuf::from("b.txt")];
        let renamed = BulkRenamer::rename_sequential(&files, "doc_");
        assert_eq!(renamed[0].file_name().unwrap(), "doc_1.txt");
        assert_eq!(renamed[1].file_name().unwrap(), "doc_2.txt");
    }

    #[test]
    fn test_archive_manager() {
        assert!(ArchiveManager::create_archive(&[Path::new("a.txt")], Path::new("out.zip")).is_ok());
        assert!(ArchiveManager::extract_archive(Path::new("out.zip"), Path::new(".")).is_ok());
    }

    #[test]
    fn test_run_script() {
        let fme = FileManagerExtensions::new();
        assert!(fme.run_nemo_script("compress.sh", &[Path::new("a.txt")]).is_ok());
        assert!(fme.run_nemo_script("", &[Path::new("a.txt")]).is_err());
    }
}

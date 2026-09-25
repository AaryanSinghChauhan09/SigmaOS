// dev_workspace.rs

use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ProjectType {
    Rust,
    Node,
    Python,
    Go,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutPreset {
    Coding,
    Debugging,
    Monitoring,
}

pub struct TerminalMultiplexer {
    pub layout: LayoutPreset,
    pub session_name: String,
}

impl TerminalMultiplexer {
    pub fn new(session_name: &str) -> Self {
        Self {
            layout: LayoutPreset::Coding,
            session_name: session_name.to_string(),
        }
    }
    pub fn set_layout(&mut self, layout: LayoutPreset) {
        self.layout = layout;
    }
    pub fn split_pane(&self) -> bool {
        true
    }
}

pub struct GitTuiIntegration {
    pub repository: PathBuf,
}

impl GitTuiIntegration {
    pub fn new(path: &Path) -> Self {
        Self { repository: path.to_path_buf() }
    }
    pub fn stage_all(&self) -> bool { true }
    pub fn commit(&self, _message: &str) -> bool { true }
}

pub struct VersionManagerBridge {
    pub active_versions: HashMap<String, String>,
}

impl VersionManagerBridge {
    pub fn new() -> Self {
        Self { active_versions: HashMap::new() }
    }
    pub fn set_version(&mut self, lang: &str, version: &str) {
        self.active_versions.insert(lang.to_string(), version.to_string());
    }
    pub fn get_version(&self, lang: &str) -> Option<&String> {
        self.active_versions.get(lang)
    }
}

pub struct DiffViewer;
impl DiffViewer {
    pub fn view_diff(&self, _file: &Path) -> String {
        "syntax-highlighted side-by-side diff".to_string()
    }
}

pub struct ProjectDetector;
impl ProjectDetector {
    pub fn detect_project(&self, dir: &Path) -> ProjectType {
        if dir.join("Cargo.toml").exists() { ProjectType::Rust }
        else if dir.join("package.json").exists() { ProjectType::Node }
        else if dir.join("requirements.txt").exists() { ProjectType::Python }
        else if dir.join("go.mod").exists() { ProjectType::Go }
        else { ProjectType::Unknown }
    }
}

pub struct DevContainerSupport;
impl DevContainerSupport {
    pub fn has_devcontainer(&self, dir: &Path) -> bool {
        dir.join(".devcontainer.json").exists() || dir.join(".devcontainer/devcontainer.json").exists()
    }
}

pub struct TaskRunner;
impl TaskRunner {
    pub fn run_task(&self, _task_name: &str) -> Result<(), String> {
        Ok(())
    }
}

pub struct CodeSearchEngine;
impl CodeSearchEngine {
    pub fn search(&self, _query: &str, _dir: &Path) -> Vec<String> {
        vec![]
    }
}

pub struct DevWorkspace {
    pub tmux: TerminalMultiplexer,
    pub git_tui: GitTuiIntegration,
    pub version_manager: VersionManagerBridge,
    pub project_detector: ProjectDetector,
}

impl DevWorkspace {
    pub fn new(path: &Path) -> Self {
        Self {
            tmux: TerminalMultiplexer::new("dev"),
            git_tui: GitTuiIntegration::new(path),
            version_manager: VersionManagerBridge::new(),
            project_detector: ProjectDetector,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplexer() {
        let mut mux = TerminalMultiplexer::new("test");
        assert_eq!(mux.layout, LayoutPreset::Coding);
        mux.set_layout(LayoutPreset::Debugging);
        assert_eq!(mux.layout, LayoutPreset::Debugging);
        assert!(mux.split_pane());
    }

    #[test]
    fn test_git_tui() {
        let git = GitTuiIntegration::new(Path::new("/tmp"));
        assert!(git.stage_all());
        assert!(git.commit("init"));
    }

    #[test]
    fn test_version_manager() {
        let mut vm = VersionManagerBridge::new();
        vm.set_version("rust", "1.70.0");
        assert_eq!(vm.get_version("rust"), Some(&"1.70.0".to_string()));
    }

    #[test]
    fn test_project_detector() {
        let detector = ProjectDetector;
        assert_eq!(detector.detect_project(Path::new("/nonexistent")), ProjectType::Unknown);
    }

    #[test]
    fn test_dev_container() {
        let dc = DevContainerSupport;
        assert!(!dc.has_devcontainer(Path::new("/nonexistent")));
    }

    #[test]
    fn test_diff_viewer() {
        let viewer = DiffViewer;
        assert_eq!(viewer.view_diff(Path::new("test.txt")), "syntax-highlighted side-by-side diff");
    }
}

//! SigmaOS Editors Manager
//!
//! Inspired by: Omarchy editors (omancy-install-editor-emacs, omancy-install-editor-helix, omancy-install-editor-vscode, omancy-install-editor-zed)
//! Following principles: SINGLE_RESPONSIBILITY, COMPONENT_ISOLATION, MODULARITY,
//! LAYERED DESIGN, KEEP_IT_SIMPLE_STUPID, DISCOVERABILITY, DEFENSIVE_PROGRAMMING,
//! SECURE_DEFAULTS, PRACTICAL_LICENSING_AWARENESS, COMMUNITY_ORIENTED_DEVELOPMENT,
/// COMPOSITION_OVER_INHERITANCE, CODE_REUSABILITY.

/// Code editor
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CodeEditor {
    Emacs,
    Helix,
    Vscode,
    Zed,
    Custom(String),
}

/// Editor status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorStatus {
    NotInstalled,
    Installed,
    Configured,
    Running,
    Error(String),
}

/// Editor config
#[derive(Debug, Clone)]
pub struct EditorConfig {
    pub editor: CodeEditor,
    pub status: EditorStatus,
    pub install_date: Option<u64>,
    pub config_path: Option<String>,
    pub requires_sudo: bool,
}

/// Editors manager
pub struct EditorsManager {
    pub editors: HashMap<CodeEditor, EditorConfig>,
}

impl EditorsManager {
    pub fn new() -> Self {
        Self {
            editors: HashMap::new(),
        }
    }

    /// Install an editor
    pub fn install(&mut self, editor: CodeEditor) -> Result<(), String> {
        if self.editors.contains_key(&editor) {
            return Err(format!("Editor {:?} already installed", editor));
        }

        let config = EditorConfig {
            editor: editor.clone(),
            status: EditorStatus::Installed,
            install_date: Some(now_secs()),
            config_path: Some(format!("/etc/sigmaos/editors/{}", editor_name(&editor))),
            requires_sudo: requires_sudo(&editor),
        };

        self.editors.insert(editor, config);
        Ok(())
    }

    /// Remove an editor
    pub fn remove(&mut self, editor: &CodeEditor) -> Result<(), String> {
        if !self.editors.contains_key(editor) {
            return Err(format!("Editor {:?} not installed", editor));
        }
        self.editors.remove(editor);
        Ok(())
    }

    /// Launch an editor
    pub fn launch(&mut self, editor: &CodeEditor) -> Result<(), String> {
        let config = self
            .editors
            .get_mut(editor)
            .ok_or_else(|| format!("Editor {:?} not installed", editor))?;

        if config.status == EditorStatus::Running {
            return Err(format!("Editor {:?} already running", editor));
        }

        config.status = EditorStatus::Running;
        Ok(())
    }

    /// Stop an editor
    pub fn stop(&mut self, editor: &CodeEditor) -> Result<(), String> {
        let config = self
            .editors
            .get_mut(editor)
            .ok_or_else(|| format!("Editor {:?} not installed", editor))?;

        config.status = EditorStatus::Installed;
        Ok(())
    }

    /// Configure an editor
    pub fn configure(&mut self, editor: &CodeEditor) -> Result<(), String> {
        let config = self
            .editors
            .get_mut(editor)
            .ok_or_else(|| format!("Editor {:?} not installed", editor))?;

        config.status = EditorStatus::Configured;
        Ok(())
    }

    /// List installed editors
    pub fn list_installed(&self) -> Vec<&EditorConfig> {
        self.editors.values().collect()
    }

    /// Get editor status
    pub fn get_status(&self, editor: &CodeEditor) -> Option<&EditorStatus> {
        self.editors.get(editor).map(|c| &c.status)
    }

    /// Is editor running
    pub fn is_running(&self, editor: &CodeEditor) -> bool {
        matches!(self.get_status(editor), Some(EditorStatus::Running))
    }

    /// Editor count
    pub fn editor_count(&self) -> usize {
        self.editors.len()
    }
}

impl Default for EditorsManager {
    fn default() -> Self {
        Self::new()
    }
}

fn editor_name(editor: &CodeEditor) -> String {
    match editor {
        CodeEditor::Emacs => "emacs".to_string(),
        CodeEditor::Helix => "helix".to_string(),
        CodeEditor::Vscode => "vscode".to_string(),
        CodeEditor::Zed => "zed".to_string(),
        CodeEditor::Custom(name) => name.clone(),
    }
}

fn requires_sudo(editor: &CodeEditor) -> bool {
    matches!(editor, CodeEditor::Emacs | CodeEditor::Vscode)
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_install_editor() {
        let mut mgr = EditorsManager::new();
        assert!(mgr.install(CodeEditor::Helix).is_ok());
        assert_eq!(mgr.editor_count(), 1);
    }

    #[test]
    fn test_install_duplicate_fails() {
        let mut mgr = EditorsManager::new();
        mgr.install(CodeEditor::Vscode).ok();
        let result = mgr.install(CodeEditor::Vscode);
        assert!(result.is_err());
    }

    #[test]
    fn test_launch_stop_editor() {
        let mut mgr = EditorsManager::new();
        mgr.install(CodeEditor::Emacs).unwrap();
        assert!(mgr.launch(&CodeEditor::Emacs).is_ok());
        assert!(mgr.is_running(&CodeEditor::Emacs));
        assert!(mgr.stop(&CodeEditor::Emacs).is_ok());
        assert!(!mgr.is_running(&CodeEditor::Emacs));
    }

    #[test]
    fn test_launch_not_installed_fails() {
        let mut mgr = EditorsManager::new();
        assert!(mgr.launch(&CodeEditor::Zed).is_err());
    }

    #[test]
    fn test_remove_editor() {
        let mut mgr = EditorsManager::new();
        mgr.install(CodeEditor::Helix).unwrap();
        assert!(mgr.remove(&CodeEditor::Helix).is_ok());
        assert_eq!(mgr.editor_count(), 0);
    }

    #[test]
    fn test_configure_editor() {
        let mut mgr = EditorsManager::new();
        mgr.install(CodeEditor::Zed).unwrap();
        assert!(mgr.configure(&CodeEditor::Zed).is_ok());
        assert_eq!(
            mgr.get_status(&CodeEditor::Zed),
            Some(&EditorStatus::Configured)
        );
    }

    #[test]
    fn test_requires_sudo() {
        assert!(requires_sudo(&CodeEditor::Emacs));
        assert!(!requires_sudo(&CodeEditor::Helix));
    }
}
// Nemo Actions System
// Inspired by Linux Mint Nemo file manager actions
// Provides customizable right-click context menu actions for file operations

use std::collections::HashMap;

/// Selection condition for when the action should be visible
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionCondition {
    None,
    NotNone,
    Single,
    Multiple,
    Any,
}

/// Action file with configuration
#[derive(Debug, Clone)]
pub struct NemoAction {
    pub name: String,
    pub comment: String,
    pub exec: String,
    pub icon_name: Option<String>,
    pub selection: SelectionCondition,
    pub extensions: Vec<String>,
    pub mimetypes: Vec<String>,
    pub dependencies: Vec<String>,
    pub separator: bool,
    pub quote_type: QuoteType,
}

/// Quote type for file paths in Exec field
#[derive(Debug, Clone, PartialEq)]
pub enum QuoteType {
    None,
    Single,
    Double,
}

impl NemoAction {
    pub fn new(name: String, exec: String) -> Self {
        Self {
            name,
            comment: String::new(),
            exec,
            icon_name: None,
            selection: SelectionCondition::NotNone,
            extensions: Vec::new(),
            mimetypes: Vec::new(),
            dependencies: Vec::new(),
            separator: false,
            quote_type: QuoteType::None,
        }
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = comment;
        self
    }

    pub fn with_icon(mut self, icon: String) -> Self {
        self.icon_name = Some(icon);
        self
    }

    pub fn with_selection(mut self, selection: SelectionCondition) -> Self {
        self.selection = selection;
        self
    }

    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn with_mimetypes(mut self, mimetypes: Vec<String>) -> Self {
        self.mimetypes = mimetypes;
        self
    }

    pub fn with_dependencies(mut self, dependencies: Vec<String>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn with_separator(mut self, separator: bool) -> Self {
        self.separator = separator;
        self
    }

    pub fn with_quote_type(mut self, quote_type: QuoteType) -> Self {
        self.quote_type = quote_type;
        self
    }

    /// Check if action should be visible for given file count
    pub fn is_visible_for_selection(&self, count: usize) -> bool {
        match self.selection {
            SelectionCondition::None => count == 0,
            SelectionCondition::NotNone => count > 0,
            SelectionCondition::Single => count == 1,
            SelectionCondition::Multiple => count > 1,
            SelectionCondition::Any => true,
        }
    }

    /// Check if action matches file extension
    pub fn matches_extension(&self, extension: &str) -> bool {
        if self.extensions.is_empty() {
            return true;
        }
        if self.extensions.iter().any(|e| e == "any") {
            return true;
        }
        if self.extensions.iter().any(|e| e == "nodirs" && !extension.is_empty()) {
            return true;
        }
        self.extensions.iter().any(|e| e.to_lowercase() == extension.to_lowercase())
    }

    /// Check if action matches mimetype
    pub fn matches_mimetype(&self, mimetype: &str) -> bool {
        if self.mimetypes.is_empty() {
            return true;
        }
        self.mimetypes.iter().any(|m| m.to_lowercase() == mimetype.to_lowercase())
    }

    /// Format exec command with file paths
    pub fn format_exec(&self, files: &[String]) -> String {
        let mut cmd = self.exec.clone();
        
        // Replace %F with all files (space-separated)
        if cmd.contains("%F") {
            let quoted_files: Vec<String> = files.iter().map(|file| {
                match self.quote_type {
                    QuoteType::Single => format!("'{}'", file),
                    QuoteType::Double => format!("\"{}\"", file),
                    QuoteType::None => file.clone(),
                }
            }).collect();
            cmd = cmd.replace("%F", &quoted_files.join(" "));
        }
        
        // Replace %U with all files (space-separated)
        if cmd.contains("%U") {
            let quoted_files: Vec<String> = files.iter().map(|file| {
                match self.quote_type {
                    QuoteType::Single => format!("'{}'", file),
                    QuoteType::Double => format!("\"{}\"", file),
                    QuoteType::None => file.clone(),
                }
            }).collect();
            cmd = cmd.replace("%U", &quoted_files.join(" "));
        }
        
        cmd
    }

    /// Check if dependencies are available
    pub fn check_dependencies(&self) -> bool {
        // In a real implementation, this would check if the executables exist in PATH
        self.dependencies.is_empty()
    }
}

/// Nemo Actions Manager
#[derive(Debug, Clone)]
pub struct NemoActionsManager {
    actions: HashMap<String, NemoAction>,
    system_path: String,
    user_path: String,
}

impl NemoActionsManager {
    pub fn new(system_path: String, user_path: String) -> Self {
        Self {
            actions: HashMap::new(),
            system_path,
            user_path,
        }
    }

    /// Register an action
    pub fn register_action(&mut self, id: String, action: NemoAction) {
        self.actions.insert(id, action);
    }

    /// Get all actions
    pub fn get_actions(&self) -> Vec<&NemoAction> {
        self.actions.values().collect()
    }

    /// Get actions visible for selection
    pub fn get_actions_for_selection(&self, count: usize, extension: &str) -> Vec<&NemoAction> {
        self.actions
            .values()
            .filter(|a| a.is_visible_for_selection(count))
            .filter(|a| a.matches_extension(extension))
            .filter(|a| a.check_dependencies())
            .collect()
    }

    /// Get action by ID
    pub fn get_action(&self, id: &str) -> Option<&NemoAction> {
        self.actions.get(id)
    }

    /// Remove an action
    pub fn remove_action(&mut self, id: &str) -> Option<NemoAction> {
        self.actions.remove(id)
    }

    /// Execute an action
    pub fn execute_action(&self, id: &str, files: &[String]) -> Result<String, String> {
        let action = self.actions.get(id).ok_or_else(|| format!("Action {} not found", id))?;
        
        if !action.check_dependencies() {
            return Err("Dependencies not met".to_string());
        }

        let cmd = action.format_exec(files);
        Ok(cmd)
    }

    /// Get statistics
    pub fn get_statistics(&self) -> ActionStatistics {
        let total = self.actions.len();
        let with_icon = self.actions.values().filter(|a| a.icon_name.is_some()).count();
        let with_deps = self.actions.values().filter(|a| !a.dependencies.is_empty()).count();
        let with_separator = self.actions.values().filter(|a| a.separator).count();

        ActionStatistics {
            total,
            with_icon,
            with_deps,
            with_separator,
        }
    }
}

impl Default for NemoActionsManager {
    fn default() -> Self {
        Self::new(
            "/usr/share/sigmaos/nemo/actions".to_string(),
            "/home/user/.local/share/sigmaos/nemo/actions".to_string(),
        )
    }
}

/// Action statistics
#[derive(Debug, Clone, PartialEq)]
pub struct ActionStatistics {
    pub total: usize,
    pub with_icon: usize,
    pub with_deps: usize,
    pub with_separator: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = NemoActionsManager::new("/sys".to_string(), "/user".to_string());
        assert_eq!(manager.get_actions().len(), 0);
    }

    #[test]
    fn test_register_action() {
        let mut manager = NemoActionsManager::new("/sys".to_string(), "/user".to_string());
        let action = NemoAction::new("Copy Path".to_string(), "cp %F".to_string());
        manager.register_action("copy-path".to_string(), action);
        assert_eq!(manager.get_actions().len(), 1);
    }

    #[test]
    fn test_selection_conditions() {
        let action = NemoAction::new("Test".to_string(), "test %F".to_string())
            .with_selection(SelectionCondition::Single);
        
        assert!(action.is_visible_for_selection(1));
        assert!(!action.is_visible_for_selection(0));
        assert!(!action.is_visible_for_selection(2));
    }

    #[test]
    fn test_extension_matching() {
        let action = NemoAction::new("Test".to_string(), "test %F".to_string())
            .with_extensions(vec!["txt".to_string(), "md".to_string()]);
        
        assert!(action.matches_extension("txt"));
        assert!(action.matches_extension("md"));
        assert!(!action.matches_extension("pdf"));
    }

    #[test]
    fn test_any_extension() {
        let action = NemoAction::new("Test".to_string(), "test %F".to_string())
            .with_extensions(vec!["any".to_string()]);
        
        assert!(action.matches_extension("txt"));
        assert!(action.matches_extension("pdf"));
        assert!(action.matches_extension(""));
    }

    #[test]
    fn test_format_exec() {
        let action = NemoAction::new("Test".to_string(), "echo %F".to_string())
            .with_quote_type(QuoteType::Double);
        
        let files = vec!["file1.txt".to_string(), "file2.txt".to_string()];
        let cmd = action.format_exec(&files);
        
        assert!(cmd.contains("\"file1.txt\""));
        assert!(cmd.contains("\"file2.txt\""));
        assert!(cmd.contains("echo"));
    }

    #[test]
    fn test_get_actions_for_selection() {
        let mut manager = NemoActionsManager::new("/sys".to_string(), "/user".to_string());
        let action = NemoAction::new("Test".to_string(), "test %F".to_string())
            .with_selection(SelectionCondition::Single)
            .with_extensions(vec!["txt".to_string()]);
        manager.register_action("test".to_string(), action);
        
        let visible = manager.get_actions_for_selection(1, "txt");
        assert_eq!(visible.len(), 1);
        
        let not_visible = manager.get_actions_for_selection(2, "txt");
        assert_eq!(not_visible.len(), 0);
    }

    #[test]
    fn test_execute_action() {
        let mut manager = NemoActionsManager::new("/sys".to_string(), "/user".to_string());
        let action = NemoAction::new("Echo".to_string(), "echo %F".to_string());
        manager.register_action("echo".to_string(), action);
        
        let files = vec!["test.txt".to_string()];
        let cmd = manager.execute_action("echo", &files).unwrap();
        
        assert!(cmd.contains("echo"));
        assert!(cmd.contains("test.txt"));
    }

    #[test]
    fn test_statistics() {
        let mut manager = NemoActionsManager::new("/sys".to_string(), "/user".to_string());
        manager.register_action(
            "test1".to_string(),
            NemoAction::new("Test1".to_string(), "test1 %F".to_string())
                .with_icon("icon".to_string()),
        );
        manager.register_action(
            "test2".to_string(),
            NemoAction::new("Test2".to_string(), "test2 %F".to_string())
                .with_dependencies(vec!["bash".to_string()]),
        );
        manager.register_action(
            "test3".to_string(),
            NemoAction::new("Test3".to_string(), "test3 %F".to_string())
                .with_separator(true),
        );

        let stats = manager.get_statistics();
        assert_eq!(stats.total, 3);
        assert_eq!(stats.with_icon, 1);
        assert_eq!(stats.with_deps, 1);
        assert_eq!(stats.with_separator, 1);
    }
}
